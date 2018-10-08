use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use std::sync::Arc;
use std::ops::Deref;
use std::ops::DerefMut;

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