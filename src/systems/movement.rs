use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    intention_to_move: &IntentionToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if let Ok(entry) = ecs.entry_ref(intention_to_move.entity) {
        if let Ok(fov) = entry.get_component::<FieldOfView>() {
            commands.add_component(intention_to_move.entity, fov.clone_dirty());

            if entry.get_component::<Player>().is_ok() {
                camera.on_player_move(intention_to_move.destination);
                fov.visible_tiles
                    .iter()
                    .for_each(|pos| map.revealed_tiles[map_idx(pos.x, pos.y)] = true)
            }
        }

        if entry.get_component::<Player>().is_ok() {
            camera.on_player_move(intention_to_move.destination);
        }
    }

    if map.can_enter_tile(intention_to_move.destination) {
        // adding a component that already exists will replace it instead
        // this is how movement/entity positions get updated via the IntentionToMove message
        commands.add_component(intention_to_move.entity, intention_to_move.destination);

        if ecs
            .entry_ref(intention_to_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(intention_to_move.destination);
        }
    }

    commands.remove(*entity);
}
