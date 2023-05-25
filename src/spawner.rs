use crate::prelude::*;

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
    ));
}

pub fn spawn_enemy(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph, view_distance) = match rng.roll_dice(1, 10) {
        1..=8 => make_goblin(),
        _ => make_orc(),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        PersuingPlayer,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        FieldOfView::new(view_distance),
    ));
}

pub fn make_goblin() -> (i32, String, FontCharType, i32) {
    (1, "Goblin".to_string(), to_cp437('g'), 5)
}

pub fn make_orc() -> (i32, String, FontCharType, i32) {
    (2, "Orc".to_string(), to_cp437('o'), 6)
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

pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('!'),
        },
        Name("Healing Potion".to_string()),
        ProvidesHealing { amount: 6 },
    ));
}

pub fn spawn_magic_mapper(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('{'),
        },
        Name("Dungeon Map".to_string()),
        ProvidesDungeonMap {},
    ));
}

pub fn spawn_entity(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let roll = rng.roll_dice(1, 6);
    match roll {
        1 => spawn_healing_potion(ecs, pos),
        // TODO: This should only spawn once per map
        2 => spawn_magic_mapper(ecs, pos),
        _ => spawn_enemy(ecs, rng, pos),
    }
}
