#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod xlog;

use std::ffi::CString;

fn init_log() {
    let file_path = "/tmp/xlog_test";
    let file_path = CString::new(file_path).unwrap();
    let name_prefix = CString::new("lark").unwrap();
    let pub_key = CString::new("160d418c88e556a72848317d265ad0e7fbfef3790620cff62dbc5f19314839948ff92f7d9ec80c2ae6aab6767455f0fb8e3c51cf73ec8fce1eb8ccd7d2906272").unwrap();
    unsafe {
        xlog::appender_open(xlog::TAppenderMode_kAppednerAsync, file_path.as_ptr(), name_prefix.as_ptr(), pub_key.as_ptr());
    }
}

fn info(log: &str) {
    let timeval = xlog::timeval {
        tv_sec: 0,
        tv_usec: 0
    };
    let info = xlog::XLoggerInfo_t {
        level: xlog::TLogLevel_kLevelInfo,
        tag: CString::new("xlog").unwrap().as_ptr(),
        filename: CString::new("lib.rs").unwrap().as_ptr(),
        func_name: CString::new("test").unwrap().as_ptr(),
        line: 1,
        timeval,
        pid: 0,
        tid: 0,
        maintid: 0
    };

    unsafe {
        xlog::xlogger_Write(&info, CString::new(log).unwrap().as_ptr());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        ::init_log();
        ::info("hello world");
        ::info("test xlog");
        unsafe {
            ::xlog::appender_close();
        }
    }
}
