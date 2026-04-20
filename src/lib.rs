pub mod task;
pub mod worker;
pub mod sched;

#[cfg(test)]
mod tests {
    use super::*;
    use super::sched::*;
    use super::task::*;
    use super::worker::*;

    #[test]
    fn happy_path() {
        let scheduler = Scheduler::<String>::new(4);

        for i in 0..10 {
            let item = WorkItem::new(
                format!("task-{i}").as_str(),
                Box::new(move || Ok(format!("result-{i}")))
            );

            let _ = scheduler.submit(item).is_ok();
        }

        let results = scheduler.shutdown();
        assert_eq!(results.len(), 10);
        for r in &results {
            assert!(r.outcome.is_ok());
        }
    }
    
    /*
    #[test]
    fn handles_failures() {
        let scheduler = Scheduler::<String>::new(2);

        scheduler.submit(WorkItem::new("good", Box::new(|| Ok("ok".into())))).unwrap();
        scheduler.submit(WorkItem::new("bad", Box::new(|| Err("boom".into())))).unwrap();

        let results = scheduler.shutdown();
        assert_eq!(results.len(), 2);

        let failures: Vec<_> = results.iter()
            .filter(|r| r.outcome.is_err())
            .collect();
        assert_eq!(failures.len(), 1);
    }
    */
}
