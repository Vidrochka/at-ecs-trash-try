use std::any::{TypeId, type_name};

#[derive(Debug)]
pub struct TypeInfo {
    pub id: TypeId,
    pub name: &'static str,
}

impl TypeInfo {
    pub fn from_type<TType: 'static>() -> Self {
        Self {
            id: TypeId::of::<TType>(),
            name: type_name::<TType>(),
        }
    }
}