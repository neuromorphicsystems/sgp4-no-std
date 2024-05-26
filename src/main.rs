#![allow(internal_features)]
#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(naked_functions)]

use num_traits::Float;

extern crate compiler_builtins;

#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start() {
    core::arch::asm!("mov rdi, rsp", "call main", options(noreturn))
}

#[no_mangle]
pub unsafe fn main(_stack_top: *const u8) {
    let elements = sgp4::Elements::from_tle(
        "1 25544U 98067A   20194.88612269 -.00002218  00000-0 -31515-4 0  9992".as_bytes(),
        "2 25544  51.6461 221.2784 0001413  89.1723 280.4612 15.49507896236008".as_bytes(),
    )
    .unwrap();
    let constants = sgp4::Constants::from_elements(&elements).unwrap();
    for hours in 0..24 {
        let mut buffer = [0u8; 1024];
        print_label_and_integer("t (m) = ", hours * 60, &mut buffer);
        let prediction = constants
            .propagate(sgp4::MinutesSinceEpoch((hours * 60) as f64))
            .unwrap();
        print_label_and_integer(
            "    rx (m) = ",
            (prediction.position[0] * 1000.0).round() as i64,
            &mut buffer,
        );
        print_label_and_integer(
            "    ry (m) = ",
            (prediction.position[1] * 1000.0).round() as i64,
            &mut buffer,
        );
        print_label_and_integer(
            "    rz (m) = ",
            (prediction.position[2] * 1000.0).round() as i64,
            &mut buffer,
        );
        print_label_and_integer(
            "    ṙx (m.s⁻¹) = ",
            (prediction.velocity[0] * 1000.0).round() as i64,
            &mut buffer,
        );
        print_label_and_integer(
            "    ṙy (m.s⁻¹) = ",
            (prediction.velocity[1] * 1000.0).round() as i64,
            &mut buffer,
        );
        print_label_and_integer(
            "    ṙz (m.s⁻¹) = ",
            (prediction.velocity[2] * 1000.0).round() as i64,
            &mut buffer,
        );
    }
    unsafe { exit(1) }
}

unsafe fn exit(code: i32) -> ! {
    let syscall_number: u64 = 60;
    core::arch::asm!(
        "syscall",
        in("rax") syscall_number,
        in("rdi") code,
        options(noreturn)
    )
}

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { exit(1) };
}

unsafe fn print(fd: u32, buf: *const u8, length: usize) {
    let syscall_number: u64 = 1;
    core::arch::asm!(
        "syscall",
        inout("rax") syscall_number => _,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") length,
        lateout("rcx") _, lateout("r11") _,
        options(nostack)
    );
}

fn write_integer(integer: u64, buffer: &mut [u8]) -> usize {
    let mut length = 0;
    if integer >= 10 {
        length += write_integer(integer / 10, buffer);
    }
    buffer[length] = b'0' + (integer % 10) as u8;
    length += 1;
    length
}

fn print_label_and_integer(label: &'static str, integer: i64, buffer: &mut [u8]) {
    buffer[0..label.len()].copy_from_slice(label.as_bytes());
    let mut length = label.len();
    if integer < 0 {
        buffer[length] = b'-';
        length += 1;
    }
    length += write_integer(integer.abs() as u64, &mut buffer[length..]);
    buffer[length] = b'\n';
    length += 1;
    unsafe { print(1, buffer.as_ptr(), length) };
}
