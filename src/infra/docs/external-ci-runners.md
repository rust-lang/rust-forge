# External CI Runners

The Rust Project mainly runs Continuous Integration (CI) in GitHub Actions.
When GitHub Actions do not natively support a target, we use
[self-hosted runners](https://docs.github.com/en/actions/concepts/runners/self-hosted-runners).

## Available software

GitHub Actions self-hosted runners usually do not contain all the software that
is available in GitHub Actions hosted runners (e.g. `ubuntu-latest`).

If you see a CI failure on a self-hosted runner, check if the software you need
is available. If it is not, you can install it in your GitHub Actions workflow.

## Policy

If you want to add a new external CI runner, please read our
[policy](https://github.com/rust-lang/infra-team/blob/main/service-catalog/rust-ci/external-runners/README.md)
first.

## Configured external CI runners

This section lists the external CI runners that are configured to run in the CI of the Rust Project.

Here is an explanation of the fields in the list below:

- `max concurrency`: The available number of runners of this type.
  This determines the maximum number of GitHub Actions jobs that can run on the runner.
  The concurrency is shared among repositories. If all runners are busy, GitHub Actions will queue
  the jobs until a runner is available.
- `runs-on`: The label(s) that you should use in the `runs-on` field of a GitHub Actions workflow to
  run this runner.
  To check where a runner is used, search for the label in
  [GitHub Search](https://github.com/search?q=org%3Arust-lang%20%5B%22self-hosted%22%2C%20%22linux%22%2C%20%22powerpc64%22%2C%20%22musl%22%5D&type=code).
- `repositories`: The repositories that are enabled to use the runner. To add your repository to the
  list, please open a PR to this file.
  Make sure the workflow you want to run the runner on passes [`zizmor`](https://zizmor.sh/).
- `contact`: The contact information of the person(s) responsible for maintaining the runner.
  The recommended way to contact them is via Zulip, by opening a new topic in the
  [#t-infra](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra) channel and mentioning
  the contact person(s).

### `powerpc64-unknown-linux-musl`

- max concurrency: 4
- `runs-on`: `["self-hosted", "linux", "powerpc64", "musl"]`
- repositories:
  - [`compiler-builtins`](https://github.com/rust-lang/compiler-builtins)
  - [`libc`](https://github.com/rust-lang/libc)
- contact:
  - Zulip: [Aelin](https://rust-lang.zulipchat.com/#user/338253)
  - Email: `aelin@postmarketos.org` (faster response)

### `riscv64gc-unknown-linux-gnu`

- `runs-on`:
  - `ubuntu-24.04-riscv` for the Ubuntu 24 runner.
  - `ubuntu-26.04-riscv` for the Ubuntu 26 runner.
- repositories:
  - [`rustc_codegen_cranelift`](https://github.com/rust-lang/rustc_codegen_cranelift) (enabled, unused)
- [documentation](https://riscv-runners.riseproject.dev/)

### `powerpc64le-unknown-linux-gnu`

- `runs-on`: `ubuntu-24.04-ppc64le`
- type: [IBM runners](#ibm-runners)

### `s390x-unknown-linux-gnu`

This target is available in various external runners.

*Canonical runner:*

- `runs-on`: `self-hosted-linux-s390x-resolute-large-rust` (ubuntu-26.04)
- repositories: it works on all repositories, there's no need to enable it.
- type: [Canonical runners](#canonical-runners).

*IBM runner:*

- `runs-on`: `ubuntu-24.04-s390x`
- type: [IBM runners](#ibm-runners).

## Types of runners

This section groups info common to more than one runner.

### Canonical runners

- max concurrency: 10
- [GitHub App](https://github.com/organizations/rust-lang/settings/installations/142169754)
- [Operator Repository](https://github.com/canonical/github-runner-operator)
- contact: downtimes are reported in the
  [Canonical Runners downtime Zulip topic](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra/topic/Canonical.20Runners.20downtime/with/609185710)
- repositories: the app is installed org-wide, so all repositories can use the runners.

### IBM runners

- [GitHub App](https://github.com/apps/power-z-gha-runner/)
- repositories:
  - [libc](https://github.com/rust-lang/libc)
  - [stdarch](https://github.com/rust-lang/stdarch)
  - [compiler-builtins](https://github.com/rust-lang/compiler-builtins)

We can't install this GitHub App in the `rust-lang/rust` repository
because the app requires admin permission to run.
See [this](https://github.com/IBM/actionspz/issues/20) GitHub issue for more info.

<details>
<summary><i>Infrastructure details</i></summary>

- We installed the GitHub App from the
  [rust-lang-owner](https://github.com/rust-lang-owner) user account.
- The runners were requested in [this](https://github.com/IBM/actionspz/issues/24) and the linked GitHub issues.

</details>
