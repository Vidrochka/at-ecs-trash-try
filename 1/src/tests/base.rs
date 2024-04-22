#[cfg(test)]
pub mod base {
    use tokio::sync::RwLock;
    use futures::FutureExt;
    use itertools::izip;
    use type_uuid::TypeUuid;

    use crate::{archetype::{Archetype, self}, component, chunk, world::{self, World, WriteComponents, ReadComponents}, call, entity::EntityId, system::ISystem};

    #[derive(Debug, TypeUuid)]
    #[uuid = "2ac0c046-bf65-4857-9095-0137d418521c"]
    pub struct Speed {
        pub x: u32,
        pub y: u32,
        pub z: u32,
    }
    
    #[derive(Debug, TypeUuid)]
    #[uuid = "2ac0c046-bf65-4857-9095-0137d418522c"]
    pub struct Position {
        pub x: u32,
        pub y: u32,
        pub z: u32,
    }

    #[derive(Debug, TypeUuid)]
    #[uuid = "48aa0287-11c0-490c-bd8d-2bce62d9c6ed"]
    pub struct MoveSystem {
        offset: u32,
    }

    pub struct MoveSystemProps<'system> {
        pub archetypes: Vec<&'system Archetype>,
        pub entity_id: ReadComponents<'system, EntityId>,
        pub speed: ReadComponents<'system, Speed>,
        pub position: WriteComponents<'system, Position>,
    }

    impl ISystem for MoveSystem {
        type TProps<'frame> = MoveSystemProps<'frame>;

        async fn query<'frame>(&mut self, world: &'frame World) -> Option<Self::TProps<'frame>> {
            let archetypes_ids = world::query(world, |x| {
                archetype::has::<Speed>(x) &&
                archetype::has::<Position>(x)
            });
    
            let archetypes =  world::archetypes(world, &archetypes_ids);
    
            if archetypes.len() == 0 {
                return None;
            }
    
            let (
                entity_id,
                position,
                speed
            ) = world::get::<(&EntityId, &mut Position, &Speed)>(world).await?;
    
            Some(MoveSystemProps {
                archetypes,
                entity_id: entity_id,
                speed: speed,
                position: position,
            })
        }

        async fn system<'frame>(&mut self, MoveSystemProps {
            archetypes,
            speed,
            entity_id,
            mut position,
        }: Self::TProps<'frame>, _world: &'frame World) {
            for archetype in archetypes {
                let entity_id_chunk_ids = archetype::chunk_ids_by_type::<EntityId>(&archetype);
                let speed_chunk_ids = archetype::chunk_ids_by_type::<Speed>(&archetype);
                let position_chunk_ids = archetype::chunk_ids_by_type::<Position>(&archetype);
        
                let (
                    Some(entity_id_chunk_ids),
                    Some(speed_chunk_ids),
                    Some(position_chunk_ids)
                ) = (entity_id_chunk_ids, speed_chunk_ids, position_chunk_ids) else {
                    continue;
                };
        
                for (entity_id_chunk_id, speed_chunk_id, position_chunk_id) in izip!(entity_id_chunk_ids, speed_chunk_ids, position_chunk_ids) {
                    let entity_id_chunk = component::chunk(&entity_id, *entity_id_chunk_id);
                    let speed_chunk = component::chunk(&speed, *speed_chunk_id);
                    let position_chunk = component::chunk_mut(&mut position, *position_chunk_id);
        
                    let (
                        Some(entity_id_chunk),
                        Some(speed_chunk),
                        Some(position_chunk)
                    ) = (entity_id_chunk, speed_chunk, position_chunk) else {
                        continue;
                    };
        
                    let entity_id_components = chunk::components(entity_id_chunk);
                    let speed_components = chunk::components(speed_chunk);
                    let position_components = chunk::components_mut(position_chunk);
        
                    for (entity_id, speed, position) in izip!(entity_id_components, speed_components, position_components) {
                        println!("{entity_id:?}");
    
                        position.x += speed.x + self.offset;
                        position.y += speed.y + self.offset;
                        position.z += speed.z + self.offset;
                    }
                }
            }
        }
    }
    
    // pub async fn move_system_query(world: &World) -> Option<(
    //     Vec<&Archetype>,
    //     ReadComponents<EntityId>,
    //     WriteComponents<Position>,
    //     ReadComponents<Speed>
    // )> {
    //     let archetypes_ids = world::query(world, |x| {
    //         archetype::has::<Speed>(x) &&
    //         archetype::has::<Position>(x)
    //     });

    //     let archetypes =  world::archetypes(world, &archetypes_ids);

    //     if archetypes.len() == 0 {
    //         return None;
    //     }

    //     let (
    //         entity_id,
    //         position,
    //         speed
    //     ) = world::get::<(&EntityId, &mut Position, &Speed)>(world).await?;

    //     Some((
    //         archetypes,
    //         entity_id,
    //         position,
    //         speed,
    //     ))
    // }

    // pub async fn move_system_props_builder<'system>((
    //     archetypes,
    //     entity_id,
    //     position,
    //     speed,
    // ): (
    //     Vec<&'system Archetype>,
    //     ReadComponents<'system, EntityId>,
    //     WriteComponents<'system, Position>,
    //     ReadComponents<'system, Speed>,
    // )) -> MoveSystemProps<'system> {
    //     MoveSystemProps {
    //         archetypes,
    //         entity_id: entity_id,
    //         speed: speed,
    //         position: position,
    //         counter: 0,
    //     }
    // }
    
    // async fn move_system<'system>(MoveSystemProps {
    //         archetypes,
    //         speed,
    //         entity_id,
    //         mut position,
    //         counter
    //     }:
    //     MoveSystemProps<'system>
    // ) {
    //     for archetype in archetypes {
    //         let entity_id_chunk_ids = archetype::chunk_ids_by_type::<EntityId>(&archetype);
    //         let speed_chunk_ids = archetype::chunk_ids_by_type::<Speed>(&archetype);
    //         let position_chunk_ids = archetype::chunk_ids_by_type::<Position>(&archetype);
    
    //         let (
    //             Some(entity_id_chunk_ids),
    //             Some(speed_chunk_ids),
    //             Some(position_chunk_ids)
    //         ) = (entity_id_chunk_ids, speed_chunk_ids, position_chunk_ids) else {
    //             continue;
    //         };
    
    //         for (entity_id_chunk_id, speed_chunk_id, position_chunk_id) in izip!(entity_id_chunk_ids, speed_chunk_ids, position_chunk_ids) {
    //             let entity_id_chunk = component::chunk(&entity_id, *entity_id_chunk_id);
    //             let speed_chunk = component::chunk(&speed, *speed_chunk_id);
    //             let position_chunk = component::chunk_mut(&mut position, *position_chunk_id);
    
    //             let (
    //                 Some(entity_id_chunk),
    //                 Some(speed_chunk),
    //                 Some(position_chunk)
    //             ) = (entity_id_chunk, speed_chunk, position_chunk) else {
    //                 continue;
    //             };
    
    //             let entity_id_components = chunk::components(entity_id_chunk);
    //             let speed_components = chunk::components(speed_chunk);
    //             let position_components = chunk::components_mut(position_chunk);
    
    //             for (entity_id, speed, position) in izip!(entity_id_components, speed_components, position_components) {
    //                 print!("{entity_id:?}");

    //                 position.x += speed.x;
    //                 position.y += speed.y;
    //                 position.z += speed.z;
    //             }
    //         }
    //     }
    // }
    
    #[tokio::test]
    async fn base() {
        let world = std::sync::Arc::new(RwLock::new(World::default()));

        {
            let mut world = world.write().await;
            for i in 0..2 {
                let entity_id = world::add_entity(&mut world, (
                    Position {
                        x: 0,
                        y: 0,
                        z: 0,
                    },
                    Speed {
                        x: i,
                        y: 2,
                        z: 0,
                    },
                )).await;
            }
            for _ in 0..2 {
                let entity_id = world::add_entity(&mut world, (
                    Position {
                        x: 0,
                        y: 0,
                        z: 0,
                    },
                )).await;
            }
            for i in 0..2 {
                let entity_id = world::add_entity(&mut world, (
                    Speed {
                        x: i,
                        y: 2,
                        z: 0,
                    },
                )).await;
            }
        }

        let world_clone = world.clone();
        
        let task1 = tokio::spawn(async move {            
            call::system(MoveSystem { offset: 5 }, world_clone).await;
        });

        let world_clone2 = world.clone();

        let task2 = tokio::spawn(async move {
            call::system(MoveSystem { offset: 2 }, world_clone2).await;
        });

        task1.await.unwrap();
        task2.await.unwrap();

        println!("{world:#?}");
    }
}