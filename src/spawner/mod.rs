mod template;

use crate::prelude::*;
use template::*;

const INITIAL_PLAYER_MAX_HEALTH: i32 = 20;
const PLAYER_VIEW_DISTANCE: i32 = 8;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: INITIAL_PLAYER_MAX_HEALTH,
            max: INITIAL_PLAYER_MAX_HEALTH,
        },
        Name("You".to_string()),
        FieldOfView::new(PLAYER_VIEW_DISTANCE),
        Damage(1),
    ));
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    resources: &mut Resources,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, resources, level, spawn_points);
}
