use piston::{
    window::WindowSettings,
    event_loop::*,
    input::*,
};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

mod draw;
mod snake;
mod game;

use game::Game;






fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Snake!!!", (400,400))
                                    .graphics_api(opengl)
                                    .exit_on_esc(true)
                                    .build()
                                    .unwrap();


    let mut game = Game::new();
    let mut gl = GlGraphics::new(opengl);
    
    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |c, g| {
                game.render(&c, g);
            })
        }

        if let Some(_) = event.update_args() {
            game.update();
        }

        if let Some(args) = event.button_args() {
            if args.state == ButtonState::Press {
                game.button_pressed(&args.button);
            }
        }
    }
}
