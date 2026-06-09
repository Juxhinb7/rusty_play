use std::{marker::PhantomData, time::{Duration, Instant}};

use sdl2::{EventPump, Sdl, render::{Canvas, TextureCreator}, video::{Window, WindowContext}};

use crate::{contexts::{Context, setup::Setup}, ecs::world::World, profiler::{Profiler}};

pub struct ForSettings;
pub struct ForStart;
pub struct ForUpdate;
pub struct ForBuild;

type Settings = Option<((u32, u32), Canvas<Window>, TextureCreator<WindowContext>, EventPump)>;

pub type RustyErrorResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct GameBuilder<Phase, WindowSettings = Settings, Start = (), Update = ()> {
    sdl2_context: Sdl,
    should_quit: bool,
    is_debug_mode: bool,
    window_settings: WindowSettings,
    profiler: Profiler,
    start: Start,
    update: Update,
    _marker: PhantomData<Phase>,
}

impl GameBuilder<ForSettings> {
    pub fn new() -> Self {
        GameBuilder {
            sdl2_context: sdl2::init().unwrap(),
            should_quit: false,
            is_debug_mode: false,
            window_settings: None,
            profiler: Profiler::default(),
            start: (),
            update: (),
            _marker: PhantomData,
        }
    }

    pub fn with_debug_mode(mut self) -> Self {
        self.is_debug_mode = true;
        self
    }
    pub fn set_window_settings(self, title: &str, width: u32, height: u32) -> GameBuilder<ForStart, Settings, (), ()> {
        let video_subsystem = self.sdl2_context.video().unwrap();
        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let window_size = (width, height);
        let canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();
        let texture_creator= canvas.texture_creator();
        let event_pump = self.sdl2_context.event_pump().unwrap();

        GameBuilder {
            sdl2_context: self.sdl2_context,
            should_quit: self.should_quit,
            is_debug_mode: self.is_debug_mode,
            window_settings: Some((window_size, canvas, texture_creator, event_pump)),
            profiler: self.profiler, 
            start: (), 
            update: (), 
            _marker: PhantomData
        }
        
    }

}

impl <WS> GameBuilder<ForStart, WS, (), ()> {
        pub fn set_start_callback<F>(self, f: F) -> GameBuilder<ForUpdate, WS, F, ()> 
    where 
        F: Fn(&mut Context<Setup>) -> Result<(), Box<dyn std::error::Error>>,
    {
        println!("Startup system registered");
        GameBuilder {
            sdl2_context: self.sdl2_context,
            should_quit: self.should_quit,
            is_debug_mode: self.is_debug_mode,
            window_settings: self.window_settings,
            profiler: self.profiler,
            start: f,
            update: (),
            _marker: PhantomData,
        }
    }
}

impl<WS, ST> GameBuilder<ForUpdate, WS, ST> {
    pub fn set_update_callback<F>(self, f: F) -> GameBuilder<ForBuild, WS, ST, F> 
    where 
        F: Fn(&mut Context<Setup>) -> RustyErrorResult<()>,
        ST: Fn(&mut Context<Setup>) -> RustyErrorResult<()>,
    {
        println!("Update system registered");
        GameBuilder {
            sdl2_context: self.sdl2_context,
            should_quit: self.should_quit,
            is_debug_mode: self.is_debug_mode,
            window_settings: self.window_settings,
            profiler: self.profiler,
            start: self.start,
            update: f,
            _marker: PhantomData,
        }
    }
}

impl<WS, ST, UP> GameBuilder<ForBuild, WS, ST, UP> 
where 
    ST: Fn(&mut Context<Setup>) -> RustyErrorResult<()>,
    UP: Fn(&mut Context<Setup>) -> RustyErrorResult<()>,
    WS: Into<Option<( (u32, u32), Canvas<Window>, TextureCreator<WindowContext>, EventPump)>>
{
    pub fn run(self) -> RustyErrorResult<()> {
        let ( window_size, canvas, texture_creator, event_pump ) = self.window_settings.into().unwrap();

        let setup = Setup {
            sdl2_context: self.sdl2_context,
            should_quit: self.should_quit,
            canvas,
            texture_creator: &texture_creator,
            event_pump,
            window_size,
            profiler: self.profiler,
            world: World::new()
        };

        let mut ctx = Context {
            inner: setup
        };

        (self.start)(&mut ctx)?;

        let target_frame_time: Duration = Duration::from_micros(16_667); // ~60 FPS
        let mut frame_count = 0;
        let mut fps_timer = Instant::now();



        while !ctx.inner.should_quit {

            let frame_start = Instant::now();

            (self.update)(&mut ctx)?;
            
            frame_count += 1;

            if fps_timer.elapsed() >= std::time::Duration::from_secs(1) {
                ctx.inner.profiler.fps_config.fps = frame_count;
                frame_count = 0;
                fps_timer = Instant::now();
                
 

            }

            let frame_time = frame_start.elapsed();
            if frame_time < target_frame_time {
                std::thread::sleep(target_frame_time - frame_time);
            }

            ctx.inner.canvas.present();

            
            
        }

        Ok(())
    }
}

