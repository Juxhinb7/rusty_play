use std::{collections::BTreeMap, rc::Rc};

use sdl2::render::Texture;

use crate::ecs::{component::{Component, ComponentKind}};

 /// The entity can be a player, a enemy, a object,  or a powerup
#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub enum EntityKind {
    Player,
    Enemy,
    Object,
    PowerUp
}

/// An entity is represented here. Entities can be anything that exists e.g. buildings, airplanes, cars, items, characters, animals etc.
pub struct Entity<'r> {
    /// All entities have tags where they can be grouped and accessed through it
    tag: EntityKind,
    /// All entities have textures
    pub texture: Rc<Texture<'r>>,
    /// The width of the entity
    width: u32,
    /// The height of the entity
    height: u32,
    components: BTreeMap<ComponentKind, Component>,
}

impl<'r> Entity<'r> {
    pub fn new(tag: EntityKind, texture: Rc<Texture<'r>>, width: u32, height: u32) -> Entity<'r> {
        Entity {
            tag,
            texture,
            width,
            height,
            components: BTreeMap::new(),
        }
    }

    pub fn add_component(&mut self, component: Component) -> &mut Self {
        let key = match &component {
            Component::Transform(_) => ComponentKind::Transform,
            Component::Animation(_) => ComponentKind::Animation,
            Component::Health(_) => ComponentKind::Health,
            Component::Weapon(_) => ComponentKind::Weapon,
        };
        
        self.components.insert(key, component);
        self
    }

    pub fn get_tag(&self) -> &EntityKind {
        &self.tag
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_component(&self, component_kind: ComponentKind) -> Option<&Component> {
        self.components.get(&component_kind)
    }

    pub fn get_component_mut(&mut self, component_kind: ComponentKind) -> Option<&mut Component> {
        self.components.get_mut(&component_kind)
    }
}

