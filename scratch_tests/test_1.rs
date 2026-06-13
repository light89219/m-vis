fn main() {
    let mut data = vec![0u8; 10_000_000];
    println!("PID: {}", std::process::id());
    loop {
        data[0] = data[0].wrapping_add(1);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
