use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::Arc;
use std::marker::PhantomData;
use typemap::{TypeMap, ShareMap, Key};

use crate::service::{Service, ServiceReadGuard, ServiceWriteGuard};

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
}

// Resolving methods which allow services to be retrieved from the Service Container
impl ServiceContainer {
    pub fn resolve<S: Send + Sync + 'static> (&self) -> Result<&Service<S>, ()> {
        match self.services.get::<KeyType<S>>() {
            Some(service) => Ok(service),
            None          => Err(()),
        }
    }

    pub fn resolve_immutable_ref<S: Send + Sync + 'static> (&self) -> Result<ServiceReadGuard<S>, ()> {
        match self.services.get::<KeyType<S>>() {
            Some(service) => service.immutable_ref(),
            _             => Err(()),
        }
    }

    pub fn resolve_mutable_ref<S: Send + Sync + 'static> (&self) -> Result<ServiceWriteGuard<S>, ()> {
        match self.services.get::<KeyType<S>>() {
            Some(service) => service.mutable_ref(),
            _             => Err(()),
        }
    }
}