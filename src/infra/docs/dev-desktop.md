# Dev Desktops

The dev desktops provide maintainers and contributors to the Rust Project with
free access to high-powered cloud compute. They are part of the
[Cloud Compute Program] by the [Rust Foundation].

| Machine            | Architecture | Perf enabled | Location       |
|--------------------|--------------|--------------|----------------|
| `dev-desktop-eu-1` | `aarch64`    | Yes          | Germany        |
| `dev-desktop-eu-2` | `amd64`      | No           | Netherlands    |
| `dev-desktop-us-1` | `aarch64`    | Yes          | N. Virgina, US |
| `dev-desktop-us-2` | `amd64`      | No           | Washington, US |

## How to apply to the program

At this time, access to the program and the compute instances is limited to
maintainers and core contributors of the Rust Project. While the program is
under development, it is limited to [certain teams]. If you are in one of these
teams, you should automatically have access.

If you feel like your work on the Rust project would be significantly improved
by access to a powerful build machine, reach out to [infra@rust-lang.org] with
the following information:

- Your GitHub handle
- A short description of how you would use and benefit from the dev desktops

## How to connect to a dev desktop

Each user has their own account on the dev desktops. The account is named after
the user’s GitHub handle, with `gh-` as a prefix. For example, a user with the
GitHub handle `user` will have a user account with the name `gh-user` on the dev
desktop.

Users can connect to the dev desktop with SSH. The dev desktops use public key
authentication, and automatically fetch the user’s public keys from GitHub.

You can connect to the instance with the following command:

```shell
ssh <your-username>@<name>.infra.rust-lang.org
```

Replace `<name>` with the machine name from the table at the top of the page.
For example, connect to `dev-desktop-eu-1` using the hostname
`dev-desktop-eu-1.infra.rust-lang.org`.

If you don’t have a public key on GitHub, read the following guides that explain
how to create an SSH key and add it to your GitHub account. It might take a few
minutes after the key has been added before the dev desktops get updated.

- [Generating a new SSH key and adding it to the ssh-agent](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent)
- [Adding a new SSH key to your GitHub account](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account)

To make the command easier, you can configure an alias in your `~/.ssh/config` like so:

```ssh-config
Host rustvm
  User <your-username>
  HostName <name>.infra.rust-lang.org
```

Then you can connect with `ssh rustvm`.

## How to set up your account

When connecting to the machine for the first time, there are a few things you
might want to do.

First, check that your Git username and email are configured correctly.

```shell
git config -l --global
```

You can configure your username and email address with:

```shell
git config --global user.name "Your name"
git config --global user.email "your-email"
```

## How to customize your shell

You can set your default shell on the dev desktops by adding yourself to a
configuration file in the [`rust-lang/simpleinfra`][simpleinfra-vars-user-config]
repository. Open `ansible/roles/dev-desktop/defaults/main.yml`, look for the
variable `vars_user_config`, and add yourself to the list.

```yaml
vars_user_config:
  - username: gh-jdno
    shell: /usr/bin/zsh
  - username: gh-WaffleLapkin
    shell: /usr/bin/fish
```

Open a pull request and request a review from `@rust-lang/infra` (or ping us in
`#t-infra` on Zulip).

After the pull request is merged, an infrastructure admin has to deploy the
new configuration to the dev desktops. Only after that will your default shell
be changed.

## How to install a Rust toolchain

The dev desktops don’t have Rust pre-installed, but instead make it easy to
install a specific toolchain from a local repository or worktree.

First, you want to run the following command to install `rustup`:

```shell
/usr/local/bin/init.sh
```

If you don't want or need to work with your own version of Rust, you can skip
the next section and start working.

If you haven't done so yet, open the [rust-lang/rust] repository on GitHub and
create a fork in your personal account. Then connect to the dev desktop and run
the following script:

```shell
/usr/local/bin/setup_rust.sh
```

The script will clone your personal fork to the dev desktop, check out the
latest version from [rust-lang/rust], and compile it. Once that's done, it will
link the stages so that you can work with them locally.

The directory contains more scripts to manage worktrees and Rust versions. Run
`help.sh` to get a list and a short description of them.

## How to interact with GitHub

