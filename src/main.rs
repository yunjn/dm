mod data;
mod parser;
mod target;

use data::*;

fn main() {
    // parser::parser("assets/pcap/long.pcapng");
    let mut target = Target::from_pcap("assets/pcap/long.pcapng");
    // target.print();
    target.data = target.data[0..10].to_vec();
    // target.into_editor("test");
    target.into_skl_txt("abc");
}
