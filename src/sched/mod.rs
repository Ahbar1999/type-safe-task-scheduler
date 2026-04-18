use std::sync::{Mutex, Arc};
use std::thread;
use crate::worker::WorkItem;
use crate::task::{TaskId, TaskResult};
use thiserror::Error;
use std::sync::mpsc;

#[derive(Error, Debug)]
pub enum SchedulerError {
    #[error("scheduler is shut down")]
    ShutDown,

    #[error("task {0:?} failed: {1}")]
    TaskFailed(TaskId, String),
    
    #[error("channel send error")]
    ChannelError(#[from] std::sync::mpsc::SendError<()>),
    
    #[error("worker panicked")]
    WorkerPanic,
}

struct Scheduler<R: Send + 'static> {
    sender: Option<mpsc::Sender<WorkItem<R>>>,
    results: mpsc::Receiver<TaskResult<R>>,
}

impl<R: Send + 'static> Scheduler<R> {
    pub fn new(num_workers: usize) -> Self {
        let (job_tx, job_rx) = mpsc::channel::<WorkItem<R>>();
        let (results_tx, results_rx) = mpsc::channel::<TaskResult<R>>();
        
        let sync_job_rx = Arc::new(Mutex::new(job_rx));

        thread::scope(|s| {
            for _ in 0..num_workers {
                let _ = s.spawn(|| {
                    while let Ok(job_rx) = sync_job_rx.try_lock() {
                        if let Ok(job) = job_rx.try_recv() {
                            let result = TaskResult {
                                id: job.id,
                                name: job.name,
                                outcome: (job.work)()
                            };
                            
                            // should print "Ok" or "SchedulerError(Receiver Error)"  
                            println!("{:?}", results_tx.send(result));
                        }
                    }
                });
            }
        });
        
        Self {
            sender: Some(job_tx), 
            results: results_rx
        }
    }
    
    // submit a task
    pub fn submit(&self, item: WorkItem<R>) -> Result<TaskId, SchedulerError> {
        unimplemented!()    
    }
}


 
