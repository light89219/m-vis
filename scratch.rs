use mach2::kern_return::KERN_SUCCESS;
use mach2::port::mach_port_t;
use mach2::traps::mach_task_self;
use mach2::vm::mach_vm_region;
use mach2::vm_prot::{VM_PROT_EXECUTE, VM_PROT_READ, VM_PROT_WRITE};
use mach2::vm_region::{vm_region_basic_info_64, vm_region_info_t, VM_REGION_BASIC_INFO_64};
use std::mem;

fn main() {
    let task = unsafe { mach_task_self() };
    
    let mut address: mach2::vm_types::mach_vm_address_t = 0;
    
    loop {
        let mut size: mach2::vm_types::mach_vm_size_t = 0;
        let mut info: vm_region_basic_info_64 = unsafe { mem::zeroed() };
        let mut info_count = mem::size_of::<vm_region_basic_info_64>() as mach2::message::mach_msg_type_number_t
            / mem::size_of::<i32>() as mach2::message::mach_msg_type_number_t;
        let mut object_name: mach_port_t = 0;

        let res = unsafe {
            mach_vm_region(
                task,
                &mut address,
                &mut size,
                VM_REGION_BASIC_INFO_64,
                &mut info as *mut _ as vm_region_info_t,
                &mut info_count,
                &mut object_name,
            )
        };

        if res != KERN_SUCCESS {
            break;
        }

        let is_shared = info.shared != 0;
        let is_exec = info.protection & VM_PROT_EXECUTE != 0;
        let is_read = info.protection & VM_PROT_READ != 0;
        let is_write = info.protection & VM_PROT_WRITE != 0;

        if is_shared {
            println!("0x{:x} - 0x{:x} | shared:{} r:{},w:{},x:{}", address, address + size, is_shared, is_read, is_write, is_exec);
        }

        address = address.saturating_add(size);
    }
}
