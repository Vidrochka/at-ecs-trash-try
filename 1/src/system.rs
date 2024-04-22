use type_uuid::TypeUuid;

use crate::world::World;


pub trait ISystem: TypeUuid + Sync + Send {
    type TProps<'frame>;

    async fn query<'frame>(&mut self, world: &'frame World) -> Option<Self::TProps<'frame>>;
    async fn system<'frame>(&mut self, props: Self::TProps<'frame>, world: &'frame World);
}