use std::{fmt::Display, marker::PhantomData};

use sdl2::{render::{Canvas, TextureCreator}, ttf::Sdl2TtfContext, video::{Window, WindowContext}};

use crate::{bootstrap::RustyErrorResult, math::Position, rendering::draw_texture};

pub struct Startup;
pub struct ForBuild;

pub struct UIBuilder<'a, Phase, T = ()> {
    font_path: &'a str,
    point_size: u16,
    texture_creator: &'a TextureCreator<WindowContext>,
    ttf_context: Sdl2TtfContext,
    text: T,
    text_color: sdl2::pixels::Color,
    text_position: Position,
    _marker: PhantomData<Phase>,

}

impl <'a> UIBuilder<'a, Startup, ()> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        UIBuilder {
            font_path: "src/assets/bpg-supersquare-webfont.ttf",
            point_size: 24,
            texture_creator: texture_creator,
            ttf_context: sdl2::ttf::init().unwrap(),
            text: (),
            text_color: sdl2::pixels::Color::MAGENTA,
            text_position: Position { x: 1100, y: 5 },
            _marker: PhantomData
        }
    }

    pub fn with_font(mut self, font_path: &'a str, point_size: u16) -> Self {
        self.font_path = font_path;
        self.point_size = point_size;
        return self
    }

    pub fn with_text_color(mut self, text_color: sdl2::pixels::Color) -> Self {
        self.text_color = text_color;
        return self;
    }

    pub fn with_text_position(mut self, text_position: Position) -> Self {
        self.text_position = text_position;
        return self;
    }

    pub fn set_text<T>(self, text: T) -> UIBuilder<'a, ForBuild, T>
    where
        T: Display,
    {
        UIBuilder {
            font_path: self.font_path,
            point_size: self.point_size,
            texture_creator: self.texture_creator,
            ttf_context: self.ttf_context,
            text,
            text_color: self.text_color,
            text_position: self.text_position,
            _marker: PhantomData,
        }
    }

    
}

impl <'a, T> UIBuilder<'a, ForBuild, T> where T: Display {
    pub fn build(self, canvas: &mut Canvas<Window>) -> RustyErrorResult<()> {
        let font = self.ttf_context.load_font(self.font_path, self.point_size)?;
        let surface = font
            .render(&format!("{}", self.text))
            .blended(self.text_color)?;
        let texture = self.texture_creator.create_texture_from_surface(&surface)?;
        draw_texture(canvas, &texture, self.text_position, (surface.width(), surface.height()))?;
        Ok(())
    }
}