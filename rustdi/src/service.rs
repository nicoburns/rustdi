use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use std::sync::Arc;
use std::ops::Deref;
use std::ops::DerefMut;

use super::traits::Resolver;
use super::resolve_error::ResolveError;

// Service enum which encapsulates the various different ways which services
// can be bound to the container, and allows us to do runtime checking.
pub enum Service<R: Resolver, T> {
    SingletonArc(Arc<T>),
    SingletonRwLock(Arc<RwLock<T>>),
    SingletonMutex(Arc<Mutex<T>>),
    Factory(Arc<fn(&R) -> T>),
}

impl<R: Resolver, T> Service<R, T> {
    pub fn immutable_ref (&self, resolver: &R) -> Result<ServiceReadGuard<T>, ResolveError> {
        return match self {
            Service::SingletonArc(service)    => Ok(ServiceReadGuard::Arc(service.clone())),
            Service::SingletonRwLock(service) => service.read().map(ServiceReadGuard::RwLock).map_err(|_| ResolveError::Poisoned),
            Service::SingletonMutex(service)  => service.lock().map(ServiceReadGuard::Mutex).map_err(|_| ResolveError::Poisoned),
            Service::Factory(factory)         => Ok(ServiceReadGuard::Owned(factory(resolver))),
        }
    }

    pub fn mutable_ref (&self, resolver: &R) -> Result<ServiceWriteGuard<T>, ResolveError> {
        return match self {
            Service::SingletonArc(_)          => Err(ResolveError::MutImmutable),
            Service::SingletonRwLock(service) => service.write().map(ServiceWriteGuard::RwLock).map_err(|_| ResolveError::Poisoned),
            Service::SingletonMutex(service)  => service.lock().map(ServiceWriteGuard::Mutex).map_err(|_| ResolveError::Poisoned),
            Service::Factory(factory)         => Ok(ServiceWriteGuard::Owned(factory(resolver))),
        }
    }

    pub fn owned_value (&self, resolver: &R) -> Result<T, ResolveError> {
        return match self {
            Service::SingletonArc(_)    => Err(ResolveError::OwnedImmutable),
            Service::SingletonRwLock(_) => Err(ResolveError::OwnedMutable),
            Service::SingletonMutex(_)  => Err(ResolveError::OwnedMutable),
            Service::Factory(factory)   => Ok(factory(resolver)),
        }
    }
}

pub enum ServiceReadGuard<'a, T: 'a> {
    Arc(Arc<T>),
    RwLock(RwLockReadGuard<'a, T>),
    Mutex(MutexGuard<'a, T>),
    Ref(&'a T),
    Owned(T),
}
impl<'a, T> Deref for ServiceReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            ServiceReadGuard::Arc(guard)      => &*guard,
            ServiceReadGuard::RwLock(guard)   => &*guard,
            ServiceReadGuard::Mutex(guard)    => &*guard,
            ServiceReadGuard::Ref(reference) => reference,
            ServiceReadGuard::Owned(value)    => &*value,
        }
    }
}

pub enum ServiceWriteGuard<'a, T: 'a> {
    RwLock(RwLockWriteGuard<'a, T>),
    Mutex(MutexGuard<'a, T>),
    Ref(&'a mut T),
    Owned(T),
}
impl<'a, T> Deref for ServiceWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            ServiceWriteGuard::RwLock(guard)  => &*guard,
            ServiceWriteGuard::Mutex(guard)   => &*guard,
            ServiceWriteGuard::Ref(reference) => reference,
            ServiceWriteGuard::Owned(value)   => &*value,
        }
    }
}
impl<'a, T: 'a> DerefMut for ServiceWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            ServiceWriteGuard::RwLock(guard)  => &mut *guard,
            ServiceWriteGuard::Mutex(guard)   => &mut *guard,
            ServiceWriteGuard::Ref(reference) => reference,
            ServiceWriteGuard::Owned(value)   => &mut *value,
        }
    }
}