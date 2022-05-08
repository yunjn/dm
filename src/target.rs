// @Samuel
#![allow(unused)]
use crate::data::*;
use crate::parser::*;
use std::fs::File;
use std::io::{Read, Write};

#[inline]
fn inv(speed: f64, sensor_value: f64, previous_error: f64) -> f64 {
    (speed + 0.01 * previous_error) / 0.16 + sensor_value
}

impl Target {
    pub fn from_params(file_path: &str) -> Self {
        let mut lines = String::new();
        let mut params_file = File::open(file_path).expect("Error opening params file");
        params_file.read_to_string(&mut lines).unwrap();
        let lines: Vec<_> = lines.split('\n').collect();

        let mut targets: Vec<Vec<f64>> = Vec::new();
        let mut frame = vec![0.0f64; 21];
        frame[20] = 0.02;
        let mut state = "s0";

        for line in lines.iter() {
            if line.starts_with("/") || line.starts_with("#") {
                continue;
            }

            let line: Vec<_> = line.split("\t").collect();
            let key: Vec<_> = line[0].split("_").collect();
            let idx = get_joint_idx(key[key.len() - 1]);

            if idx == usize::MAX {
                continue;
            }

            if key[key.len() - 2] != state {
                state = key[key.len() - 2];
                targets.push(frame.clone());
            }

            frame[idx] = line[1].parse().unwrap_or(0.0);
        }
        targets.push(frame);
        Self { data: targets }
    }

    pub fn from_editor(file_path: &str) -> Self {
        let mut lines = String::new();
        let mut editor_file = File::open(file_path).expect("Error opening editor file");
        editor_file.read_to_string(&mut lines).unwrap();

        let mut targets: Vec<Vec<f64>> = Vec::new();
        let frames: Vec<_> = lines.split('\n').collect();

        for frame_str in frames.iter() {
            let frame_str: Vec<_> = frame_str.split(' ').collect();
            // exclude blank line
            if frame_str.len() <= 2 {
                continue;
            }
            let frame_str = &frame_str[0..frame_str.len() - 2];
            let wait: f64 = frame_str[0].parse().unwrap_or(200.0);
            let frame_str = &frame_str[3..];
            let mut frame: Vec<f64> = Vec::new();

            frame_str
                .iter()
                .for_each(|v| frame.push(v.parse().unwrap_or(0.0)));

            frame.push(wait);
            targets.push(frame);
        }
        Self { data: targets }
    }

    pub fn from_pcap(file_path: &str) -> Self {
        let mut cur_err = vec![0.0; 20];
        let mut pre_err = vec![0.0; 20];

        let (sensor_values, mut speeds) = parser(file_path);

        for i in 0..speeds.len() {
            pre_err = cur_err.clone();
            for j in 0..20 {
                speeds[i][j] = inv(speeds[i][j], sensor_values[i][j], pre_err[j]);
                cur_err[j] = speeds[i][j] - sensor_values[i][j];
            }
        }

        // speeds
        //     .iter()
        //     .zip(sensor_values.iter())
        //     .for_each(|(sp, sv)| {
        //         pre_err
        //             .iter()
        //             .zip(cur_err.iter())
        //             .for_each(|(mut pe, ce)| pe = ce);

        //         sp.iter()
        //             .zip(sv.iter().zip(pre_err.iter().zip(cur_err.iter())))
        //             .for_each(|(mut p, (v, (pe, mut ce)))| {
        //                 let t = &inv(*p, *v, *pe);
        //                 p = t;
        //                 ce = &(t - *v);
        //             });
        //     });

        Self { data: speeds }
    }

    pub fn print(&self) {
        &self.data.iter().for_each(|frame| println!("{:?}", frame));
        println!("frames: {}", self.data.len());
    }

