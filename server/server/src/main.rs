mod game;
mod network;
mod storage;

use game::{command::Command, engine::Engine, physics::PhysicsManager};
use network::manager::NetworkManager;
use storage::mem_db::MemDB;

use r2d2;
use std::{
    sync::mpsc::channel,
    thread,
    time::{Duration, Instant},
};
use tracing_subscriber;

// ticks per second
const TICK_RATE: u16 = 20;
const MS_PER_TICK: u16 = 1000 / TICK_RATE;

const PORT: &str = "8080";
const MEMDB_ADDR: &str = "redis://127.0.0.1/";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mem_db = redis::Client::open(MEMDB_ADDR).unwrap();
    let pool = r2d2::Pool::builder().build(mem_db).unwrap();

    let physics_mem_db = MemDB::new(pool.get().unwrap());

    let _physics = thread::spawn(move || {
        let mut physics = PhysicsManager::new(physics_mem_db);

        let fixed_time_step = MS_PER_TICK as f32 / 1000.0;
        let mut previous = Instant::now();
        let mut accumulator = 0.0f32;

        loop {
            let now = Instant::now();
            let frame_time = now.duration_since(previous).as_secs_f32();
            previous = now;

            // Clamp to avoid spiral of death on huge lags
            let frame_time = frame_time.min(0.25);

            accumulator += frame_time;

            while accumulator >= fixed_time_step {
                physics.tick(fixed_time_step);
                accumulator -= fixed_time_step;
            }

            thread::sleep(Duration::from_millis(1));
        }
    });

    let (command_tx, command_rx) = channel::<Command>();
    let engine_mem_db = MemDB::new(pool.get().unwrap());
    let _engine_commands = thread::spawn(move || {
        let mut engine = Engine::new(engine_mem_db, command_rx);
        loop {
            engine.handle_commands();
            thread::sleep(Duration::from_millis(1));
        }
    });

    let network_mem_db = MemDB::new(pool.get().unwrap());
    let mut network = NetworkManager::new(PORT, network_mem_db, command_tx).await;
    let _ = network.start().await;
}
