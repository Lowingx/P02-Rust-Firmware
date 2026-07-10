# P02 — Rust Firmware

> Firmware embarcado em Rust: sensor → serial. Primeiro passo para BCI.

## O Que É

Projeto de firmware em Rust `no_std` que lê um sensor (temperatura/luminosidade) e transmite via serial. Microcontrolador: ESP32 com `esp-hal`.

**Por que Rust:** Segurança de memória sem garbage collector — essencial para dispositivos implantáveis.

## Pré-requisitos

- P01 concluído (Arduino básico)
- Rust instalado (rustup)
- ESP32 ou Raspberry Pi Pico
- Sensor LM35 (temperatura) ou LDR (luminosidade)

## Setup

```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Adicionar target ESP32
rustup target add xtensa-esp32-none-elf

# Instalar espflash
cargo install espflash
```

## Estrutura

```
src/
├── main.rs          # Entry point
├── sensor.rs        # Leitura do sensor
├── serial.rs        # Comunicação serial
└── error.rs         # Tratamento de erros
```

## Etapas

### E1: Setup do Toolchain
- Instalar Rust
- Configurar target ESP32
- Rodar "Hello World"

### E2: Blink em Rust
- Reimplementar P01-E2 em Rust
- Entender no_std

### E3: Leitura de Sensor
- Conectar LM35 ou LDR
- Ler valor via ADC
- Transmitir via serial

### E4: Tratamento de Erro
- Implementar Result<T, E>
- Tratar erros sem panic

### E5: Documentação
- README completo
- Diagrama do circuito
- Instruções de uso

## Status

- [ ] E1: Setup do Toolchain
- [ ] E2: Blink em Rust
- [ ] E3: Leitura de Sensor
- [ ] E4: Tratamento de Erro
- [ ] E5: Documentação

## Tech Stack

![Rust](https://img.shields.io/badge/Rust-0a0805?style=for-the-badge&logo=rust&logoColor=c9a227&labelColor=0a0805)
![ESP32](https://img.shields.io/badge/ESP32-0a0805?style=for-the-badge&logo=espressif&logoColor=c9a227&labelColor=0a0805)
