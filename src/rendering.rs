use sdl2::{rect::Rect, render::{Canvas, Texture}, video::{Window}};

use crate::{bootstrap::RustyErrorResult, math::Position};

pub fn draw_texture(
    canvas: &mut Canvas<Window>, 
    texture: &Texture, 
    position: Position,
    (width, height): (u32, u32)) -> RustyErrorResult<()> {
    
    canvas.copy(texture, None, Rect::new(position.x, position.y, width, height))?;


    Ok(())
}


