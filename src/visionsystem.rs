use crate::components::{Vision, Position, Player};
use crate::map::Map;
use specs::{System, ReadStorage, WriteStorage, WriteExpect, Entities};

/*
Calculate field of vision for all active entities
*/

pub struct VisionSystem;

impl<'a> System<'a> for VisionSystem {
    type SystemData = (WriteStorage<'a, Vision>,
                       ReadStorage<'a, Position>,
                       Entities<'a>,
                       ReadStorage<'a, Player>,
                       WriteExpect<'a, Map>);

    fn run(&mut self, (mut vision, pos,  entities, player, mut map): Self::SystemData) {
        use specs::Join;
        for (v, pos, ent) in (&mut vision, &pos, &entities).join() {

            v.field_of_vision = map.get_tiles_in_view(pos.x, pos.y);

            // if this is the player_entity, send their fov to the map
            let _p : Option<&Player> = player.get(ent);
            if let Some(_p) = _p {
                map.visible = v.field_of_vision.clone();
                for tile in v.field_of_vision.iter(){
                        map.visited.insert(*tile);
                    }
                }

        }
    }
}