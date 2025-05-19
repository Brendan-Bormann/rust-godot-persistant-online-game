pub struct Action {
    player_id: String,
    action_type: ActionType,
    args: Vec<u8>,
}

enum ActionType {
    None,
    SetBearing,
}

impl Action {
    pub fn new(player_id: String, action_type: ActionType, args: Vec<u8>) -> Self {
        Action {
            player_id,
            action_type,
            args,
        }
    }
}
