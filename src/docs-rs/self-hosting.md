# Self hosting a docs.rs instance

These are instructions for deploying the server in a production environment. For instructions on developing locally without docker-compose, see [Developing without docker-compose][no-docker-compose].

<!-- NOTE: This link is outdated, and should probably be migrated to this site. -->
[no-docker-compose]: https://github.com/rust-lang/docs.rs/wiki/Developing-without-docker-compose

Here is a breakdown of what it takes to turn a regular server into its own version of docs.rs.

Beware: This process is rather rough! Attempts at cleaning it up, automating setup components, etc, would be greatly appreciated!

## Requirements

The commands and package names on this page will assume an Ubuntu server running systemd, but hopefully the explanatory text should give enough information to adapt to other systems. Note that docs.rs depends on the host being `x86_64-unknown-linux-gnu`.

Docs.rs has a few basic requirements:

* Rust (preferably via `rustup`)
* Git
* CMake, GCC, G++, and `pkg-config` (to build dependencies for crates and docs.rs itself)
* OpenSSL, zlib, curl, and `libmagic` (to link against)
* PostgreSQL
* LXC tools (doc builds run inside an LXC container)

```console
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
$ source $HOME/.cargo/env
# apt install build-essential git curl cmake gcc g++ pkg-config libmagic-dev libssl-dev zlib1g-dev postgresql lxc-utils
```

## The `cratesfyi` user

To help things out later on, we can create a new unprivileged user that will run the server process. This user will own all the files required by the docs.rs process. This user will need to be able to run `lxc-attach` through `sudo` to be able to run docs builds, so give it a sudoers file at the same time:

```console
# adduser --disabled-login --disabled-password --gecos "" cratesfyi
# echo 'cratesfyi  ALL=(ALL) NOPASSWD: /usr/bin/lxc-attach' > /etc/sudoers.d/cratesfyi
```

(The name `cratesfyi` is a historical one: Before the site was called "docs.rs", it was called "crates.fyi" instead. If you want to update the name of the user, feel free! Just be aware that the name `cratesfyi` will be used throughout this document.)

## The "prefix" directory

In addition to the LXC container, docs.rs also stores several related files in a "prefix" directory. This directory can be stored anywhere, but the `cratesfyi` user needs to be able to access it:

```console
# mkdir /cratesfyi-prefix
# chown cratesfyi:cratesfyi /cratesfyi-prefix
```

Now we can set up some required folders. To make sure they all have proper ownership, run them all as `cratesfyi`:

```console
$ sudo -u cratesfyi mkdir -vp /cratesfyi-prefix/documentations /cratesfyi-prefix/public_html /cratesfyi-prefix/sources
$ sudo -u cratesfyi git clone https://github.com/rust-lang/crates.io-index.git /cratesfyi-prefix/crates.io-index
$ sudo -u cratesfyi git --git-dir=/cratesfyi-prefix/crates.io-index/.git branch crates-index-diff_last-seen
```

(That last command is used to set up the `crates-index-diff` crate, so we can start monitoring new crate releases.)

## LXC container

To help contain what crates' build scripts can access, documentation builds run inside an LXC container. To create one inside the prefix directory:

```console
# LANG=C lxc-create -n cratesfyi-container -P /cratesfyi-prefix -t download -- --dist ubuntu --release bionic --arch amd64
# ln -s /cratesfyi-prefix/cratesfyi-container /var/lib/lxc
# chmod 755 /cratesfyi-prefix/cratesfyi-container
# chmod 755 /var/lib/lxc
```

(To make deployment simpler, it's important that the OS the container is using is the same as the host! In this case, the host is assumed to be running 64-bit Ubuntu 20.04. If you make the container use a different release or distribution, you'll need to build docs.rs separately inside the container when deploying.)

You'll also need to configure networking for the container. The following is a sample `/etc/default/lxc-net` that enables NAT networking for the container:

```console
USE_LXC_BRIDGE="true"
LXC_BRIDGE="lxcbr0"
LXC_ADDR="10.0.3.1"
LXC_NETMASK="255.255.255.0"
LXC_NETWORK="10.0.3.0/24"
LXC_DHCP_RANGE="10.0.3.2,10.0.3.254"
LXC_DHCP_MAX="253"
LXC_DHCP_CONFILE=""
LXC_DOMAIN=""
```

In addition, you'll need to set the container's configuration to use this. Add the following lines to `/cratesfyi-prefix/cratesfyi-container/config`:

