use std::sync::{Mutex, Arc};
use std::thread;
use crate::worker::WorkItem;
use crate::task::{TaskId, TaskResult};
use thiserror::Error;
use std::sync::mpsc;

#[derive(Error, Debug)]
pub enum SchedulerError<T> {
    #[error("scheduler is shut down")]
    ShutDown,

    #[error("task {0:?} failed: {1}")]
    TaskFailed(TaskId, String),
    
    #[error("channel send error")]
    ChannelError(#[from] std::sync::mpsc::SendError<T>),
    
    #[error("worker panicked")]
    WorkerPanic,
}

pub struct Scheduler<R: Send + 'static> {
    sender: Option<mpsc::Sender<WorkItem<R>>>,
    results: mpsc::Receiver<TaskResult<R>>,
}

impl<R: Send + 'static> Scheduler<R> {
    pub fn new(num_workers: usize) -> Self {
        let (job_tx, job_rx) = mpsc::channel::<WorkItem<R>>();
        let (results_tx, results_rx) = mpsc::channel::<TaskResult<R>>();
        // start the worker loo
        thread::spawn (move || {
            let shared_job_rx = Arc::new(Mutex::new(job_rx));
            let thread_ids: Vec<usize> = (0..num_workers).collect();

            thread::scope(|s| {
                for thread_id in thread_ids.iter() {
                    let _ = s.spawn(|| {
                        let _results_tx = results_tx.clone();
                        println!("spawned thread #{:?}", thread_id.clone());
                        while let Ok(job_rx) = shared_job_rx.try_lock() {
                            if let Ok(job) = job_rx.try_recv() {   // if we got the lock we
                                                                            // necessarily recv next
                                                                            // job on this thread;
                                                                            // if we had used a
                                                                            // nonblocking call then
                                                                            // shared_job_rx lock is
                                                                            // still held causing a
                                                                            // deadlock until this
                                                                            // thread proceeds which is
                                                                            // essentially the same as
                                                                            // blocking call
                                println!("thread {:?} received a job {:?}", thread_id.clone(), job.id);
                                let result = TaskResult {
                                    id: job.id,
                                    name: job.name,
                                    outcome: (job.work)()
                                };
                                
                                // should print "Ok" or "SchedulerError(Receiver Error)"
                                println!("{:?}", _results_tx.send(result));
                            }
                        }
                    });
                }
            });
        });
        
        Self {
            sender: Some(job_tx), 
            results: results_rx
        }
    }
    
    // submit a task
    pub fn submit(&self, item: WorkItem<R>) -> Result<TaskId, SchedulerError<WorkItem<R>>> {
        let item_id = item.id.clone();
        if let Err(msg) = self.sender.as_ref().unwrap().send(item) {
            Err(msg.into())
        } else {
            Ok(item_id)
        }
    }

    pub fn shutdown(self) -> Vec<TaskResult<R>> {
        drop(self.sender);
        
        let mut results = Vec::new();
        
        while let Ok(result) = self.results.try_recv() {
            results.push(result);
        } 

        results
    }
}
