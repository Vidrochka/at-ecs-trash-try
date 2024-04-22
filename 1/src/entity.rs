use type_uuid::TypeUuid;
use uuid::Uuid;

use crate::world::{World, self};

#[derive(Debug, TypeUuid, Clone, Copy)]
#[uuid = "2ac0c046-bf65-4857-9095-0137d418520c"]
pub struct EntityId(Uuid);

pub fn new() -> EntityId {
    EntityId(Uuid::new_v4())
}

// pub fn new(world: &mut World) -> EntityId {
//     let id = EntityId(Uuid::new_v4());

//     let archetype = world::empty_archetype(world);

//     id
// }