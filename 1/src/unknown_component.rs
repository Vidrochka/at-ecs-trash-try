use std::{any::Any, sync::Arc, fmt::Debug};

use tokio::sync::RwLock;
use type_uuid::TypeUuid;
use uuid::Uuid;

use crate::{type_info::TypeInfo, component::{IComponents, self}};

pub trait IUknownComponent where Self: Sync + Send {
    fn into_boxed(self: Box<Self>) -> Box<dyn Any + Sync + Send>;
    fn type_info(&self) -> TypeInfo;
    fn component_uuid(&self) -> Uuid;
    fn new_components_array(&self) -> Arc<RwLock<dyn IComponents>>;
}

impl<TComponent: 'static + Sync + Send + TypeUuid + Debug> IUknownComponent for TComponent {
    fn into_boxed(self: Box<Self>) -> Box<dyn Any + Sync + Send> {
        self
    }

    fn type_info(&self) -> TypeInfo {
        TypeInfo::from_type::<TComponent>()
    }

    fn component_uuid(&self) -> Uuid {
        Uuid::from_bytes(TComponent::UUID)
    }

    fn new_components_array(&self) -> Arc<RwLock<dyn IComponents>> {
        Arc::new(RwLock::new(component::new::<TComponent>()))
    }
}

pub trait IntoComponentsInfo where Self: Sized {
    fn into_components_info(self) -> Vec<Box<dyn IUknownComponent>>;
}

impl IntoComponentsInfo for () {
    fn into_components_info(self) -> Vec<Box<dyn IUknownComponent>> {
        vec![]
    }
}

impl<T1: 'static + Sync + Send + TypeUuid + Debug> IntoComponentsInfo for (T1,) {
    fn into_components_info(self) -> Vec<Box<dyn IUknownComponent>> {
        let (component1, ): (T1,) = self;
        vec![Box::new(component1)]
    }
}

impl<T1: 'static + Sync + Send + TypeUuid + Debug, T2: 'static + Sync + Send + TypeUuid + Debug> IntoComponentsInfo for (T1, T2,) {
    fn into_components_info(self) -> Vec<Box<dyn IUknownComponent>> {
        let (component1, component2): (T1, T2,) = self;
        vec![Box::new(component1), Box::new(component2)]
    }
}

impl<T1: 'static + Sync + Send + TypeUuid + Debug, T2: 'static + Sync + Send + TypeUuid + Debug, T3: 'static + Sync + Send + TypeUuid + Debug> IntoComponentsInfo for (T1, T2, T3,) {
    fn into_components_info(self) -> Vec<Box<dyn IUknownComponent>> {
        let (component1, component2, component3): (T1, T2, T3,) = self;
        vec![Box::new(component1), Box::new(component2), Box::new(component3)]
    }
}