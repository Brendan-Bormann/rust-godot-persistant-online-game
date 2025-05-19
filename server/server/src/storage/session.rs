// use super::mem_db::MemDB;
// use crate::network::session::Session;
// use redis::Commands;

// impl MemDB {
//     pub fn create_session(&mut self, session: &Session) -> Result<(), ()> {
//         let key = format!("session:{}", session.peer);
//         let encoded = bitcode::encode(session);
//         let _: () = self.con.set(key, encoded).unwrap();
//         Ok(())
//     }

//     pub fn update_session(&mut self, session: &Session) -> Result<(), ()> {
//         let key = format!("session:{}", session.peer);
//         let encoded = bitcode::encode(session);
//         let _: () = self.con.set(key, encoded).unwrap();
//         Ok(())
//     }

//     pub fn find_session(&mut self, addr: &str) -> Result<Option<Session>, ()> {
//         let key = format!("session:{}", addr);
//         let encoded: Vec<u8> = self.con.get(key).unwrap();
//         Ok(Some(bitcode::decode(&encoded).expect("Decoding failed")))
//     }

//     pub fn delete_session(&mut self, session: &Session) -> Result<(), ()> {
//         let key = format!("session:{}", session.peer);
//         let _: () = self.con.del(key).unwrap();
//         Ok(())
//     }

//     pub fn find_all_sessions(&mut self) -> Vec<Session> {
//         let session_keys = self.get_all_keys("session");
//         let mut sessions: Vec<Session> = vec![];

//         for session_key in session_keys {
//             let encoded: Vec<_> = self.con.get(session_key).unwrap();
//             let session = bitcode::decode(&encoded).expect("Decoding failed");
//             sessions.push(session);
//         }

//         sessions
//     }
// }
