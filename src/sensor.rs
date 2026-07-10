/// Módulo de leitura de sensor
/// 
/// Fornece funções para ler sensores analógicos (LM35, LDR)
/// e converter para valores físicos.

use esp_hal::analog::adc::{AdcChannel, AdcConfig};

/// Lê temperatura do LM35
/// 
/// # Arguments
/// * `adc` - Controlador ADC configurado
/// 
/// # Returns
/// Temperatura em graus Celsius
pub fn read_temperature(adc: &mut AdcConfig) -> f32 {
    let raw = adc.read();
    let voltage = (raw as f32) * 3.3 / 4095.0;
    voltage * 100.0  // LM35: 10mV por grau
}

/// Lê luminosidade do LDR
/// 
/// # Arguments
/// * `adc` - Controlador ADC configurado
/// 
/// # Returns
/// Valor de 0.0 (escuro) a 1.0 (claro)
pub fn read_light(adc: &mut AdcConfig) -> f32 {
    let raw = adc.read();
    (raw as f32) / 4095.0
}

/// Lê valor bruto do ADC
/// 
/// # Arguments
/// * `adc` - Controlador ADC configurado
/// 
/// # Returns
/// Valor de 0 a 4095 (12 bits)
pub fn read_raw(adc: &mut AdcConfig) -> u16 {
    adc.read()
}
