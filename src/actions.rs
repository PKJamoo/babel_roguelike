use specs::{Entity};

pub enum Action {
    MoveAction{id: Entity, x: i32, y: i32},
    NoAction
}

pub enum Event {
    
}