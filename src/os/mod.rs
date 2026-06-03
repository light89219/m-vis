#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

//pub use windows::WindowsMemory as PlatformMemory;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;

//pub use linux::LinuxMemory as PlatformMemory;

use crate::types::{HeapBlock, ModuleInfo, Region};

//pub fn provider() -> PlatformMemory {
//    PlatformMemory
//}

pub trait MemoryProvider {
    fn walk_regions(&self, pid: u32) -> Vec<Region>;
    fn walk_heap(&self, pid: u32) -> Vec<HeapBlock>;
    fn list_modules(&self, pid: u32, flag: String) -> Vec<ModuleInfo>;
}
