extern crate typemap;

mod service;
mod container;

pub use service::{Service, ServiceReadGuard, ServiceWriteGuard};
pub use container::ServiceContainer;