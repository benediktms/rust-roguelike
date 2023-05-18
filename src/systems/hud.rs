use std::format;

use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    let mut draw_batch = DrawBatch::new();
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query
        .iter(ecs)
        .next()
        .expect("Player health not found");
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();
    let mut items_query = <(&Item, &Name, &Carried)>::query();
    let mut y = 3;
    items_query
        .iter(ecs)
        .filter(|(_i, _n, carried)| carried.0 == player)
        .for_each(|(_i, name, _c)| {
            draw_batch.target(3);
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
            y += 1;
        });

    if y > 3 {
        draw_batch.target(3);
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.target(2);
    draw_batch.print_centered(1, "Explore the dungeon");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!("Health: {} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );
    draw_batch.submit(10000).expect("Batch error")
}
