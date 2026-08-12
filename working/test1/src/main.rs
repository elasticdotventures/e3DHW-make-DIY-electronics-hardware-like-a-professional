
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::{
    gpio::{IOPin,PinDriver},
    peripherals::Peripherals,
    delay::FreeRtos
};
use esp_println::println;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led_pin = PinDriver::output(peripherals.pins.gpio3).unwrap();
    loop {
        led_pin.set_low().unwrap();
        println!("Led on");
        FreeRtos::delay_ms(1000);
        led_pin.set_high().unwrap();
        println!("Led off");
        FreeRtos::delay_ms(1000);
    }
    
}
