use crate::warehouse::resources::ttf::TTF;

pub mod ttf;

#[derive(PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum ResourceKind {
    TTF
}

pub enum Resource {
    TTF(TTF)
}

impl From<TTF> for Resource {
    fn from(value: TTF) -> Self {
        Resource::TTF(value)
    }
}