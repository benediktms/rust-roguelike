use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        let delta = match key {
            // arrow keys
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),

            // wasd keys
            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::S => Point::new(0, 1),

            // numpad keys
            VirtualKeyCode::Numpad4 => Point::new(-1, 0),
            VirtualKeyCode::Numpad6 => Point::new(1, 0),
            VirtualKeyCode::Numpad8 => Point::new(0, -1),
            VirtualKeyCode::Numpad2 => Point::new(0, 1),

            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query().filter(component::<Player>());

            players.iter_mut(ecs).for_each(|pos| {
                let desitnation = *pos + delta;

                if map.can_enter_tile(desitnation) {
                    *pos = desitnation;
                    camera.on_player_move(desitnation);
                }
            })
        }
    }
}
