use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Item)]
#[read_component(Carried)]
#[write_component(Health)]
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
            // player actions
            VirtualKeyCode::G => {
                let (player_entity, player_pos) = players
                    .iter(ecs)
                    .find_map(|(entity, pos)| Some((*entity, *pos)))
                    .unwrap();
                let mut items = <(Entity, &Item, &Point)>::query();
                // TODO: I think this will pick up all items at the location at once
                // probably there should be a way to pick up a specific item
                items
                    .iter(ecs)
                    .filter(|(_e, _i, &item_pos)| item_pos == player_pos)
                    .for_each(|(entity, _item, _item_pos)| {
                        commands.remove_component::<Point>(*entity);
                        commands.add_component(*entity, Carried(player_entity));
                    });

                Point::new(0, 0)
            }
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),
            // skip turn
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        let mut has_taken_action = false;

        if delta.x != 0 || delta.y != 0 {
            let mut has_target = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    has_target = true;
                    has_taken_action = true;
                    commands.push((
                        (),
                        IntentionToAttack {
                            attacker: player_entity,
                            target: *entity,
                        },
                    ));
                });

            if !has_target {
                has_taken_action = true;
                commands.push((
                    (),
                    IntentionToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            };
        };

        if !has_taken_action {
            if let Ok(mut health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max, health.current + 1)
            }
        }

        *turn_state = TurnState::PlayerTurn
    }
}

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _)| Some(*entity))
        .unwrap();

    let item_entity = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .enumerate()
        .filter(|(item_count, _)| *item_count == n)
        .find_map(|(_, (entity, _, _))| Some(*entity));

    if let Some(item_entity) = item_entity {
        commands.push((
            (),
            ActivateItem {
                used_by: player,
                item: item_entity,
            },
        ));
    };

    Point::zero()
}
