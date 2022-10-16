# Dia 1

## Actividades 

- [x] Instalación de Rust
- [x] Test de la instalación 
- [x] Creación y test de un primer programa
- [x] Capitulo 1 del libro ([Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html))
- [x] Instalar la extensión **rust-analyzer** de Rust de Visual Studio Code

### Actividad 1 - Instalación de Rust

Para la instalación de rust se siguió la sección [Installation](https://doc.rust-lang.org/book/ch01-01-installation.html). Como la maquina tiene Windows, se siguieron las instrucciones del siguiente [link](https://www.rust-lang.org/tools/install):
1. Primero se instralo [Visual Studio C++ Build tools](https://visualstudio.microsoft.com/es/visual-cpp-build-tools/), esto se ejecuto sin problemas.
2. Luego se descargo la aplicación rustup al presionar el boton **[DOWNLOAD RUSTUP-INIT.EXE (64-BIT)](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)**

No obstante en [Set up your dev environment on Windows for Rust](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup?source=recommendations) se explica muy bien el proceso en windows.

**Conclusiones:**
1. El proceso de instalación fue exitoso. La siguiente salida muestra el resultado que se obtuvo:

```
Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  C:\Users\Usuario\.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  C:\Users\Usuario\.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  C:\Users\Usuario\.cargo\bin

This path will then be added to your PATH environment variable by
modifying the HKEY_CURRENT_USER/Environment/PATH registry key.

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>1

info: profile set to 'default'
info: default host triple is x86_64-pc-windows-msvc
info: syncing channel updates for 'stable-x86_64-pc-windows-msvc'
info: latest update on 2022-09-22, rust version 1.64.0 (a55dd71d5 2022-09-19)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-std'
info: downloading component 'rustc'
 63.0 MiB /  63.0 MiB (100 %)  24.9 MiB/s in  2s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 18.7 MiB /  18.7 MiB (100 %)   2.1 MiB/s in 16s ETA:  0s
info: installing component 'rust-std'
 24.7 MiB /  24.7 MiB (100 %)   6.0 MiB/s in  4s ETA:  0s
info: installing component 'rustc'
 63.0 MiB /  63.0 MiB (100 %)   6.7 MiB/s in  9s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-x86_64-pc-windows-msvc'

  stable-x86_64-pc-windows-msvc installed - rustc 1.64.0 (a55dd71d5 2022-09-19)


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload its PATH environment variable to include
Cargo's bin directory (%USERPROFILE%\.cargo\bin).

Press the Enter key to continue.
```

### Actividad 2 - Test de la instalación 

El chequeo de la instalación fue exitoso, esto se probo con el comando que arrojo lo esperado:

```
rustc --version
```

Despues de chequear se actualizó y esto tambien fue exitoso:

```
rustup update
```

Para llamar la documentación local

```
rustup doc
```

### Actividad 3 - Primer programa de test

#### Compilación sencilla usando el rustc

Se siguio la sección 1.2 [Hello, World!](https://doc.rust-lang.org/book/ch01-02-hello-world.html). Se siguieron las instrucciones y el resultado fue el esperado.

1. Creación del directorio de trabajo
   
   ```
   mkdir hello_world
   cd hello_world
   ```

2. Creación del archivo [main.rs](./hello_world/main.rs)
   
   ```rs
   fn main() {
       println!("Hello, world!");
   }
   ```

3. Compilación
   
   ```
   rustc main.rs
   ```

4. Ejecución
   
   ```
   ./main
   ```
   
   El resultado fue el esperado y se muestra a continuación:
   
   ```
   Hello, world!
   ```

Para generar codigo automaticamente ver https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html

#### Compilación sencilla usando cargo

Se siguio la sección 1.3 [Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html).

Cargo es el sistema de compilación y el gestor de paquetes; este permite realizar tareas como:
* Compilar codigo.
* Descargar y compilar dependencias (librerias de las que depende el codigo).

La ventaja de usar Cargo, es que agregar dependencias se hace mucho mas facil. Para verificar la instalación de Cargo usamos:

```
cargo --version
```
A continuación se muestra como crear un proyecto en cargo:

1. Crear un proyecto con cargo:
   
   ```
   cargo new hello_cargo
   cd hello_cargo
   ```

2. Los archivos y directorios generados son:
   * [hello_cargo](./hello_cargo/): Directorio del proyecto  
     *  [src](./hello_cargo/src/): Directorio de codigo
        * [main.rs](./hello_cargo/src/main.rs): Archivo fuente don el main  
     *  [Cargo.toml](./hello_cargo/Cargo.toml): Archivo de configuración en formato [TOML](https://toml.io/en/) (Tom’s Obvious, Minimal Language). Aqui se agregan las dependencias. En Rust los paquetes de codigo son conocidos como **crates**    

3. Compile el programa ejecutando dentro del directorio raiz del proyecto el comando:
   
   ```
   cargo build
   ```

4. Este comando genera el ejecutable **hello_cargo** dentro del directorio **target/debug**, asi para 
   correrlo use:

   ```
   cargo run
   ```

   El comando ```cargo run``` si se usa compila y ejecuta permitiendo hacer todo el procedimiento en un solo paso.

   Otro comando util es ```cargo check```, el cual se usa periodicamente para verificar que todo este bien pero no genera el ejecutable. Finalmente, cuando ya se esta seguro que lo que se quiere es generar el ejecutable se usa ```cargo build```.


Para trabajar con proyectos existentes siga el siguiente template:

```
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

## Referencias

1. https://github.com/rust-unofficial/awesome-rust
2. https://github.com/jondot/rust-how-do-i-start
3. https://doc.rust-lang.org/book/
4. https://github.com/rust-lang/book/
