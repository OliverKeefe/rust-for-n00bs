#[link(name = "kernel32")]
extern {
    fn GetLastError() -> u32;
    fn OpenProcess(dwDesiredProcess: u32, bInheritHandle: bool, dwProcessId: u32)-> *mut c_void;
}

fn main() {
    let handle = unsafe {
        OpenProcess(0xF01FF, false, 26768)
    };

    if handle == null_mut() {
        println!("OpenProcess failed: {}", unsafe { GetLastError() });
    } else {
        println!("hProcess: {x?}", handle);
    }
}
