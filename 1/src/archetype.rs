use std::collections::{HashMap, BTreeSet};

use type_uuid::TypeUuid;
use uuid::Uuid;

use crate::entity::EntityId;

#[derive(Debug, Clone, Default)]
pub struct Archetype {
    chunk_ids: HashMap<Uuid, Vec<usize>>,
}

pub fn chunk_ids_by_type<TComponent: 'static + TypeUuid>(archetype: &Archetype) -> Option<&[usize]> {
    chunk_ids(archetype, Uuid::from_bytes(TComponent::UUID))
}

pub fn chunk_ids(archetype: &Archetype, component_uuid: Uuid) -> Option<&[usize]> {
    archetype.chunk_ids.get(&component_uuid)
        .map(|x| x.as_ref())
}

pub fn add_chunk_id(archetype: &mut Archetype, type_uuid: Uuid, chunk_id: usize) {
    archetype.chunk_ids.entry(type_uuid)
        .or_default()
        .push(chunk_id)
}

pub fn has<TComponent: 'static + TypeUuid>(archetype: &Archetype) -> bool {
    archetype.chunk_ids.contains_key(&Uuid::from_bytes(TComponent::UUID))
}

pub fn is_empty(archetype: &Archetype) -> bool {
    archetype.chunk_ids.len() == 0 &&
    archetype.chunk_ids.contains_key(&Uuid::from_bytes(EntityId::UUID))
}

pub fn new_empty() -> Archetype {
    let mut archetype = Archetype::default();

    archetype.chunk_ids.insert(Uuid::from_bytes(EntityId::UUID), Vec::new());

    archetype
}

pub fn new(components_info: BTreeSet<Uuid>) -> Archetype {
    Archetype {
        chunk_ids: HashMap::from_iter(components_info.into_iter()
            .map(|id| (id, vec![]))
        )
    }
}