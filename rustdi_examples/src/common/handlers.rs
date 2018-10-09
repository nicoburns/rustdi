
use super::models::{AppConfig, AppState, s3};

// Use the #[inject] macro to define IoC container compatible handlers
#[inject]
pub fn write_handler(_config: &AppConfig, state: &mut AppState) {
    state.subject = "penguins".to_string();
}

#[inject]
pub fn read_handler(_config: &AppConfig, state: &AppState) {
    println!("{} {}!", state.greeting, state.subject);
}

#[inject]
pub fn s3_handler(_config: &AppConfig, client: s3::S3Client) {
    client.list_objects();
    client.get_object();
}

// #[inject]
// pub fn show(_req: Request, _db: Connection, _s3: self::s3::S3Client) -> impl Future<Item=Response, Error=()> {
//     return futures::future::ok(Response {});
// }