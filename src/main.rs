use std::rc::Rc;

use rusty_play::{bootstrap::{GameBuilder, RustyErrorResult}, contexts::{Context, setup::Setup}, ecs::{component::{Component, ComponentKind, animation::Animation, health::Health, transform::Transform}, entity::EntityKind}, math::{Position, Velocity}, rendering::draw_texture, warehouse::{ResourceManager, resources::{ttf::TTF}}};
use sdl2::{event::Event, image::LoadTexture, keyboard::Keycode};

fn start(ctx: &mut Context<Setup>) -> RustyErrorResult<()> {
    let background_texture = ctx.inner.texture_creator.load_texture("nebula.png")?;
    let player_texture = Rc::new(ctx.inner.texture_creator.load_texture("spacecraft.png")?);
    
    let player = ctx.inner.world.create_entity(EntityKind::Player, player_texture.clone(), 64, 64)?;
    
    let mut resource_manager = ResourceManager::new();
    resource_manager
        .add_resource(
            TTF {
                sdl2_ttf_context: sdl2::ttf::init().unwrap()
            }.into()
        );


    player.borrow_mut()
        .add_component(
            Transform {
                position: Position { x: 550, y: 500 },
                velocity: Velocity { x: 20, y: 20 },
                initial_position: Position { x: 550, y: 650 },
                initial_velocity: Velocity { x: 20, y: 20 }
            }.into()
        )
        .add_component(
            Health {
                max: 100,
                current: 100
            }.into()
        );

    ctx.inner.world.set_background_texture(Some(background_texture));
    ctx.inner.world.set_resource_manager(Some(resource_manager));

    Ok(())

}

fn update(ctx: &mut Context<Setup>) -> RustyErrorResult<()> {

    let handle_events = |ctx: &mut Context<Setup>| -> RustyErrorResult<()> {

        for event in ctx.inner.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | 
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    ctx.inner.should_quit = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    if let Some(player) = ctx.inner.world.get_entities_by_tag(EntityKind::Player) {
                        if let Some(Component::Transform(transform)) = player[0].borrow_mut().get_component_mut(ComponentKind::Transform) {
                            transform.position.x += transform.velocity.x;
                        }
                    } 
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    if let Some(player) = ctx.inner.world.get_entities_by_tag(EntityKind::Player) {
                        if let Some(Component::Transform(transform)) = player[0].borrow_mut().get_component_mut(ComponentKind::Transform) {
                            transform.position.x -= transform.velocity.x;
                        }
                    }
                },
                _ => {}
            }

        }

        ctx.inner.world.update();

        Ok(())
    };

    let render = |ctx: &mut Context<Setup>, _: &mut Vec<Animation>| -> RustyErrorResult<()> {
        ctx.inner.canvas.clear();

        let background_texture = ctx.inner.world
            .get_background_texture()
            .as_ref()
            .ok_or("Error getting background texture")?;

        draw_texture(
            &mut ctx.inner.canvas,
            background_texture,
            Position { x: 0, y: 0 },
            (1200, 600),
        )?;

        if let Some(player_entities) = ctx.inner.world.get_entities_by_tag(EntityKind::Player) {
            if let Some(player) = player_entities.get(0) {
                let texture = player.borrow().texture.clone();

                if let Some(Component::Transform(transform)) = player.borrow().get_component(ComponentKind::Transform) {
                    draw_texture(
                        &mut ctx.inner.canvas,
                        &texture,
                        Position { x: transform.position.x, y: transform.position.y },
                        (player.borrow().get_width(), player.borrow().get_height()),
                    )?;
                }


            }
        }

        Ok(())
    };

    handle_events(ctx)?;

    render(ctx, &mut vec![Animation::new()])?;

    let resource_manager = ctx.inner.world.get_resource_manager().as_ref().ok_or("ResourceManager not initialized")?;

    ctx.inner.profiler.display_diagnostics(&ctx.inner.sdl2_context, resource_manager, &mut ctx.inner.canvas, &ctx.inner.texture_creator)?;






    Ok(())
}


fn main() -> RustyErrorResult<()> {
    GameBuilder::new()
        .set_window_settings("Test", 1200, 600)
        .set_start_callback(start)
        .set_update_callback(update)
        .run()?;

    Ok(())
}