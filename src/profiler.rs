use memory_stats::{memory_stats};
use sdl2::{Sdl, render::{Canvas, TextureCreator}, ttf::Font, video::WindowContext};

use crate::{bootstrap::RustyErrorResult, errors::RustyError, math::Position, rendering::draw_texture, warehouse::{ResourceManager, resources::{Resource, ResourceKind}}};

pub struct FPSConfig {
    pub fps: u32,
}

pub struct Profiler {
    pub font: &'static str,
    pub point_size: u16,
    pub fps_config: FPSConfig
}

impl Default for Profiler {
    fn default() -> Self {
        Self { 
            font: "",
            point_size: 12,
            fps_config: FPSConfig { 
                fps: Default::default(), 
            } 
        }
    }
}

impl Profiler {

    pub fn display_diagnostics(&self, sdl2_context: &Sdl, resource_manager: &ResourceManager, canvas: &mut Canvas<sdl2::video::Window>, texture_creator: &TextureCreator<WindowContext>) -> RustyErrorResult<()> {
        self.get_fps(resource_manager, canvas, texture_creator)?;
        self.get_physical_memory_usage(resource_manager, canvas, texture_creator)?;
        self.get_current_platform(resource_manager, canvas, texture_creator)?;
        self.get_current_video_driver(sdl2_context, resource_manager, canvas, texture_creator)?;

        Ok(())
    }
    
    fn get_fps(&self, resource_manager: &ResourceManager, canvas: &mut Canvas<sdl2::video::Window>, texture_creator: &TextureCreator<WindowContext>) -> RustyErrorResult<()> {

        if let Some(Resource::TTF(ttf)) = resource_manager.get_resource(ResourceKind::TTF) {
            let font = ttf.sdl2_ttf_context.load_font(self.font, self.point_size)?;
            self.configure_text(canvas, texture_creator, &font, &format!("Performance: {} FPS", self.fps_config.fps), Position { x: 0, y: 5 })?;
            return Ok(());
        }
        
        Err(Box::new(RustyError("Error getting fps metrics".into())))
    }

    fn get_physical_memory_usage(&self, resource_manager: &ResourceManager, canvas: &mut Canvas<sdl2::video::Window>, texture_creator: &TextureCreator<WindowContext>) -> RustyErrorResult<()> {
        
        if let Some(usage) = memory_stats() {
            if let Some(Resource::TTF(ttf)) = resource_manager.get_resource(ResourceKind::TTF) {
                let font = ttf.sdl2_ttf_context.load_font(self.font, self.point_size)?;
                self.configure_text(canvas, texture_creator, &font, &format!("Physical memory usage: {} MiB", usage.physical_mem / 1024 / 1024), Position { x: 0, y: 20})?;
                return Ok(());
            }
        } 

        
        Err(Box::new(RustyError("Error getting ram usage".into())))

    }

    pub fn get_current_platform(&self, resource_manager: &ResourceManager, canvas: &mut Canvas<sdl2::video::Window>, texture_creator: &TextureCreator<WindowContext>) -> RustyErrorResult<()> {
        if let Some(Resource::TTF(ttf)) = resource_manager.get_resource(ResourceKind::TTF) {
            let font = ttf.sdl2_ttf_context.load_font(self.font, self.point_size)?;
            self.configure_text(canvas, texture_creator, &font, &format!("Platform: {}", sdl2::get_platform()), Position { x: 0, y: 40 })?;
            return Ok(())

        }
        Err(Box::new(RustyError("Error rendering text for the platform.".into())))
    }

    pub fn get_current_video_driver(&self, sdl2_context: &Sdl, resource_manager: &ResourceManager, canvas: &mut Canvas<sdl2::video::Window>, texture_creator: &TextureCreator<WindowContext>) -> RustyErrorResult<()> {
        let video = sdl2_context.video()?;
        
        
        if let Some(Resource::TTF(ttf)) = resource_manager.get_resource(ResourceKind::TTF) {
            let font = ttf.sdl2_ttf_context.load_font(self.font, self.point_size)?;
            self.configure_text(canvas, texture_creator, &font, &format!("Current video driver: {}", video.current_video_driver()), Position { x: 0, y: 60 })?;
            return Ok(())
        } 
        
        Err(Box::new(RustyError("Error getting video driver".into())))
    }

    fn configure_text(
        &self,
        canvas: &mut Canvas<sdl2::video::Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
        text: &str,
        position: Position,
    ) -> RustyErrorResult<()> {
        
        let surface = font
            .render(text)
            .blended(sdl2::pixels::Color::WHITE)?;

        let texture = texture_creator.create_texture_from_surface(&surface)?;
        draw_texture(canvas, &texture, position, (surface.width(), surface.height()))?;

        Ok(())
    }
}   
