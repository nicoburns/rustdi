extern crate rustdi;
#[macro_use] extern crate rustdi_derive;

use std::sync::Arc;
use std::sync::RwLock;

use rustdi::ServiceContainer;


// Dummy types for testing DI with
#[derive(Clone, Debug)]
struct AppConfig;

pub mod s3 {
    #[derive(Clone, Debug)]
    pub struct S3Client(pub String);
}


// Use the #[inject] macro to define IoC container compatible handlers
#[inject]
fn write_handler(_config: &AppConfig, client: &mut s3::S3Client) {
    client.0 = "penguins".into();
}

#[inject]
fn read_handler(_config: &AppConfig, client: &s3::S3Client) {
    println!("Hello {}", client.0);
}

#[inject]
fn invalid_handler(_config: &mut AppConfig) {
    println!("Hello world");
}

// #[inject]
// fn show(_req: Request, _db: Connection, _s3: self::s3::S3Client) -> impl Future<Item=Response, Error=()> {
//     return futures::future::ok(Response {});
// }


fn main() {

    // Create IoC service container and bind services
    let container = {
        let mut c = ServiceContainer::new();
        c.bind_singleton_arc(Arc::new(AppConfig));
        c.bind_singleton_rwlock(Arc::new(RwLock::new(s3::S3Client("world".into()))));
        Arc::new(c)
    };

    // Test resolving references out of the container manually
    println!("Testing container manually...");
    {
        let mut client = container.resolve_write::<s3::S3Client>().unwrap();
        client.0 = "frogs".into();
    }
    {
        let client = container.resolve_read::<s3::S3Client>().unwrap();
        println!("Hello {}", client.0);
    }

    // Test resolving references out of the container using the #[inject] macro
    println!("Testing injectable handlers...");
    write_handler(&container);
    read_handler(&container);

    // Test resolving references out of the container using the #[inject] macro
    // with the handlers running in seperate threads
    println!("Testing injectable handlers running in threads...");
    std::thread::spawn({
        let container = container.clone();
        move || { write_handler(&container); }
    }).join().unwrap();
    std::thread::spawn({
        let container = container.clone();
        move || { read_handler(&container); }
    }).join().unwrap();

    // Testing invalid handler. We cannot get a mutable reference to
    // an Arc singleton so we just panic in this case.
    println!("Testing invalid handler (expect panic)...");
    invalid_handler(&container);
}