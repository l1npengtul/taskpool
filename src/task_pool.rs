use tokio::runtime::Runtime;
use crate::task::Task;

pub struct TaskPool<T> where T: Task {
    runtime: Runtime,
}