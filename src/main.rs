use std::sync::Arc;
use std::sync::RwLock;
use futures::Future;
use rustdi_macro::{inject, show_streams};

pub mod ioc;
use crate::ioc::Service;


// Dummy types for testing DI with
struct Request;
struct Response;
struct Connection;

#[derive(Clone, Debug)]
struct AppConfig;
impl Service for AppConfig {}

pub mod s3 {
    use crate::ioc::Service;
    #[derive(Clone, Debug)]
    pub struct S3Client(pub String);
    impl Service for S3Client {}
}


fn main() {

    // #[inject]
    // fn show(_req: Request, _db: Connection, _s3: self::s3::S3Client) -> impl Future<Item=Response, Error=()> {
    //     return futures::future::ok(Response {});
    // }

    let mut container = ioc::ServiceContainer::new();
    container.bind_singleton(Arc::new(AppConfig));
    container.bind_singleton(Arc::new(RwLock::new(s3::S3Client("hello".into()))));

    let container = container.freeze();
    let thread_container = container.clone();
    std::thread::spawn(move || {
        let mut client = thread_container.resolve_write::<s3::S3Client>().unwrap();
        client.0 = "world".into();
    }).join().unwrap();

    println!("Testing container manually...");
    let client = container.resolve_read::<s3::S3Client>().unwrap();
    println!("{}", client.0);

    println!("Testing injectable handler...");
    handler(&container);
}

#[inject]
fn handler(config: AppConfig, client: s3::S3Client) {
    println!("{}", client.0);
}