use crate::Chunk;



pub fn call_system<'chunk, TProps, TResult>(chunk: &'chunk mut Chunk, builder: impl FnOnce(&'chunk mut Chunk) -> Option<TProps>, system: impl FnOnce(TProps) -> TResult) -> Option<TResult> {
    let props = builder(chunk)?;
    let result = system(props);
    Some(result)
}