
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use std::sync::Arc;
use std::convert::Into;
use std::ops::Deref;
use std::ops::DerefMut;
use typemap::{TypeMap, SendMap, Key};

struct KeyType<T>(T);

// Singleton Service Container
#[derive(Debug)]
pub enum ServiceSingleton<T: Service> {
    Bare(Arc<T>),
    RwLock(Arc<RwLock<T>>),
    Mutex(Arc<Mutex<T>>),
}
impl<T: Service> From<Arc<T>> for ServiceSingleton<T> {
    fn from(service: Arc<T>) -> Self { return ServiceSingleton::Bare(service); }
}
impl<T: Service> From<Arc<RwLock<T>>> for ServiceSingleton<T> {
    fn from(service: Arc<RwLock<T>>) -> Self { return ServiceSingleton::RwLock(service); }
}
impl<T: Service> From<Arc<Mutex<T>>> for ServiceSingleton<T> {
    fn from(service: Arc<Mutex<T>>) -> Self { return ServiceSingleton::Mutex(service); }
}
impl<S: Service> ServiceSingleton<S> {
    pub fn read (&self) -> Result<ServiceSingletonReadGuard<S>, ()> {
        return match self {
            ServiceSingleton::Bare(service)   => Ok(ServiceSingletonReadGuard::Bare(service.clone())),
            ServiceSingleton::RwLock(service) => Ok(ServiceSingletonReadGuard::RwLock(service.read().unwrap())),
            ServiceSingleton::Mutex(service)  => Ok(ServiceSingletonReadGuard::Mutex(service.lock().unwrap())),
        }
    }

    pub fn write (&self) -> Result<ServiceSingletonWriteGuard<S>, ()> {
        return match self {
            ServiceSingleton::Bare(_)   => Err(()),
            ServiceSingleton::RwLock(service) => Ok(ServiceSingletonWriteGuard::RwLock(service.write().unwrap())),
            ServiceSingleton::Mutex(service)  => Ok(ServiceSingletonWriteGuard::Mutex(service.lock().unwrap())),
        }
    }
}

pub enum ServiceSingletonReadGuard<'a, T: Service> {
    Bare(Arc<T>),
    RwLock(RwLockReadGuard<'a, T>),
    Mutex(MutexGuard<'a, T>),
}
impl<'a, T: Service> Deref for ServiceSingletonReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            ServiceSingletonReadGuard::Bare(guard)   => &*guard,
            ServiceSingletonReadGuard::RwLock(guard) => &*guard,
            ServiceSingletonReadGuard::Mutex(guard)  => &*guard,
        }
    }
}

pub enum ServiceSingletonWriteGuard<'a, T: Service> {
    RwLock(RwLockWriteGuard<'a, T>),
    Mutex(MutexGuard<'a, T>),
}
impl<'a, T: Service> Deref for ServiceSingletonWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            ServiceSingletonWriteGuard::RwLock(guard) => &*guard,
            ServiceSingletonWriteGuard::Mutex(guard)  => &*guard,
        }
    }
}
impl<'a, T: Service> DerefMut for ServiceSingletonWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            ServiceSingletonWriteGuard::RwLock(guard) => &mut *guard,
            ServiceSingletonWriteGuard::Mutex(guard)  => &mut *guard,
        }
    }
}



#[derive(Debug)]
pub struct ServiceFactory<T: Service>(T);
#[derive(Debug)]
pub enum ServiceValue<T: Service> {
    Singleton(ServiceSingleton<T>),
    Factory(ServiceFactory<T>),
}
impl<T: Service + 'static> Key for KeyType<T> { type Value = ServiceValue<T>; }



pub trait Service {}

pub struct ServiceContainer {
    services: TypeMap
}

impl ServiceContainer {

    pub fn new () -> Self {
        ServiceContainer{services: TypeMap::new()}
    }

    pub fn bind_singleton<S: Service + 'static, T: Into<ServiceSingleton<S>>> (&mut self, service: T) {
        let value = ServiceValue::Singleton(service.into());
        self.services.insert::<KeyType<S>>(value);
    }

    pub fn resolve<S: Service + 'static> (&self) -> Result<&ServiceValue<S>, ()> {
        match self.services.get::<KeyType<S>>() {
            Some(value) => Ok(value),
            None        => Err(()),
        }
    }

    pub fn resolve_read<S: Service + 'static> (&self) -> Result<ServiceSingletonReadGuard<S>, ()> {
        match self.services.get::<KeyType<S>>() {
            Some(ServiceValue::Singleton(value)) => value.read(),
            _ => Err(()),
        }
    }

    pub fn resolve_write<S: Service + 'static> (&self) -> Result<ServiceSingletonWriteGuard<S>, ()> {
        match self.services.get::<KeyType<S>>() {
            Some(ServiceValue::Singleton(value)) => value.write(),
            _ => Err(()),
        }
    }

}