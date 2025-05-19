use std::{thread, time::Duration};

use shared::game::{game_state::GameState, player::Player, vector::Vector2};
use std::sync::mpsc;
use tokio::sync::watch;
use tracing::info;

use super::command::{Command, CommandResponse};

pub struct GameManager {
    state: GameState,
    state_watch_tx: watch::Sender<GameState>,
    cmd_rx: mpsc::Receiver<Command>,
    rolling_id: i16,
}

impl GameManager {
    pub fn new() -> (Self, watch::Receiver<GameState>, mpsc::Sender<Command>) {
        let initial_state = GameState::new();
        let (state_tx, state_rx) = watch::channel(initial_state.clone());
        let (cmd_tx, cmd_rx) = mpsc::channel::<Command>();

        (
            GameManager {
                state: initial_state,
                state_watch_tx: state_tx,
                cmd_rx,
                rolling_id: 1,
            },
            state_rx,
            cmd_tx,
        )
    }
}

impl GameManager {
    pub fn start(&mut self) {
        info!("GameManager started");

        let mut player = Player::new("1".into(), "NoobMaster".into());
        player.input_direction = Vector2::new(1.0, 0.0);
        self.state.players.insert(player.id.clone(), player.clone());

        let mut player = Player::new("2".into(), "TheLegend27".into());
        player.input_direction = Vector2::new(-1.0, 0.0);
        self.state.players.insert(player.id.clone(), player.clone());

        let mut player = Player::new("3".into(), "WickedxD".into());
        player.input_direction = Vector2::new(0.0, 1.0);
        self.state.players.insert(player.id.clone(), player.clone());

        loop {
            self.recv_commands();
            self.sync_state();
            self.apply_input((1.0 / 20.0) as f64);
            thread::sleep(Duration::from_millis(1000 / 20));
        }
    }

    fn next_id(&mut self) -> String {
        let id = self.rolling_id;
        self.rolling_id += 1;

        id.to_string()
    }

    fn recv_commands(&mut self) {
        let cmds: Vec<_> = self.cmd_rx.try_iter().collect();

        if cmds.len() > 0 {
            // info!("GameManager batched {} commands", cmds.len());

            for cmd in cmds {
                self.process_command(cmd);
            }
        }
    }

    fn process_command(&mut self, command: Command) {
        let args = command.arguments.clone();
        match command.cmd_type {
            0 => {
                if let Some(data) = args {
                    let username: String = bitcode::decode(&data).unwrap();
                    let player = Player::new(self.next_id(), username);
                    self.state.players.insert(player.id.clone(), player.clone());
                    command.respond(CommandResponse::new(Ok(Some(bitcode::encode(&player.id)))));
                    return;
                }
            }
            _ => {}
        }

        command.respond_err_code(-1);
    }

    fn sync_state(&mut self) {
        self.state_watch_tx.send_if_modified(|old_state| {
            if *old_state == self.state {
                false
            } else {
                *old_state = self.state.clone();
                true
            }
        });
    }

    fn apply_input(&mut self, delta_time: f64) {
        self.state.players.iter_mut().for_each(|(_, player)| {
            player.apply_input(delta_time);
        });
    }
}
