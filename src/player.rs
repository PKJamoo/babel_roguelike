use crate::gui::Tcod as Tcod;
use crate::RunState as RunState;
use crate::actions::*;
use crate::entities::Entity;
use tcod::input::Key;
use tcod::input::KeyCode::*;

pub fn increment_cursor(cursor: &mut i16, menu_size: i16){
    if *cursor == menu_size { *cursor = 0; } else { *cursor += 1; }
}

pub fn decrement_cursor(cursor: &mut i16, menu_size: i16) {
    if *cursor == 0 { *cursor = menu_size;} else { *cursor -= 1;}
}

pub fn read_keys(tcod: &mut Tcod, id: Entity) ->  Action{
    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key { code: Escape, .. } => std::process::exit(0x0), // exit game
        // movement keys
        Key { code: Up, .. } => Action::MoveAction{id: id, x: 0, y: -1},
        Key { code: Down, .. } => Action::MoveAction{id: id, x: 0, y: 1},
        Key { code: Left, .. } => Action::MoveAction{id: id, x: -1, y: 0},
        Key { code: Right, .. } => Action::MoveAction{id: id, x: 1, y: 0},
        //numpad keys (handled counterclockwise)
        Key { code: NumPad1, .. } => Action::MoveAction{id: id, x: -1, y: 1},
        Key { code: NumPad2, .. } => Action::MoveAction{id: id, x: 0, y: 1},
        Key { code: NumPad3, .. } => Action::MoveAction{id: id, x: 1, y: 1},
        Key { code: NumPad6, .. } => Action::MoveAction{id: id, x: 1, y: 0},
        Key { code: NumPad9, .. } => Action::MoveAction{id: id, x: 1, y: -1},
        Key { code: NumPad8, .. } => Action::MoveAction{id: id, x: 0, y: -1},
        Key { code: NumPad7, .. } => Action::MoveAction{id: id, x: -1, y: -1},
        Key { code: NumPad4, .. } => Action::MoveAction{id: id, x: -1, y: 0},
        _ => Action::NoAction
    }
}

pub fn handle_main_menu_events(tcod: &mut Tcod, cursor: &mut i16, menu_size: i16) -> RunState {
    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key {code: Up, ..} => {
                               decrement_cursor(cursor, menu_size);
                               RunState::MainMenu
                            },
        Key {code: Down, ..} => {
                                 increment_cursor(cursor, menu_size);
                                 RunState::MainMenu
                                },
        Key {code: Enter, ..} => {// return new RunState of chosen menu item
                                  match cursor {
                                      0 => RunState::NewGame,
                                      1 => RunState::LoadGame,
                                      2 => RunState::Options,
                                      3 => std::process::exit(0x0),
                                      _ => RunState::MainMenu
                                  }

        },
        // return no change if there is no input/no recognized commands
        _ => RunState::MainMenu
    }



}