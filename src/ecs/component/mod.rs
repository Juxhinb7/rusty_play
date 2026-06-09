use crate::ecs::component::{animation::Animation, health::Health, transform::Transform, weapon::Weapon};

pub mod animation;
pub mod transform;
pub mod health;
pub mod weapon;

#[derive(PartialEq, PartialOrd, Ord, Eq)]
pub enum ComponentKind {
    Transform,
    Animation,
    Health,
    Weapon, 
}

pub enum Component {
    Transform(Transform),
    Animation(Animation),
    Health(Health),
    Weapon(Weapon),
}

impl From<Transform> for Component {
    fn from(value: Transform) -> Self {
        Component::Transform(value)
    }
}

impl From<Animation> for Component {
    fn from(value: Animation) -> Self {
        Component::Animation(value)
    }
}

impl From<Health> for Component {
    fn from(value: Health) -> Self {
        Component::Health(value)
    }
}

impl From<Weapon> for Component {
    fn from(value: Weapon) -> Self {
        Component::Weapon(value)
    }
}