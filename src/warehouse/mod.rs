use std::collections::HashMap;

use crate::warehouse::resources::{Resource, ResourceKind};

pub mod resources;

pub struct ResourceManager {
    resources: HashMap<ResourceKind, Resource>
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager { resources: HashMap::new() }
    }

    pub fn add_resource(&mut self, resource: Resource) -> &mut Self {
        let key = match &resource {
            Resource::TTF(_) => ResourceKind::TTF
        };

        self.resources.insert(key, resource);
        return self
    }

    pub fn get_resource(&self, resource_kind: ResourceKind) -> Option<&Resource> {
        self.resources.get(&resource_kind)
    }

    pub fn get_resource_mut(&mut self, resource_kind: ResourceKind) -> Option<&mut Resource> {
        self.resources.get_mut(&resource_kind)
    }
}