use core::ptr;
use windows_result::{Error, HRESULT};
use windows_sys::Wdk::System::OfflineRegistry;

fn main() {
    let hive_path = r"D:\Windows\System32\config\SYSTEM";
    let mut path: Vec<u16> = hive_path.encode_utf16().collect();
    path.push(0);
    let mut hive = ptr::null_mut();
    let result = unsafe { OfflineRegistry::OROpenHive(path.as_ptr(), &mut hive) };
    if result != 0 {
        eprintln!("Error: {}", Error::from_hresult(HRESULT::from_win32(result)));
    } else {
        println!("Hive opened successfully");
    }

    let key_path = r"ControlSet001\Services";
    let mut path: Vec<u16> = key_path.encode_utf16().collect();
    path.push(0);
    let mut reg_key = ptr::null_mut();
    let result = unsafe { OfflineRegistry::OROpenKey(hive, path.as_ptr(), &mut reg_key) };
    if result != 0 {
        eprintln!("Error: {}", Error::from_hresult(HRESULT::from_win32(result)));
    } else {
        println!("Key opened successfully");
    }

    let mut index = 0;
    loop {
        let mut subkey_name_length: u32 = 256;
        let mut subkey_name = vec![0u16; subkey_name_length as usize];
        let result = unsafe {
            OfflineRegistry::OREnumKey(
                reg_key,
                index,
                subkey_name.as_mut_ptr(),
                &mut subkey_name_length,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        if result != 0 {
            eprintln!("Error: {}", Error::from_hresult(HRESULT::from_win32(result)));
            break;
        }
        let subkey_name = String::from_utf16_lossy(&subkey_name[..subkey_name_length as usize]);
        println!("Subkey: {}", subkey_name);
        index += 1;
    }

    println!("Closing key");
    let result = unsafe { OfflineRegistry::ORCloseKey(reg_key) };
    if result != 0 {
        eprintln!("Error: {}", Error::from_hresult(HRESULT::from_win32(result)));
    }

    println!("Closing hive");
    let result = unsafe { OfflineRegistry::ORCloseHive(hive) };
    if result != 0 {
        eprintln!("Error: {}", Error::from_hresult(HRESULT::from_win32(result)));
    }
}
