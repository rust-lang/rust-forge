These are instructions for developing docs.rs locally. For deploying in a production environment, see https://github.com/rust-lang/docs.rs/wiki/Self-hosting-outside-the-Vagrant-VM.

While the docker-compose allows for easier setup of the required dependencies and environment, here is a breakdown of how to use the service without an outer docker container. This is useful e.g. for quickly iterating during development.

Note that this does not remove the docker dependency altogether, since docs.rs uses docker at runtime for sandboxing. This just allows you to run commands more quickly since you are building in debug mode instead of release and are also caching more of the build.

## Requirements

The commands and package names on this page will assume an Ubuntu server running systemd, but hopefully the explanatory text should give enough information to adapt to other systems.

Docs.rs has a few basic requirements:

* Rust (preferably via `rustup`)
* Git
* CMake, GCC, G++, and `pkg-config` (to build dependencies for crates and docs.rs itself)
* OpenSSL, zlib, curl, and `libmagic` (to link against)
* PostgreSQL

```console
$ curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
$ . $HOME/.cargo/env
# apt install build-essential git curl cmake gcc g++ pkg-config libmagic-dev libssl-dev zlib1g-dev postgresql
$ sudo -u postgres psql -c "CREATE USER cratesfyi WITH PASSWORD 'password';"
$ sudo -u postgres psql -c "CREATE DATABASE cratesfyi OWNER cratesfyi;"
```

## Building the site

Be warned - this builds over 350 crates! Depending on your computer, this may
take upwards of 10 minutes.

```console
$ git clone https://github.com/rust-lang/docs.rs && cd docs.rs
$ cargo build
```

## The "prefix" directory

docs.rs stores several files in a "prefix" directory. This can be anywhere, but if you put it in the doc.rs repo, it should go under the ./ignored/ directory so that it is not seen by git or the docker daemon.

```console
$ mkdir -p ignored/cratesfyi-prefix
$ cd ignored/cratesfyi-prefix
$ mkdir -vp documentations public_html sources
$ git clone https://github.com/rust-lang/crates.io-index && cd crates.io-index
$ git branch crates-index-diff_last-seen
```

(That last command is used to set up the `crates-index-diff` crate, so we can start monitoring new crate releases.)

## Docker group

docs.rs needs to run docker containers for sandboxing. Therefore, you need to be in the 'docker' group to build crates. If you are already in the docker group, you can skip this step (you can check with `groups`).

```console
# usermod -a -G docker "$USER"
$ # now logout and back in to your shell
$ cd /path/to/docs.rs/ignored/cratesfyi-prefix
```

## Environment for the server

To ensure that the docs.rs server is configured properly, we need to set a few environment variables. This is most easily done by making a shell script.

```sh
$ cd ..
$ echo '
export CRATESFYI_PREFIX=.
# or add an appropriate username/password as necessary
export CRATESFYI_DATABASE_URL=postgresql://cratesfyi:password@localhost
export CRATESFYI_GITHUB_USERNAME=
export CRATESFYI_GITHUB_ACCESSTOKEN=
export RUST_LOG=cratesfyi,rustwide=info
' > env.sh
```

This last command assumes you put the shell script in `./env.sh`,
but you can name it anything as long as it is in the current directory.

```console
$ . ./env.sh
$ cargo run database migrate
$ cargo run database update-search-index
$ cargo run database update-release-activity
# This will take between 5 and 30 minutes on the first run, depending on your internet speed.
# It downloads the rustops/crates-build-env crates which is over 4 GB.
# It does not currently display a progress bar, this is https://github.com/rust-lang/rustwide/issues/9
# As a workaround, you can run `docker pull rustops/crates-build-env` in a separate terminal.
$ cargo run build crate rand 0.5.5
```

## Building on platforms other than Linux

This is not currently possible. We assume in several places that the rustup toolchain is x86_64-unknown-linux-gnu. As a result, the only way to build on Mac or Alpine is to use the docker-compose file.

## Resetting the database

Occasionally, if you make changes to the migrations in a PR, those migrations will be saved in the database but will not be in the code when you switch back to the master branch. In this case, there is no way to undo the migration without knowing exactly which version of the code made the change (`cargo migrate` will have no effect). Here is a convenient shell script to reset the database so you don't have to remember how to undo your changes.

NOTE: DO NOT RUN THIS IN PRODUCTION.

```sh
#!/bin/sh
set -euv

. ./env.sh

sudo -u postgres dropdb cratesfyi --if-exists
sudo -u postgres psql -c 'CREATE DATABASE cratesfyi OWNER cratesfyi;'
cargo run database migrate
```