use sdl2::{EventPump, Sdl, render::{Canvas, TextureCreator}, video::WindowContext};

use crate::{ecs::world::World, profiler::Profiler};

pub struct Setup<'a> {
    pub sdl2_context: Sdl,
    pub should_quit: bool,
    pub canvas: Canvas<sdl2::video::Window>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub event_pump: EventPump,  
    pub window_size: (u32, u32),
    pub profiler: Profiler,
    
    pub world: World<'a>
}