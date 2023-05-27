use crate::prelude::*;

use legion::systems::CommandBuffer;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/templates.ron").expect("Failed to open template file");

        from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        resources: &mut Resources,
        level: usize,
        spawn_points: &[Point],
    ) {
        // FIXME:  resources need to be loaded here in order to flush the command buffer
        // but this is probably incorrect since `default` sounds like it will reset things
        let mut available_entities = Vec::new();

        self.entities
            .iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|t| {
                // FIXME: this seems still push more than one dungeon map
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            });

        let mut commands = CommandBuffer::new(ecs);

        spawn_points.iter().for_each(|p| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(p, entity, &mut commands);
            }
        });

        commands.flush(ecs, resources);
    }

    pub fn spawn_entity(&self, point: &Point, template: &Template, commands: &mut CommandBuffer) {
        let entity = commands.push((
            point.clone(),
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph),
            },
            Name(template.name.clone()),
        ));

        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item {}),
            EntityType::Enemy => {
                commands.add_component(entity, Enemy {});
                commands.add_component(entity, FieldOfView::new(6));
                commands.add_component(entity, PersuingPlayer {});
                commands.add_component(
                    entity,
                    Health {
                        current: template.hp.unwrap(),
                        max: template.hp.unwrap(),
                    },
                )
            }
        }

        if let Some(provides) = &template.provides {
            provides
                .iter()
                .for_each(|(provides, amount)| match provides.as_str() {
                    "Healing" => {
                        commands.add_component(entity, ProvidesHealing { amount: *amount })
                    }
                    "MagicMap" => commands.add_component(entity, ProvidesDungeonMap {}),
                    _ => {
                        println!("WARNING: Unknown item type: {}", provides);
                    }
                });
        }

        if let Some(damage) = &template.base_damage {
            commands.add_component(entity, Damage(*damage));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Weapon {});
            }
        }
    }
}
