// @Samuel
#![allow(unused)]
use crate::data::*;
use crate::parser::*;
use std::fs::File;
use std::io::{Read, Write};

fn inv(speed: f64, sensor_value: f64, previous_error: f64) -> f64 {
    (speed + 0.01 * previous_error) / 0.16 + sensor_value
}

impl Target {
    // pub fn from_skl_txt(txt: &str, skl: &str) -> Target {
    //     Target {}
    // }
    // pub fn from_editor(file_path: &str) -> Target {
    //     Target {}
    // }
    pub fn from_pcap(file_path: &str) -> Target {
        let (sensor_values, mut speeds) = parser(file_path);
        let mut cur_err = vec![0.0; 20];
        let mut pre_err = vec![0.0; 20];

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

        Target { data: speeds }
    }

    pub fn print(&self) {
        for frame in &self.data {
            println!("\n{:?}", frame);
        }
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

            let wait = String::from("\nwait ") + &frame[20].to_string() + " end\nENDSTATE\n\n";
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
                let joint_key = String::from(" EFF_") + &joint.to_uppercase() + " ";
                let joint_val =
                    String::from("$") + file_name + "_s" + &frame_idx.to_string() + "_" + &joint;

                let mut txt_joint_val = joint_val.clone();
                txt_joint_val.remove(0); // remove '$'
                let line = txt_joint_val + "\t" + &frame[idx].to_string() + "\n";
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
            let wait = String::from("\nwait ") + &frame[20].to_string() + " end\nENDSTATE\n\n";

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
            let line = (frame[20] * 1000.0).to_string();
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
