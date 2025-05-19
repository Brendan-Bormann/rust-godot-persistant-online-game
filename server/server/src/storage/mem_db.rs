use r2d2::PooledConnection;
use redis::{Client, Commands, PubSubCommands};

pub struct MemDB {
    pub con: PooledConnection<Client>,
}

impl MemDB {
    pub fn new(con_from_pool: PooledConnection<Client>) -> MemDB {
        MemDB { con: con_from_pool }
    }
}

impl MemDB {
    pub fn get_next_id(&mut self, key: &str) -> String {
        let key = format!("next_key:{}", key);

        let next_id: u32 = self.con.get(&key).unwrap();
        self.con.set::<String, u32, u32>(key, next_id + 1).unwrap();

        next_id.to_string()
    }

    pub fn get_all_keys(&mut self, pre: &str) -> Vec<String> {
        let k = format!("{}:*", pre);
        let collection = self.con.keys(&k).unwrap();

        collection
    }

    pub fn wipe(&mut self) {
        let _result: String = redis::cmd("FLUSHALL").query(&mut self.con).unwrap();
    }
}
