use windows::Win32::System::Diagnostics::ToolHelp::*;
use read_process_memory::{CopyAddress,ProcessHandle};
use std::convert::TryFrom;
use bytemuck::{bytes_of_mut, Pod};
use std::io;

use sysinfo::System;


pub fn new(target_process_name: &str) -> ProcessRead {
    let pid = get_process_id(target_process_name).unwrap();

    let base_address = get_base_address(pid, target_process_name).unwrap() as usize;

    let handle = get_handle(pid).unwrap();



    ProcessRead {
        process_name: target_process_name.to_string(),
        pid: pid,
        base_address: base_address,
        handle: handle
    }
}


pub struct ProcessRead {

    process_name: String,
    pid: u32,
    base_address: usize,
    handle: ProcessHandle
}



impl ProcessRead {

    pub fn read_memory_chain(&self, offsets: &[usize]) -> io::Result<usize> {
        let mut current_address =  self.base_address;
        for &offset in offsets {
            let mut buffer = [0u8; std::mem::size_of::<usize>()];
            self.handle.copy_address(current_address+offset, &mut buffer)?;
            current_address = usize::from_ne_bytes(buffer);
        }
        Ok(current_address)
    }
    pub fn read_memory_list<T: Pod>(&self, destination: &mut T, address: usize) -> io::Result<()> {

        let buffer = bytes_of_mut(destination);
        self.handle.copy_address(address, buffer)?;
        Ok(())
    }

}

fn get_process_id(target_process_name: &str) -> Option<u32> {
    let system = System::new_all();
    for (pid, process) in system.processes() {
        if process.name() == target_process_name {
            return Some(pid.as_u32());
        }
    }
    println!("Could not find process: {}", target_process_name);
    None
}

fn get_base_address(pid: u32, target_module_name: &str) -> Option<*const u8> {
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid).ok()? };
    let mut entry = MODULEENTRY32 {
        dwSize: std::mem::size_of::<MODULEENTRY32>() as u32,
        ..Default::default()
    };

    if unsafe { Module32First(snapshot, &mut entry).is_ok() } {
        loop {
            let module_name = String::from_utf16_lossy(
                &entry.szModule.iter().take_while(|&&c| c != 0).map(|&c| c as u16).collect::<Vec<u16>>()
            );
            if module_name == target_module_name {
                return Some(entry.modBaseAddr as *const u8);
            }
            if unsafe { Module32Next(snapshot, &mut entry).is_err() } {
                break;
            }
        }
    }
    println!("Could not find module: {}", target_module_name);
    None
}

fn get_handle(pid: u32) -> io::Result<ProcessHandle> {
    let handle = ProcessHandle::try_from(pid)?;
    Ok(handle)
}

