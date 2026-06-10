use crate::ecs::component::{animation_component::AnimationComponent, health_component::HealthComponent, transform_component::TransformComponent, weapon_component::WeaponComponent};

pub mod animation_component;
pub mod transform_component;
pub mod health_component;
pub mod weapon_component;

#[derive(PartialEq, PartialOrd, Ord, Eq)]
pub enum ComponentKind {
    Transform,
    Animation,
    Health,
    Weapon, 
}

pub enum Component {
    Transform(TransformComponent),
    Animation(AnimationComponent),
    Health(HealthComponent),
    Weapon(WeaponComponent),
}

impl From<TransformComponent> for Component {
    fn from(value: TransformComponent) -> Self {
        Component::Transform(value)
    }
}

impl From<AnimationComponent> for Component {
    fn from(value: AnimationComponent) -> Self {
        Component::Animation(value)
    }
}

impl From<HealthComponent> for Component {
    fn from(value: HealthComponent) -> Self {
        Component::Health(value)
    }
}

impl From<WeaponComponent> for Component {
    fn from(value: WeaponComponent) -> Self {
        Component::Weapon(value)
    }
}