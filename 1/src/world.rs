use std::{collections::{HashMap, BTreeSet}, any::Any, sync::Arc, future::Future, fmt::Debug};

// use async_lock::{RwLock, futures::{Write, Read}, RwLockWriteGuard, RwLockReadGuard};
use tokio::sync::{RwLock, RwLockWriteGuard, RwLockMappedWriteGuard, RwLockReadGuard};
use futures::future::BoxFuture;
use itertools::Itertools;
use type_uuid::TypeUuid;
use uuid::Uuid;

use crate::{archetype::{Archetype, self}, component::{Components, IComponents, self}, entity::{EntityId, self}, unknown_component::IntoComponentsInfo};


#[derive(Debug, Default)]
pub struct World {
    archetypes: HashMap<BTreeSet<Uuid>, Archetype>,
    components: HashMap<Uuid, Arc<RwLock<dyn IComponents>>>
}

pub fn archetypes<'world: 'arch, 'arch>(world: &'world World, keys: &Vec<BTreeSet<Uuid>>) -> Vec<&'arch Archetype> {
    keys.iter()
        .map(|uuids| world.archetypes.get(uuids))
        .flatten()
        .collect_vec()
}

// pub fn empty_archetype(world: &mut World) -> &mut Archetype {
//     if let Some(archetype) = world.archetypes.iter_mut().find(|x| archetype::is_empty(x)) {
//         return archetype;
//     }

//     world.archetypes.push(archetype::new_empty());

//     world.archetypes.last_mut().unwrap()
// }

pub async fn add_entity(world: &mut World, components: impl IntoComponentsInfo) -> EntityId {
    let entity_id = entity::new();

    let components = components.into_components_info();

    let mut components = components.into_iter()
        .dedup_by(|c1, c2| c1.component_uuid() == c2.component_uuid())
        .filter(|x| x.component_uuid() != Uuid::from_bytes(EntityId::UUID))
        .collect_vec();

    components.push(Box::new(entity_id));

    let components_uuid = components.iter()
        .map(|x| x.component_uuid())
        .collect::<BTreeSet<_>>();

    let archetype = world.archetypes.entry(components_uuid.clone())
        .or_insert_with(|| archetype::new(components_uuid));

    for component in components {
        let component_uuid = component.component_uuid();

        let components = world.components.entry(component.component_uuid())
            .or_insert_with(|| component.new_components_array());

        let mut components_read_guard = components.write().await;

        let boxed_component = component.into_boxed();

        let chunk_ids = archetype::chunk_ids(archetype, component_uuid).unwrap();

        let result = components_read_guard.push(boxed_component, chunk_ids);

        let push_action = result.unwrap();

        match push_action {
            crate::component::PushComponentAction::NewChunk { address } => {
                archetype::add_chunk_id(archetype, component_uuid, component::chunk_idx(&address));
                // TODO: добавлять в lookup карты в архетип и мир
            },
            crate::component::PushComponentAction::PushToChunk { address } => {
                // если добавили компонент в существующий чане, нет смысла обновлять список чанков
                // TODO: добавлять в lookup карты в архетип и мир
            },
        }
    }

    entity_id
}

pub fn query(world: &World, filter: impl Fn(&Archetype) -> bool) -> Vec<BTreeSet<Uuid>> {
    world.archetypes.iter() 
        .filter(|(_key, archetype)| filter(archetype))
        .map(|(key, _archetype)| key.clone())
        .collect()
}

pub async fn get<'access, TAccessQuery>(world: &'access World) -> Option<TAccessQuery::TAccess<'access>>
where
    TAccessQuery: IAccessManager,
{
    TAccessQuery::extract(world).await
}

pub trait IAccessManager {
    type TAccess<'access>: 'access;

    async fn extract<'access>(world: &'access World) -> Option<Self::TAccess<'access>>;
}

pub trait IAccessVariant {
    type TAccess<'access>: 'access;

    async fn extract<'access>(components: &'access Arc<RwLock<dyn IComponents>>) -> Self::TAccess<'access>;

    fn type_uuid() -> Uuid;
}

impl<T: 'static + Sync + Send + Debug> IAccessVariant for &mut T
where T: TypeUuid
{
    type TAccess<'access> = RwLockMappedWriteGuard<'access, Components<T>>;
    
    async fn extract<'access>(components: &'access Arc<RwLock<dyn IComponents>>) -> Self::TAccess<'access> {
        let guard = components.write().await;
        let components = RwLockWriteGuard::map(guard, |guard| guard.as_mut_any().downcast_mut::<Components<T>>().unwrap());

        components
    }

    fn type_uuid() -> Uuid {
        Uuid::from_bytes(T::UUID)
    }
}

impl<T: 'static + Sync + Send + Debug> IAccessVariant for &T
where T: TypeUuid
{
    type TAccess<'access> = RwLockReadGuard<'access, Components<T>>;

    async fn extract<'access>(components: &'access Arc<RwLock<dyn IComponents>>) -> Self::TAccess<'access> {
        let guard = components.read().await;
        let components = RwLockReadGuard::map(guard, |guard| guard.as_any().downcast_ref::<Components<T>>().unwrap());

        components
    }

    fn type_uuid() -> Uuid {
        Uuid::from_bytes(T::UUID)
    }
}

pub type WriteComponents<'access, T> = RwLockMappedWriteGuard<'access, Components<T>>;
pub type ReadComponents<'access, T> = RwLockReadGuard<'access, Components<T>>;

impl<T1: IAccessVariant, T2: IAccessVariant> IAccessManager for (T1, T2) {
    type TAccess<'access> = (T1::TAccess<'access>, T2::TAccess<'access>);

    async fn extract<'world>(world: &'world World) -> Option<Self::TAccess<'world>> {
        let mut uuids = vec![
            T1::type_uuid(),
            T2::type_uuid(),
        ];

        uuids.sort();

        let mut components_t1 = None;
        let mut components_t2 = None;

        for uuid in uuids {
            if T1::type_uuid() == uuid {
                let component_1 = world.components.get(&T1::type_uuid())?;
                components_t1 = Some(T1::extract(component_1).await);
                continue;
            }

            if T2::type_uuid() == uuid {
                let component_2 = world.components.get(&T2::type_uuid())?;
                components_t2 = Some(T2::extract(&component_2).await);
                continue;
            }
        }

        return Some((components_t1?, components_t2?));
    }
}

impl<T1: IAccessVariant, T2: IAccessVariant, T3: IAccessVariant> IAccessManager for (T1, T2, T3) {
    type TAccess<'access> = (T1::TAccess<'access>, T2::TAccess<'access>, T3::TAccess<'access>);

    async fn extract<'world>(world: &'world World) -> Option<Self::TAccess<'world>> {
        let mut uuids = vec![
            T1::type_uuid(),
            T2::type_uuid(),
            T3::type_uuid(),
        ];

        uuids.sort();

        let mut components_t1 = None;
        let mut components_t2 = None;
        let mut components_t3 = None;

        for uuid in uuids {
            if T1::type_uuid() == uuid {
                let component_1 = world.components.get(&T1::type_uuid())?;
                components_t1 = Some(T1::extract(component_1).await);
                continue;
            }

            if T2::type_uuid() == uuid {
                let component_2 = world.components.get(&T2::type_uuid())?;
                components_t2 = Some(T2::extract(&component_2).await);
                continue;
            }

            if T3::type_uuid() == uuid {
                let component_3 = world.components.get(&T3::type_uuid())?;
                components_t3 = Some(T3::extract(&component_3).await);
                continue;
            }
        }

        return Some((components_t1?, components_t2?, components_t3?));
    }
}