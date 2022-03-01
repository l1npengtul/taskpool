use std::future::Future;

pub trait Task: Send + Sync + Future + 'static {}

impl<T> Task for T where T: Send + Sync + Future + 'static {}