
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
pub enum ServiceValue<T: Service> {
    Bare(Arc<T>),
    RwLock(Arc<RwLock<T>>),
    Mutex(Arc<Mutex<T>>),
}
impl<T: Service + Send + Sync> From<Arc<T>> for ServiceValue<T> {
    fn from(service: Arc<T>) -> Self { return ServiceValue::Bare(service); }
}
impl<T: Service + Send + Sync> From<Arc<RwLock<T>>> for ServiceValue<T> {
    fn from(service: Arc<RwLock<T>>) -> Self { return ServiceValue::RwLock(service); }
}
impl<T: Service + Send> From<Arc<Mutex<T>>> for ServiceValue<T> {
    fn from(service: Arc<Mutex<T>>) -> Self { return ServiceValue::Mutex(service); }
}
impl<S: Service> ServiceValue<S> {
    pub fn read (&self) -> Result<ServiceValueReadGuard<S>, ()> {
        return match self {
            ServiceValue::Bare(service)   => Ok(ServiceValueReadGuard::Bare(service.clone())),
            ServiceValue::RwLock(service) => Ok(ServiceValueReadGuard::RwLock(service.read().unwrap())),
            ServiceValue::Mutex(service)  => Ok(ServiceValueReadGuard::Mutex(service.lock().unwrap())),
        }
    }

    pub fn write (&self) -> Result<ServiceValueWriteGuard<S>, ()> {
        return match self {
            ServiceValue::Bare(_)   => Err(()),
            ServiceValue::RwLock(service) => Ok(ServiceValueWriteGuard::RwLock(service.write().unwrap())),
            ServiceValue::Mutex(service)  => Ok(ServiceValueWriteGuard::Mutex(service.lock().unwrap())),
        }
    }
}

pub enum ServiceValueReadGuard<'a, T: Service> {
    Bare(Arc<T>),
    RwLock(RwLockReadGuard<'a, T>),
    Mutex(MutexGuard<'a, T>),
}
impl<'a, T: Service> Deref for ServiceValueReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            ServiceValueReadGuard::Bare(guard)   => &*guard,
            ServiceValueReadGuard::RwLock(guard) => &*guard,
            ServiceValueReadGuard::Mutex(guard)  => &*guard,
        }
    }
}

pub enum ServiceValueWriteGuard<'a, T: Service> {
    RwLock(RwLockWriteGuard<'a, T>),
    Mutex(MutexGuard<'a, T>),
}
impl<'a, T: Service> Deref for ServiceValueWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            ServiceValueWriteGuard::RwLock(guard) => &*guard,
            ServiceValueWriteGuard::Mutex(guard)  => &*guard,
        }
    }
}
impl<'a, T: Service> DerefMut for ServiceValueWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            ServiceValueWriteGuard::RwLock(guard) => &mut *guard,
            ServiceValueWriteGuard::Mutex(guard)  => &mut *guard,
        }
    }
}

impl<T: Service + 'static> Key for KeyType<T> { type Value = ServiceValue<T>; }



pub trait Service {}

pub struct ServiceContainer {
    services: ShareMap
}

impl ServiceContainer {

    pub fn new () -> Self {
        ServiceContainer{services: TypeMap::custom()}
    }

    pub fn bind_singleton<S: Service + Send + Sync + 'static, T: Into<ServiceValue<S>>> (&mut self, service: T) {
        let value = service.into();
        self.services.insert::<KeyType<S>>(value);
    }

    pub fn resolve<S: Service + Send + Sync + 'static> (&self) -> Result<&ServiceValue<S>, ()> {
        match self.services.get::<KeyType<S>>() {
            Some(value) => Ok(value),
            None        => Err(()),
        }
    }

    pub fn resolve_read<S: Service + Send + Sync + 'static> (&self) -> Result<ServiceValueReadGuard<S>, ()> {
        match self.services.get::<KeyType<S>>() {
            Some(value) => value.read(),
            _           => Err(()),
        }
    }

    pub fn resolve_write<S: Service + Send + Sync + 'static> (&self) -> Result<ServiceValueWriteGuard<S>, ()> {
        match self.services.get::<KeyType<S>>() {
            Some(value) => value.write(),
            _           => Err(()),
        }
    }

}