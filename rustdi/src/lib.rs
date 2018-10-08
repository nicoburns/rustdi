extern crate typemap;

mod container;
pub use container::ServiceContainer;

mod service;
pub use service::{Service, ServiceReadGuard, ServiceWriteGuard};

mod resolve_error;
pub use resolve_error::ResolveError;