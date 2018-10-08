extern crate typemap;

mod traits;
pub use traits::{Inject, Resolver};

mod container;
pub use container::ServiceContainer;

mod service;
pub use service::{Service, ServiceReadGuard, ServiceWriteGuard};

mod resolve_error;
pub use resolve_error::ResolveError;