use std::{collections::HashMap, any::{TypeId, Any}, sync::Arc, borrow::BorrowMut};

use uuid::Uuid;

use crate::{entity_id::{EntityId, self}, type_info::TypeInfo};

#[derive(Debug)]
pub enum AddActionError {
    InvalidComponentsCollectionType{
        required: TypeInfo,
        // found: TypeId,
    },
    InvalidComponentType {
        required: TypeInfo,
        found: TypeId,
    },
}

pub struct ComponentsInfo{
    components: Box<dyn Any>,
    push_component: Box<dyn Fn(&mut Box<dyn Any>, Box<dyn Any>) -> Result<(), AddActionError>>,
    // debug_action: Box<dyn Fn(&Box<dyn Any>) -> Result<(), AddActionError>>,
}

impl std::fmt::Debug for ComponentsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentInfo")
            .field("components", &self.components)
            // .field("add_action", &self.add_action)
            // .field("debug_action", &self.debug_action)
            .finish()
    }
}

pub fn new_components_info<TComponent: 'static>() -> ComponentsInfo {
    ComponentsInfo {
        components: Box::new(Vec::<TComponent>::new()),
        push_component: Box::new(|components, component| {

            let compoennts = components.downcast_mut::<Vec<TComponent>>()
                .ok_or(AddActionError::InvalidComponentsCollectionType {
                    required: TypeInfo::from_type::<Vec<TComponent>>(),
                })?;

            let component = Box::<TComponent>::into_inner(component.downcast::<TComponent>()
                .map_err(|e| AddActionError::InvalidComponentType {
                    required: TypeInfo::from_type::<TComponent>(),
                    found: e.type_id(),
                })?);

            compoennts.push(component);

            Ok(())
        }),
        // debug_action: components,
    }
}

pub fn extract_components<'components, TComponent: 'static>(components_info: &'components mut ComponentsInfo) -> Option<&'components mut Vec<TComponent>> {
    components_info.components.downcast_mut::<Vec<TComponent>>()
}

pub fn push_component(components_info: &mut ComponentsInfo, component: Box<dyn Any>) -> Result<(), AddActionError> {
    (components_info.push_component)(&mut components_info.components, component)
}

#[derive(Default, Debug)]
pub struct Chunk {
    components: HashMap<TypeId, ComponentsInfo>,
}

#[derive(Debug)]
pub enum AddEntityError {
    AddActionNotFound,
    ComponentTypeNotFound,
    EntityComponentTypeNotFound,
    InvalidComponentsCollectionType,
    AddActionError(AddActionError),
}

pub fn add(chank: &mut Chunk, components: Vec<(TypeId, Box<dyn Any>)>) -> Result<EntityId, AddEntityError> {
    for (type_id, component) in components {
        let components = chank.components.get_mut(&type_id)
            .ok_or(AddEntityError::ComponentTypeNotFound)?;

        push_component(components, component)
            .map_err(|e| AddEntityError::AddActionError(e))?;
    }

    let components = chank.components.get_mut(&TypeId::of::<EntityId>())
        .ok_or(AddEntityError::EntityComponentTypeNotFound)?;

    let entity_id = EntityId(Uuid::new_v4());

    push_component(components, Box::new(entity_id) as Box<dyn Any>);

    Ok(entity_id)
}

#[derive(Debug)]
pub enum BuildError {
    AddComponentError(AddComponentError)
}

pub fn build(builder: impl FnOnce(&mut Chunk) -> Result<(), AddComponentError>) -> Result<Chunk, BuildError> {
    let mut chunk = Chunk::default();

    _ = builder(&mut chunk)
        .map_err(|e| BuildError::AddComponentError(e))?;

    add_component::<EntityId>(&mut chunk)
        .map_err(|e| BuildError::AddComponentError(e))?;

    Ok(chunk)
}

#[derive(Debug)]
pub enum AddComponentError {
    ComponentAlreadyExists(TypeInfo)
}

pub fn add_component<TComponent: 'static>(
    chunk: &mut Chunk,
) -> Result<(), AddComponentError>
// where
//     TAction: Fn(Box<dyn Any>, &mut Box<dyn Any>) -> Result<(), AddActionError>
{
    let old = chunk.components.try_insert(
        TypeId::of::<TComponent>(),
        new_components_info::<TComponent>(),
    ).map_err(|_| AddComponentError::ComponentAlreadyExists(TypeInfo::from_type::<TComponent>()));

    Ok(())
}

pub fn has<TComponent>(chunk: &Chunk) -> bool {
    todo!("add check component existance")
}

pub fn updated<TComponent>(chunk: &Chunk) -> bool {
    todo!("add check component has updates")
}

pub fn get<'component, TComponents: IComponents>(chank: &'component mut Chunk) -> Option<TComponents::TResult<'component>> {
    TComponents::get(chank)
}
 
pub trait IComponents {
    type TResult<'component>;

    fn get<'component>(chank: &'component mut Chunk) -> Option<Self::TResult<'component>>;
}

impl<T1: 'static, T2: 'static> IComponents for (&mut T1, &mut T2) {
    type TResult<'component> = (&'component mut [T1], &'component mut [T2]);

    fn get<'component>(chank: &'component mut Chunk) -> Option<Self::TResult<'component>> {
        let [component1, component2] = chank.components.get_many_mut([
            &TypeId::of::<T1>(),
            &TypeId::of::<T2>(),
        ])?;

        let t1 = extract_components::<T1>(component1)?;
        let t2 = extract_components::<T2>(component2)?;
        
        Some((t1.borrow_mut(), t2.borrow_mut()))
    }
}


// pub trait IComponentSource<TComponent> {
//     fn get(&mut self) -> Option<[TComponent; 32]>;
// }

// impl<TComponent> IComponentSource<TComponent> for Chunk {
//     fn get(&mut self) -> Option<[TComponent; 32]> {
//         None
//     }
// }

// pub trait IComponentSource<TComponent> {
//     fn get(&mut self) -> Option<[TComponent; 32]>;
// }