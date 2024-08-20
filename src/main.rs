#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "testmain"]

pub trait Test {
    fn run(&self) -> ();
}

impl<T> Test for T where T: Fn(), {
    fn run(&self) {
        sprint!("{}...\t", core::any::type_name::<T>());
        self();
        sprintln!("[OK]");
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Test()]) {
    use qemu::exit_qemu;

    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(qemu::QemuExitCode::Success);
}

mod qemu;
mod serial;
mod vga;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sprintln!("[failed]\n");
    sprintln!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World");

    #[cfg(test)]
    testmain();

    loop {}
}
