// @Samuel
#![allow(unused)]

pub const JOINT_NAMES: [&str; 20] = [
    "la1", "la2", "la3", "la4", "ll1", "ll2", "ll3", "ll4", "ll5", "ll6", "rl1", "rl2", "rl3",
    "rl4", "rl5", "rl6", "ra1", "ra2", "ra3", "ra4",
];

pub fn get_joint_idx(joint_name: &str) -> usize {
    match joint_name as &str {
        "la1" => 0,
        "la2" => 1,
        "la3" => 2,
        "la4" => 3,
        "ll1" => 4,
        "ll2" => 5,
        "ll3" => 6,
        "ll4" => 7,
        "ll5" => 8,
        "ll6" => 9,
        "rl1" => 10,
        "rl2" => 11,
        "rl3" => 12,
        "rl4" => 13,
        "rl5" => 14,
        "rl6" => 15,
        "ra1" => 16,
        "ra2" => 17,
        "ra3" => 18,
        "ra4" => 19,
        "wait" => 20,
        _ => usize::MAX,
    }
}

pub struct MetaData {
    data: f64,
    set_type: u8, // 0 settar | 1 inctar | 2 reset | 3 wait
}

pub struct Target {
    pub data: Vec<Vec<f64>>,
    // joints_name: Vec<String>,
}
