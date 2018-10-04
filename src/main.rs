#![feature(futures_api)]

use std::sync::Arc;
use std::sync::RwLock;
use futures::Future;
use rustdi_macro::show_streams;

pub mod ioc;

struct Request;
struct Response;
struct Connection;
pub mod s3 {
    use crate::ioc::Service;
    pub struct S3Client(pub String);
    impl Service for S3Client {}
}

fn main() {

    #[show_streams]
    fn show(_req: Request, _db: Connection, _s3: self::s3::S3Client) -> impl Future<Item=Response, Error=()> {
        return futures::future::ok(Response {});
    }

    let mut container = ioc::ServiceContainer::new();
    container.bind_singleton(Arc::new(RwLock::new(s3::S3Client("hello".into()))));

    std::thread::spawn(move ||{
        let mut client = container.resolve_write::<s3::S3Client>().unwrap();
        client.0 = "world".into();
    }).join().unwrap();

    let client = container.resolve_read::<s3::S3Client>().unwrap();
    println!("{}", client.0);
}
