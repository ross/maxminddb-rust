use ipnetwork::IpNetwork;

use fallible_iterator::FallibleIterator;
use maxminddb::{Within, geoip2};

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let reader = maxminddb::Reader::open_readfile(
        args.next()
            .ok_or("First argument must be the path to the IP database")?,
    )
    .unwrap();
    let cidr: String = args
        .next()
        .ok_or("Second argument must be the IP address and mask in CIDR notation")?
        .parse()
        .unwrap();
    let ip_net = if cidr.contains(":") {
        IpNetwork::V6(cidr.parse().unwrap())
    } else {
        IpNetwork::V4(cidr.parse().unwrap())
    };
    let mut iter: Within<geoip2::City, _> = reader.within(ip_net).map_err(|e| e.to_string())?;
    while let Some(i) = iter.next().map_err(|e| e.to_string())? {
        println!("ip_net={}, info={:#?}", i.ip_net, i.info);
    }
    Ok(())
}
