extern crate rustdi;
#[macro_use] extern crate rustdi_derive;

use std::sync::Arc;
use std::sync::RwLock;

use rustdi::ServiceContainer;


// Dummy types for testing DI with
#[derive(Clone, Debug)]
struct AppConfig;

#[derive(Clone, Debug)]
struct AppState{
    greeting: String,
    subject: String,
}

pub mod s3 {
    #[derive(Clone, Debug)]
    pub struct S3Client();

    impl S3Client {
        pub fn list_objects (&self) {}
        pub fn get_object (&self) {}
        pub fn put_object (&self) {}
    }
}


// Use the #[inject] macro to define IoC container compatible handlers
#[inject]
fn write_handler(_config: &AppConfig, state: &mut AppState) {
    state.subject = "penguins".to_string();
}

#[inject]
fn read_handler(_config: &AppConfig, state: &AppState) {
    println!("{} {}!", state.greeting, state.subject);
}

#[inject]
fn s3_handler(_config: &AppConfig, client: s3::S3Client) {
    client.list_objects();
    client.get_object();
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
        c.bind_singleton_rwlock(Arc::new(RwLock::new(AppState{
            greeting: "hello".into(),
            subject:  "world".into(),
        })));
        c.bind_factory(|| s3::S3Client());
        Arc::new(c)
    };

    // Test resolving references out of the container manually
    println!("Testing container manually...");
    {
        let mut state = container.resolve_mutable_ref::<AppState>().unwrap();
        state.subject = "frogs".into();
    }
    {
        let state = container.resolve_immutable_ref::<AppState>().unwrap();
        println!("Hello {}", state.subject);
    }
    let client = container.resolve_owned_value::<s3::S3Client>().unwrap();
    client.list_objects();

    // Test resolving references out of the container using the #[inject] macro
    println!("Testing injectable handlers...");
    write_handler(&container).unwrap();
    read_handler(&container).unwrap();

    // Test resolving references out of the container using the #[inject] macro
    // with the handlers running in seperate threads
    println!("Testing injectable handlers running in threads...");
    std::thread::spawn({
        let container = container.clone();
        move || { write_handler(&container).unwrap(); }
    }).join().unwrap();
    std::thread::spawn({
        let container = container.clone();
        move || { read_handler(&container).unwrap(); }
    }).join().unwrap();

    // Testing factory service handler
    println!("Testing factory service handler (expect panic)...");
    s3_handler(&container).unwrap();
}