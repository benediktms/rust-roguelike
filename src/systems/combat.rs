use crate::prelude::*;

#[system]
#[read_component(IntentionToAttack)]
#[read_component(Damage)]
#[read_component(Carried)]
#[write_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &IntentionToAttack)>::query();

    let targets: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.target))
        .collect();

    targets.iter().for_each(|(message, attacker, target)| {
        let is_player = ecs
            .entry_ref(*target)
            .expect("Unable to find player component")
            .get_component::<Player>()
            .is_ok();

        let base_damage = if let Ok(atk) = ecs.entry_ref(*attacker) {
            if let Ok(damage) = atk.get_component::<Damage>() {
                damage.0
            } else {
                0
            }
        } else {
            0
        };

        let weapon_damage: i32 = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, dmg)| dmg.0)
            .sum();

        let total_damage = base_damage + weapon_damage;

        if let Ok(mut health) = ecs
            .entry_mut(*target)
            .expect("Unable to get heatlh component")
            .get_component_mut::<Health>()
        {
            health.current -= total_damage;

            if health.current < 1 && !is_player {
                commands.remove(*target);
            }
        }

        commands.remove(*message);
    })
}
