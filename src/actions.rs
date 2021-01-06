use specs::{Entity};
use crate::map::Map;

pub enum Action {
    MoveAction{id: Entity, x: i32, y: i32},
    NoAction
}

// This struct takes an action and checks with the game state that it is a valid action.
pub struct ActionVerifier {}

impl ActionVerifier {

    pub fn verify_move(&self, map: & Map, x: i32, y: i32) -> bool{
        return true;
    }
}