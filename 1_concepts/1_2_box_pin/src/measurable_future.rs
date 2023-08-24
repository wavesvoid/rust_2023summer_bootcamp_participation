
use std::pin::Pin;
use std::future::Future;
use std::task::{Poll, Context};
use std::time::Instant;


#[pin_project::pin_project]
struct MeasurableFuture<Fut> {
    #[pin]
    inner_future: Fut,
    started_at: Option<Instant>,
}


impl<Fut: Future> MeasurableFuture<Fut> {
    fn new(f: Fut) -> Self {
        Self {
            inner_future: f,
            started_at: None,
        }
    }
}


impl<Fut: Future> Future for MeasurableFuture<Fut> {
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let start = this.started_at.get_or_insert_with(Instant::now);
        let fut = this.inner_future.poll(cx);
        let elapsed = start.elapsed();

        if fut.is_ready() {
            println!("The execution time took: {:?} ns", elapsed.as_nanos());
        }

        fut
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    

    #[tokio::test]
    async fn test_future() {
        let duration = Duration::from_secs(2);
        let fut = || async {
            tokio::time::sleep(duration).await;
            duration
        };
        let mesfut = MeasurableFuture::new(fut());
        let exec_duration = mesfut.await;

        println!("[TeST] Future execution took: {:?}, original time: {:?}", 
                exec_duration,
                duration);
        
        assert_eq!(duration, exec_duration);
    }
}