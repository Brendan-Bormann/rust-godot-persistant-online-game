// use std::io;
// use std::io::{Read, Write};
// use std::os::unix::net::UnixStream;

// use super::packet::Packet;

// pub struct PacketUDSStream {
//     pub stream: UnixStream,
// }

// impl PacketUDSStream {
//     pub fn new(stream: UnixStream) -> Self {
//         PacketUDSStream { stream }
//     }

//     pub fn recv_packet(&mut self) -> Result<Packet, io::Error> {
//         let buf = self.read()?;
//         let packet = bitcode::decode::<Packet>(&buf).map_err(|e| {
//             io::Error::new(io::ErrorKind::InvalidData, format!("decode error: {}", e))
//         })?;
//         Ok(packet)
//     }

//     pub fn read(&mut self) -> io::Result<Vec<u8>> {
//         let mut len_buf = [0u8; 4];
//         self.stream.read_exact(&mut len_buf)?;
//         let len = u32::from_be_bytes(len_buf) as usize;

//         let mut buf = vec![0u8; len];
//         self.stream.read_exact(&mut buf)?;
//         Ok(buf)
//     }

//     pub fn send_packet(&mut self, packet: &Packet) -> Result<(), io::Error> {
//         let data = bitcode::encode(packet);
//         self.write(&data)?;
//         Ok(())
//     }

//     pub fn write(&mut self, data: &[u8]) -> io::Result<()> {
//         let len = data.len() as u32;
//         self.stream.write_all(&len.to_be_bytes())?;
//         self.stream.write_all(data)?;
//         Ok(())
//     }
// }
