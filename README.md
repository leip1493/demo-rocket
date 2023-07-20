# Demo Rocket

Este repositorio contiene una aplicación de demostración escrita en Rust utilizando el framework Rocket.

## Requisitos

Asegúrate de tener instalado Rust y Cargo en tu sistema antes de ejecutar esta aplicación.

## Instalación

Sigue estos pasos para ejecutar la aplicación en tu entorno local:

1. Clona este repositorio: `git clone https://github.com/tu-usuario/demo-rocket.git`
2. Navega al directorio del repositorio: `cd demo-rocket`
3. Ejecuta la aplicación con Cargo: `cargo run`

## Configuración del entorno

Antes de ejecutar la aplicación, asegúrate de configurar el archivo de entorno. Este repositorio incluye un archivo `.env.example` que puedes utilizar como base. Sigue estos pasos para configurar el archivo `.env`:

1. Renombra el archivo `.env.example` a `.env`.
2. Abre el archivo `.env` en un editor de texto y configura las variables de entorno según tus necesidades.

Asegúrate de no incluir información confidencial en el archivo `.env` y nunca lo compartas públicamente.

## Comandos de Cargo

A continuación se muestran algunos comandos útiles de Cargo que puedes utilizar en este proyecto:

```bash
# Compila y ejecuta la aplicación en tu entorno local
cargo run

# Compila la aplicación sin ejecutarla
cargo build

# Ejecuta las pruebas automatizadas incluidas en el proyecto
cargo test

# Genera la documentación del proyecto en formato HTML
cargo doc
```

Recuerda ejecutar estos comandos en tu terminal o línea de comandos en un entorno que tenga Rust y Cargo correctamente instalados. Asegúrate de navegar al directorio del repositorio antes de ejecutar los comandos.
## Dependencias

Esta aplicación depende de las siguientes bibliotecas y versiones:

- entity 0.1.0
- migration 0.1.0
- rocket 0.5.0-rc.3 (con la característica "json")
- serde 1.0 (con la característica "derive")
- sea-orm 0.11.3 (con las características "sqlx-postgres", "runtime-tokio-native-tls" y "macros")
- sea-orm-rocket 0.5.2
- async-trait 0.1
- dotenvy 0.15.7
- rocket_dyn_templates 0.1.0-rc.3 (con la característica "handlebars")

## Licencia

Este proyecto está bajo la [licencia MIT](LICENSE).
