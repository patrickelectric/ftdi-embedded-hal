use embedded_hal::blocking::spi::Write;
use ftdi_embedded_hal as hal;
use std::thread::sleep;
use std::time::Duration;

#[cfg(all(feature = "ftdi", feature = "libftd2xx"))]
compile_error!("features 'ftdi' and 'libftd2xx' cannot be enabled at the same time");

#[cfg(not(any(feature = "ftdi", feature = "libftd2xx")))]
compile_error!("one of features 'ftdi' and 'libftdi2xx' shall be enabled");

fn main() {
    #[cfg(feature = "libftd2xx")]
    let device = libftd2xx::Ft2232h::with_description("Dual RS232-HS A").unwrap();

    #[cfg(feature = "ftdi")]
    let device = ftdi::find_by_vid_pid(0x0403, 0x6010)
        .interface(ftdi::Interface::A)
        .open()
        .unwrap();

    let hal = hal::FtHal::init_freq(device, 3_000_000).unwrap();
    let mut spi = hal.spi().unwrap();

    // spi sequence for ws2812 color value 0x10
    let b1 = [0x92, 0x69, 0x24];

    // spi sequence for ws2812 color value 0x00
    let b0 = [0x92, 0x49, 0x24];

    // spi sequences for single led of specific color
    let g = [b1, b0, b0];
    let r = [b0, b1, b0];
    let b = [b0, b0, b1];
    let x = [b0, b0, b0];

    // initial pattern
    let mut pattern = vec![r, r, g, g, x, x, b, b];

    println!("ready to go...");

    loop {
        println!("next pattern...");
        let stream = pattern
            .clone()
            .into_iter()
            .flatten()
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>();

        spi.write(stream.as_slice()).unwrap();
        sleep(Duration::from_millis(400));
        // rotate pattern
        pattern.rotate_right(1);
    }
}
