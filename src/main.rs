use std::sync::Arc;
use std::sync::RwLock;
use rustdi_macro::inject;

pub mod ioc;
use crate::ioc::Service;

// Dummy types for testing DI with
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

    let mut container = ioc::ServiceContainer::new();
    container.bind_singleton(Arc::new(AppConfig));
    container.bind_singleton(Arc::new(RwLock::new(s3::S3Client("world".into()))));

    let container = Arc::new(container);
    let thread_container = container.clone();

    println!("Testing container manually...");
    std::thread::spawn(move || {
        let mut client = thread_container.resolve_write::<s3::S3Client>().unwrap();
        client.0 = "frogs".into();
    }).join().unwrap();
    {
        let client = container.resolve_read::<s3::S3Client>().unwrap();
        println!("Hello {}", client.0);
    }

    println!("Testing injectable handler...");
    write_handler(&container);
    read_handler(&container);
}

#[inject]
fn write_handler(_config: &AppConfig, client: &mut s3::S3Client) {
    client.0 = "penguins".into();
}

#[inject]
fn read_handler(_config: &AppConfig, client: &s3::S3Client) {
    println!("Hello {}", client.0);
}

// #[inject]
// fn show(_req: Request, _db: Connection, _s3: self::s3::S3Client) -> impl Future<Item=Response, Error=()> {
//     return futures::future::ok(Response {});
// }