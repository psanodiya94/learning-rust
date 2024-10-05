# Rust Installation Guide

## Installation
The first step is to install Rust. We'll download Rust through rustup, a command line tool for managing Rust versions and associated tools. You'll need an internet connection for the download.

> Note: If you prefer not to use rustup for some reason, please see the Other Rust Installation Methods page for more options.

The following steps install the latest stable version of the Rust compiler. Rust's stability guarantees ensure that all the examples in the book that compile will continue to compile with newer Rust versions. The output might differ slightly between versions because Rust often improves error messages and warnings. In other words, any newer, stable version of Rust you install using these steps should work as expected with the content of this book.

## Command Line Notation
In this chapter and throughout the book, we'll show some commands used in the terminal. Lines that you should enter in a terminal all start with `$`. You don't need to type the `$` character; it's the command line prompt shown to indicate the start of each command. Lines that don't start with `$` typically show the output of the previous command. 

Additionally, PowerShell-specific examples will use `>` rather than `$`.

## Installing rustup on Linux or macOS
If you're using Linux or macOS, open a terminal and enter the following command:

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

The command downloads a script and starts the installation of the rustup tool, which installs the latest stable version of Rust. You might be prompted for your password. If the install is successful, the following line will appear:

```bash
$ Rust is installed now. Great!
```

On macOS, you can get a C compiler by running:

```bash
$ xcode-select --install
```

Linux users should generally install GCC or Clang, according to their distribution’s documentation. For example, if you use Ubuntu, you can install the build-essential package.

## Installing rustup on Windows
On Windows, go to https://www.rust-lang.org/tools/install and follow the instructions for installing Rust. At some point in the installation, you’ll be prompted to install Visual Studio. This provides a linker and the native libraries needed to compile programs. If you need more help with this step, see https://rust-lang.github.io/rustup/installation/windows-msvc.html

The rest of this book uses commands that work in both cmd.exe and PowerShell. If there are specific differences, we’ll explain which to use.

## Troubleshooting
To check whether you have Rust installed correctly, open a shell and enter this line:

```bash
$ rustc --version
```

You should see the version number, commit hash, and commit date for the latest stable version that has been released, in the following format:

```bash
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

If you see this information, you have installed Rust successfully! If you don’t see this information, check that Rust is in your %PATH% system variable as follows.

In Windows CMD, use:

```bash
> echo %PATH%
```

In PowerShell, use:

```bash
> echo $env:Path
```

In Linux and macOS, use:

```bash
$ echo $PATH
```

If that’s all correct and Rust still isn’t working, there are a number of places you can get help. Find out how to get in touch with other Rustaceans (a silly nickname we call ourselves) on the community page.

## Updating and Uninstalling
Once Rust is installed via rustup, updating to a newly released version is easy. From your shell, run the following update script:

```bash
$ rustup update
```

To uninstall Rust and rustup, run the following uninstall script from your shell:

```bash
$ rustup self uninstall
```

## Local Documentation
The installation of Rust also includes a local copy of the documentation so that you can read it offline. Run rustup doc to open the local documentation in your browser.

Any time a type or function is provided by the standard library and you’re not sure what it does or how to use it, use the application programming interface (API) documentation to find out!

