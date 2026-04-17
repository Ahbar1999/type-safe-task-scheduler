pub mod task {
    use std::marker::PhantomData;
     
    // zero sized marker types
    struct Pending;
    struct Running;
    struct Completed;
    struct Failed;
    
    #[derive(Debug, Clone, PartialEq)]
    pub struct TaskId(pub u64);

    struct Task<State, R> {
        id: TaskId,
        name: String,
        _state: PhantomData<State>,
        _result: PhantomData<R>
    }

    impl<State, R> Task<State, R> {
        // consumes self returning the task promotedt to the state
        pub fn start(self) -> Task<Running, R> {
            Task {
                id: self.id,
                name: self.name,
                _state: PhantomData,
                _result: self._result
            }
        }

        pub fn end_success(self, result: PhantomData<R>) -> Task<Completed, R> {
            Task {
                id: self.id,
                name: self.name,
                _state: PhantomData,
                _result: result, 
            }
        }
        
        pub fn end_failed(self) -> Task<Failed, R> {
            Task {
                id: self.id,
                name: self.name,
                _state: PhantomData,
                _result: self._result, 
            }
        }
    } 
}
