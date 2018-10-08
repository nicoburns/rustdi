use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::Arc;
use std::marker::PhantomData;
use typemap::{TypeMap, ShareMap, Key};

use super::service::{Service, ServiceReadGuard, ServiceWriteGuard};
use super::traits::Resolver;
use super::resolve_error::ResolveError;

// TypeMap requires us to use key and value types
struct KeyType<T>(PhantomData<T>);
impl<T: 'static> Key for KeyType<T> { type Value = Service<T>; }

// The ServiceContainer itself: just a wrapper around a TypeMap<Send + Sync>
pub struct ServiceContainer {
    services: ShareMap
}
impl ServiceContainer {
    pub fn new () -> Self {
        ServiceContainer{services: TypeMap::custom()}
    }
}

// Binding methods which allow services to be added to the Service Container
impl ServiceContainer {
    pub fn bind_singleton_arc<S: Send + Sync + 'static> (&mut self, service: Arc<S>) {
        let value = Service::SingletonArc(service);
        self.services.insert::<KeyType<S>>(value);
    }

    pub fn bind_singleton_rwlock<S: Send + Sync + 'static> (&mut self, service: Arc<RwLock<S>>) {
        let value = Service::SingletonRwLock(service);
        self.services.insert::<KeyType<S>>(value);
    }

    pub fn bind_singleton_mutex<S: Send + Sync + 'static> (&mut self, service: Arc<Mutex<S>>) {
        let value = Service::SingletonMutex(service);
        self.services.insert::<KeyType<S>>(value);
    }

    pub fn bind_factory<S: Send + Sync + 'static> (&mut self, factory: fn() -> S) {
        let value = Service::Factory(Arc::new(factory));
        self.services.insert::<KeyType<S>>(value);
    }
}

// Resolving methods which allow services to be retrieved from the Service Container
impl Resolver for ServiceContainer {
    type Error = ResolveError;

    fn resolve_owned_value<S: Send + Sync + 'static> (&self) -> Result<S, ResolveError> {
        match self.services.get::<KeyType<S>>() {
            Some(service) => service.owned_value(),
            None          => Err(ResolveError::NonExist),
        }
    }

    fn resolve_immutable_ref<S: Send + Sync + 'static> (&self) -> Result<ServiceReadGuard<S>, ResolveError> {
        match self.services.get::<KeyType<S>>() {
            Some(service) => service.immutable_ref(),
            _             => Err(ResolveError::NonExist),
        }
    }

    fn resolve_mutable_ref<S: Send + Sync + 'static> (&self) -> Result<ServiceWriteGuard<S>, ResolveError> {
        match self.services.get::<KeyType<S>>() {
            Some(service) => service.mutable_ref(),
            _             => Err(ResolveError::NonExist),
        }
    }
}