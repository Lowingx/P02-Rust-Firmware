/// Módulo de comunicação serial
/// 
/// Fornece funções para enviar dados via UART.

use esp_hal::serial::Serial;

/// Envia string via serial
/// 
/// # Arguments
/// * `serial` - Porta serial
/// * `msg` - Mensagem para enviar
pub fn send_string(serial: &mut Serial, msg: &str) {
    let _ = serial.write_str(msg);
}

/// Envia temperatura formatada
/// 
/// # Arguments
/// * `serial` - Porta serial
/// * `temp` - Temperatura em graus Celsius
pub fn send_temperature(serial: &mut Serial, temp: f32) {
    let msg = format!("TEMP:{:.1}\n", temp);
    send_string(serial, &msg);
}

/// Envia luminosidade formatada
/// 
/// # Arguments
/// * `serial` - Porta serial
/// * `light` - Luminosidade (0.0 a 1.0)
pub fn send_light(serial: &mut Serial, light: f32) {
    let msg = format!("LIGHT:{:.3}\n", light);
    send_string(serial, &msg);
}

/// Envia dados brutos do ADC
/// 
/// # Arguments
/// * `serial` - Porta serial
/// * `raw` - Valor bruto (0-4095)
pub fn send_raw(serial: &mut Serial, raw: u16) {
    let msg = format!("RAW:{}\n", raw);
    send_string(serial, &msg);
}
