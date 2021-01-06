use crate::components::{Position, Blocking};
use crate::map::Map;
use specs::{System, ReadStorage, WriteExpect};

/*
Determine which squares are walkable/unwalkable each tick
*/


pub struct MapBlockingSystem;

impl<'a> System<'a> for MapBlockingSystem {
    type SystemData = (ReadStorage<'a, Position>,
                       ReadStorage<'a, Blocking>,
                       WriteExpect<'a, Map>);

    fn run(&mut self, (pos, blocking, mut map): Self::SystemData) {
        use specs::Join;

        let width = map.width;
        // set blocking for each wall
        map.set_tile_blocked();
        // set blocking for each entity
        for (pos, _) in (&pos, &blocking).join() {
            map.blocked[(pos.x + pos.y*width) as usize] = true;
        }



    }
}