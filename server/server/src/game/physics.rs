use rapier3d::{na::clamp, prelude::*};
use shared::game::{
    player::Player,
    vector::{Vector2, Vector3},
};
use std::collections::{HashMap, HashSet};

use crate::storage::mem_db::{self, MemDB};

pub struct PhysicsManager {
    pub gravity: Vector<Real>,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    pub physics_hooks: (),
    pub event_handler: (),
    pub collider_set: ColliderSet,
    pub rigid_body_set: RigidBodySet,

    pub entity_list: HashMap<String, RigidBodyHandle>,
    pub mem_db: MemDB,
}

impl PhysicsManager {
    pub fn new(mem_db: MemDB) -> Self {
        let gravity = vector![0.0, -20.0, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        let physics_hooks = ();
        let event_handler = ();
        let mut collider_set = ColliderSet::new();
        let rigid_body_set = RigidBodySet::new();

        let floor = ColliderBuilder::cuboid(1000.0, 0.1, 1000.0)
            .translation(vector![0.0, -0.1, 0.0])
            .build();
        collider_set.insert(floor);

        PhysicsManager {
            gravity,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            query_pipeline,
            physics_hooks,
            event_handler,
            collider_set,
            rigid_body_set,
            entity_list: HashMap::new(),
            mem_db,
        }
    }
}

impl PhysicsManager {
    pub fn create_entity(&mut self, id: &str, position: Vector3) -> RigidBodyHandle {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![position.x, position.y, position.z])
            .linear_damping(10.0)
            .gravity_scale(1.0)
            .build();
        let collider = ColliderBuilder::cuboid(0.5, 0.5, 0.5).build();
        let rb_handle = self.rigid_body_set.insert(rigid_body);
        self.collider_set
            .insert_with_parent(collider, rb_handle, &mut self.rigid_body_set);

        self.entity_list.insert(id.into(), rb_handle);

        rb_handle
    }

    pub fn get_entity(&mut self, id: &str) -> Option<&mut RigidBody> {
        self.entity_list
            .get(id)
            .and_then(|h| self.rigid_body_set.get_mut(*h))
    }

    pub fn update_entity(
        &mut self,
        delta_time: f32,
        id: &str,
        input_direction: Vector2,
        rotation: f32,
        speed: f32,
    ) {
        if let Some(entity) = self.get_entity(id) {
            let mut velocity = *entity.linvel();
            let rotated_di = input_direction.clone().rotate(rotation);

            velocity.x += clamp(rotated_di.x * speed * delta_time, -5.0, 5.0);
            velocity.z += clamp(rotated_di.y * speed * delta_time, -5.0, 5.0);

            entity.set_linvel(velocity, true);
        }
    }

    pub fn update_entities(&mut self, delta_time: f32) {
        let mut players = self.mem_db.get_all_players();

        for player in &mut players {
            let entity = self.get_entity(&player.id).cloned();

            if entity.is_some() {
                self.update_entity(
                    delta_time,
                    &player.id,
                    player.input_direction,
                    player.rotation,
                    player.speed,
                );

                let new_translation = self.get_entity(&player.id).unwrap().translation();
                let new_position =
                    Vector3::new(new_translation.x, new_translation.y, new_translation.z);

                player.position = new_position;
                let _ = self.mem_db.upsert_player(player.to_owned());
            } else {
                self.create_entity(&player.id, player.position);
            }
        }

        let all_players: Vec<Player> = self.mem_db.get_all_players();
        let valid_ids: HashSet<_> = all_players.iter().map(|p| &p.id).collect();

        let dangling_ids: Vec<String> = self
            .entity_list
            .keys()
            .filter(|id| !valid_ids.contains(*id))
            .cloned()
            .collect();

        for id in dangling_ids {
            self.remove_entity(&id);
            self.entity_list.remove(&id);
        }
    }

    pub fn remove_entity(&mut self, id: &str) {
        if let Some(handle) = self.entity_list.remove(id) {
            self.rigid_body_set.remove(
                handle,
                &mut self.island_manager,
                &mut self.collider_set,
                &mut self.impulse_joint_set,
                &mut self.multibody_joint_set,
                true,
            );
        }
    }

    pub fn step(&mut self, delta_time: f32) {
        self.integration_parameters.dt = delta_time;
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &self.physics_hooks,
            &self.event_handler,
        );
    }

    pub fn tick(&mut self, delta_time: f32) {
        self.step(delta_time);
        self.update_entities(delta_time);
    }
}
