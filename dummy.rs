fn main() { let _d = vec![0u8; 10_000_000]; println!("My PID is: {}", std::process::id()); loop { std::thread::sleep(std::time::Duration::from_secs(1)); } }
