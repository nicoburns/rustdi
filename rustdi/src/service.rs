use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use std::sync::Arc;
use std::ops::Deref;
use std::ops::DerefMut;

use super::resolve_error::ResolveError;

// Service enum which encapsulates the various different ways which services
// can be bound to the container, and allows us to do runtime checking.
#[derive(Debug)]
pub enum Service<T> {
    SingletonArc(Arc<T>),
    SingletonRwLock(Arc<RwLock<T>>),
    SingletonMutex(Arc<Mutex<T>>),
}

impl<S> Service<S> {
    pub fn immutable_ref (&self) -> Result<ServiceReadGuard<S>, ResolveError> {
        return match self {
            Service::SingletonArc(service)    => Ok(ServiceReadGuard::Arc(service.clone())),
            Service::SingletonRwLock(service) => service.read().map(ServiceReadGuard::RwLock).map_err(|_| ResolveError::Poisoned),
            Service::SingletonMutex(service)  => service.lock().map(ServiceReadGuard::Mutex).map_err(|_| ResolveError::Poisoned),
        }
    }

    pub fn mutable_ref (&self) -> Result<ServiceWriteGuard<S>, ResolveError> {
        return match self {
            Service::SingletonArc(_)          => Err(ResolveError::MutImmutable),
            Service::SingletonRwLock(service) => service.write().map(ServiceWriteGuard::RwLock).map_err(|_| ResolveError::Poisoned),
            Service::SingletonMutex(service)  => service.lock().map(ServiceWriteGuard::Mutex).map_err(|_| ResolveError::Poisoned),
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
            ServiceReadGuard::Arc(guard)    => &*guard,
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