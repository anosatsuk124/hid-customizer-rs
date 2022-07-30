#![no_main]
#![no_std]

mod usb_host_driver;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    let mut pac = rp_pico::pac::Peripherals::take().unwrap();
    let sio = rp2040_hal::Sio::new(pac.SIO);
    let mut watchdog = rp2040_hal::watchdog::Watchdog::new(pac.WATCHDOG);
    let mut clocks = rp2040_hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    exit()
}
