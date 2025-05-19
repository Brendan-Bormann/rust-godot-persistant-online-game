use godot::classes::{CharacterBody3D, ICharacterBody3D};
use godot::prelude::*;
use shared::game::player::Player;
use shared::util::math::lerp;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct PlayerObject {
    #[var]
    pub id: GString,
    #[var]
    pub username: GString,
    pub network_position: Vector3,
    pub base: Base<CharacterBody3D>,
}

#[godot_api]
impl PlayerObject {
    #[func]
    pub fn update_position(&mut self) {
        let pos = self.base().get_position();
        let lerp_speed = 0.05;

        let new_pos = Vector3 {
            x: lerp(pos.x, self.network_position.x, lerp_speed),
            y: lerp(pos.y, self.network_position.y, lerp_speed),
            z: lerp(pos.z, self.network_position.z, lerp_speed),
        };

        self.base_mut().set_position(new_pos);
    }

    pub fn network_set_player(&mut self, player: &Player) {
        let pos = self.base().get_position();

        self.id = player.id.clone().into();
        self.username = player.username.clone().into();
        let updated_position = Vector3 {
            x: player.position.x,
            y: player.position.y,
            z: player.position.z,
        };

        self.network_position = updated_position;

        if pos.x == 0.0 && pos.y == 0.0 && pos.z == 0.0 {
            self.base_mut().set_position(updated_position);
        }
    }
}