    // For txt type
    pub fn into_skl(&self, file_name: &str) {
        let mut skill = File::create("assets/out/".to_string() + file_name + ".skl")
            .expect("Error creating file");
        skill
            .write("STARTSKILL SKILL_TEST_LEFT_LEG\n\n".as_bytes())
            .expect("Error writing STARTSJILL");

        for frame in &self.data {
            skill
                .write("STARTSTATE\n".as_bytes())
                .expect("Error writing STARTSTATE");

            let mut left_str = String::from("settar");
            let mut right_str = String::from("\nsettar");

            for joint in &JOINT_NAMES {
                let idx = get_joint_idx(joint);
                let joint_key = String::from(" EFF_") + &joint.to_uppercase() + " ";

                if &joint[0..1] == "l" {
                    left_str.push_str(&joint_key);
                    let value = frame[idx].to_string();
                    left_str.push_str(&value);
                } else {
                    right_str.push_str(&joint_key);
                    let value = frame[idx].to_string();
                    right_str.push_str(&value);
                }
            }
            left_str.push_str(" end");
            right_str.push_str(" end");
            skill
                .write(left_str.as_bytes())
                .expect("Error writing left");
            skill
                .write(right_str.as_bytes())
                .expect("Error writing right");

            let wait = String::from("\nwait ")
                + &frame.get(20).unwrap_or(&0.02).to_string()
                + " end\nENDSTATE\n\n";
            skill.write(wait.as_bytes()).expect("Error writing time");
        }
        skill
            .write("ENDSKILL\n\n".as_bytes())
            .expect("Error writing ENDSKILL");
        skill
            .write("REFLECTSKILL SKILL_TEST_LEFT_LEG SKILL_TEST_RIGHT_LEG".as_bytes())
            .expect("Error writing REFLECTSKILL");
    }

    // For skl & txt type
    pub fn into_skl_txt(&self, file_name: &str) {
        let mut txt = File::create("assets/out/".to_string() + file_name + ".txt")
            .expect("Error creating file");
        let mut skl = File::create("assets/out/".to_string() + file_name + ".skl")
            .expect("Error creating file");

        let mut frame_idx = 0i32; // if change this variable to 1 will cause bug !!!

        skl.write(format!("STARTSKILL SKILL_{}_LEFT_LEG\n\n", file_name.to_uppercase()).as_bytes())
            .expect("Error writing STARTSJILL");

        for frame in &self.data {
            skl.write("STARTSTATE\n".as_bytes())
                .expect("Error writing STARTSTATE");

            let mut left_str = String::from("settar");
            let mut right_str = String::from("\nsettar");
            let mut index = 0i32;

            for joint in &JOINT_NAMES {
                let idx = get_joint_idx(joint);
                // .skl
                let joint_key = String::from(" EFF_") + &joint.to_uppercase() + " ";
                let joint_val =
                    String::from("$") + file_name + "_s" + &frame_idx.to_string() + "_" + &joint;

                // .txt
                let mut txt_joint_key = joint_val.clone();
                txt_joint_key.remove(0); // remove '$'
                let line = txt_joint_key + "\t" + &frame[idx].to_string() + "\n";
                txt.write(line.as_bytes()).expect("Error writing params");

                if &joint[0..1] == "l" {
                    let line = joint_key + &joint_val;
                    left_str.push_str(&line);
                } else {
                    let line = joint_key + &joint_val;
                    right_str.push_str(&line);
                }
                index += 1;
            }

            frame_idx += 1;
            let wait = String::from("\nwait ")
                + &(frame.get(20).unwrap_or(&200.0) / 1000.0).to_string()
                + " end\nENDSTATE\n\n";

            left_str.push_str(" end");
            right_str.push_str(" end");
            skl.write(left_str.as_bytes()).expect("Error writing left");
            skl.write(right_str.as_bytes())
                .expect("Error writing right");
            skl.write(wait.as_bytes()).expect("Error writing time");
        }

        skl.write("ENDSKILL\n\n".as_bytes())
            .expect("Error writing ENDSKILL");

        skl.write(
            format!(
                "REFLECTSKILL SKILL_{}_LEFT_LEG SKILL_{}_RIGHT_LEG",
                file_name.to_uppercase(),
                file_name.to_uppercase()
            )
            .as_bytes(),
        )
        .expect("Error writing REFLECTSKILL");
    }

    // For editor type
    pub fn into_editor(&self, file_name: &str) {
        let mut editor = File::create("assets/out/".to_string() + file_name + ".txt")
            .expect("Error creating file");

        for frame in &self.data {
            let line = (frame.get(20).unwrap_or(&0.02) * 1000.0).to_string();
            let mut line = line + " 0 0 ";

            for joint in &JOINT_NAMES {
                let idx = get_joint_idx(joint);
                line = line + &frame[idx].to_string() + " ";
            }

            line = line + "0.0 0.0\n";
            editor.write(line.as_bytes()).expect("Error writing editor");
        }
    }
}
