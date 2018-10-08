
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use std::sync::Arc;
use std::convert::Into;
use std::ops::Deref;
use std::ops::DerefMut;
use typemap::{TypeMap, ShareMap, Key};

struct KeyType<T>(T);

// Singleton Service Container
#[derive(Debug)]
pub enum Service<T> {
    SingletonArc(Arc<T>),
    SingletonRwLock(Arc<RwLock<T>>),
    SingletonMutex(Arc<Mutex<T>>),
}

impl<S> Service<S> {
    pub fn read (&self) -> Result<ServiceReadGuard<S>, ()> {
        return match self {
            Service::SingletonArc(service)   => Ok(ServiceReadGuard::Arc(service.clone())),
            Service::SingletonRwLock(service) => Ok(ServiceReadGuard::RwLock(service.read().unwrap())),
            Service::SingletonMutex(service)  => Ok(ServiceReadGuard::Mutex(service.lock().unwrap())),
        }
    }

    pub fn write (&self) -> Result<ServiceWriteGuard<S>, ()> {
        return match self {
            Service::SingletonArc(_)   => Err(()),
            Service::SingletonRwLock(service) => Ok(ServiceWriteGuard::RwLock(service.write().unwrap())),
            Service::SingletonMutex(service)  => Ok(ServiceWriteGuard::Mutex(service.lock().unwrap())),
        }
    }
}

pub enum ServiceReadGuard<'a, T> {
    Arc(Arc<T>),
    RwLock(RwLockReadGuard<'a, T>),
    Mutex(MutexGuard<'a, T>),
}
impl<'a, T> Deref for ServiceReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            ServiceReadGuard::Arc(guard)   => &*guard,
            ServiceReadGuard::RwLock(guard) => &*guard,
            ServiceReadGuard::Mutex(guard)  => &*guard,
        }
    }
}

pub enum ServiceWriteGuard<'a, T> {
    RwLock(RwLockWriteGuard<'a, T>),
    Mutex(MutexGuard<'a, T>),
}
impl<'a, T> Deref for ServiceWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            ServiceWriteGuard::RwLock(guard) => &*guard,
            ServiceWriteGuard::Mutex(guard)  => &*guard,
        }
    }
}
impl<'a, T> DerefMut for ServiceWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            ServiceWriteGuard::RwLock(guard) => &mut *guard,
            ServiceWriteGuard::Mutex(guard)  => &mut *guard,
        }
    }
}

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