use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

pub async fn execute_with_limit<F, Fut>(
    max_concurrent_tasks: usize,
    tasks: Vec<String>,
    process_fn: F,
) where
    F: Fn(String) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));
    let process_fn = Arc::new(process_fn);
    let mut handles = Vec::new();

    for task_id in tasks {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let process_fn = Arc::clone(&process_fn); // 克隆 Arc

        handles.push(task::spawn(async move {
            process_fn(task_id).await;
            drop(permit);
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
