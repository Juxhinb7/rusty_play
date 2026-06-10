use crate::warehouse::resources::ttf::TTFResource;

pub mod ttf;

#[derive(PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum ResourceKind {
    TTF
}

pub enum Resource {
    TTF(TTFResource)
}

impl From<TTFResource> for Resource {
    fn from(value: TTFResource) -> Self {
        Resource::TTF(value)
    }
}