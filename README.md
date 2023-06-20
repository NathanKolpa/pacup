# pacup

Pacup is a free and open-source program written in Rust that facilitates the synchronization of Pacman packages across
multiple devices. With pacup, you can effortlessly manage and replicate your preferred software packages on different
systems by utilizing a simple plain text file.

## Features

* **Fast and Lightweight** Built with Rust, pacup is designed to be as efficient as possible.
* **Extensible** Pacup follows the Unix philosophy and allows for extended functionality by using simple text files and
  programmable standard I/O
* **Easy to Use** Pacup offers a very simple command-line-interface (only 1 command!)

## Usage

PacUp utilizes a plain text file to manage the package list. The file follows a straightforward format where each line
represents a package to be synchronized. To get started, follow the steps below:

### Create a `packagelist` file

Start by creating a plain text file named packagelist that will serve as your package list. You can choose one of the
following locations to store the packagelist file:

* $XDG_CONFIG_HOME/pacup/packagelist
* $HOME/.packagelist
* /etc/pacup/packagelist

For example, you can create the packagelist file in the chosen location and add the following lines:

```text
# Basic utils
+ neovim
+ zsh

* ungoogled-chromium-xdg-bin # An AUR package
```

In this example, we have specified three packages: neovim and zsh as normal packages (prefixed with `+`), and
ungoogled-chromium-xdg-bin as an AUR package (prefixed with `*`). You can add more packages following the same format,
with each package on a new line.

### Run `pacup`

To synchronize the packagelist with your system packages, simply run the pacup command in your terminal or command
prompt:

```
$ pacup
Installing 1 missing package(s)
/usr/bin/sudo /usr/bin/pacman -Sy neovim zsh
[sudo] enter password for anon: 
```

## Installation

To install PacUp, you have two options: installing the PacUp package from the AUR or building from source.

### Option 1: Install from the AUR

Install the AUR package with your preferred AUR helper, in this case trizen:

```bash
trizen -S pacup
```

### Option 2: Build from source

If you prefer to build pacup from source, follow these steps:

1. Ensure that you have Rust and Cargo installed on your system. If you haven't installed Rust and Cargo yet, you can
   follow the official installation guide at https://www.rust-lang.org/tools/install.

2. Clone the pacup repository by executing the following command:

```bash
git clone git@github.com:NathanKolpa/pacup.git
```

3. Navigate to the pacup directory:

```bash
cd pacup
```

4. Build the project using Cargo:

```bash
cargo build --release
```

5. After the build process is complete, you will find the pacup binary in the target/release directory. You can (for example) place the binary in `/usr/bin` like so:
```bash
sudo install target/release/pacup /usr/bin
```