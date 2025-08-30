use std::env;
use windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoW;
use windows::Win32::UI::WindowsAndMessaging::SPI_SETDESKWALLPAPER;
use windows::Win32::UI::WindowsAndMessaging::SPIF_UPDATEINIFILE;
use windows::core::PCWSTR;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_image>", args[0]);
        return;
    }
    let path: &String = &args[1];

    let wide: Vec<u16> = OsStr::new(path).encode_wide().chain(Some(0)).collect();
    unsafe {
        match SystemParametersInfoW(SPI_SETDESKWALLPAPER, 0, Some(PCWSTR(wide.as_ptr()).0 as *mut _), SPIF_UPDATEINIFILE) {
            Ok(_) => println!("Wallpaper changed successfully!"),
            Err(e) => eprintln!("Error changing wallpaper: {:?}", e),
        }
    }
}
