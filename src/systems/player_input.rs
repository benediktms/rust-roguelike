use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    if let Some(key) = *key {
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
            // skip turn
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        if delta.x != 0 || delta.y != 0 {
            let mut has_target = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    has_target = true;
                    commands.push((
                        (),
                        IntentionToAttack {
                            attacker: player_entity,
                            target: *entity,
                        },
                    ));
                });

            if !has_target {
                commands.push((
                    (),
                    IntentionToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            };
        };

        *turn_state = TurnState::PlayerTurn
    }
}
