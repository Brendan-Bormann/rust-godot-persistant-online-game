use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Command {
    pub command: String,
    pub subcommand: String,
    pub args: Vec<String>,
    pub issuer: SocketAddr,
}

impl Command {
    pub fn new(command: &str, subcommand: &str, args: Vec<String>, issuer: &SocketAddr) -> Command {
        Command {
            command: command.into(),
            subcommand: subcommand.into(),
            args,
            issuer: issuer.clone(),
        }
    }
}
