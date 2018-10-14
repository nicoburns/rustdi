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
impl<T: 'static> Key for KeyType<T> { type Value = Service<ServiceContainer, T>; }

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

    pub fn bind_factory<S: Send + Sync + 'static> (&mut self, factory: fn(&Self) -> S) {
        let value = Service::Factory(Arc::new(factory));
        self.services.insert::<KeyType<S>>(value);
    }
}

// Resolving methods which allow services to be retrieved from the Service Container
impl Resolver for ServiceContainer {
    type Error = ResolveError;

    fn resolve_owned_value<S: 'static> (&self) -> Result<S, ResolveError> {
        match self.services.get_unchecked::<KeyType<S>>() {
            Some(service) => service.owned_value(&self),
            None          => Err(ResolveError::NonExist),
        }
    }

    fn resolve_immutable_ref<S: 'static> (&self) -> Result<ServiceReadGuard<S>, ResolveError> {
        match self.services.get_unchecked::<KeyType<S>>() {
            Some(service) => service.immutable_ref(&self),
            _             => Err(ResolveError::NonExist),
        }
    }

    fn resolve_mutable_ref<S: 'static> (&self) -> Result<ServiceWriteGuard<S>, ResolveError> {
        match self.services.get_unchecked::<KeyType<S>>() {
            Some(service) => service.mutable_ref(&self),
            _             => Err(ResolveError::NonExist),
        }
    }
}



// Find a value in the map and get a reference to it.
//
// It's technically safe to remove the Implements<A> bound from K::Value as the bound
// still exists on the insert method so no such values can exist in the map and this
// function will simply return None
//
// However, this allows us to move the check from compile time to runtime, which is useful
// as it allows us to remove it from the Resolver trait which frees up other resolvers to
// return values which aren't Send+Sync if they want to.
//
// Use of unsafe to access data is fine as we only need to do this to work aoru
use std::any::{Any, TypeId};
use unsafe_any::UnsafeAnyExt;

trait TypeMapExt {
    fn get_unchecked<K: Key>(&self) -> Option<&K::Value> where K::Value: Any;
}
impl TypeMapExt for ShareMap {
    fn get_unchecked<K: Key>(&self) -> Option<&K::Value> where K::Value: Any {
        unsafe {self.data()}.get(&TypeId::of::<K>()).map(|v| unsafe {
            v.downcast_ref_unchecked::<K::Value>()
        })
    }
}