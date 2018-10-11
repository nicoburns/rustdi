
use super::service::{ServiceReadGuard, ServiceWriteGuard};

pub trait Resolver {
    type Error;

    fn resolve_immutable_ref<S: 'static>(&self) -> Result<ServiceReadGuard<S>, Self::Error>;
    fn resolve_mutable_ref<S: 'static>(&self) -> Result<ServiceWriteGuard<S>, Self::Error>;
    fn resolve_owned_value<S: 'static>(&self) -> Result<S, Self::Error>;
}

pub trait Inject {
    type Return;

    fn inject<R: Resolver>(&self, resolver: R) -> Result<Self::Return, R::Error>;
}