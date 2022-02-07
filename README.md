[![salty-ci](https://github.com/andrw85/salty/actions/workflows/rust.yml/badge.svg)](https://github.com/andrw85/salty/actions/workflows/rust.yml)

# Salty
Salty is another open implementation of a password management system.

security principles for hashing: https://crackstation.net/hashing-security.htm#normalhashing

Salty uses a key stretching algorithm call PBKDF2 with a random generated salt of 63 bytes to reduce vulnerabilities of brute-force attacks. The ouput of this hashing function is also a sha256 key.

## TODOS:

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

## Creating a vault

```
$ salty create-vault
Insert Vault master password:

```

## Adding an entry to the vault
```
$ salty add-site
error: The following required arguments were not provided:
    --site <site>
    --user <user>

USAGE:
    salty add-site --hasher-salt <hasher-salt> --site <site> --user <user>

For more information try --help
```

```
$ salty add-site -u andrew -s test.com
Insert Vault master password:

Insert site password:

```

## Showing entries stored in encrypted vault

```
$ salty show-entries
Insert Vault master password:

Account {
    sites: {
        AccountEntry {
            site_name: "test.com",
            user_name: "andrew",
            pwd: "123as-123=as2",
        },
    },
}
```
