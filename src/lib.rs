use serde_json::from_str;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::time::Duration;

#[no_mangle]
pub extern "C" fn GetHttp(url: *const c_char) -> *mut c_char {
    let c_url_str = unsafe {
        assert!(!url.is_null());
        CStr::from_ptr(url).to_str().expect("")
    };

    if c_url_str.is_empty() {
        return CString::new("").expect("").into_raw();
    }

    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(2))
        .user_agent("kLauncherV3")
        .build();

    let client = match client {
        Ok(v) => v,
        Err(_) => return CString::new("").expect("").into_raw(),
    };

    let res = client
        .get(c_url_str)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send();

    let res = match res {
        Ok(v) => v,
        Err(_) => return CString::new("").expect("").into_raw(),
    };

    let body = res.text();
    let body = match body {
        Ok(v) => v,
        Err(_) => return CString::new("").expect("").into_raw(),
    };

    let c_body = CString::new(body).expect("").into_raw();
    return c_body;
}

#[no_mangle]
pub extern "C" fn GetStatus(url: *const c_char) -> i32 {
    let c_url_str = unsafe {
        assert!(!url.is_null());
        CStr::from_ptr(url).to_str().expect("")
    };

    if c_url_str.is_empty() {
        return 0;
    }

    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(2))
        .user_agent("kLauncherV3")
        .build();

    let client = match client {
        Ok(v) => v,
        Err(_) => return 404,
    };

    let res = client
        .get(c_url_str)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send();

    let res = match res {
        Ok(v) => v,
        Err(_) => return 404,
    };

    let status = from_str(res.status().as_str());

    match status {
        Ok(v) => return v,
        Err(_) => return 404,
    };
}
