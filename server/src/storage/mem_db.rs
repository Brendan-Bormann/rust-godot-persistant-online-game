use redis::{Client, Commands, Connection, FromRedisValue, RedisResult, ToRedisArgs};

pub struct MemDB {
    client: Client,
}

impl MemDB {
    pub fn new() -> MemDB {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        MemDB { client }
    }
}

impl MemDB {
    pub fn set<T: ToRedisArgs>(&mut self, pre: &str, key: &str, value: T) -> RedisResult<()> {
        let k = format!("{}:{}", pre, key);
        let mut con: Connection = self.client.get_connection().unwrap();
        redis::cmd("SET").arg(k).arg(value).query(&mut con)
    }

    pub fn get<T: FromRedisValue>(&mut self, pre: &str, key: &str) -> RedisResult<T> {
        let k = format!("{}:{}", pre, key);
        let mut con: Connection = self.client.get_connection().unwrap();
        con.get::<&str, T>(&k)
    }

    pub fn get_raw<T: FromRedisValue>(&mut self, key: &str) -> RedisResult<T> {
        let mut con: Connection = self.client.get_connection().unwrap();
        con.get::<&str, T>(&key)
    }

    pub fn get_all_keys(&mut self, pre: &str) -> Vec<String> {
        let k = format!("{}:*", pre);
        let mut con: Connection = self.client.get_connection().unwrap();
        let collection = con.keys(&k).unwrap();

        collection
    }

    pub fn get_next_id(&mut self, key_name: &str) -> u32 {
        let pre = "next_key";

        match self.get::<u32>(pre, &key_name) {
            Ok(value) => {
                self.set(pre, key_name, value + 1).unwrap();
                value
            }
            Err(_) => {
                self.set(pre, key_name, 1).unwrap();
                1
            }
        }
    }
}
