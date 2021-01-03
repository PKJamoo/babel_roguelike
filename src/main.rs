mod player;
mod gui;
mod map;
mod components;
mod actions;
#[macro_use]
mod lib;

mod turnsystem;
use turnsystem::TurnSystem;
mod visionsystem;
use visionsystem::VisionSystem;

use specs::{World, WorldExt, Builder, Entity, RunNow};
use components::{Position, Sprite, Actor, Speed, Vision, Player};
use map::{Map};
use tcod::colors::*;
use rand::prelude::*;
use std::collections::HashSet;


// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

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

    let mut vision_system = VisionSystem{};
    vision_system.run_now(ecs);

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
            run_systems(ecs);
            tcod.render_game(ecs);

            let player_id = ecs.fetch::<Entity>();
            let mut turn_queue = ecs.write_resource::<Vec<Entity>>();
            while !((*turn_queue).is_empty()) {
                if (*turn_queue).remove(0) == (*player_id) {
                    *state = RunState::PlayerTurn;
                }

            }
        },
        RunState::PlayerTurn => {
            let player_id = ecs.fetch::<Entity>();
            let map = ecs.fetch::<Map>();
            let act = player::read_keys(tcod, *player_id);
            // TODO: this should be handled by the event queue most likely
            if let actions::Action::MoveAction { id: _, x, y } = act {
                let mut pos_store = ecs.write_storage::<Position>();
                let player_pos = pos_store.get_mut(*player_id);
                if let Some(player_pos) = player_pos{
                    if map.can_move_to(player_pos.x + x, player_pos.y + y){
                    player_pos.x += x;
                    player_pos.y += y;
                    *state = RunState::ActiveGame;
                    }
                }

            }
        },
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

    // init rng
    let mut rng = thread_rng();

    // register component types with ECS
    ecs.register::<Player>();
    ecs.register::<Position>();
    ecs.register::<Sprite>();
    ecs.register::<Actor>();
    ecs.register::<Speed>();
    ecs.register::<Vision>();

    //create player entity
    let player_entity = ecs.create_entity().with(Actor{action_points:0, threshold: 5})
                                            .with(Position{x: SCREEN_WIDTH/2, y: SCREEN_HEIGHT/2})
                                            .with(Sprite{sprite: '@', color: WHITE })
                                            .with(Speed{speed: 1})
                                            .with(Vision{field_of_vision: HashSet::new()})
                                            .with(Player{}).build();
    
    // create test monstar
    ecs.create_entity().with(Actor{action_points: 0, threshold: 5})
                       .with(Position{x: rng.gen_range(0..(SCREEN_WIDTH - 1)), y: rng.gen_range(0..(SCREEN_HEIGHT - 1))})
                       .with(Sprite{sprite: 'o', color: RED})
                       .with(Vision{field_of_vision: HashSet::new()})
                       .with(Speed{speed: 2}).build();

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