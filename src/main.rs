use std::path::Path;
use net_trace::*;
fn main() {
    let path = Path::new("./example/pcap/rsasnakeoil2.pcap");
    let res = match parse_pcap(path) {
        Ok(res) => res,
        Err(e) => {
            println!("{:#?}", e);
            return;
        }
    };
    println!("{:#?}", res);
}
