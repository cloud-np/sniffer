pub mod packet_reader {
    use std::process;

    use pnet::datalink::Channel::Ethernet;
    use pnet::datalink::{self};
    use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
    use pnet::packet::ip::IpNextHeaderProtocols;
    use pnet::packet::ipv4::Ipv4Packet;
    use pnet::packet::tcp::TcpPacket;
    use pnet::packet::udp::UdpPacket;
    use pnet::packet::Packet;

    pub fn read(interface_name: String) {
        let interfaces = datalink::interfaces();
        let interface = interfaces
            .into_iter()
            .find(|iface| iface.name == interface_name)
            .unwrap_or_else(|| {
                eprintln!("No such interface '{}'", interface_name);
                process::exit(1);
            });

        println!("interface selected: {}", interface);

        let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!(
                "An error occurred when creating the datalink channel: {}",
                e
            ),
        };

        loop {
            match rx.next() {
                Ok(packet) => {
                    if let Some(packet) = EthernetPacket::new(packet) {
                        println!("Source MAC: {}", packet.get_source());
                        println!("Destination MAC: {}", packet.get_destination());

                        match_packet(&packet);
                    }
                }
                Err(e) => {
                    eprintln!("An erro occured while reading from the channel: {}", e);
                }
            }
        }
    }

    fn match_packet(packet: &EthernetPacket<'_>) {
        match packet.get_ethertype() {
            EtherTypes::Ipv4 => {
                if let Some(ipv4) = Ipv4Packet::new(packet.payload()) {
                    println!(
                        "IPv4 Packet: {} -> {}",
                        ipv4.get_source(),
                        ipv4.get_destination()
                    );

                    match ipv4.get_next_level_protocol() {
                        IpNextHeaderProtocols::Tcp => {
                            if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
                                println!(
                                    "TCP Packet: {} -> {}\nPayload:\n{}",
                                    tcp.get_source(),
                                    tcp.get_destination(),
                                    map_hex_dump(tcp.payload())
                                );
                            }
                        }
                        IpNextHeaderProtocols::Udp => {
                            if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                                println!(
                                    "UDP Packet: {} -> {}\nPayload:\n{}",
                                    udp.get_source(),
                                    udp.get_destination(),
                                    map_hex_dump(udp.payload())
                                );
                            }
                        }
                        protocol => println!("Other protocol: {:?}", protocol),
                    }
                }
            }
            ethertype => println!("Other ethertype: {:?}", ethertype),
        }
    }

    fn map_hex_dump(payload: &[u8]) -> String {
        let hex_dump: String = payload
            .chunks(16)
            .map(|chunk| {
                let hex = chunk
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ");

                let ascii = chunk
                    .iter()
                    .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
                    .collect::<String>();

                format!("{:<47} | {}", hex, ascii)
            })
            .collect::<Vec<_>>()
            .join("\n");

        hex_dump
    }
}
