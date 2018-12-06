pub mod io;
pub mod interrupt;
pub mod timer;
pub mod paging;
pub mod memory;
pub mod compiler_rt;
pub mod consts;
pub mod cpu;

#[cfg(feature = "board_qemu")]
#[path = "board/qemu/mod.rs"]
pub mod board;

#[cfg(feature = "board_zedboard")]
#[path = "board/zedboard/mod.rs"]
pub mod board;

#[no_mangle]
pub extern fn rust_main(hartid: usize, dtb: usize, hart_mask: usize) -> ! {
    unsafe { cpu::set_cpu_id(hartid); }

    if hartid != 0 {
        while unsafe { !cpu::has_started(hartid) }  { }
        println!("Hello RISCV! in hart {}, {}, {}", hartid, dtb, hart_mask);
        others_main();
        unreachable!();
    }

    memory::clear_bss();
    crate::logging::init();
    interrupt::init();
    memory::init();
    timer::init();
    crate::process::init();

    #[cfg(feature = "board_zedboard")]
    board::lvna::test_prm();

    unsafe { cpu::start_others(hart_mask); }
    println!("Hello RISCV! in hart {}, {}, {}", hartid, dtb, hart_mask);
    crate::kmain();
}

fn others_main() -> ! {
    interrupt::init();
    memory::init_other();
    timer::init();
    crate::kmain();
}

#[cfg(feature = "board_fpga")]
global_asm!(include_str!("boot/boot.asm"));
global_asm!(include_str!("boot/entry.asm"));
global_asm!(include_str!("boot/trap.asm"));