extern crate nix;
use std::os::unix::io::RawFd;
use nix::fcntl::open;
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;
use nix::sys::uio::IoVec;
use nix::sys::uio::writev;

pub struct Kernellogger {
    klog_level: u8,
    klog_fd:RawFd,
    pub prefix:String,
}

macro_rules! tag {
    () => {"{}: {}:{}: {}"};
}

macro_rules! info {
    ($self:ident, $fmt:expr) => {
        let a = $self.Logger;
        let mut message = String::new();
        write!(message, tag!(), a.prefix, file!(), line!(), format_args!($fmt)).unwrap();
        //let message = format!($fmt);
        a.write_message(message);
    };

    ($self:ident, $fmt:expr, $($arg:tt)+) => {
        let a = $self.Logger;
        let mut message = String::new();
        write!(message, tag!(), a.prefix, file!(), line!(), format_args!($fmt, $($arg)+)).unwrap();
        //let message = format!($fmt, $($arg)+);
        a.write_message(message);
    };
}

pub fn mrom_klog_init(level:u8, prefix:&str) -> Kernellogger {
        let result = open("/dev/kmsg", OFlag::O_WRONLY, Mode::empty());
        match result {
            Ok(fd) => Kernellogger {klog_level: level, klog_fd: fd, prefix:prefix.to_string()},
            Err(error) => 
            {
                println!("/dev/kmsg open failed {}", error);
                Kernellogger {klog_level: level, klog_fd: 0, prefix: prefix.to_string()}
            },
        }
}

impl Kernellogger {
    pub fn write_message(&self, message:String) {
        if self.klog_fd == 0 {
            println!("Abe pehle fd to banale");
        } else {
            let iovec = IoVec::from_slice(message.as_bytes());
            writev(self.klog_fd, &[iovec]);
        }
    }
}