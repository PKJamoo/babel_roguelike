mod player;
mod gui;
mod map;
mod components;
mod actions;
#[macro_use]
mod lib;

use specs::{World, WorldExt, Builder, Entity};
use components::{Position, Sprite};
use map::Map;
use tcod::colors::*;

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 60; // 60 frames-per-second maximum

pub enum RunState {
    PlayerTurn,
    ActiveGame,
    Inventory,
    MainMenu,
    NewGame,
    LoadGame,
    SaveGame,
    Options,
    GameOver
}

fn game_loop(state: &mut RunState, tcod: &mut gui::Tcod, ecs: &mut World){
    match state {
        RunState::MainMenu  => {
                        tcod.render_main_menu(ecs);
                        *state = player::handle_main_menu_events(tcod, ecs);
                        
                     },
        RunState::NewGame => {
                        // opening cinematic
                        // character select
                        // start game
                        *state = RunState::ActiveGame;
        },
        RunState::ActiveGame  => {
            tcod.render_game(ecs);
            // TODO: the 0 here is hardcoded/fake this should be replaced with a real EntityId
            let player_id = ecs.fetch::<Entity>();
            let act = player::read_keys(tcod, *player_id);
            // TODO: this should be handled by the event queue most likely
            if let actions::Action::MoveAction { id: _, x, y } = act {
                let mut pos_store = ecs.write_storage::<Position>();
                let player_pos = pos_store.get_mut(*player_id);
                if let Some(player_pos) = player_pos{
                player_pos.x += x;
                player_pos.y += y;
                }
            }
        },
        RunState::Inventory  => {},
        RunState::LoadGame => {},
        RunState::SaveGame  => {},
        RunState::Options => {},
        RunState::GameOver => {},
        _ => {}
    }
}


fn main() {

    // init ecs
    let mut ecs = World::new();

    // register component types with ECS
    ecs.register::<Position>();
    ecs.register::<Sprite>();

    //create player entity
    let player_entity = ecs.create_entity().with(Position{x: SCREEN_WIDTH/2, y: SCREEN_HEIGHT/2})
                                             .with(Sprite{sprite: '@', color: WHITE }).build();

    // insert entity id as resource for easy access
    ecs.insert(player_entity);
    
    // set gamestate

    let main_menu = gui::Menu::new(3, vec_of_strings!["New Game", "Load Game", "Options", "Quit"]);
    let current_level = Map::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    ecs.insert(main_menu);
    ecs.insert(current_level);

    // gui init code
    tcod::system::set_fps(LIMIT_FPS);
    let mut tcod = gui::Tcod::new();

    let mut state = RunState::MainMenu;
    while !tcod.root.window_closed() {
        game_loop(&mut state, &mut tcod, &mut ecs);
    }
}