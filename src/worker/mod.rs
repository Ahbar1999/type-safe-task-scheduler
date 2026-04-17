pub mod worker {
    use crate::task::task::TaskId;
    use std::sync::atomic::{AtomicU64, Ordering};

    pub struct WorkItem<R: Send + 'static> {
        id: TaskId,
        name: String,
        work: Box<dyn FnOnce() -> Result<R, String> + Send>
    }
    
    static TASK_ID_COUNTER: AtomicU64 = AtomicU64::new(42);

    impl<R: Send + 'static> WorkItem<R> {
        pub fn new(name: &str, work: Box<dyn FnOnce() -> Result<R, String> + Send>) -> WorkItem<R> {
            WorkItem {
                id: TaskId(TASK_ID_COUNTER.fetch_add(1, Ordering::Relaxed)),
                name: name.to_owned(),
                work
            } 
        }
    }
}
