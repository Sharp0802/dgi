pub trait OrElseAsync<T> {
    async fn or_else_async(self, f: impl AsyncFnOnce() -> Option<T>) -> Option<T>;

    async fn unwrap_or_else_async(self, f: impl AsyncFnOnce() -> T) -> T;
}

impl<T> OrElseAsync<T> for Option<T> {
    async fn or_else_async(self, f: impl AsyncFnOnce() -> Option<T>) -> Option<T> {
        match self {
            None => f().await,
            Some(v) => Some(v),
        }
    }

    async fn unwrap_or_else_async(self, f: impl AsyncFnOnce() -> T) -> T {
        match self {
            None => f().await,
            Some(v) => v,
        }
    }
}