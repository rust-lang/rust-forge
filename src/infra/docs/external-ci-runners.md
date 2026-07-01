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
- `repositories`: The repositories that are enabled to use the runner. To add your repository to the
  list, please open a PR to this file.
  Make sure the workflow you want to run the runner on passes [`zizmor`](https://zizmor.sh/).
- `contact`: The contact information of the person(s) responsible for maintaining the runner.
  The recommended way to contact them is via Zulip, by opening a new topic in the
  [#t-infra](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra) channel and mentioning
  the contact person(s).

### `powerpc64-unknown-linux-musl`

- max concurrency: 4
- repositories:
  - [`compiler-builtins`](https://github.com/rust-lang/compiler-builtins)
  - [`libc`](https://github.com/rust-lang/libc)
- [example workflow](https://github.com/rust-lang/compiler-builtins/pull/1219)
- contact:
  - Zulip: [Aelin](https://rust-lang.zulipchat.com/#user/338253)
  - Email: `aelin@postmarketos.org` (faster response)

### `ubuntu-2X.04-riscv`

- variants:
  - `ubuntu-24.04-riscv`
  - `ubuntu-26.04-riscv`
- repositories:
  - [`rustc_codegen_cranelift`](https://github.com/rust-lang/rustc_codegen_cranelift) (enabled, unused)
- [documentation](https://riscv-runners.riseproject.dev/)
