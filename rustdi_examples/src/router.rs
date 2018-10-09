extern crate rustdi;
#[macro_use] extern crate rustdi_derive;
extern crate hyper;
extern crate futures;
extern crate pretty_env_logger;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::any::TypeId;
use std::mem;
use hyper::{Body, Chunk, Client, Method, Request, Response, Server, StatusCode, header, Error};
use hyper::service::service_fn;
use futures::{future, Future, Stream};
use rustdi::{Resolver, ServiceContainer, ResolveError, ServiceReadGuard, ServiceWriteGuard};

pub mod common{
    pub mod models;
    pub mod handlers;
}
use common::models::{AppConfig, AppState, s3};
use common::handlers::{read_handler, write_handler};

#[inject]
pub fn route_handler(req: &Request<Body>, state: &mut AppState) {
    println!("{}", req.uri().path());
    state.subject = "penguins".to_string();
}

struct RequestResolver<ReqT: 'static, ResolverT> {
    pub request: ReqT,
    pub resolver: Arc<ResolverT>,
}

impl<ReqT: 'static, ResolverT: Resolver> Resolver for RequestResolver<ReqT, ResolverT> {
    type Error = ResolverT::Error;

    fn resolve_immutable_ref<S: 'static>(&self) -> Result<ServiceReadGuard<S>, Self::Error> {
        
        // Use of mem::transmute is safe as types must be identical if TypeId's are identical
        if TypeId::of::<S>() == TypeId::of::<ReqT>() {
            return Ok(ServiceReadGuard::Ref(unsafe { std::mem::transmute::<&ReqT, &S>(&self.request) }));
        }

        self.resolver.resolve_immutable_ref::<S>()
    }

    fn resolve_mutable_ref<S: 'static>(&self) -> Result<ServiceWriteGuard<S>, Self::Error> {
        self.resolver.resolve_mutable_ref::<S>()
    }

    fn resolve_owned_value<S: 'static>(&self) -> Result<S, Self::Error> {
        self.resolver.resolve_owned_value::<S>()
    }
}

struct Router<R: Resolver> {
    resolver: Arc<R>,
    routes: HashMap<(Method, String), Box<Fn(&RequestResolver<Request<Body>, R>) -> Result<(), ResolveError> + Send + Sync + 'static>>, 
}

impl<R: Resolver> Router<R> {
    pub fn new (resolver: Arc<R>) -> Router<R> {
        Router{resolver, routes: HashMap::new()}
    }

    pub fn add<H>(&mut self, method: Method, path: &str, handler: H) where H: Fn(&RequestResolver<Request<Body>, R>) -> Result<(), ResolveError> + Send + Sync + 'static {
        let key = (method, path.to_string());
        let handler = Box::new(handler) as Box<Fn(&RequestResolver<Request<Body>, R>) -> Result<(), ResolveError> + Send + Sync + 'static>;
        self.routes.insert(key, handler);
    }

    pub fn lookup(&self, method: Method, path: &str) -> Option<&Box<Fn(&RequestResolver<Request<Body>, R>) -> Result<(), ResolveError> + Send + Sync + 'static>> {
        self.routes.get(&(method, path.to_string()))
    }

    pub fn handle_request(&self, request: Request<Body>) {
        match self.lookup(request.method().clone(), request.uri().path()) {
            Some(handler) => {
                let resolver = RequestResolver{request, resolver: self.resolver.clone()};
                handler(&resolver).unwrap();
            },
            None => {},
        };
    }
}

fn create_container() -> ServiceContainer {
    let mut c = ServiceContainer::new();
    c.bind_singleton_arc(Arc::new(AppConfig));
    c.bind_singleton_rwlock(Arc::new(RwLock::new(AppState{
        greeting: "hello".into(),
        subject:  "world".into(),
    })));
    c.bind_factory(|| s3::S3Client());
    c
}

fn create_router(container: Arc<ServiceContainer>) -> Router<ServiceContainer> {
    let mut r = Router::new(container);
    r.add(Method::GET, "/state/write", write_handler);
    r.add(Method::GET, "/state/echo", route_handler);
    r
}

fn main() {
    pretty_env_logger::init();

    let addr = "127.0.0.1:1337".parse().unwrap();

    hyper::rt::run(future::lazy(move || {

        // Create IoC service container and bind services
        let container = Arc::new( create_container() );
        let router = Arc::new( create_router(container) );

        let new_service = move || {
            // Move a clone of `client` into the `service_fn`.
            let router = router.clone();
            service_fn(move |req| {
                router.handle_request(req);
                futures::future::ok::<Response<Body>, Error>(Response::new("OK".into()))
            })
        };

        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(|e| eprintln!("server error: {}", e));

        println!("Listening on http://{}", addr);

        server
    }));
}