use crate::map::{Tile};

use specs::{Component, VecStorage, NullStorage};
use tcod::colors::Color;
use std::collections::HashSet;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Player;

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

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Actor{
    pub action_points: i32,
    pub threshold: i32
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Speed{
    pub speed: i32
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Vision {
    // TODO: Make vision types enum/struct(?) i.e. normal, infrared, xray, etc
    pub field_of_vision: HashSet<Tile>
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Blocking;