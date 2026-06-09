use crate::math::{Position, Velocity};

pub struct Transform {
    pub position: Position,
    pub velocity: Velocity,
    pub initial_position: Position,
    pub initial_velocity: Velocity
}