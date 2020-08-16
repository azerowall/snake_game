use std::collections::LinkedList;

use graphics::{
    types::Color,
    context::Context,
};
use opengl_graphics::GlGraphics;

use super::draw::draw_block;



#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Left, Right, Up, Down
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match *self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}


#[derive(PartialEq, Clone, Copy)]
pub struct Block {
    pub x: u32,
    pub y: u32,
}

impl From<(u32, u32)> for Block {
    fn from((x, y): (u32, u32)) -> Block {
        Block { x, y }
    }
}


pub struct Snake {
    dir: Direction,
    body: LinkedList<Block>,
    color: Color,
}

impl Snake {
    pub fn new(x: u32, y: u32, size: u32, color: Color, dir: Direction) -> Self {
        let mut body = LinkedList::<Block>::new();
        for _ in 0..size {
            body.push_front((x, y).into());
        }

        Snake {
            dir,
            body,
            color,
        }
    }

    pub fn change_direction(&mut self, dir: Direction) {
        if dir.opposite() != self.dir {
            self.dir = dir;
        }
    }

    pub fn direction(&self) -> Direction {
        self.dir
    }

    pub fn grow(&mut self) {
        let tail = *self.body.back().unwrap();
        self.body.push_back(tail);
    }

    pub fn head(&self) -> &Block {
        self.body.front().unwrap()
    }

    pub fn is_overlap_tail(&self, x: u32, y: u32) -> bool {
        for p in self.body.iter().skip(1) {
            if p.x == x && p.y == y {
                return true;
            }
        }
        false
    }

    pub fn is_overlap(&self, x: u32, y: u32) -> bool {
        self.head().x == x && self.head().y == y || self.is_overlap_tail(x, y)
    }

    pub fn render(&self, context: &Context, gl: &mut GlGraphics) {
        for p in &self.body {
            draw_block(p.x, p.y, self.color, context, gl);
        }
    }

    pub fn update(&mut self) {
        let head = self.body.front().expect("Empty body");
        let mut head = (*head).clone();

        match self.dir {
            Direction::Left => head.x -= 1,
            Direction::Right => head.x += 1,
            Direction::Up => head.y -= 1,
            Direction::Down => head.y += 1,
        };
        
        self.body.push_front(head);
        self.body.pop_back().unwrap();
    }
}