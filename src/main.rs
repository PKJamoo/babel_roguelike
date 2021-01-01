mod player;
mod gui;
mod map;
mod components;
mod actions;
#[macro_use]
mod lib;

use specs::{World, WorldExt, Builder, Entity};
use components::{Position, Sprite};
use actions::Action;
use actions::Event;
use map::Map;
use tcod::console::*;
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


pub struct GameState {
    player_x: i32,
    player_y: i32,
    main_menu: gui::Menu,
    current_action: actions::Action,
    current_level: map::Map,
    event_queue: Vec<Event>
    // hold map
}

fn game_loop(state: &mut RunState, tcod: &mut gui::Tcod, game: &mut GameState, ecs: &mut World){
    match state {
        RunState::MainMenu  => {
                        tcod.render_main_menu(game.main_menu.clone());
                        *state = player::handle_main_menu_events(tcod, &mut game.main_menu.cursor_index, game.main_menu.menu_size);
                     },
        RunState::NewGame => {
                        // opening cinematic
                        // character select
                        // start game
                        *state = RunState::ActiveGame;
        },
        RunState::ActiveGame  => {
            tcod.render_game(game, ecs);
            // TODO: the 0 here is hardcoded/fake this should be replaced with a real EntityId
            let player_id = ecs.fetch::<Entity>();
            let act = player::read_keys(tcod, *player_id);
            // TODO: this should be handled by the event queue most likely
            if let actions::Action::MoveAction { id, x, y } = act {
                let mut pos_store = ecs.write_storage::<Position>();
                let mut player_pos = pos_store.get_mut(*player_id);
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
    let main_menu = gui::Menu{
                             cursor_index: 0,
                             menu_size: 3,
                             options: vec_of_strings!["New Game", "Load Game", "Options", "Quit"]
    };

    let mut game = GameState{player_x: SCREEN_WIDTH/2,
        player_y: SCREEN_HEIGHT/2,
        main_menu,
        current_action: Action::NoAction,
        current_level: Map::new(SCREEN_WIDTH, SCREEN_HEIGHT),
        event_queue: Vec::new()};

    // gui init code
    tcod::system::set_fps(LIMIT_FPS);
    let mut tcod = gui::Tcod::new();

    let mut state = RunState::MainMenu;
    while !tcod.root.window_closed() {
        game_loop(&mut state, &mut tcod, &mut game, &mut ecs);
    }
}