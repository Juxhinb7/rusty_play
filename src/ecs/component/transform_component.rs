use crate::math::{Position, Velocity};

pub struct TransformComponent {
    pub position: Position,
    pub velocity: Velocity,
    pub initial_position: Position,
    pub initial_velocity: Velocity
}