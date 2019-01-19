# Sardinburk [![Donate](https://img.shields.io/badge/Donate-PayPal-blue.svg?style=flat-square)](https://paypal.me/mnpn03/)

Sardinburk is a direct chat application between several devices. Sardinburk is made in [Rust](https://www.rust-lang.org/).
Sardinburk allows for users to have a nickname and communicate both locally and world-wide.
See the usage section below.

Disclaimer: Sardinburk is not meant to be reliable. It's meant to be simple!
If you're looking for any kind of security, reliability, or really anything, I'll link a better program here when it's available!

### Table of Content
- [Installation](#installation)
- [Usage](#usage)
- [Contribution](#contribution)
- [License](#license)

### Installation
If you don't want to compile the thing yourself, download the latest release [here](https://github.com/Mnpn03/Sardinburk/releases).

If you instead want to compile Sardinburk, you can do so by getting [Rust](https://www.rust-lang.org/).
Once that is installed, clone the repository:
`git clone git@github.com:Mnpn03/Sardinburk.git` to clone with SSH, or
`git clone https://github.com/Mnpn03/Sardinburk.git` to clone with HTTPS.
Then you simply build it by running `cargo build --release`.

### Usage
```
$ sardinburk --help
=> sardinburk...
$ sardinburk 127.0.0.1 --name John
=> Hello world, John! You're the user with ID 2...
$ sardinburk --name Martin
=> Hello world, Martin! Others can join you by providing...
```
Supply an IP to connect to it, otherwise you're the host!
### Contribution
To contribute to the project, simply create a [Pull Request](https://github.com/Mnpn03/Sardinburk/pulls) or an [Issue](https://github.com/Mnpn03/Sardinburk/issues).

If you want to create an [Issue](https://github.com/Mnpn03/Sardinburk/issues), please clearly state the bug and/or ways to replicate it (if it's a bug/glitch/exploit).

If you want to create a [Pull Request](https://github.com/Mnpn03/Sardinburk/pulls), please clearly state what you've changed and if it has resolved an issue, and if so - which one.

Following these short guidelines will make it easier and faster for your Issue/Pull Request to be reviewed and dealt with.
Thanks!

### License
Sardinburk is FOSS that comes with no warranty. Read more about the license used [here](https://github.com/Mnpn03/Sardinburk/blob/master/LICENSE).
