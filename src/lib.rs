extern crate dns_lookup;

use std::net::IpAddr;
use std::io::Error;
use dns_lookup::{lookup_host, lookup_addr};

pub fn check(ip_addr: IpAddr) -> Result<bool, Error> {
    let ip_addr: IpAddr = ip_addr.into();
    let hostname = lookup_addr(&ip_addr)?;

    if !(hostname.ends_with(".google.com") || hostname.ends_with(".googlebot.com")) {
        return Ok(false);
    }

    let ips: Vec<IpAddr> = lookup_host(&hostname)?;
    Ok(ips.contains(&ip_addr))
}

#[cfg(test)]
mod tests {
    use check;

    #[test]
    fn it_works() {
        assert_eq!(check("1.1.1.1".parse().unwrap()).unwrap(), false);
        assert_eq!(check("123.123.123.123".parse().unwrap()).unwrap(), false);
        assert_eq!(check("66.249.66.1".parse().unwrap()).unwrap(), true);
        assert_eq!(check("66.249.90.77".parse().unwrap()).unwrap(), true);
    }
}
