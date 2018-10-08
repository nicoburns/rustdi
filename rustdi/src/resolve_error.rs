use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ResolveError {
    NonExist,
    Poisoned,
    MutImmutable,
    OwnedMutable,
    OwnedImmutable,
}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            ResolveError::NonExist => write!(f, "Tried to resolve a non-existent service"),
            ResolveError::Poisoned => write!(f, "Tried to resolve a service whose lock is poisoned"),
            ResolveError::MutImmutable => write!(f, "Tried to get mutable reference to immutable service"),
            ResolveError::OwnedMutable => write!(f, "Tried to get owned value from mutable singleton service"),
            ResolveError::OwnedImmutable => write!(f, "Tried to get owned value from immutable singleton service"),
        }
        
    }
}

impl Error for ResolveError {}