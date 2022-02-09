[![salty-ci](https://github.com/andrw85/salty/actions/workflows/rust.yml/badge.svg)](https://github.com/andrw85/salty/actions/workflows/rust.yml)

# Salty
Salty is another open implementation of a password management system.

The connection between the client and the vault server is encrypted using an TLS channel, this avoids the risk of a man in the middle attack.
The Client side produces a hash salted with a site-specific string and it is then sent on an encrypted channel to the server. On the server it is one more time hashed and salted with a random generated salt, stored on the server. Salty uses a key stretching algorithm call PBKDF2 with a random generated salt of 63 bytes to reduce vulnerabilities of brute-force attacks. The ouput of this hashing function is also a sha256 key.

![Class Diagram](http://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/andrw85/salty/main/uml/architecture.puml)

Security principles for hashing: https://crackstation.net/hashing-security.htm#normalhashing


## Example of intended usage on CLI client(not implemented yet):
```
$ salty login andrew
Insert master password: *******************
Error: Account or password invalid!

$ salty create andrew
Insert master password: ********************
verify master password: ********************
Account created, you are now logged in!

$ salty addentry
Insert title for the entry: www.gmail.com
Insert User name: andrew@gmail.com
Insert password: ********
Entry inserted!

$ date
Wed 09 Feb 2022 07:29:47 PM CET

$ date # after 1 min
Wed 09 Feb 2022 07:30:47 PM CET

$ salty show
You are not logged in!

$ salty addentry
You are not logged in!

$ salty login andrew
Insert master password: ********************
Logged in!

$ salty addentry
Insert title for the entry: www.mail.de
Insert User name: andrew@mail.de
Insert password: ********
Entry inserted!

$ salty show 
Displaying contents in the vault (passwords hidden by default).
{
	{
		"title": "www.gmail.com"
		"user": "andrew@gmail.com"
		"password": "*********"
	},
	{
		"title": "www.mail.de"
		"user": "andrew@mail.de"
		"password": "**********"
	},
}

$ salty show --all
Displaying contents in the vault including passwords.
{
	{
		"title": "www.gmail.com"
		"user": "andrew@gmail.com"
		"password": "12345678"
	},
	{
		"title": "www.mail.de"
		"user": "andrew@mail.de"
		"password": "myCoolPassword"
	}
}


```

## Dependencies

To build locally without development environment you need:

- cargo package installed

Optional dependencies for development environment:

- GNU make
- Docker

## TODOS:

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

## How to start the development environment

There is a Makefile provided that can be used to build a docker container with the development environment.

To build and run it execute the following command:
```
$ make
```
The previous command will give you the following output:
```
docker build --build-arg USER_ID=1234 --build-arg GROUP_ID=235 -t salty  - < Dockerfile
Sending build context to Docker daemon  2.048kB
Step 1/9 : FROM rust:latest
 ---> 269d9943b0d3
Step 2/9 : ARG USER_ID
 ---> Using cache
 ---> d4006db69878
Step 3/9 : ARG GROUP_ID
 ---> Using cache
 ---> 0ea9ff6a717b
Step 4/9 : RUN echo "root:root" | chpasswd
 ---> Using cache
 ---> 47e27a74b4a9
Step 5/9 : RUN groupadd -g $GROUP_ID salty
 ---> Using cache
 ---> 2b173556ab8c
Step 6/9 : RUN useradd -m -r -u $USER_ID -g $GROUP_ID salty
 ---> Using cache
 ---> b9f0e827bafa
Step 7/9 : WORKDIR /home/salty
 ---> Using cache
 ---> dd4996830ecb
Step 8/9 : USER salty
 ---> Using cache
 ---> 18818121e326
Step 9/9 : CMD ["/bin/bash"]
 ---> Using cache
 ---> e668f91c6358
Successfully built e668f91c6358
Successfully tagged salty:latest
docker run -v /home/andrew/rust/salty:/home/salty -it salty:latest
salty@2e9dab4c671e:~$
```

If you have already built the docker image previously and you want to skip the docker image building phase and access the docker container CLI run:

```
 $ make run
```
You will end up with a prompt within the docker container. You can use it to build the project by running:

```
salty@4856ba2b491e:~$ cargo build
```
## How to install salty

```
$ cargo install --path salty_cli/
  Installing salty v0.1.0 (/Users/andrew/src/salty)
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
