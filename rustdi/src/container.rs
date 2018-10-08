use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::Arc;
use std::marker::PhantomData;
use typemap::{TypeMap, ShareMap, Key};

use crate::service::{Service, ServiceReadGuard, ServiceWriteGuard};

// TypeMap requires us to use key and value types
struct KeyType<T>(PhantomData<T>);
impl<T: 'static> Key for KeyType<T> { type Value = Service<T>; }

pub struct ServiceContainer {
    services: ShareMap
}

impl ServiceContainer {

    pub fn new () -> Self {
        ServiceContainer{services: TypeMap::custom()}
    }

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

    pub fn resolve<S: Send + Sync + 'static> (&self) -> Result<&Service<S>, ()> {
        match self.services.get::<KeyType<S>>() {
            Some(value) => Ok(value),
            None        => Err(()),
        }
    }

    pub fn resolve_read<S: Send + Sync + 'static> (&self) -> Result<ServiceReadGuard<S>, ()> {
        match self.services.get::<KeyType<S>>() {
            Some(value) => value.read(),
            _           => Err(()),
        }
    }

    pub fn resolve_write<S: Send + Sync + 'static> (&self) -> Result<ServiceWriteGuard<S>, ()> {
        match self.services.get::<KeyType<S>>() {
            Some(value) => value.write(),
            _           => Err(()),
        }
    }

}