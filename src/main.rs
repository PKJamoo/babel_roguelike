mod player;
mod gui;
mod map;
use map::{Map};
mod components;
mod actions;
mod pathfinding;
use pathfinding::astar_search;
#[macro_use]
mod lib;

mod turnsystem;
use turnsystem::TurnSystem;
mod visionsystem;
use visionsystem::VisionSystem;
mod mapblockingsystem;
use mapblockingsystem::MapBlockingSystem;

use specs::{World, WorldExt, Builder, Entity, RunNow};
use components::{Position, Sprite, Actor, Speed, Vision, Player, Blocking};

use tcod::colors::*;
use rand::prelude::*;
use std::collections::HashSet;


// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

pub enum RunState {
    Tick,
    ActiveTurn,
    PlayerTurn,
    AITurn{id: Entity},
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

    let mut vision_system = VisionSystem{};
    vision_system.run_now(ecs);

    let mut map_blocking_system = MapBlockingSystem{};
    map_blocking_system.run_now(ecs);

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
                        *state = RunState::Tick;
        },

        // run all once per frame systems, etc
        RunState::Tick => {
            run_systems(ecs);
            tcod.render_game(ecs);
            *state = RunState::ActiveTurn;

        },
        // work through all the turns that occur on a single tick
        RunState::ActiveTurn  => {
            tcod.render_game(ecs);
            let player_id = ecs.fetch::<Entity>();
            let mut turn_queue = ecs.write_resource::<Vec<Entity>>();

            // branch to whatever entity type's turn it is or return to tick state
            let ent = (*turn_queue).pop();
            match ent {
                Some(ent) => {
                    if ent == (*player_id) {
                    *state = RunState::PlayerTurn;
                    }
                    else {
                    *state = RunState::AITurn{id: ent};
                    }
                },
                None => {*state = RunState::Tick;}

            }
        },
        // handle player input, and change states
        RunState::PlayerTurn => {
            let player_id = ecs.fetch::<Entity>();
            let map = ecs.fetch::<Map>();
            let act = player::read_keys(tcod, *player_id);
            // TODO: this should be handled by the event queue most likely
            if let actions::Action::MoveAction { id: _, x, y } = act {
                let mut pos_store = ecs.write_storage::<Position>();
                let player_pos = pos_store.get_mut(*player_id);
                if let Some(player_pos) = player_pos{
                    if map.is_exit_valid(player_pos.x + x, player_pos.y + y){
                        player_pos.x += x;
                        player_pos.y += y;
                        // return to turn queue
                        *state = RunState::ActiveTurn;
                    }
                }

            }
        },
        // handle any ai entities turns
        RunState::AITurn{id} => {
            let player = ecs.fetch::<Entity>();
            let map = ecs.write_resource::<Map>();
            let mut pos_store = ecs.write_storage::<Position>();
            let player_pos = pos_store.get(*player);
            if let Some(player_pos) = player_pos{
                // reassign variable to drop borrow of pos_store
                let player_pos = (player_pos.x, player_pos.y);
                let mons_pos = pos_store.get_mut(*id);
                if let Some(mons_pos) = mons_pos {
                    if map.get_distance_sq(mons_pos.x, mons_pos.y, player_pos.0, player_pos.1) > 2.0 {
                        let path = astar_search(map.get_index(mons_pos.x, mons_pos.y), map.get_index(player_pos.0, player_pos.1), &map);
                        if path.success && path.steps.len()>1 {
                            mons_pos.x = path.steps[1] as i32 % map.width;
                            mons_pos.y = path.steps[1] as i32 / map.width;
                        }
                    }
                }
            }
            *state = RunState::ActiveTurn;
        },
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

    // init rng
    let mut rng = thread_rng();

    // register component types with ECS
    ecs.register::<Player>();
    ecs.register::<Position>();
    ecs.register::<Sprite>();
    ecs.register::<Actor>();
    ecs.register::<Speed>();
    ecs.register::<Vision>();
    ecs.register::<Blocking>();

    //create player entity
    let player_entity = ecs.create_entity().with(Actor{action_points:0, threshold: 5})
                                            .with(Position{x: SCREEN_WIDTH/2, y: SCREEN_HEIGHT/2})
                                            .with(Sprite{sprite: '@', color: WHITE })
                                            .with(Speed{speed: 1})
                                            .with(Vision{field_of_vision: HashSet::new()})
                                            .with(Player{}).build();
    
    // create test monstar
    ecs.create_entity().with(Actor{action_points: 0, threshold: 10})
                       .with(Position{x: SCREEN_WIDTH/2, y: SCREEN_HEIGHT/2 + 5})
                       .with(Sprite{sprite: 'o', color: RED})
                       .with(Vision{field_of_vision: HashSet::new()})
                       .with(Speed{speed: 1})
                       .with(Blocking{}).build();

    // create gamestate resources
    let main_menu = gui::Menu::new(3, vec_of_strings!["New Game", "Load Game", "Options", "Quit"]);
    let current_level = Map::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let turn_queue : Vec<Entity> = Vec::new();

    ecs.insert(main_menu);
    ecs.insert(current_level);
    ecs.insert(turn_queue);
    ecs.insert(player_entity);

    // gui init code
    let mut tcod = gui::Tcod::new();

    let mut state = RunState::MainMenu;
    while !tcod.root.window_closed() {
        game_loop(&mut state, &mut tcod, &mut ecs);
    }
}