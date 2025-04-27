use r2d2::PooledConnection;
use redis::{Client, Commands, RedisResult};

pub struct MemDB {
    pub con: PooledConnection<Client>,
}

impl MemDB {
    pub fn new(con_from_pool: PooledConnection<Client>) -> MemDB {
        MemDB { con: con_from_pool }
    }
}

impl MemDB {
    pub fn get(&mut self, pre: &str, key: &str) -> RedisResult<Option<String>> {
        let k = format!("{}:{}", pre, key);
        self.con.get(&k)
    }

    pub fn get_raw(&mut self, key: &str) -> RedisResult<Option<String>> {
        self.con.get(&key)
    }

    pub fn set(&mut self, pre: &str, key: &str, value: &str) -> RedisResult<()> {
        let k = format!("{}:{}", pre, key);
        self.con.set(k, value)
    }

    pub fn del(&mut self, pre: &str, key: &str) -> RedisResult<()> {
        let k = format!("{}:{}", pre, key);
        self.con.del(k)
    }

    pub fn get_all_keys(&mut self, pre: &str) -> Vec<String> {
        let k = format!("{}:*", pre);
        let collection = self.con.keys(&k).unwrap();

        collection
    }

    pub fn get_next_id(&mut self, key_name: &str) -> String {
        let pre = "next_key";

        match self.get(pre, &key_name) {
            Ok(result) => match result {
                Some(next_id) => {
                    let next_id: u32 = next_id.parse().expect("failed to parse id!");
                    self.set(pre, key_name, &(next_id + 1).to_string()).unwrap();
                    next_id.to_string()
                }
                None => {
                    let next_id: u32 = 1;
                    self.set(pre, key_name, &(next_id + 1).to_string()).unwrap();
                    next_id.to_string()
                }
            },
            Err(e) => {
                panic!("Failed to get next id! {}", e)
            }
        }
    }

    pub fn wipe(&mut self) {
        let _result: String = redis::cmd("FLUSHALL").query(&mut self.con).unwrap();
    }
}
