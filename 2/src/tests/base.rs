use std::any::TypeId;

use crate::{Chunk, get, system::call_system, build, add_component, add};

#[derive(Debug)]
pub struct Speed {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Debug)]
pub struct Position {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

pub struct MoveSystemProps<'chunk> {
    pub speed: &'chunk mut [Speed],
    pub position: &'chunk mut [Position],
    pub counter: u32,
}

pub fn move_system(MoveSystemProps{ speed, position, .. }: MoveSystemProps) -> () {
    for (position, speed) in speed.iter_mut().zip(position) {
        position.x += speed.x;
        position.y += speed.y;
        position.z += speed.z;
    }


    ()
}

pub fn move_system_prepare<'chunk>(chunk: &'chunk mut Chunk) -> Option<MoveSystemProps<'chunk>> {
    let (speed, position) = get::<(&mut Speed, &mut Position)>(chunk)?;

    Some(MoveSystemProps {
        speed,
        position,
        counter: 10
    })
}

#[test]
fn base() {
    // let mut chunk = Default::default();

    let mut chunk = build(|chunk| {
        add_component::<Speed>(chunk)?;
        add_component::<Position>(chunk)?;

        Ok(())
    }).unwrap();

    add(&mut chunk, vec![
        (TypeId::of::<Speed>(), Box::new(Speed { x: 1, y: 2, z: 0 })),
        (TypeId::of::<Position>(), Box::new(Position { x: 0, y: 0, z: 0 })),
    ]).unwrap();

    let result = call_system(&mut chunk, move_system_prepare, move_system);

    println!("{chunk:#?}")
}