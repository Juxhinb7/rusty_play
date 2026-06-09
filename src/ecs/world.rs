use std::{cell::RefCell, collections::{btree_map::Entry, BTreeMap}, rc::Rc};

use sdl2::render::Texture;

use crate::{bootstrap::RustyErrorResult, ecs::{component::{Component, ComponentKind}, entity::{Entity, EntityKind}}, errors::RustyError, warehouse::ResourceManager};

/// Main system for interacting with the world.
pub struct World<'r> {
    pub available_entities: Vec<Rc<RefCell<Entity<'r>>>>,
    entities_map: BTreeMap<EntityKind, Vec<Rc<RefCell<Entity<'r>>>>>,
    background_texture: Option<Texture<'r>>,
    resource_manager: Option<ResourceManager>
}


impl<'r> std::fmt::Display for World<'r> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl<'r> World<'r> {
    /// Create new instance of the world system
    pub fn new() -> Self {
        Self { available_entities: Vec::new(), entities_map: BTreeMap::new(),  background_texture: None, resource_manager: None}
    }

    /// Create a new entity
    pub fn create_entity(&mut self, tag: EntityKind, texture: Rc<Texture<'r>>, width: u32, height: u32) -> RustyErrorResult<Rc<RefCell<Entity<'r>>>>  {
        let entity = Rc::new(RefCell::new(Entity::new(tag, texture, width, height)));
        let entity_clone = Rc::clone(&entity);
        match tag {
            EntityKind::Player => {
                for entity in &self.available_entities {
                    if let EntityKind::Player = *entity.borrow_mut().get_tag() {
                        return Err(Box::new(RustyError("There is already an entity of tag EntityKind::Player".into())))
                    }
                }

                self.available_entities.push(entity_clone);
            },
            _ => {
                self.available_entities.push(entity_clone);
            }
        }

        Ok(entity)
    }


    /// Retrieve an array of entities matching a tag
    pub fn get_entities_by_tag(&self, tag: EntityKind) -> Option<&Vec<Rc<RefCell<Entity<'r>>>>> {
        self.entities_map.get(&tag)
    }

    /// Retrieve all entities
    pub fn get_all_entities(&self) -> Rc<&BTreeMap<EntityKind, Vec<Rc<RefCell<Entity<'r>>>>>> {
        let entities_map = Rc::new(&self.entities_map);
        Rc::clone(&entities_map)
    }

    /// Update world to reflect changes
    pub fn update(&mut self) {
        for entity in &self.available_entities {
            let entity_clone = Rc::clone(&entity);
            match self.entities_map.entry(*entity.borrow_mut().get_tag()) {
                Entry::Vacant(e) => { e.insert(vec![entity_clone]); },
                Entry::Occupied(mut e) => { e.get_mut().push(entity_clone)}
            }
        }

        self.available_entities.clear();

        for item in self.entities_map.iter_mut() {
            item.1.retain(|x| { 
                 if let Some(Component::Health(health)) = x.borrow_mut().get_component(ComponentKind::Health) {
                    // Just like real world if an entity has some health it still exists in the world, otherwise it ceases to exist.
                    if health.current > 0 {
                        return true;
                    }
                 }
                 return false;
            })
        }
    }

    /// Set a new background texture
    pub fn set_background_texture(&mut self, texture: Option<Texture<'r>>) {
        self.background_texture = texture;
    }


    /// Retrieve the background texture
    pub fn get_background_texture(&self) -> &Option<Texture<'r>> {
        &self.background_texture
    }

    /// Set a new resource manager
    pub fn set_resource_manager(&mut self, resource_manager: Option<ResourceManager>) {
        self.resource_manager = resource_manager;
    }

    /// Retrieve the resource manager
    pub fn get_resource_manager(&mut self) -> &Option<ResourceManager> {
        &self.resource_manager
    }


}     

