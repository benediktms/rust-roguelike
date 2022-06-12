use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    intention_to_move: &IntentionToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(intention_to_move.destination) {
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
