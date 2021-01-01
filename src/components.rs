use specs::{Component, VecStorage};
use tcod::colors::Color;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub sprite: char,
    pub color: Color
}