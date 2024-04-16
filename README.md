# otp
Rust implementation of the One-Time-Pad encryption algorithm

I am basing the design of this program on the `minigrep` example in the [Rust Book](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html) and [this](state_diagram.mmd) state diagram I made using [Mermaid](https://mermaid.live/). 

`otp` works with ASCII characters and the first release is out! I'm planning to update it with support for UTF-8 characters when I figure out how to encrypt them with OTP. 

Usage: 
```
otp [args] <plaintext | ciphertext key>

    Where args include: 
        -h, --help      Display this message
        -v, --version   Display version information
        -e, --encrypt   [plaintext]\tEncrypt some ASCII plaintext
        -d, --decrypt   [ciphertext] [key]\tDecrypt some ASCII ciphertext with a key
```

# Installation
## Binary Installation
If you don't know what this means, then it is probably the one you want.

To install `otp`, please see the latest [Release](https://github.com/kcajeel/otp/releases) and download the attached archive labeled with your operating system and architecture.

### Windows
Download the `windows-[your architecture].zip` from the releases page and unzip the archive. 

Then, you want to add `otp.exe` to your PATH environment variable to be able to run it outside of the directory it's stored in.
That's it! To update, replace the old `otp.exe` with the new one and it should work.

### Linux / MacOS
Download the `linux-[your architecture].tar.gz` from the releases page and unzip the archive with tar:
- Note: This will extract into your current working directory. If you want to extract the archive into its own directory, it's recommended that you create that directory and move the archive file there before extracting it.
```sh
$ tar -xvf linux-[your architecture].tar.gz
```
Then, you will want to either symlink or move the `otp` file to your `/usr/local/bin`. This will add the program to your system's PATH and you'll be able to run `otp`.

---

## Building from Source
If you're based and want to build `otp` yourself, you've come to the right place. Because this program was written with Rust, the build instructions are the same for all operating systems. 

### Installing Cargo
You need to have `cargo` installed to build this program. If you already have `cargo`, skip to the next section. To install `cargo`, visit [this website](https://www.rust-lang.org/tools/install) to install Rust and `cargo`. 

### Building
Now that `cargo` is installed, you just need to download and extract the source code included with the most recent [release](https://github.com/kcajeel/otp/releases) and run 
```sh
$ cargo build --release
```
in the directory where the source code is stored. When compilation is finished, you should follow the [steps above](#binary-installation) for installing the binary file to your system's PATH. 

---

## Contributing / Bug Fixes
If you've found a bug in the program, please submit an [Issue](https://github.com/kcajeel/otp/issues) and I'll try to look at it and resolve it soon. 

If you have a feature you'd like to add or if you would like to fix a bug, please make a fork of this repository and add your changes to your local fork. Then, open a Pull Request to submit your changes for me to look over to add to the main repo. 