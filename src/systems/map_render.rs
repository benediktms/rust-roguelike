use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Box<dyn MapTheme>,
    ecs: &SubWorld,
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let point = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let idx = map_idx(x, y);

            // FIXME: stairs seems to obstruct the view of the player
            // this shoudln't happen
            if map.in_bounds(point)
                && (player_fov.visible_tiles.contains(&point) | map.revealed_tiles[idx])
            {
                let tint = if player_fov.visible_tiles.contains(&point) {
                    WHITE
                } else {
                    GREY
                };

                let glyph = theme.tile_to_render(map.tiles[idx]);

                match map.tiles[idx] {
                    TileType::Floor => {
                        draw_batch.set(point - offset, ColorPair::new(tint, BLACK), glyph)
                    }
                    TileType::Wall => {
                        draw_batch.set(point - offset, ColorPair::new(tint, BLACK), glyph)
                    }
                    TileType::Exit => {
                        draw_batch.set(point - offset, ColorPair::new(tint, BLACK), glyph)
                    }
                };
            }
        }
    }

    draw_batch.submit(0).expect("Barch error");
}
