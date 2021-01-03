use tcod::colors::*;
use tcod::console::*;
use specs::{World, WorldExt, Entity};
use super::{SCREEN_HEIGHT, SCREEN_WIDTH, Map};
use crate::map::TileType;
use crate::components::{Position, Sprite};



pub struct Tcod {
    pub root: Root,
    pub con: Offscreen,
}

// struct for all menu interfaces, i.e. lists of different options to choose from.
#[derive(Clone)]
pub struct Menu {
    pub cursor_index: i16,
    pub menu_size: i16,
    pub options: Vec<String>
}

impl Menu {
    pub fn new(num_options: i16, options: Vec<String>) -> Self{
        
        Menu{
            cursor_index: 0,
            menu_size: num_options,
            options: options
        }

    }
}

impl Tcod {

    pub fn new() -> Self {
        let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("BABEL")
        .init();

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    return Tcod { root, con };
    }

    pub fn render_main_menu(&mut self, ecs: &mut  World) {
        let menu = ecs.fetch::<Menu>();
        self.con.set_default_background(BLACK);
        self.con.set_default_foreground(WHITE);
        self.con.set_alignment(TextAlignment::Center);
        self.con.clear();
        self.con.print(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, "BABEL");
        for (pos, option) in menu.options.iter().enumerate() {
            self.con.print(SCREEN_WIDTH/2, SCREEN_HEIGHT/2 + 1 + pos as i32, option);
        }
        self.con.put_char(SCREEN_WIDTH/2 - 5, SCREEN_HEIGHT/2 + 1 + (menu.cursor_index as i32), '>', BackgroundFlag::None);

        blit( &self.con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT),
        &mut self.root, (0, 0), 1.0, 1.0,);
        self.root.flush();
    }

    pub fn render_game(&mut self,  ecs: &mut World) {
        // for map -> render all tiles in view
        // for entities -> render all objects in view
        self.con.set_default_foreground(WHITE);
        self.con.clear();

        // Render map
        let mut current_level = ecs.write_resource::<Map>();
        for tile in current_level.visited.iter() {
          let tile_char: char;
          let tile_color: Color;
          match tile.tile_type {
              TileType::Ground => {tile_char = '.'; tile_color = LIGHT_GREY},
              TileType::Water => {tile_char = '.'; tile_color = BLUE},
              TileType::Wall => {tile_char = 'X'; tile_color = LIGHT_GREY},
          }
          self.con.put_char_ex(tile.x, tile.y, tile_char, tile_color, BLACK);
        }

        // retrieve storages from ecs
        let pos_store = ecs.read_storage::<Position>();
        let sprite_store = ecs.read_storage::<Sprite>();  

        for tile in (*current_level).visible.iter() {
            let tile_char: char;
            let tile_color: Color;
            match tile.tile_type {
                TileType::Ground => {tile_char = '.'; tile_color = LIGHTEST_GREY},
                TileType::Water => {tile_char = '.'; tile_color = LIGHT_BLUE},
                TileType::Wall => {tile_char = 'X'; tile_color = WHITE},
            }
            self.con.put_char_ex(tile.x, tile.y, tile_char, tile_color, BLACK);
        }

            // Render visible entities
            use specs::Join;
            for (pos, sprite) in (&pos_store, &sprite_store).join() {
                // TODO: Fix Map data structure to separate tiles from type
                if (*current_level).tile_in_view(pos.x, pos.y){
                self.con.put_char_ex(pos.x, pos.y, sprite.sprite, sprite.color, BLACK);
                }
            }


        blit( &self.con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut self.root, (0, 0), 1.0, 1.0,
        );

        self.root.flush();
    }
}