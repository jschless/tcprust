use std::collections::HashMap;
use std::net::Ipv4Addr;

mod tcp;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() {
    let iface = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).expect("failed to create");
    let mut connections: HashMap<Quad, tcp::TcpState> = Default::default();
    loop {
        let mut buf = [0; 1504];
        let read = iface.recv(&mut buf).unwrap();

        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);

        if eth_proto != 0x0800 {
            // only accept IPv4 headers
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..read]) {
            Ok(ip_hdr) => {
                let src = ip_hdr.source_addr();
                let dst = ip_hdr.destination_addr();
                let proto = ip_hdr.protocol();

                if proto != 0x06 {
                    continue;
                }

                let ip_hdr_sz = ip_hdr.slice().len();
                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + ip_hdr_sz..read]) {
                    Ok(tcp_hdr) => {
                        let datai = 4 + ip_hdr.slice().len() + tcp_hdr.slice().len();
                        connections
                            .entry(Quad {
                                src: (src, tcp_hdr.source_port()),
                                dst: (dst, tcp_hdr.destination_port()),
                            })
                            .or_default()
                            .on_packet(ip_hdr, tcp_hdr, &buf[datai..read]);
                    }
                    Err(e) => {
                        eprintln!("ignoring weird TCP packet {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("ignoring weird packet {:?}", e);
            }
        }
    }
}
