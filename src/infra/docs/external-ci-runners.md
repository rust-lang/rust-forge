# External CI Runners

The Rust Project mainly runs Continuous Integration (CI) in GitHub Actions.
When GitHub Actions does not natively support a target, we use
[self-hosted runners](https://docs.github.com/en/actions/concepts/runners/self-hosted-runners).

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
