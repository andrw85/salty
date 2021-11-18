[![salty-ci](https://github.com/andrw85/salty/actions/workflows/rust.yml/badge.svg)](https://github.com/andrw85/salty/actions/workflows/rust.yml)

# Salty
Salty is another open implementation of a password management system.

security principles for hashing: https://crackstation.net/hashing-security.htm#normalhashing

Salty uses a key stretching algorithm call PBKDF2 with a random generated salt of 63 bytes to reduce vulnerabilities of brute-force attacks. The ouput of this hashing function is also a sha256 key.

## TODOS:

- Refactor PasswordQuery so that it can be implemented specifically for different clients. For example in a cli client we want an specific behaviour for asking and confirming the password different than in a web client. Naming could be something similar to PasswordQueryCli or PasswordQueryWeb...
- Change salt anytime user tries accessing the vault and he introduces a valid master password.

- Continue creating the web client using egui-eframe.

- Increment tests

## Running the help command

```
$ salty
salty 0.1.0
Andrew O'Doherty <andrew.olv@gmail.com>
Salty is an open implementation of a password management system.

USAGE:
    salty <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add-site
    create-vault
    generator       A password generation tool
    help            Prints this message or the help of the given subcommand(s)
    show-entries
```

## How to install salty

```
$ cargo install --path salty_cli/
  Installing salty v0.1.0 (/Users/aodoher/src/salty)
    Updating crates.io index
  Downloaded ab_glyph_rasterizer v0.1.5
  Downloaded libc v0.2.106
  Downloaded 2 crates (565.1 KB) in 0.84s
   Compiling libc v0.2.106
   Compiling version_check v0.9.3
   Compiling cfg-if v1.0.0
   Compiling proc-macro2 v1.0.32
   Compiling unicode-xid v0.2.2
   Compiling syn v1.0.81
   Compiling typenum v1.14.0
   Compiling autocfg v1.0.1
   Compiling log v0.4.14
   Compiling bitflags v1.3.2
...
```
