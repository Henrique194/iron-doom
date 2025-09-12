use std::{ffi::{CStr, CString}, ptr};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::ERROR_SUCCESS,
        System::Registry::{
            RegCloseKey, RegOpenKeyExA, RegQueryValueExA, HKEY, HKEY_LOCAL_MACHINE, KEY_READ,
            KEY_WOW64_32KEY, REG_SZ, REG_VALUE_TYPE,
        },
    },
};

#[derive(Debug)]
pub struct LocalSoftwareRegistry {
    key: HKEY,
    sub_key: String,
    name: String,
}

impl LocalSoftwareRegistry {
    pub fn new(sub_key: &str, name: &str) -> Self {
        Self {
            key: HKEY_LOCAL_MACHINE,
            sub_key: String::from("Software\\") + sub_key,
            name: String::from(name),
        }
    }

    pub fn query_value(&self) -> Option<String> {
        let mut key_result = HKEY(ptr::null_mut());
        let mut val_type = REG_VALUE_TYPE(0);
        let mut len: u32 = 0;

        let sub_key = CString::new(self.sub_key.as_str()).unwrap();
        let sub_key = PCSTR::from_raw(sub_key.as_bytes_with_nul().as_ptr());

        let value = CString::new(self.name.as_str()).unwrap();
        let value = PCSTR::from_raw(value.as_bytes_with_nul().as_ptr());

        unsafe {
            let error_code = RegOpenKeyExA(
                self.key,
                sub_key,
                None,
                KEY_READ | KEY_WOW64_32KEY,
                &mut key_result,
            );
            if error_code != ERROR_SUCCESS {
                return None;
            }

            let error_code = RegQueryValueExA(
                key_result,
                value,
                None,
                Some(&mut val_type),
                None,
                Some(&mut len),
            );
            if error_code != ERROR_SUCCESS || val_type != REG_SZ {
                let _ = RegCloseKey(key_result);
                return None;
            }

            let size_buf = (len as usize) + 1;
            let mut buf = vec![0; size_buf];

            let error_code = RegQueryValueExA(
                key_result,
                value,
                None,
                Some(&mut val_type),
                Some(buf.as_mut_ptr()),
                Some(&mut len),
            );
            let _ = RegCloseKey(key_result);

            if error_code != ERROR_SUCCESS {
                return None;
            }

            buf[size_buf - 1] = 0;
            let c_str = CStr::from_bytes_until_nul(&buf).unwrap();

            Some(c_str.to_str().unwrap().to_owned())
        }
    }
}
