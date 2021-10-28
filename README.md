[![salty-ci](https://github.com/andrw85/salty/actions/workflows/rust.yml/badge.svg)](https://github.com/andrw85/salty/actions/workflows/rust.yml)

# Salty
Salty is another open implementation of a password management system.

## TODOS:

- Create a web client using wasm.

- Investigate whether secure syncing between different computers would be possible using some kind
  of authentication mechanism (maybe using some p2p protocol).
  
- Increment tests

## Running the help command

```
$ ./salty 
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
$ target/debug/salty create-vault
Insert Vault master password: 

```

## Adding an entry to the vault
```
$ target/debug/salty add-site 
error: The following required arguments were not provided:
    --site <site>
    --user <user>

USAGE:
    salty add-site --hasher-salt <hasher-salt> --site <site> --user <user>

For more information try --help
```

```
$ target/debug/salty add-site -u andrew -s test.com
Insert Vault master password: 

Insert site password: 

```

## Showing entries stored in encrypted vault

```
$ target/debug/salty show-entries
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
