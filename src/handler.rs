use std::marker::PhantomData;
use crate::errors::ProjectionError;
use crate::event::Event;
use crate::mapping::ResolveMapping;
use crate::projection::Projection;

#[async_trait::async_trait]
pub(crate) trait Handler<T: ResolveMapping>: 'static + Sync + Send {
    async fn apply(&self, actor: &mut Option<T>, payload: Vec<u8>, seq: &mut u64) -> Result<(), ProjectionError>;
}

pub struct ProjectionResolver<T: ResolveMapping, E: Event> {
    _projection: PhantomData<T>,
    _event: PhantomData<E>
}

impl<T: ResolveMapping, E: Event> Default for ProjectionResolver<T, E> {
    fn default() -> Self {
        Self { _projection: Default::default(), _event: Default::default() }
    }
}

#[async_trait::async_trait]
impl<T: ResolveMapping, E: Event> Handler<T> for ProjectionResolver<T, E> 
    where T: Projection<E>
{
    async fn apply(&self, actor: &mut Option<T>, payload: Vec<u8>, seq: &mut u64) -> Result<(), ProjectionError> {
        let event = E::from_bytes(&payload)?;
        T::projection(actor, event, seq).await.map_err(|_| ProjectionError::Projection)?;
        Ok(())
    }
}
