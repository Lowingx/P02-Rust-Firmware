# Guia de Estudo — P02 Rust Firmware

> O que estudar antes, durante e depois de cada etapa.

---

## ANTES DE COMEÇAR

### Pré-requisitos

1. **P01 concluído** — você precisa ter montado o circuito LED + botão
2. **Arduino básico** — saber o que é GPIO, Serial, loop
3. **Rust básico** — variáveis, funções, ownership

### Conceitos de Rust

**1. Ownership:**
```rust
let x = String::from("hello");  // x é o dono
let y = x;                       // x foi movido para y
// println!("{}", x);            // ERRO! x não existe mais
```

**2. Borrowing:**
```rust
fn imprime(texto: &String) {     // empréstimo imutável
    println!("{}", texto);
}

let s = String::from("hello");
imprime(&s);                     // s ainda existe
```

**3. Result<T, E>:**
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Divisão por zero"))
    } else {
        Ok(a / b)
    }
}
```

**4. no_std:**
```rust
#![no_std]          // não usa biblioteca padrão
#![no_main]         // não usa main() do C

#[entry]
fn main() -> ! {
    loop {}
}
```

---

## ETAPA POR ETAPA

### E1: Setup do Toolchain (1 hora)

**O que fazer:**
1. Instalar Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Instalar espflash: `cargo install espflash`
3. Criar projeto: `cargo generate --git https://github.com/esp-rs/esp-idf-template`
4. Rodar: `cargo run`

**Material:**
- [The Rust Book](https://doc.rust-lang.org/book/) — capítulos 1-3
- [esp-rs](https://docs.esp-rs.org/) — documentação ESP32 + Rust

**Checklist:**
- [ ] Rust instalado (`rustc --version`)
- [ ] Target ESP32 configurado
- [ ] Projeto compila
- [ ] "Hello World" roda no ESP32

---

### E2: Blink em Rust (1 hora)

**O que fazer:**
- Reimplementar o blink do P01 em Rust
- Entender a diferença entre Arduino C++ e Rust embarcado

**Conceitos-chave:**
```rust
// Configurar pino como saída
let mut led = peripherals.GPIO2.into_push_pull_output();

// Ligado
led.set_high();

// Desligado
led.set_low();
```

**Material:**
- [[Rust_Embarcado_Basico]] — no_std, embedded-hal
- Exemplo: esp-hal examples no GitHub

**Checklist:**
- [ ] Blink funcionando em Rust
- [ ] Entende a diferença entre C++ e Rust embarcado
- [ ] Entende o que é no_std

---

### E3: Leitura de Sensor (2 horas)

**O que fazer:**
1. Conectar LM35 (temperatura) ao ADC do ESP32
2. Ler valor analógico
3. Converter para temperatura
4. Transmitir via serial

**Circuito:**
```
LM35 VCC → 3.3V
LM35 GND → GND
LM35 OUT → GPIO 34 (ADC)
```

**Código conceitual:**
```rust
// Ler ADC
let valor = adc.read();

// Converter para temperatura
let temperatura = (valor as f32) * 3.3 / 4095.0 * 100.0;

// Transmitir
serial.println(format!("Temperatura: {:.1}°C", temperatura));
```

**Material:**
- Datasheet do LM325
- [[ADS1299_Completo]] — conceitos de ADC

**Checklist:**
- [ ] Sensor conectado corretamente
- [ ] Leitura ADC funcionando
- [ ] Temperatura convertida corretamente
- [ ] Dados aparecem no Serial Monitor

---

### E4: Tratamento de Erro (1 hora)

**O que fazer:**
- Usar `Result<T, E>` em vez de `unwrap()`
- Tratar erros de leitura
- Nunca usar `panic!` em dispositivo embarcado

**Exemplo:**
```rust
fn ler_sensor() -> Result<f64, &'static str> {
    match adc.read() {
        Ok(valor) => Ok(valor as f64 * 3.3 / 4095.0 * 100.0),
        Err(_) => Err("Falha na leitura do ADC"),
    }
}

match ler_sensor() {
    Ok(temp) => serial.println(format!("T: {:.1}°C", temp)),
    Err(e) => serial.println(format!("Erro: {}", e)),
}
```

**Checklist:**
- [ ] Todos os erros tratados com Result
- [ ] Nenhum `unwrap()` no código principal
- [ ] Mensagens de erro claras

---

### E5: Documentação (30 min)

**O que fazer:**
1. Atualizar README com instruções
2. Adicionar diagrama do circuito
3. Explicar como rodar

**Checklist:**
- [ ] README completo
- [ ] Instruções de instalação
- [ ] Diagrama do circuito
- [ ] Exemplo de uso

---

## RESUMO

| Etapa | Conceito Principal | Tempo |
|-------|-------------------|-------|
| E1 | Setup Rust + ESP32 | 1 hora |
| E2 | Blink em Rust, no_std | 1 hora |
| E3 | Leitura de sensor, ADC | 2 horas |
| E4 | Tratamento de erro, Result | 1 hora |
| E5 | Documentação | 30 min |

**Tempo total: ~5.5 horas**

---

## Diferenças: Arduino C++ vs Rust

| Conceito | Arduino C++ | Rust |
|----------|-------------|------|
| Main | `void setup()` + `void loop()` | `#[entry] fn main() -> !` |
| Variáveis | `int x = 5;` | `let x: i32 = 5;` |
| Mutabilidade | `int x = 5; x = 10;` | `let mut x = 5; x = 10;` |
| Erros | `if (erro) { }` | `match resultado { Ok() =>, Err() => }` |
| Biblioteca padrão | Sim | Não (no_std) |

## Depois de P02

Quando terminar P02, você terá:
- ✅ Rust embarcado funcionando
- ✅ Leitura de sensores reais
- ✅ Comunicação serial
- ✅ Tratamento de erros
- ✅ Conceitos de no_std

**Próximo:** bci-toolkit (biblioteca Python para EEG)
