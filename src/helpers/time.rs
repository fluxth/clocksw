extern crate libc;
use std::ptr;

pub struct CLocalTime;

impl CLocalTime {
    pub fn now() -> libc::time_t {
        unsafe {
            libc::time(ptr::null_mut())
        }
    }

    pub fn tm_new(time: libc::time_t) -> libc::tm {
        unsafe {
            let mut tm = libc::tm {
                tm_sec: 0,
                tm_min: 0,
                tm_hour: 0,
                tm_mday: 0,
                tm_mon: 0,
                tm_year: 0,
                tm_wday: 0,
                tm_yday: 0,
                tm_isdst: 0,
                tm_gmtoff: 0,
                tm_zone: ptr::null()
            };

            *libc::localtime_r(&time, &mut tm)
        }
    }

    pub fn tm_modify(tm_struct: libc::tm, time: libc::time_t) -> libc::tm {
        let mut tm = tm_struct;
        unsafe {
            *libc::localtime_r(&time, &mut tm)
        }
    }

    pub fn sleep_until(time: libc::time_t) {
        let timespec: libc::timespec = libc::timespec {
            tv_sec: time,
            tv_nsec: 0
        };

        unsafe {
            libc::clock_nanosleep(
                libc::CLOCK_REALTIME,
                libc::TIMER_ABSTIME,
                &timespec,
                ptr::null_mut()
            );
        }
    }
}
