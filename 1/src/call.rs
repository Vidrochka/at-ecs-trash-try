use std::{any::TypeId, sync::Arc};

use futures::{future::BoxFuture, Future};
use tokio::sync::RwLock;

use crate::{world::World, system::ISystem};


// pub async fn system<'system_call, TQuery, TQueryFut, TQueryResult, TPropsBuilder, TPropsBuilderFut, TProps, TSystem, TSystemFut>(world: &'system_call World, query: TQuery, props_builder: TPropsBuilder, system: TSystem)
// where
//     for<'world> TQuery: FnOnce(&'system_call World) -> TQueryFut + 'system_call,
//     TQueryFut: Future<Output = Option<TQueryResult>>,
//     TQueryResult: 'system_call,
//     for<'props_builder> TPropsBuilder: FnOnce(TQueryResult) -> TPropsBuilderFut + 'system_call,
//     TPropsBuilderFut: Future<Output = TProps>,
//     TProps: 'system_call,
//     for<'system> TSystem: FnOnce(TProps) -> TSystemFut + 'system_call,
//     TSystemFut: Future<Output = ()>,
//     TSystem: 'static,
// {
//     let system_id = TypeId::of::<TSystem>();
//     println!("{system_id:?}");

//     let Some(result) = query(world).await else {
//         return;
//     };

//     let props = props_builder(result).await;

//     system(props).await;
// }

pub async fn system<'system_call, TSystem: ISystem>(mut system: TSystem, world: Arc<RwLock<World>>) {
    let world = world.read().await;

    let Some(props) = system.query(&world).await else {
        return;
    };

    system.system(props, &world).await;
}