```console
lxc.net.0.type = veth
lxc.net.0.link = lxcbr0
```

Now you can reload the LXC network configuration, start up the container, and set it up to auto-start when the host boots:

```console
# systemctl restart lxc-net
# systemctl enable lxc@cratesfyi-container.service
# systemctl start lxc@cratesfyi-container.service
```

Now we need to do some setup *inside* this container. You can either copy all these commands so that each one attaches on its own, or you can run `lxc-console -n cratesfyi-container` to open a root shell inside the container and skip the `lxc-attach` prefix.

```console
# lxc-attach -n cratesfyi-container -- apt update
# lxc-attach -n cratesfyi-container -- apt upgrade
# lxc-attach -n cratesfyi-container -- apt install curl ca-certificates binutils gcc libc6-dev libmagic1 pkg-config build-essential
```

Inside the container, we also need to set up a `cratesfyi` user, and install Rust for it. In addition to the base Rust installation, we also need to install all the default targets so that we can build docs for all the Tier 1 platforms. The Rust compiler installed inside the container is the one that builds all the docs, so if you want to use a new Rustdoc feature, this is the compiler to update.

```console
lxc-attach -n cratesfyi-container -- adduser --disabled-login --disabled-password --gecos "" cratesfyi
lxc-attach -n cratesfyi-container -- su - cratesfyi -c 'curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly'
lxc-attach -n cratesfyi-container -- su - cratesfyi -c 'rustup target add i686-apple-darwin'
lxc-attach -n cratesfyi-container -- su - cratesfyi -c 'rustup target add i686-pc-windows-msvc'
lxc-attach -n cratesfyi-container -- su - cratesfyi -c 'rustup target add i686-unknown-linux-gnu'
lxc-attach -n cratesfyi-container -- su - cratesfyi -c 'rustup target add x86_64-apple-darwin'
lxc-attach -n cratesfyi-container -- su - cratesfyi -c 'rustup target add x86_64-pc-windows-msvc'
```

Now that we have Rust installed inside the container, we can use a trick to give the `cratesfyi` user on the *host* the same Rust compiler as the *container*. By symlinking the following directories into its user directory, we don't need to track a *third* toolchain.

```console
for directory in .cargo .rustup .multirust; do  [[ -h /home/cratesfyi/$directory ]] || sudo -u cratesfyi ln -vs /var/lib/lxc/cratesfyi-container/rootfs/home/cratesfyi/$directory /home/cratesfyi/; done
```

## Environment for the `cratesfyi` user

To ensure that the docs.rs server is configured properly, we need to set a few environment variables. The primary ones are going into a separate environment file, so we can load them into the systemd service that will manage the server.

Write the following into `/home/cratesfyi/.cratesfyi.env`. If you have a GitHub access token that the site can use to collect repository information, add it here, but otherwise leave it blank. The variables need to exist, but they can be blank to skip that collection.

```console
CRATESFYI_PREFIX=/cratesfyi-prefix
CRATESFYI_DATABASE_URL=postgresql://cratesfyi:password@localhost
CRATESFYI_CONTAINER_NAME=cratesfyi-container
CRATESFYI_GITHUB_USERNAME=
CRATESFYI_GITHUB_ACCESSTOKEN=
RUST_LOG=cratesfyi
```

Now add the following to `/home/cratesfyi/.profile`:

```sh
export $(cat $HOME/.cratesfyi.env | xargs -d '\n')
export PATH="$HOME/.cargo/bin:$PATH"
export PATH="$PATH:$HOME/docs.rs/target/release"
```

## Docs.rs build

Now we can actually clone and build the docs.rs source! The location of it doesn't matter much, but again, we want it to be owned by `cratesfyi` so it can build and run the final executable. In addition, we copy the built `cratesfyi` binary into the container so that it can be used to arrange builds on the inside.

```console
sudo -u cratesfyi git clone https://github.com/rust-lang-nursery/docs.rs.git ~cratesfyi/docs.rs
sudo su - cratesfyi -c 'cd ~/docs.rs && cargo build --release'
cp -v /home/cratesfyi/docs.rs/target/release/cratesfyi /var/lib/lxc/cratesfyi-container/rootfs/usr/local/bin
```

## PostgreSQL

Now that we have the repository built, we can use it to set up the database. Docs.rs uses a Postgres database to store information about crates and their documentation. To set one up, we first need to ask Postgres to create the database, and then run the docs.rs command to create the initial tables and content:

