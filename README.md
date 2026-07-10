# P02 — Rust Firmware

> Firmware embarcado em Rust: sensor → serial. Primeiro passo para BCI.

## O Que É

Projeto de firmware em Rust `no_std` que lê um sensor de temperatura (LM35) e transmite via serial. Microcontrolador: ESP32 com `esp-hal`.

**Por que Rust:** Segurança de memória sem garbage collector — essencial para dispositivos implantáveis.

## Hardware Necessário

| Item | Qtd | Onde Comprar |
|------|-----|--------------|
| ESP32 DevKit v1 | 1 | Mercado Livre (~R$35) |
| LM35 (sensor temperatura) | 1 | Mercado Livre (~R$5) |
| Protoboard | 1 | Mercado Livre (~R$15) |
| Jumpers | 5 | Mercado Livre (~R$5) |

**Total: ~R$55**

## Circuito

```
LM35 VCC → 3.3V
LM35 GND → GND
LM35 OUT → GPIO 34 (ADC)
```

## Setup

```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Adicionar target ESP32
rustup target add xtensa-esp32-none-elf

# Instalar espflash
cargo install espflash

# Compilar e flashar
cargo run
```

## Estrutura do Código

```
src/
├── main.rs      # Entry point principal
├── sensor.rs    # Leitura de sensores
├── serial.rs    # Comunicação serial
└── error.rs     # Tratamento de erros
```

## Como Rodar

1. Conectar ESP32 via USB
2. Rodar `cargo run`
3. Abrir Serial Monitor (115200 baud)
4. Ver dados de temperatura sendo enviados

## Status

- [x] Estrutura do projeto
- [x] Código principal
- [x] Módulo de sensor
- [x] Módulo serial
- [x] Tratamento de erros
- [ ] Teste em hardware real

## Tech Stack

![Rust](https://img.shields.io/badge/Rust-0a0805?style=for-the-badge&logo=rust&logoColor=c9a227&labelColor=0a0805)
![ESP32](https://img.shields.io/badge/ESP32-0a0805?style=for-the-badge&logo=espressif&logoColor=c9a227&labelColor=0a0805)
