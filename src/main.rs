mod data;
mod parser;
mod target;

use data::*;

fn main() {
    // let target = Target::from_editor("assets/test");
    let target = Target::from_params("assets/out/kick_walk_19m.txt");
    // let mut target = Target::from_pcap("assets/pcap/long.pcapng");
    // target.data = target.data[0..10].to_vec();
    // target.into_skl("test");
    // target.into_editor("test");
    // target.into_skl_txt("test");
    target.print();
}
