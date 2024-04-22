use std::{collections::HashSet, any::{Any, type_name}, fmt::Debug};

use type_uuid::TypeUuid;

use crate::{chunk::{ComponentsChunk, self}, type_info::TypeInfo};

#[derive(Debug)]
pub struct Components<TComponent> where TComponent: Sync + Send + TypeUuid + Debug {
    chunks: Vec<ComponentsChunk<TComponent>>,
}

pub fn new<TComponent: Sync + Send + TypeUuid + Debug>() -> Components<TComponent> {
    Components::<TComponent> {
        chunks: Vec::with_capacity(32),
    }
}

pub fn chunk<TComponent: Sync + Send + TypeUuid + Debug>(components: &Components<TComponent>, chunk_id: usize) -> Option<&ComponentsChunk<TComponent>> {
    components.chunks.get(chunk_id)
}

pub fn chunk_mut<TComponent: Sync + Send + TypeUuid + Debug>(components: &mut Components<TComponent>, chunk_id: usize) -> Option<&mut ComponentsChunk<TComponent>> {
    components.chunks.get_mut(chunk_id)
}

#[derive(Debug)]
pub enum PushError {
    InvalidComponentType { expected: TypeInfo },
    InvalidChunkIndex { index: usize }
}

pub struct ComponentAddress {
    chunk_idx: usize,
    component_idx: usize,
}

pub fn chunk_idx(component_address: &ComponentAddress) -> usize {
    component_address.chunk_idx
}

pub enum PushComponentAction {
    NewChunk { address: ComponentAddress },
    PushToChunk { address: ComponentAddress },
}

pub trait IComponents: Any + Sync + Send + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn push(&mut self, component: Box<dyn Any>, chunk_idxes: &[usize]) -> Result<PushComponentAction, PushError>;
}

impl<TComponent: 'static + Sync + Send + TypeUuid + Debug> IComponents for Components<TComponent> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn push(&mut self, component: Box<dyn Any>, chunk_idxes: &[usize]) -> Result<PushComponentAction, PushError> {
        let component = component.downcast::<TComponent>()
            .map_err(|_| PushError::InvalidComponentType { expected: TypeInfo::from_type::<TComponent>() })?;

        let component = *component;

        for chunk_idx in chunk_idxes {
            let chunk = self.chunks.get_mut(*chunk_idx)
                .ok_or_else(|| PushError::InvalidChunkIndex { index: *chunk_idx })?;

            if !chunk::is_full_filled(chunk) {
                let component_idx = chunk::push(chunk, component);

                return Ok(PushComponentAction::PushToChunk {
                    address: ComponentAddress {
                        chunk_idx: *chunk_idx,
                        component_idx,
                    }
                });
            }
        }
        
        let mut chunk = chunk::new_with_capacity(32);

        let component_idx = chunk::push(&mut chunk, component);

        self.chunks.push(chunk);

        let chunk_idx = self.chunks.len() - 1;

        return Ok(PushComponentAction::NewChunk {
            address: ComponentAddress {
                chunk_idx,
                component_idx,
            }
        });
    }
}

// pub fn push<TComponent>(components: &mut Components<TComponent>, archetype_chunks_ids: &HashSet<usize>) -> usize {

// }