```console
sudo -u postgres sh -c "psql -c \"CREATE USER cratesfyi WITH PASSWORD 'password';\""
sudo -u postgres sh -c "psql -c \"CREATE DATABASE cratesfyi OWNER cratesfyi;\""
sudo su - cratesfyi -c "cd ~/docs.rs && cargo run --release -- database init"
sudo su - cratesfyi -c "cd ~/docs.rs && cargo run --release -- build add-essential-files"
sudo su - cratesfyi -c "cd ~/docs.rs && cargo run --release -- build crate rand 0.5.5"
sudo su - cratesfyi -c "cd ~/docs.rs && cargo run --release -- database update-search-index"
sudo su - cratesfyi -c "cd ~/docs.rs && cargo run --release -- database update-release-activity"
```

## Server configuration

We're almost there! At this point, we've got all the pieces in place to run the site. Now we can set up a systemd service that will run the daemon that will collect crate information, orchestrate builds, and serve the website. The following systemd service file can be placed in `/etc/systemd/system/cratesfyi.service`:

```systemd
[Unit]
Description=Cratesfyi daemon
After=network.target postgresql.service

[Service]
User=cratesfyi
Group=cratesfyi
Type=forking
PIDFile=/cratesfyi-prefix/cratesfyi.pid
EnvironmentFile=/home/cratesfyi/.cratesfyi.env
ExecStart=/home/cratesfyi/docs.rs/target/release/cratesfyi daemon
WorkingDirectory=/home/cratesfyi/docs.rs

[Install]
WantedBy=multi-user.target
```

Enabling and running that will serve the website on `http://localhost:3000`, so if you want to route public traffic to it, you'll need to set up something like nginx to proxy the connections to it.

## Updating Rust

If you want to update the Rust compiler used to build crates (and the Rustdoc that comes with it), you need to make sure you don't interrupt any existing crate builds. The daemon waits for 60 seconds between checking for new crates, so you need to make sure you catch it during that window. Since we hooked the daemon into systemd, the logs will be available in its journal. Running `journalctl -efu cratesfyi` (it may need to be run as root if nothing appears) will show the latest log output and show new entries as they appear. You're looking for a message like "Finished building new crates, going back to sleep" or "Queue is empty, going back to sleep", which indicates that the crate-building thread is waiting.

To prevent the queue from building more crates, run the following:

```console
sudo su - cratesfyi -c "cd ~/docs.rs && cargo run --release -- build lock"
```

This will create a lock file in the prefix directory that will prevent more crates from being built. At this point, you can update the rustc inside the container and add the rustdoc static files to the database:

```console
lxc-attach -n cratesfyi-container -- su - cratesfyi -c 'rustup update'
sudo su - cratesfyi -c "cd ~/docs.rs && cargo run --release -- build add-essential-files"
```

Once this is done, you can unlock the queue to allow crates to build again:

```console
sudo su - cratesfyi -c "cd ~/docs.rs && cargo run --release -- build unlock"
```

And we're done! New crates will start being built with the new rustc. If you want to rebuild any existing docs with the new rustdoc, you need to manually build them - there's no automated way to rebuild failed docs or docs from a certain rust version yet.

## Updating docs.rs

To update the code for docs.rs itself, you can follow a similar approach. First, watch the logs so you can stop the daemon from building more crates. (You can replace the lock command with a `systemctl stop cratesfyi` if you don't mind the web server being down while you build.)

```console
# journalctl -efu cratesfyi
(wait for build daemon to sleep)
$ sudo su - cratesfyi -c "cd ~/docs.rs && cargo run --release -- build lock"
```

Once the daemon has stopped, you can start updating the code and rebuilding:

```console
$ sudo su - cratesfyi -c "cd ~/docs.rs && git pull"
$ sudo su - cratesfyi -c "cd ~/docs.rs && cargo build --release"
```

Now that we have a shiny new build, we need to make sure the service is using it:

```console
# cp -v /home/cratesfyi/docs.rs/target/release/cratesfyi /var/lib/lxc/cratesfyi-container/rootfs/usr/local/bin
# systemctl restart cratesfyi
```

Next, we can unlock the builder so it can start checking new crates:

```console
$ sudo su - cratesfyi -c "cd ~/docs.rs && cargo run --release -- build unlock"
```

And we're done! Changes to the site or the build behavior should be visible now.
