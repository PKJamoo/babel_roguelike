use super::{GameState};

const MAX_OCCUPANTS: usize = 8;

struct Map {
    width: i32,
    height: i32,
    terrain: Vec<Tiles>,
    occupied: Vec<Vec<i32>>
}

impl Map {

    pub fn build_wall(&self, game: &mut GameState) {
        // add wall to entity_list with necessary components
    }

    pub fn index_to_coords(&self, index: i32) -> (i32, i32) {
        return (index%self.width, index/self.width);
    }

    pub fn coords_to_index(&self, x: i32, y: i32) -> i32 {
        return x + (y * self.width);
    }

    pub fn make_arena(width: i32, height: i32, game: &mut GameState) -> Self {
        
        
        let mut map = Map {
            width: width,
            height: height,
            terrain: vec![Tiles::Ground; (width*height) as usize],
            occupied: vec![vec![0; MAX_OCCUPANTS]; (width*height) as usize]

        };

        

        return map;

        


    }
}

#[derive(Clone)]
enum Tiles {
    Ground,
    Water
}