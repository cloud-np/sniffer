#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EtherType {
    Ipv4,
    Arp,
    Ipv6,
    WakeOnLan,
    Unkown(u16),
}

impl EtherType {

    pub fn to_u16(&self) -> u16 {
        match self {
            EtherType::Ipv4 => 0x0800,
            EtherType::Arp => 0x0806,
            EtherType::Ipv6 => 0x86DD,
            EtherType::WakeOnLan => 0x0842,
            EtherType::Unkown(other) => *other,
        }
    }

    pub fn from_u16(value: u16) -> Self {
        match value {
            0x0800 => EtherType::Ipv4,
            0x0806 => EtherType::Arp,
            0x86DD => EtherType::Ipv6,
            0x0842 => EtherType::WakeOnLan,
            other => EtherType::Unkown(other),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EthernetFrame {
    pub destination_mac: [u8; 6],
    pub source_mac: [u8; 6],
    pub ether_type: u16,
    pub payload: Vec<u8>,
}

impl EthernetFrame {
    pub fn parse(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < 14 {
            return Err("Ethernet frame too short");
        }

        let mut destination_mac = [0u8; 6];
        let mut source_mac = [0u8; 6];

        destination_mac.copy_from_slice(&data[0..6]);
        source_mac.copy_from_slice(&data[6..12]);

        let ether_type = ((data[12] as u16) << 8) | (data[13] as u16);

        let payload = data[14..].to_vec();

        Ok(Self {
            destination_mac,
            source_mac,
            ether_type,
            payload,
        })
    }

    pub fn format_mac(mac: &[u8; 6]) -> String {
        format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
        )
    }
}
