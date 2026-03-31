pub trait MapAsync<Mid, To> {
    type Input;

    async fn map_async(self, f: impl AsyncFn(Self::Input) -> Mid) -> To;
}

impl<From, To> MapAsync<To, Option<To>> for Option<From> {
    type Input = From;

    async fn map_async(self, f: impl AsyncFn(Self::Input) -> To) -> Option<To> {
        match self {
            None => None,
            Some(v) => Some(f(v).await),
        }
    }
}
