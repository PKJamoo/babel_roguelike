use super::{Actor, Speed};
use specs::{System, ReadStorage, WriteStorage, Entities, Write, Entity};

pub struct TurnSystem;

impl<'a> System<'a> for TurnSystem {
    type SystemData = (Entities<'a>,
                       WriteStorage<'a, Actor>,
                       ReadStorage<'a, Speed>,
                       Write<'a, Vec<Entity>>);


    fn run(&mut self, (entities, mut act, spd,  mut turns): Self::SystemData) {
        use specs::Join;
        for (entity, act, spd) in (&entities, &mut act, &spd).join() {
            act.action_points += spd.speed;
            if act.action_points >= act.threshold {
                act.action_points -= act.threshold;
                turns.push(entity);


            }
        }
    }
}