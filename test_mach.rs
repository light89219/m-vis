use libc::{mach_task_self, task_for_pid, mach_port_t, KERN_SUCCESS};
fn main() {
    let mut task: mach_port_t = 0;
    unsafe {
        let res = task_for_pid(mach_task_self(), 1, &mut task);
        println!("res: {}", res == KERN_SUCCESS);
    }
}
