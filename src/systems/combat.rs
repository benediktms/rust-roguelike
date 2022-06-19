use crate::prelude::*;

#[system]
#[read_component(IntentionToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &IntentionToAttack)>::query();

    let targets: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.target))
        .collect();

    targets.iter().for_each(|(message, target)| {
        if let Ok(mut health) = ecs
            .entry_mut(*target)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attach: {}", health.current);
            health.current -= 1;

            if health.current < 1 {
                commands.remove(*target);
            }

            println!("Health after attack {}", health.current);
        }

        commands.remove(*message);
    })
}
