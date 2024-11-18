use windows::Win32::System::Diagnostics::ToolHelp::*;
use read_process_memory::{CopyAddress,ProcessHandle};
use std::convert::TryFrom;
use std::io;


pub fn new(target_process_name: &str) -> ProcessRead {
    let pid = find_process_id(target_process_name).unwrap();
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
            current_address += offset;
            self.handle.copy_address(current_address, &mut buffer)?;
            current_address = usize::from_ne_bytes(buffer);
        }
        Ok(current_address)
    }
}

fn find_process_id(target_process_name: &str) -> Option<u32> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()?;
        let mut entry = PROCESSENTRY32 {
            dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
            ..Default::default()
        };

        if Process32First(snapshot, &mut entry).is_ok() {
            loop {
                let process_name = String::from_utf16_lossy(
                    &entry.szExeFile.iter().take_while(|&&c| c != 0).map(|&c| c as u16).collect::<Vec<u16>>()
                );
                if process_name == target_process_name {
                    return Some(entry.th32ProcessID);
                }
                if Process32Next(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }
    }
    println!("Could not find process: {}", target_process_name);
    None
}

fn get_base_address(pid: u32, target_module_name: &str) -> Option<*const u8> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid).ok()?;
        let mut entry = MODULEENTRY32 {
            dwSize: std::mem::size_of::<MODULEENTRY32>() as u32,
            ..Default::default()
        };

        if Module32First(snapshot, &mut entry).is_ok() {
            loop {
                let module_name = String::from_utf16_lossy(
                    &entry.szModule.iter().take_while(|&&c| c != 0).map(|&c| c as u16).collect::<Vec<u16>>()
                );
                if module_name == target_module_name {
                    return Some(entry.modBaseAddr as *const u8);
                }
                if Module32Next(snapshot, &mut entry).is_err() {
                    break;
                }
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

