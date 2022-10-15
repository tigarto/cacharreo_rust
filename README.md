# Cacharreo Rust

A modo de diario se va a ir documentando lo que va sucediendo al cacharrear con rust haciendo enfasis en comentar los casos cuando hayan problemas. La idea es ir siguiendo secuencialmente el libro [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html) e ir aprendiendo a la marcha. 

Es util tener a la mano la siguiente referencia: [Rust Language Cheat Sheet](https://cheats.rs/)

### Dia 1

* **Fecha**: 10/14/2022

## Actividades 

- [x] Instalación de Rust
- [x] Test de la instalación 
- [ ] Creación y test de un primer programa
- [ ] Capitulo 1 del libro ([Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html))

### Actividad 1 - Instalación de Rust

Para la instalación de rust se siguió la sección [Installation](https://doc.rust-lang.org/book/ch01-01-installation.html). Como la maquina tiene Windows, se siguieron las instrucciones del siguiente [link](https://www.rust-lang.org/tools/install):
1. Primero se instralo [Visual Studio C++ Build tools](https://visualstudio.microsoft.com/es/visual-cpp-build-tools/), esto se ejecuto sin problemas.
2. Luego se descargo la aplicación rustup al presionar el boton **[DOWNLOAD RUSTUP-INIT.EXE (64-BIT)](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)**

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

Se siguio la sección 1.2 [Hello, World!](https://doc.rust-lang.org/book/ch01-02-hello-world.html).


## Referencias

1. https://github.com/rust-unofficial/awesome-rust
2. https://github.com/jondot/rust-how-do-i-start
3. https://doc.rust-lang.org/book/
4. https://github.com/rust-lang/book/
