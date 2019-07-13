extern crate nix;

use nix::sys::stat::umask;
use nix::sys::stat::Mode;
use std::env;
use nix::unistd::mkdir;
use nix::mount::mount;
use nix::mount::MsFlags;
use std::fmt::Write;
#[macro_use]
mod klog;

const VERSION_TRAMPOLINE:u8 = 28;

struct main_struct {
    Logger: klog::Kernellogger,
}

fn MOUNT_OR_PANIC(source:&str, target:&str, fstype:&str, flags:MsFlags, data:&str) {
    if data.is_empty() {
        mount(Some(source), target, Some(fstype), flags, None::<&str>).unwrap_or_else(|_| panic!("Failed to mount {}", target));    
    } else {
        mount(Some(source), target, Some(fstype), flags, Some(data)).unwrap_or_else(|_| panic!("Failed to mount {}", target));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 0 {
        //process commandline here
    }

    let logger =  klog::mrom_klog_init(6, "Trampoline-rs");
    let mut mainCtx = main_struct {Logger: logger};
    

    info!(mainCtx, "Trampoline-rs version {} initialising", VERSION_TRAMPOLINE);

    umask(Mode::empty());
    let mode = Mode::from_bits(0755).expect("Failed to generate mode. Invalid mask ?");
    mkdir("/dev", mode).expect("Failed to create /dev");
    mkdir("/dev/pts", mode).expect("Failed to create /dev/pts");
    mkdir("/dev/socket", mode).expect("Failed to create /dev/socket");
    mkdir("/proc", mode).expect("Failed to create /proc");
    mkdir("/sys", mode).expect("Failed to create /sys");

    MOUNT_OR_PANIC("tmpfs", "/dev", "tmpfs", MsFlags::MS_NOSUID, "mode=0755");
    MOUNT_OR_PANIC("devpts", "/dev/pts", "devpts", MsFlags::empty(), "");
    MOUNT_OR_PANIC("proc", "/proc", "proc", MsFlags::empty(), "");
    MOUNT_OR_PANIC("sysfs", "/sys", "sysfs", MsFlags::empty(), "");
    MOUNT_OR_PANIC("pstore", "/sys/fs/pstore", "pstore", MsFlags::empty(), "");
    MOUNT_OR_PANIC("selinuxfs", "/sys/fs/selinux", "selinuxfs", MsFlags::empty(), "");

    //hook before_device_init

    info!(mainCtx, "Initialising devices...");

    
}
