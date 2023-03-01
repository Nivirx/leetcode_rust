pub fn run(address: String) -> String {
    IpAddress::from(address).to_defanged()
}

struct IpAddress(u8, u8, u8, u8);

impl IpAddress {
    fn to_defanged(&self) -> String {
        format!("{}[.]{}[.]{}[.]{}", 
                self.0, self.1, self.2, self.3)
    }
}

impl From<String> for IpAddress {
    fn from(address: String) -> Self {
        let octets: Vec<&str> = address.split('.').collect();

        IpAddress(
        octets[0].parse::<u8>().unwrap_or(0),
        octets[1].parse::<u8>().unwrap_or(0),
        octets[2].parse::<u8>().unwrap_or(0),
        octets[3].parse::<u8>().unwrap_or(0)
        )       
    }
}