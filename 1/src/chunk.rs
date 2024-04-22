
#[derive(Debug)]
pub struct ComponentsChunk<TComponent> {
    components: Vec<TComponent>
}

pub fn new_with_capacity<TComponent>(capacity: usize) -> ComponentsChunk<TComponent> {
    ComponentsChunk {
        components: Vec::with_capacity(capacity)
    }
}

pub fn components<TComponent>(chunk: &ComponentsChunk<TComponent>) -> &[TComponent] {
    &chunk.components
}

pub fn components_mut<TComponent>(chunk: &mut ComponentsChunk<TComponent>) -> &mut [TComponent] {
    &mut chunk.components
}

pub fn is_full_filled<TComponent>(chunk: &ComponentsChunk<TComponent>) -> bool {
    chunk.components.capacity() <= chunk.components.len()
}

pub fn push<TComponent>(chunk: &mut ComponentsChunk<TComponent>, component: TComponent) -> usize {
    chunk.components.push(component);
    chunk.components.len() - 1
}