use std::thread;
use std::time::Duration;
use std::sync::mpsc;

#[allow(dead_code)]
pub fn execute_with_timeout<T, F>(timeout_ms: u64, f: F) -> Option<T> 
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    
    // Spawn the closure in a separate thread
    let _handle = thread::spawn(move || {
        let result = f();
        let _ = tx.send(result);
    });

    // Wait for either the timeout or the result
    match rx.recv_timeout(Duration::from_millis(timeout_ms)) {
        Ok(result) => {
            // Result received within timeout
            Some(result)
        }
        Err(_) => {
            // Timeout occurred
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_execute_with_timeout() {
        // Example 1: Operation completes within timeout
        let result = execute_with_timeout(1000, || {
            thread::sleep(Duration::from_millis(500));
            42
        });
        assert_eq!(result, Some(42));

        // Example 2: Operation times out
        let result = execute_with_timeout(1000, || {
            thread::sleep(Duration::from_millis(2000));
            42
        });
        assert_eq!(result, None);
    }
}