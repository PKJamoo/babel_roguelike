mod player;
mod gui;
mod map;
mod components;
mod entities;
//mod systems;
//mod manager;
mod actions;
#[macro_use]
mod lib;

use entities::Entity;
use actions::Action;
use actions::Event;
use map::Map;
use tcod::console::*;

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
    turn_queue: Vec<Entity>,
    event_queue: Vec<Event>
    // hold map
}

fn game_loop(state: &mut RunState, tcod: &mut gui::Tcod, game: &mut GameState){
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
            tcod.render_game(game);
            // TODO: the 0 here is hardcoded/fake this should be replaced with a real EntityId
            let act = player::read_keys(tcod, 0);

            // TODO: this should be handled by the event queue most likely
            if let actions::Action::MoveAction { id, x, y } = act {
                game.player_x += x;
                game.player_y += y;
            }

            // iterate through turn order list
            for actor in game.turn_queue.iter_mut() {

                for event in game.event_queue.iter_mut() {
                    // handle effects queue from actions
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
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("BABEL")
        .init();

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = gui::Tcod { root, con };
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
                             turn_queue: Vec::new(),
                             event_queue: Vec::new()};

    let mut state = RunState::MainMenu;
    while !tcod.root.window_closed() {
        game_loop(&mut state, &mut tcod, &mut game);
    }
}