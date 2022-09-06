use crate::prelude::*;

#[rustfmt::skip]
#[allow(dead_code)]
const FORTRESS: (&str, i32, i32) = ("
------------
---######---
---#----#---
---#-M--#---
-###----###-
---M----M---
-###----###-
---#--M-#---
---#----#---
---######---
------------
", 12, 11);

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut placement = None;

    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &vec![mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0,
    );

    let mut attempts = 0;

    while placement.is_none() && attempts < 10 {
        let dimensions = Rect::with_size(
            rng.range(0, SCREEN_WIDTH),
            rng.range(0, SCREEN_HEIGHT),
            FORTRESS.1,
            FORTRESS.2,
        );

        let mut can_place = false;

        dimensions.for_each(|p| {
            let idx = mb.map.point2d_to_index(p);
            let distance = dijkstra_map.map[idx];
            if distance < 2000.0 && distance > 20.0 && mb.amulet_start != p {
                can_place = true;
            }
        });

        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spawns.retain(|p| !points.contains(p));
        }

        attempts += 1
    }

    if let Some(placement) = placement {
        let string_vec: Vec<char> = FORTRESS
            .0
            .chars()
            .filter(|a| *a != '\r' && *a != '\n')
            .collect();

        let mut i = 0;

        for y in placement.y..placement.y + FORTRESS.2 {
            for x in placement.x..placement.x + FORTRESS.1 {
                let idx = map_idx(x, y);
                let c = string_vec[i];

                match c {
                    'M' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(x, y));
                    }
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    _ => println!("Unknown tile `{}` found. Skipping", c),
                }
            }
            i += 1;
        }
    }
}
