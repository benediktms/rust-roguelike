use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(Enemy)]
pub fn collision(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_pos = *pos);

    let mut enemies = <(&Point, Entity)>::query().filter(component::<Enemy>());
    enemies
        .iter(ecs)
        .filter(|(pos, _)| **pos == player_pos)
        .for_each(|(_, entity)| {
            commands.remove(*entity);
        });
}