The dev desktops are designed to work with repositories on GitHub that belong to
your user account. A GitHub App is used to protect your credentials and give you
granular control over the repositories that the dev desktops can access.

First, go to <https://github.com/apps/rust-cloud-vms> to give the app access to
your repositories. It's recommended to only grant access to the repositories
that you want to use on the dev desktop, e.g. your fork of `rust-lang/rust`.

Then connect to the dev desktop and clone the repository that you want to work
on with HTTPS. From there, you can work with the repository like you would
normally do.

Under the hood, the GitHub App acts as a credentials helper for Git and
generates temporary access tokens that are scoped to the permissions that you
have granted the application. If you get an error, review the permissions and
ensure that the app is allowed to access your repository.

## How to set up remote development in Visual Studio Code

Most modern code editors provide support for remote development via SSH. This
can be used to write code locally, but execute it inside the dev desktop. While
the configuration will differ slightly, the following example for
[Visual Studio Code] should be applicable to other editors as well.

Setting up remote development with VS Code is pretty straightforward, and is
described in detail in VS Code’s documentation: [Remote Development using SSH].
In summary:

1. SSH into the dev desktop and clone the repository that you want to work on to
   a local folder
2. Then open VS Code on your machine and install
   the [Remote Development Extension Pack](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack)
3. Open the command palette and search for “Remote-SSH: Connect to host”
4. Enter your username and the instance name (`<your-username>@<instance>`)
5. Select the path for the cloned repository from step 1
6. Install any extensions that you want to run on the server (e.g.
   rust-analyzer)
7. Use VS Code to run or debug the code remotely

## How to request new packages and tools

If you need a specific tool or package that is not installed on the dev
desktops, you can request its installation by opening a pull request against
the [rust-lang/simpleinfra] repository. The Ansible role [`dev-desktop`](https://github.com/rust-lang/simpleinfra/blob/master/ansible/roles/dev-desktop)
contains a tasks called [`dependencies.yml`](https://github.com/rust-lang/simpleinfra/blob/master/ansible/roles/dev-desktop/tasks/dependencies.yml)
that lists the system packages that are installed on the machines. Add the
required package to this list, open a pull request, and ping `@rust-lang/infra`
for a review.

It helps to check <https://packages.ubuntu.com/> first and make sure that the
package is available for both `arm64` and `amd64` architectures. The dev
desktops are currently running Ubuntu 22.04 LTS.

## Available disk space

Every user has a quota of how much disk space they can use.
Currently, the quota is set to
[150GB](https://github.com/rust-lang/simpleinfra/blob/8a59e4faeb75a09b072671c74a7cb70160ebef50/ansible/roles/dev-desktop/defaults/main.yml#L7).

Exceeding the quota will result in the error `Disk quota exceeded (os error 122)`.

The [remarks on disk space] section of the rustc dev guide contains some tips on how to clean your disk space.

## How to give feedback and report issues

If you experience any problems with the dev desktops, or have feedback and
suggestions, get in touch with the [infrastructure team]:

[#t-infra on Zulip](https://rust-lang.zulipchat.com/#narrow/stream/t-infra)

We might ask you to create an issue in the [rust-lang/simpleinfra] repository.

[cloud compute program]: https://foundation.rust-lang.org/news/2021-11-16-news-announcing-cloud-compute-initiative/
[infra@rust-lang.org]: mailto:infra@rust-lang.org
[infrastructure team]: https://www.rust-lang.org/governance/teams/infra
[remote development using ssh]: https://code.visualstudio.com/docs/remote/ssh
[rust foundation]: https://foundation.rust-lang.org/
[rust-lang/rust]: https://github.com/rust-lang/rust
[rust-lang/simpleinfra]: https://github.com/rust-lang/simpleinfra
[remarks on disk space]: https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html#remarks-on-disk-space
[visual studio code]: https://code.visualstudio.com/
[certain teams]: https://github.com/search?q=repo%3Arust-lang%2Fteam+path%3Ateams%2F*.toml+dev-desktop&type=code&ref=advsearch
[simpleinfra-vars-user-config]: https://github.com/rust-lang/simpleinfra/blob/dbf839ef25155df1f33c18f151283436b0f70f3b/ansible/roles/dev-desktop/defaults/main.yml#L12:L16
