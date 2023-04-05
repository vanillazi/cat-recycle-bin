
    const KEY_PATH:&str="SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\CLSID\\{645FF040-5081-101B-9F08-00AA002F954E}\\DefaultIcon";
    const KEY_FULL:&str="full";
    const KEY_EMPTY:&str="empty";

    const CAT_IMAGE_FORMAT:&str="{},0";

    const EMPTY_FILE_NAME:&str="cat_empty.dll";
    const FULL_FILE_NAME:&str="cat_full.dll";

    const DEFAULT_WORK_DIR:&str="data";

    use std::env;
    use std::path::Path;
    use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{SMTO_ABORTIFHUNG, WM_SETTINGCHANGE};
    use winreg::RegKey;

    fn main() {
    let hklm = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let cur_ver = hklm.open_subkey_with_flags(KEY_PATH,winreg::enums::KEY_ALL_ACCESS).unwrap();

    let path:String=cur_ver.get_value(KEY_FULL).unwrap();
    let mut full =String::from("%SystemRoot%\\System32\\imageres.dll,-54");
    let mut empty =String::from("%SystemRoot%\\System32\\imageres.dll,-55");
    if path.eq(&full) {
        let ( empty1,  full1)=readFile();
         full=format!("{},0",full1);
         empty=format!("{},0",empty1);
    }
    cur_ver.set_value(KEY_FULL, &full).expect("set full failed!");
    cur_ver.set_value(KEY_EMPTY, &empty).expect("set empty failed!");
        let wParam=WPARAM(0);
        let lParam = LPARAM::default();
        let mut result =0;
        unsafe {
            windows::Win32::UI::WindowsAndMessaging::SendMessageTimeoutW(
                HWND(0xffff),
                WM_SETTINGCHANGE,
                wParam,
                lParam,
                SMTO_ABORTIFHUNG,
                500,
                Some(&mut result)
            );
        }
}
    pub fn readFile()->(String,String){
        let current=env::current_dir().unwrap();
        let data=current.join("data");
        let data=data.as_path();//Path::new("data");
        let empty=data.to_path_buf().join(EMPTY_FILE_NAME);
        let empty=empty.as_path();
        let full=data.to_path_buf().join(FULL_FILE_NAME);
        let full=full.as_path();
        (String::from(empty.to_str().unwrap()),String::from(full.to_str().unwrap()))
    }

#[cfg(test)]
pub mod tests{
    use std::env;
    use std::path::Path;

    #[test]
    pub fn readFile(){
        let current=env::current_dir().unwrap();
        let data=current.join("data");
        let data=data.as_path();//Path::new("data");
        let empty=data.to_path_buf().join(super::EMPTY_FILE_NAME);
        let empty=empty.as_path();
        let full=data.to_path_buf().join(super::FULL_FILE_NAME);
        let full=full.as_path();
        println!("{}",empty.to_str().unwrap());
        println!("{}",full.to_str().unwrap());
    }
}
