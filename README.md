# 🔐 Password Analyzer CLI

Una herramienta de línea de comandos escrita en Rust que permite analizar la seguridad de contraseñas, generar contraseñas seguras y detectar si una contraseña es común (basado en wordlists).

## 🚀 Funcionalidades

- Verifica la fuerza de contraseñas dadas (longitud, caracteres especiales, etc.).
- Detecta contraseñas que aparecen en archivos de contraseñas comunes (`wordlist.txt`).
- Genera contraseñas seguras con los criterios más importantes.
- Interfaz de línea de comandos amigable usando [`clap`](https://docs.rs/clap/).
- Salida con colores y símbolos para visualización clara (requiere soporte ANSI).

## 📦 Instalación

1. Clona el repositorio:

```bash
git clone https://github.com/RicardoUMC/Password-Analyzer-Cli.git
cd Password-Analyzer-Cli
```

2. Asegúrate de tener [Rust](https://www.rust-lang.org/tools/install) instalado.

3. Compila el proyecto:

```bash
cargo build --release
```

## 🧪 Uso

### Analizar una contraseña

```bash
cargo run -- MiC0ntraseñaSegura!
```

### Generar una contraseña segura

```bash
cargo run -- -g
```

o usando la siguiente forma:

```bash
cargo run -- --generate
```

### Analizar contraseñas desde archivo

Supón que tienes una lista en `wordlist.txt` y deseas verificar si la contraseña está incluida:

```bash
cargo run -- -c wordlist.txt "123456"
```

o usando la forma larga de la opción:

```bash
cargo run -- -common wordlist.txt "123456"
```

## 📁 Estructura del proyecto

- `src/main.rs` – Punto de entrada principal del programa.
- `src/cli/` – Módulo encargado de manejar la interfaz de línea de comandos y el procesamiento de argumentos.
- `src/common/` – Módulo para la carga de contraseñas comunes (`wordlist.txt`).
- `src/password/` – Módulo que contiene la lógica principal para el análisis y la generación de contraseñas.
- `src/utils/` – Módulo con funciones auxiliares diversas como impresiones en pantalla.
- `wordlist.txt` – Wordlist opcional de contraseñas comunes, carga tu propio archivo.

## 🧱 Tecnologías utilizadas

- Rust
- [clap](https://crates.io/crates/clap) – CLI parsing
- [regex](https://crates.io/crates/regex) – Validación de patrones
- [colored](https://crates.io/crates/colored) – Colores en terminal

## ✅ Ejemplos de salida

```bash
Received password: MiC0ntraseñaSegura!
--- Security Analysis ---

✓ Valid length (>=10)
✓ Has uppercase
✓ Has lowercase
✓ Has numbers
✓ Has symbols

Passowrd strength: [■■■■■■■■■■] 100% (Strong)
```

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT. Consulta el archivo [LICENSE](LICENSE) para más detalles.

---

## ✨ Créditos

Desarrollado por [Ricardo Mora](https://github.com/RicardoUMC).
