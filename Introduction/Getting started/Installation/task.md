## Getting Started

Let’s start your Rust journey! There’s a lot to learn, but every journey starts somewhere. In this lesson, we’ll discuss:

* Verification of Rust installation
* Writing a program that prints Hello, world!

### Command Line Notation

In this lesson and throughout the course, we’ll show you some commands used in the terminal. Lines that you should enter in the terminal all start with `$`. You don’t need to type in the `$` sign – it just indicates the start of each command. Lines that don’t start with `$` typically show the output of the previous command. Additionally, PowerShell-specific examples will use <code>&gt;</code> rather than `$`.

### Rust Installation

First, you need to set up the environment on your computer; below are a few things which you need to do before you start working.

#### Unix
1. In Terminal, run the following command to download Rust toolchain installer:
```text
$ curl https://sh.rustup.rs -sSf | sh
```
2. Now run this:
```text
$ source $HOME/.cargo/env
```
3. Restart your IDE.
4. Set up Rust toolchain (press "Setup toolchain" in the pop-up "No Rust toolchain configured").
5. You're good to go!

#### Windows
1. Download the rustup installer from https://win.rustup.rs/x86_64.
2. Run it and follow the onscreen instructions.
3. Restart your IDE.
4. Set up Rust toolchain (press "Setup toolchain" in the pop-up "No Rust toolchain configured").
5. You're good to go!


#### Verification of rustup

Check if the `rustup` has been installed.

If you’re using Linux or macOS, open a terminal by going to **View | Tool Windows | Terminal** or pressing **⌥ F12** or **Alt F12** and enter the following command:

```text
$ rustc --version
```
You should see the version number, commit hash, and commit date for the latest stable version that has been released in the following format:
```text

rustc x.y.z (abcabcabc yyyy-mm-dd)
```

If you see this information, you have installed Rust successfully! If you don’t see this information and you’re on Windows, check that Rust is in your `%PATH%` system variable.

#### Linker

Additionally, you’ll need a linker of some kind. It’s likely one is already installed, but when you try to compile a Rust program and get errors indicating that a linker could not execute, that means a linker isn’t installed on your system and you’ll need to install one manually. C compilers usually come with the correct linker. Check your platform’s documentation for how to install a C compiler. Also, some common Rust packages depend on C code and will need a C compiler. Therefore, it might be worth installing one now.

For Windows you’ll also need the C++ build tools for Visual Studio 2013 or later. The easiest way to acquire the build tools is to install [Build Tools for Visual Studio 2017](https://www.visualstudio.com/downloads/#build-tools-for-visual-studio-2017). The tools are in the Other Tools and Frameworks section.

#### Updating and Uninstalling

After you’ve installed Rust via `rustup`, updating to the latest version is easy. From your shell, run the following update script:

```text
$ rustup update
```

To uninstall Rust and 'rustup', run the following uninstall script from your shell:

```text
$ rustup self uninstall
```

#### Local Documentation

The installer also includes a copy of the documentation locally, so you can read it offline. Run `rustup doc` to open the local documentation in your browser.

Any time a type or function is provided by the standard library and you’re not sure what it does or how to use it, use the application programming interface (API) documentation to find out!

_You can refer to the following chapter in the Rust Programming Language book: [Installation](https://doc.rust-lang.org/stable/book/ch01-01-installation.html)_
