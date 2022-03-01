#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![warn(clippy::all)]

mod task_pool;
mod task;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
