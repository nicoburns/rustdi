
use super::service::{ServiceReadGuard, ServiceWriteGuard};

pub trait Resolver {
    type Error;

    fn resolve_immutable_ref<S: Send + Sync + 'static>(&self) -> Result<ServiceReadGuard<S>, Self::Error>;
    fn resolve_mutable_ref<S: Send + Sync + 'static>(&self) -> Result<ServiceWriteGuard<S>, Self::Error>;
    fn resolve_owned_value<S: Send + Sync + 'static>(&self) -> Result<S, Self::Error>;
}

pub trait Inject {
    type Return;

    fn inject<R: Resolver>(resolver: R) -> Result<Self::Return, R::Error>;
}