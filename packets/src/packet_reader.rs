use std::net::IpAddr;
use std::process;

use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;

pub fn read(interface_name: &String, is_hex_dump: bool) {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == *interface_name)
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
                    handle_ethernet_frame(&interface, &packet, is_hex_dump);
                }
            }
            Err(e) => {
                eprintln!("An error occured while reading from the channel: {}", e);
            }
        }
    }
}

fn handle_ethernet_frame(
    interface: &NetworkInterface,
    packet: &EthernetPacket,
    is_hex_dump: bool,
) {
    let interface_name = &interface.name[..];
    match packet.get_ethertype() {
        EtherTypes::Ipv4 => handle_ipv4_packet(interface_name, packet, is_hex_dump),
        EtherTypes::Ipv6 => handle_ipv6_packet(interface_name, packet, is_hex_dump),
        // EtherTypes::Arp => handle_arp_packet(interface_name, packet),
        _ => println!(
            "[{}]: Unknown packet: {} > {}; ethertype: {:?} length: {}",
            interface_name,
            packet.get_source(),
            packet.get_destination(),
            packet.get_ethertype(),
            packet.packet().len()
        ),
    }
}

fn handle_udp_packet(
    interface_name: &str,
    source: IpAddr,
    destination: IpAddr,
    packet: &[u8],
    is_hex_dump: bool,
) {
    let udp = UdpPacket::new(packet);

    if let Some(udp) = udp {
        let mut message = format!("[{}]: UDP Packet: ", interface_name,);

        message.push_str(&format!(
            "{} {} > {}; length: {}",
            message,
            source,
            destination,
            packet.len()
        ));
        if is_hex_dump {
            message.push_str(&format!("{}\n{}", message, map_hex_dump(udp.payload())));
        }
        println!("{}", message);
    } else {
        println!("[{}]: Malformed UDP Packet", interface_name);
    }
}

fn handle_tcp_packet(
    interface_name: &str,
    source: IpAddr,
    destination: IpAddr,
    packet: &[u8],
    is_hex_dump: bool,
) {
    let tcp = TcpPacket::new(packet);
    if let Some(tcp) = tcp {
        let mut message = format!("[{}]: UDP Packet: ", interface_name,);

        message.push_str(&format!(
            "{} {} > {}; length: {}",
            message,
            source,
            destination,
            packet.len()
        ));
        if is_hex_dump {
            message.push_str(&format!("{}\n{}", message, map_hex_dump(tcp.payload())));
        }
        println!("{}", message);
    } else {
        println!("[{}]: Malformed TCP Packet", interface_name);
    }
}

fn handle_transport_protocol(
    interface_name: &str,
    source: IpAddr,
    destination: IpAddr,
    protocol: IpNextHeaderProtocol,
    packet: &[u8],
    is_hex_dump: bool,
) {
    match protocol {
        IpNextHeaderProtocols::Udp => {
            handle_udp_packet(interface_name, source, destination, packet, is_hex_dump)
        }
        IpNextHeaderProtocols::Tcp => {
            handle_tcp_packet(interface_name, source, destination, packet, is_hex_dump)
        }
        // IpNextHeaderProtocols::Icmp => {
        //     handle_icmp_packet(interface_name, source, destination, packet)
        // }
        // IpNextHeaderProtocols::Icmpv6 => {
        //     handle_icmpv6_packet(interface_name, source, destination, packet)
        // }
        _ => println!(
            "[{}]: Unknown {} packet: {} > {}; protocol: {:?} length: {}",
            interface_name,
            match source {
                IpAddr::V4(..) => "IPv4",
                _ => "IPv6",
            },
            source,
            destination,
            protocol,
            packet.len()
        ),
    }
}

fn handle_ipv4_packet(interface_name: &str, ethernet: &EthernetPacket, is_hex_dump: bool) {
    let header = Ipv4Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(
            interface_name,
            IpAddr::V4(header.get_source()),
            IpAddr::V4(header.get_destination()),
            header.get_next_level_protocol(),
            header.payload(),
            is_hex_dump,
        );
    } else {
        println!("[{}]: Malformed IPv4 Packet", interface_name);
    }
}

fn handle_ipv6_packet(interface_name: &str, ethernet: &EthernetPacket, is_hex_dump: bool) {
    let header = Ipv6Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(
            interface_name,
            IpAddr::V6(header.get_source()),
            IpAddr::V6(header.get_destination()),
            header.get_next_header(),
            header.payload(),
            is_hex_dump,
        );
    } else {
        println!("[{}]: Malformed IPv6 Packet", interface_name);
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
