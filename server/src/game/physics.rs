use std::collections::HashMap;

use crate::game;
use rapier3d::prelude::*;

use super::player::Player;

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

    pub player_list: HashMap<String, RigidBodyHandle>,
}

impl PhysicsManager {
    pub fn new() -> Self {
        let gravity = vector![0.0, -9.81, 0.0];
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

        let floor = ColliderBuilder::cuboid(100.0, 0.1, 100.0)
            .translation(vector![0.0, -0.1, 0.0]) // place it just below y=0
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
            player_list: HashMap::new(),
        }
    }
}

impl PhysicsManager {
    pub fn create_player_rb(&mut self, id: &str) -> RigidBodyHandle {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 10.0, 0.0])
            .linear_damping(20.0)
            .build();
        let collider = ColliderBuilder::capsule_y(0.5, 0.5).build();

        let player_body_handle = self.rigid_body_set.insert(rigid_body);

        self.collider_set.insert_with_parent(
            collider,
            player_body_handle,
            &mut self.rigid_body_set,
        );

        self.player_list.insert(id.into(), player_body_handle);

        player_body_handle
    }

    pub fn get_player_mut(&mut self, id: &str) -> Option<&mut RigidBody> {
        self.player_list
            .get(id)
            .and_then(|h| self.rigid_body_set.get_mut(*h))
    }

    pub fn get_player(&self, id: &str) -> Option<&RigidBody> {
        self.player_list
            .get(id)
            .and_then(|h| self.rigid_body_set.get(*h))
    }

    pub fn get_player_handle(&self, id: &str) -> Option<RigidBodyHandle> {
        self.player_list.get(id).copied()
    }

    pub fn move_player(
        &mut self,
        player: &Player,
        delta_time: f32,
    ) -> Option<game::vector::Vector3> {
        match self.player_list.get(&player.id) {
            Some(_) => {
                if let Some(rb) = self.get_player_mut(&player.id) {
                    let mut velocity = *rb.linvel();

                    velocity.x += player.input_direction.x * player.speed * delta_time;
                    velocity.z += player.input_direction.y * player.speed * delta_time;

                    rb.set_linvel(velocity, true);

                    let translation = rb.translation();

                    Some(game::vector::Vector3::new(
                        translation.x,
                        translation.y,
                        translation.z,
                    ))
                } else {
                    None
                }
            }
            None => {
                let _handle = self.create_player_rb(&player.id);
                self.move_player(player, delta_time)
            }
        }
    }

    pub fn remove_player(&mut self, id: &str) {
        if let Some(handle) = self.player_list.remove(id) {
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
}
