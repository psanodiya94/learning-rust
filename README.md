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

## Hello, World!
Now that you’ve installed Rust, it’s time to write your first Rust program. It’s traditional when learning a new language to write a little program that prints the text Hello, world! to the screen, so we’ll do the same here!

>Note: This book assumes basic familiarity with the command line. Rust makes no specific demands about your editing or tooling or where your code lives, so if you prefer to use an integrated development environment (IDE) instead of the command line, feel free to use your favorite IDE. Many IDEs now have some degree of Rust support; check the IDE’s documentation for details. The Rust team has been focusing on enabling great IDE support via rust-analyzer. See Appendix D for more details.

## Creating a Project Directory
You’ll start by making a directory to store your Rust code. It doesn’t matter to Rust where your code lives, but for the exercises and projects in this book, we suggest making a projects directory in your home directory and keeping all your projects there.

Open a terminal and enter the following commands to make a projects directory and a directory for the “Hello, world!” project within the projects directory.

For Linux, macOS, and PowerShell on Windows, enter this:

```bash
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

For Windows CMD, enter this:

```bash
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

## Writing and Running a Rust Program
Next, make a new source file and call it main.rs. Rust files always end with the .rs extension. If you’re using more than one word in your filename, the convention is to use an underscore to separate them. For example, use hello_world.rs rather than helloworld.rs.

Now open the main.rs file you just created and enter the code in Listing 1-1.

Filename: main.rs
```bash
fn main() {
    println!("Hello, world!");
}
```
<small>**Listing 1-1: A program that prints Hello, world!**</small>

Save the file and go back to your terminal window in the ~/projects/hello_world directory. On Linux or macOS, enter the following commands to compile and run the file:

```bash
$ rustc main.rs
$ ./main
Hello, world!
```

On Windows, enter the command .\main.exe instead of ./main:

```bash
> rustc main.rs
> .\main.exe
Hello, world!
```

Regardless of your operating system, the string Hello, world! should print to the terminal. If you don’t see this output, refer back to the “Troubleshooting” part of the Installation section for ways to get help.

If `Hello, world!` did print, congratulations! You’ve officially written a Rust program. That makes you a Rust programmer—welcome!

## Hello, Cargo!
Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs dependencies.)

The simplest Rust programs, like the one we’ve written so far, don’t have any dependencies. If we had built the “Hello, world!” project with Cargo, it would only use the part of Cargo that handles building your code. As you write more complex Rust programs, you’ll add dependencies, and if you start a project using Cargo, adding dependencies will be much easier to do.

Because the vast majority of Rust projects use Cargo, the rest of this book assumes that you’re using Cargo too. Cargo comes installed with Rust if you used the official installers discussed in the “Installation” section. If you installed Rust through some other means, check whether Cargo is installed by entering the following in your terminal:

```bash
$ cargo --version
```

If you see a version number, you have it! If you see an error, such as command not found, look at the documentation for your method of installation to determine how to install Cargo separately.

## Creating a Project with Cargo
Let’s create a new project using Cargo and look at how it differs from our original “Hello, world!” project. Navigate back to your projects directory (or wherever you decided to store your code). Then, on any operating system, run the following:

```bash
$ cargo new hello_cargo
$ cd hello_cargo
```

The first command creates a new directory and project called hello_cargo. We’ve named our project hello_cargo, and Cargo creates its files in a directory of the same name.

Go into the hello_cargo directory and list the files. You’ll see that Cargo has generated two files and one directory for us: a Cargo.toml file and a src directory with a main.rs file inside.

It has also initialized a new Git repository along with a .gitignore file. Git files won’t be generated if you run cargo new within an existing Git repository; you can override this behavior by using cargo new --vcs=git.

>Note: Git is a common version control system. You can change cargo new to use a different version control system or no version control system by using the --vcs flag. Run cargo new --help to see the available options.

Open Cargo.toml in your text editor of choice. It should look similar to the code in Listing 1-2.

```bash
Filename: Cargo.toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
<small>**Listing 1-2: Contents of Cargo.toml generated by cargo new**</small>

This file is in the TOML (Tom’s Obvious, Minimal Language) format, which is Cargo’s configuration format.

The first line, `[package]`, is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections.

The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and the edition of Rust to use. We’ll talk about the edition key in Appendix E.

The last line, `[dependencies]`, is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates. We won’t need any other crates for this project, but we will in the first project in Chapter 2, so we’ll use this dependencies section then.

Now open src/main.rs and take a look:

Filename: src/main.rs

```bash
fn main() {
    println!("Hello, world!");
}
```

Cargo has generated a `“Hello, world!”` program for you, just like the one we wrote in Listing 1-1! So far, the differences between our project and the project Cargo generated are that Cargo placed the code in the src directory and we have a Cargo.toml configuration file in the top directory.

Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. There’s a place for everything, and everything is in its place.

If you started a project that doesn’t use Cargo, as we did with the “Hello, world!” project, you can convert it to a project that does use Cargo. Move the project code into the src directory and create an appropriate Cargo.toml file.

## Building and Running a Cargo Project
Now let’s look at what’s different when we build and run the “Hello, world!” program with Cargo! From your hello_cargo directory, build your project by entering the following command:

```bash
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

This command creates an executable file in target/debug/hello_cargo (or target\debug\hello_cargo.exe on Windows) rather than in your current directory. Because the default build is a debug build, Cargo puts the binary in a directory named debug. You can run the executable with this command:

```bash
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

If all goes well, Hello, world! should print to the terminal. Running cargo build for the first time also causes Cargo to create a new file at the top level: Cargo.lock. This file keeps track of the exact versions of dependencies in your project. This project doesn’t have dependencies, so the file is a bit sparse. You won’t ever need to change this file manually; Cargo manages its contents for you.

We just built a project with cargo build and ran it with ./target/debug/hello_cargo, but we can also use cargo run to compile the code and then run the resultant executable all in one command:

```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Using cargo run is more convenient than having to remember to run cargo build and then use the whole path to the binary, so most developers use cargo run.

Notice that this time we didn’t see output indicating that Cargo was compiling hello_cargo. Cargo figured out that the files hadn’t changed, so it didn’t rebuild but just ran the binary. If you had modified your source code, Cargo would have rebuilt the project before running it, and you would have seen this output:

```bash
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo also provides a command called cargo check. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:

```bash
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```
