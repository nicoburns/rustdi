
// Dummy types for testing DI with
#[derive(Clone, Debug)]
pub struct AppConfig;

#[derive(Clone, Debug)]
pub struct AppState{
    pub greeting: String,
    pub subject: String,
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