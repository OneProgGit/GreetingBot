pub type Res<T> = Result<T, Box<dyn std::error::Error>>;
pub type ThreadSafeRes<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
