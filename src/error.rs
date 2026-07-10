/// Módulo de tratamento de erros
/// 
/// Define tipos de erro e funções de tratamento.

/// Erros do firmware
#[derive(Debug)]
pub enum FirmwareError {
    /// Erro na leitura do ADC
    AdcError,
    /// Erro na comunicação serial
    SerialError,
    /// Valor fora da faixa
    OutOfRange(f32),
    /// Timeout
    Timeout,
}

impl core::fmt::Display for FirmwareError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FirmwareError::AdcError => write!(f, "Erro na leitura do ADC"),
            FirmwareError::SerialError => write!(f, "Erro na comunicação serial"),
            FirmwareError::OutOfRange(val) => write!(f, "Valor fora da faixa: {}", val),
            FirmwareError::Timeout => write!(f, "Timeout"),
        }
    }
}

/// Converte erro para mensagem de erro
/// 
/// # Arguments
/// * `error` - Erro do firmware
/// 
/// # Returns
/// Mensagem de erro formatada
pub fn error_to_string(error: FirmwareError) -> &'static str {
    match error {
        FirmwareError::AdcError => "ADC_READ_FAIL",
        FirmwareError::SerialError => "SERIAL_FAIL",
        FirmwareError::OutOfRange(_) => "OUT_OF_RANGE",
        FirmwareError::Timeout => "TIMEOUT",
    }
}

/// Valida se temperatura está em faixa válida
/// 
/// # Arguments
/// * `temp` - Temperatura em graus Celsius
/// 
/// # Returns
/// Resultado com temperatura ou erro
pub fn validate_temperature(temp: f32) -> Result<f32, FirmwareError> {
    if temp < -40.0 || temp > 125.0 {
        Err(FirmwareError::OutOfRange(temp))
    } else {
        Ok(temp)
    }
}
