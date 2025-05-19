use std::{
    io::ErrorKind,
    net::{TcpStream, UdpSocket},
    sync::Arc,
    thread,
    time::Duration,
};

use shared::{
    game::game_state::GameState,
    network::{packet::Packet, packet_tcp::PacketTCP, packet_udp::PacketUDP},
};

fn main() {
    println!("Hello, world!");
    let mut packet_id = 0;

    let mut local_state = GameState::new();

    let tcp = TcpStream::connect("127.0.0.1:8080").unwrap();
    let s = tcp.try_clone().unwrap();
    let addr = tcp.local_addr().unwrap().clone();
    tcp.set_read_timeout(Some(Duration::from_millis(10)))
        .unwrap();

    let mut p_tcp = PacketTCP::new(tcp);

    let udp = UdpSocket::bind(addr).unwrap();
    udp.set_read_timeout(Some(Duration::from_millis(10)))
        .unwrap();
    let mut p_udp = PacketUDP::new(Arc::new(udp));

    packet_id += 1;
    let packet = Packet::new(packet_id.to_string(), 0, 0, None);
    p_tcp.send_packet(&packet).unwrap();

    packet_id += 1;
    let packet = Packet::new(
        packet_id.to_string(),
        2,
        0,
        Some(bitcode::encode::<String>(&"TestPlayer".to_string())),
    );
    p_tcp.send_packet(&packet).unwrap();

    let mut p_tcp2 = PacketTCP::new(s);

    thread::spawn(move || {
        let mut packet_id = 1;

        let packet2 = Packet::new(packet_id.to_string(), 2, 2, None);

        loop {
            let mut new_packet = packet2.clone();
            packet_id += 1;
            new_packet.id = format!("{}", packet_id);
            p_tcp2.send_packet(&new_packet).unwrap();
            println!("send tcp packet with id: {}", packet_id);

            thread::sleep(Duration::from_millis(100));
        }
    });

    loop {
        match p_tcp.recv_packet() {
            Ok(packet) => {
                // println!(
                //     "tcp:{}, id: {}, t: {}, s: {}",
                //     p_tcp.stream.peer_addr().unwrap(),
                //     if packet.id == "" { "_" } else { &packet.id },
                //     packet.packet_type,
                //     packet.packet_subtype
                // );

                if packet.packet_type == 3 {
                    let data = packet.payload.unwrap();
                    let new_state: GameState = bitcode::decode(&data).unwrap();
                    local_state = new_state;

                    println!("local state: {:?}", local_state);
                }
            }
            Err(ref e) if e.kind() == ErrorKind::TimedOut => {}
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {}
            _ => {}
        }

        match p_udp.recv_packet() {
            Ok((_packet, addr)) => println!("got udp packet from {}", addr),
            _ => {}
        }
    }
}
