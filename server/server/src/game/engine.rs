use shared::game::player::Player;
use shared::game::vector::Vector2;
use std::sync::mpsc::Receiver;
use tracing::info;

use crate::storage::mem_db::MemDB;

use super::command::{Command, CommandType};

pub struct Engine {
    mem_db: MemDB,
    command_rx: Receiver<Command>,
}

impl Engine {
    pub fn new(mem_db: MemDB, command_rx: Receiver<Command>) -> Self {
        Engine { mem_db, command_rx }
    }
}

impl Engine {
    pub fn create_player(&mut self, username: String) -> Player {
        let new_player = Player::new(self.mem_db.get_next_id("player"), username);
        self.mem_db.upsert_player(new_player).unwrap()
    }

    pub fn handle_commands(&mut self) {
        let commands: Vec<Command> = self.command_rx.try_iter().collect();

        for command in commands {
            self.handle_command(command);
        }
    }

    pub fn set_player_input(&mut self, id: String, input_direction: Vector2, rotation: f32) {
        if let Some(player) = &mut self.mem_db.get_player(&id).expect("failed to find player") {
            player.input_direction = input_direction;
            player.rotation = rotation;
            self.mem_db.upsert_player(player.to_owned()).unwrap();
        }
    }

    pub fn handle_command(&mut self, command: Command) {
        match command.command_type {
            CommandType::Input => {
                let args = command.parse_args();
                let dir = Vector2::from_string(args[0].clone());
                let rot: f32 = args[1].parse().expect("Failed to parse f32 from str");
                self.set_player_input(command.issuing_player_id.expect("No player!"), dir, rot);
                let _ = command.respond_to.send(Ok(()));
            }
            _ => {
                let _ = command.respond_to.send(Err(()));
            }
        }
    }
}
