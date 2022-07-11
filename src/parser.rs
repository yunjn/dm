// @Samuel
#![allow(unused)]
use crate::data::*;
use pcap_file::pcapng::{ParsedBlock, PcapNgReader};
use regex::Regex;
use std::fs::File;

pub fn format_data(msg: Vec<&str>) -> Vec<f64> {
    let mut frame = vec![0.0f64; 21];
    frame[20] = 0.02;
    for joint in msg {
        let joint: Vec<_> = joint.split(' ').collect();
        // exclude head joints and other interference information
        if joint.len() == 2 && joint[0].len() != 3 {
            let joint_name = joint[0].to_string();

            if &joint_name[2..3] != "e" && &joint_name[2..3] != "j" {
                println!("Joint information error");
            }
            // remove 'e' and 'j'
            let joint_name = joint_name[..2].to_string() + &joint_name[3..].to_string();

            // exclude toe joints
            if joint_name == "ll7" || joint_name == "rl7" {
                continue;
            }

            let idx: usize = get_joint_idx(&joint_name);
            frame[idx] = joint[1].parse().unwrap_or(0.0);
        }
    }
    frame
}

pub fn parser(file_path: &str) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let file_in = File::open(file_path).expect("Error opening file");
    let pcap_reader = PcapNgReader::new(file_in).unwrap();

    let mut agent_receiving: Vec<String> = Vec::new();
    let mut agent_send: Vec<String> = Vec::new();

    // parsing blocks
    for block in pcap_reader {
        let block = block.unwrap();
        let parsed_block = block.parsed().unwrap();
        if let ParsedBlock::EnhancedPacket(raw) = parsed_block {
            let msg = String::from_utf8_lossy(&raw.data).into_owned();
            if msg.find("(time (now") != None {
                let frame = msg[msg.find("(HJ").unwrap()..].to_string();
                agent_receiving.push(frame);
            } else if msg.find("(he1") != None {
                let frame = msg[msg.find("(he1").unwrap()..msg.find("(syn)").unwrap()].to_string();
                agent_send.push(frame);
            }
        }
    }

    let mut sensor_values = Vec::new();
    // Parse the message received by the agent
    for msg in agent_receiving {
        // Consider use new api: remove_matches()
        let msg = msg.replace("(HJ (n ", "");
        let msg = msg.replace(") (ax", "");
        let msg = msg.replace("))", ")");
        let msg: Vec<_> = msg.split(")").collect();
        sensor_values.push(format_data(msg));
    }

    let mut speeds = Vec::new();
    // Parse the message sent by the agent
    for msg in agent_send {
        let msg = msg.replace(")", "");
        let msg: Vec<_> = msg.split("(").collect();
        speeds.push(format_data(msg));
    }

    println!(
        "robot receiving:[{}]  robot send:[{}]\nnumber of joints: {}",
        sensor_values.len(),
        speeds.len(),
        speeds[0].len()
    );

    (sensor_values, speeds)
}

pub fn format_data_regex(msg: &str, re: &Regex) -> Vec<f64> {
    let mut frame = vec![0.0f64; 21];
    frame[20] = 0.02;

    let msg = re.captures_iter(&msg);
    let mut idx: usize = 0;

    for joint in msg {
        let joint_name = joint.get(1).unwrap().as_str();
        // exclude head joints
        if joint_name.len() == 3 {
            continue;
        }

        // remove 'e' and 'j'
        let joint_name = joint_name[..2].to_string() + &joint_name[3..].to_string();

        // exclude toe joints
        if joint_name == "ll7" || joint_name == "rl7" {
            continue;
        }

        idx = get_joint_idx(&joint_name);

        frame[idx] = joint.get(2).unwrap().as_str().parse().unwrap_or(0.0);
    }
    frame
}

pub fn parser_regex(file_path: &str) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let file_in = File::open(file_path).expect("Error opening file");
    let pcap_reader = PcapNgReader::new(file_in).unwrap();

    let re = Regex::new(r"(?m)([hral]{1,2}[ej][1-7])[^\d+-]{1,6}([+-]?\d*\.?\d*[Ee+-]{0,2}\d+)")
        .unwrap();

    let mut speeds = Vec::new();
    let mut sensor_values = Vec::new();

    // parsing blocks
    for block in pcap_reader {
        let block = block.unwrap();
        let parsed_block = block.parsed().unwrap();
        if let ParsedBlock::EnhancedPacket(raw) = parsed_block {
            let msg = String::from_utf8_lossy(&raw.data).into_owned();
            if msg.find("(time (now") != None {
                sensor_values.push(format_data_regex(&msg, &re));
            } else if msg.find("(he1") != None {
                speeds.push(format_data_regex(&msg, &re));
            }
        }
    }

    println!(
        "robot receiving:[{}]  robot send:[{}]\nnumber of joints: {}",
        sensor_values.len(),
        speeds.len(),
        speeds[0].len()
    );

    (sensor_values, speeds)
}
