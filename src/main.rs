#![no_std]
#![no_main]

use esp_hal::prelude::*;
use esp_hal::gpio::{Input, Pull, Level};
use esp_hal::analog::adc::{AdcChannel, AdcConfig};
use esp_hal::serial::config::Config;

#[entry]
fn main() -> ! {
    // Initialize peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());
    
    // Setup Serial
    let mut serial = esp_hal::serial::Serial::new(
        peripherals.UART0,
        Config::default().baudrate(115200.Bd()),
    );
    
    // Setup ADC for LM35 temperature sensor
    let mut adc = AdcConfig::new().pin(peripherals.GPIO34).build();
    
    // Welcome message
    serial.write_str("P02 Rust Firmware - Sensor to Serial\n").ok();
    serial.write_str("Reading LM35 temperature sensor...\n\n").ok();
    
    loop {
        // Read ADC value
        let raw_value = adc.read();
        
        // Convert to voltage (3.3V reference, 12-bit ADC)
        let voltage = (raw_value as f32) * 3.3 / 4095.0;
        
        // Convert to temperature (LM35: 10mV per degree Celsius)
        let temperature = voltage * 100.0;
        
        // Format and send via serial
        let msg = format!("Temperature: {:.1}°C (Raw: {}, Voltage: {:.3}V)\n", 
                         temperature, raw_value, voltage);
        serial.write_str(&msg).ok();
        
        // Wait 1 second
        esp_hal::delay::Delay::new().delay_ms(1000u32);
    }
}
