use std::marker::PhantomData;
 
// zero sized marker types
pub struct Pending;
pub struct Running;
pub struct Completed;
pub struct Failed;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskId(pub u64);

pub struct TaskResult<R> {
    pub id: TaskId, 
    pub name: String, 
    pub outcome: Result<R, String>,
}

pub struct Task<State, R> {
    id: TaskId,
    name: String,
    _state: PhantomData<State>,
    _result: TaskResult<R>,
}

// state transitions
// Pending --> Running --> Failed
//              |
//              v
//          Completed

impl<State, R> Task<State, R> {
    // consumes self returning the task promoted to the next state
    pub fn start(self) -> Task<Running, R> {
        Task {
            id: self.id,
            name: self.name,
            _state: PhantomData,
            _result: self._result
        }
    }
} 

// running task either goes to completion(success) or failure
impl<R> Task<Running, R> {
    pub fn end_success(self, result: TaskResult<R>) -> Task<Completed, R> {
        Task {
            id: self.id,
            name: self.name,
            _state: PhantomData,
            _result: result, 
        }
    }

    pub fn end_failed(self, result: TaskResult<R>) -> Task<Failed, R> {
        Task {
            id: self.id,
            name: self.name,
            _state: PhantomData,
            _result: result, 
        }
    }
}

// pending task can only go to running 
impl<R> Task<Pending, R> {
    // resume execution
    pub fn resume(self) -> Task<Running, R> {
        Task {
            id: self.id,
            name: self.name,
            _state: PhantomData,
            _result: self._result, 
        }
    }
}
    
    

