
use super::service::{ServiceReadGuard, ServiceWriteGuard};

pub trait Resolver {
    type Error;

    fn resolve_immutable_ref<S: 'static>(&self) -> Result<ServiceReadGuard<S>, Self::Error>;
    fn resolve_mutable_ref<S: 'static>(&self) -> Result<ServiceWriteGuard<S>, Self::Error>;
    fn resolve_owned_value<S: 'static>(&self) -> Result<S, Self::Error>;
}

pub trait Inject<Ret, R: Resolver> {
    type Return;

    fn inject(&self, resolver: &R) -> Result<Self::Return, R::Error>;
}

impl<Ret, R: Resolver, T: Fn(&R) -> Result<Ret, R::Error>> Inject<Ret, R> for T {
    type Return = Ret;

    fn inject(&self, resolver: &R) -> Result<Self::Return, R::Error> {
        self(resolver)
    }
}