use piston::input::*;
use graphics::{
    types::Color,
    context::Context,
};
use opengl_graphics::GlGraphics;
use rand::Rng;

use super::draw::{draw_block, draw_rectangle};
use super::snake::{Block, Direction, Snake};

pub struct Game {
    rng: rand::rngs::ThreadRng,
    bg_color: Color,
    width: u32,
    height: u32,
    food_color: Color,
    food: Block,
    snake: Snake,
}

const SNAKE_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

impl Game {

    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            bg_color: [0.0, 0.0, 0.0, 1.0],
            width: 19,
            height: 19,
            food_color: [1.0, 0.0, 0.0, 1.0],
            food: (1,1).into(),
            snake: Snake::new(2, 2, 3, SNAKE_COLOR, Direction::Right),
        }
    }

    pub fn new_food(&mut self) {
        loop {
            self.food.x = self.rng.gen::<u32>() % (self.width - 1) + 1;
            self.food.y = self.rng.gen::<u32>() % (self.height - 1) + 1;

            if !self.snake.is_overlap(self.food.x, self.food.y) {
                break;
            }
        }
    }

    pub fn render(&mut self, context: &Context, gl: &mut GlGraphics) {
        let border_color = [0.5,0.5,0.5,1.0];
        draw_rectangle(0, 0,            self.width, 1, border_color, context, gl);
        draw_rectangle(0, self.height,  self.width, 1, border_color, context, gl);
        draw_rectangle(0,           0,  1,          self.height, border_color, context, gl);
        draw_rectangle(self.width,  0,  1,          self.height + 1, border_color, context, gl);

        graphics::clear(self.bg_color, gl);
        draw_block(self.food.x, self.food.y, self.food_color, context, gl);

        self.snake.render(context, gl);
    }
    pub fn update(&mut self) {
        self.snake.update();

        let head = self.snake.head();
        if *head == self.food {
            self.snake.grow();
            self.new_food();
        }
        let head = self.snake.head();
        if head.x == 0 || head.x == self.width ||
            head.y == 0 || head.y == self.height {
            self.restart();
            return;
        }
        if self.snake.is_overlap_tail(head.x, head.y) {
            self.restart();
        }
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new(2, 2, 3, SNAKE_COLOR, Direction::Right);
        self.new_food();
    }

    pub fn button_pressed(&mut self, btn: &Button) {
        let dir = match btn {
            &Button::Keyboard(Key::Up) => Direction::Up,
            &Button::Keyboard(Key::Down) => Direction::Down,
            &Button::Keyboard(Key::Left) => Direction::Left,
            &Button::Keyboard(Key::Right) => Direction::Right,
            _ => self.snake.direction(),
        };
        self.snake.change_direction(dir);
    }
}