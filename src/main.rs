mod player;
mod gui;
mod map;
mod components;
mod actions;
#[macro_use]
mod lib;
mod turnsystem;
use turnsystem::TurnSystem;

use specs::{World, WorldExt, Builder, Entity, RunNow};
use components::{Position, Sprite, Actor, Speed};
use map::Map;
use tcod::colors::*;

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 60; // 60 frames-per-second maximum

pub enum RunState {
    PlayerTurn,
    AITurn,
    ActiveGame,
    Inventory,
    MainMenu,
    NewGame,
    LoadGame,
    SaveGame,
    Options,
    GameOver
}

fn run_systems(ecs: &mut World) {
    let mut turn_system = TurnSystem{};
    turn_system.run_now(ecs);

    ecs.maintain();

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
            run_systems(ecs);
            let player_id = ecs.fetch::<Entity>();
            let mut turn_queue = ecs.write_resource::<Vec<Entity>>();
            while !((*turn_queue).is_empty()) {
                if (*turn_queue).remove(0) == (*player_id) {
                    let act = player::read_keys(tcod, *player_id);
                    // TODO: this should be handled by the event queue most likely
                    if let actions::Action::MoveAction { id: _, x, y } = act {
                        let mut pos_store = ecs.write_storage::<Position>();
                        let player_pos = pos_store.get_mut(*player_id);
                        if let Some(player_pos) = player_pos {
                          let  current_level = ecs.fetch::<Map>();
                          if current_level.can_move_to(player_pos.x + x, player_pos.y + y) {
                            player_pos.x += x;
                            player_pos.y += y;
                          }
                        }
                    }
                }

            }
        },
        RunState::PlayerTurn => {},
        RunState::AITurn => {},
        RunState::Inventory  => {},
        RunState::LoadGame => {},
        RunState::SaveGame  => {},
        RunState::Options => {},
        RunState::GameOver => {}
    }
}


fn main() {

    // init ecs
    let mut ecs = World::new();

    // register component types with ECS
    ecs.register::<Position>();
    ecs.register::<Sprite>();
    ecs.register::<Actor>();
    ecs.register::<Speed>();

    //create player entity
    let player_entity = ecs.create_entity().with(Actor{action_points:0, threshold: 5})
                                            .with(Position{x: SCREEN_WIDTH/2, y: SCREEN_HEIGHT/2})
                                            .with(Sprite{sprite: '@', color: WHITE })
                                            .with(Speed{speed: 1}).build();

    // insert entity id as resource for easy access
    ecs.insert(player_entity);
    
    // create gamestate resources
    let main_menu = gui::Menu::new(3, vec_of_strings!["New Game", "Load Game", "Options", "Quit"]);
    let current_level = Map::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let turn_queue : Vec<Entity> = Vec::new();

    ecs.insert(main_menu);
    ecs.insert(current_level);
    ecs.insert(turn_queue);

    // gui init code
    tcod::system::set_fps(LIMIT_FPS);
    let mut tcod = gui::Tcod::new();

    let mut state = RunState::MainMenu;
    while !tcod.root.window_closed() {
        game_loop(&mut state, &mut tcod, &mut ecs);
    }
}