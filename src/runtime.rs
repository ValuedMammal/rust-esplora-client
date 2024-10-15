use std::time::Duration;

pub trait Runtime {
    fn sleep(duration: Duration) -> impl std::future::Future<Output = ()> + Send;
}

pub struct DefaultRuntime;

#[cfg(feature = "tokio")]
impl Runtime for DefaultRuntime {
    async fn sleep(duration: Duration) {
        tokio::time::sleep(duration).await;
    }
}