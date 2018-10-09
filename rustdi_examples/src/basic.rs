extern crate rustdi;
#[macro_use] extern crate rustdi_derive;

use std::sync::Arc;
use std::sync::RwLock;

use rustdi::{Resolver, ServiceContainer};

pub mod common{
    pub mod models;
    pub mod handlers;
}
use common::models::{AppConfig, AppState, s3};
use common::handlers::{read_handler, write_handler, s3_handler};


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
    write_handler(&*container).unwrap();
    read_handler(&*container).unwrap();
    s3_handler(&*container).unwrap();

    // Test resolving references out of the container using the #[inject] macro
    // with the handlers running in seperate threads
    println!("Testing injectable handlers running in threads...");
    std::thread::spawn({
        let container = container.clone();
        move || { write_handler(&*container).unwrap(); }
    }).join().unwrap();
    std::thread::spawn({
        let container = container.clone();
        move || { read_handler(&*container).unwrap(); }
    }).join().unwrap();
    std::thread::spawn({
        let container = container.clone();
        move || { s3_handler(&*container).unwrap(); }
    }).join().unwrap();
}