# Trusted publishing

[Crates.io trusted publishing] lets a GitHub Actions workflow publish a crate
without storing a long-lived crates.io API token in GitHub Actions secrets.

Trusted publishing is the recommended way to publish Rust Project crates.

This page explains how to configure trusted publishing in Rust Project
repositories using the [`rust-lang/team`] repository and describes the
most common publishing patterns.

<div class="warning">

If your repository is not hosted under a Rust GitHub organization, you can't
configure trusted publishing through the `rust-lang/team` repository. Follow
the instructions in the [crates.io trusted publishing] documentation instead.

</div>

## 1. Set crate ownership

Make sure the crates you want to manage with trusted publishing are owned by the
[rust-lang-owner](https://crates.io/users/rust-lang-owner) account.
If they aren't, invite that account as an owner and ask in the `#t-infra` Zulip
channel to accept the invitation.

## 2. Configure the team repository

Add a `[[crates-io]]` section to your
[repository TOML file](https://github.com/rust-lang/team/tree/main/repos), as
described in the
[crates.io crate management docs](https://github.com/rust-lang/team/blob/main/docs/toml-schema.md#cratesio-crate-management).

Make sure the environment referenced by `publish-environment` is also defined.
For example:

```toml
[environments.publish]
branches = ["main"]

[[crates-io]]
publish-environment = "publish"
...
```

### Environment rules

The environment rules should match the event that triggers the publish
GitHub Actions workflow.

For example, if the publish is triggered by:

```yaml
on:
  push:
    tags: ["v*"]
```

then the environment should allow the workflow to run on the same tags pushes:

```toml
[environments.publish]
tags = ["v*"]
```

## 3. Write the GitHub Actions workflow

Every workflow that publishes to crates.io through trusted publishing needs:

- a job with `environment: <publish-environment>`;
- `permissions: id-token: write` on that job, so GitHub can issue an OIDC token;
- a workflow file whose name matches `publish-workflow` in `rust-lang/team`.

You can use one of the following approaches to publish your crate with GitHub
Actions.

### 3a. Publish using cargo

Use this approach if you want to use the [crates-io-auth-action] and run
`cargo publish` directly in the workflow.

See the "GitHub Actions Setup" section of the
[crates.io trusted publishing] documentation for more details on how to set up
the workflow and use the action.

<div class="warning">

The `environment` field of the job is mandatory for Rust Project crates, and it
needs to match the `publish-environment` field in the `rust-lang/team`
repository.

For example:

```yaml
jobs:
  publish:
    environment: publish
    ...
```

</div>

You can find examples of this approach in Rust Project repositories by using
[this GitHub search](https://github.com/search?q=org%3Arust-lang+AND+%28path%3A*.yml+OR+path%3A*.yaml%29++rust-lang%2Fcrates-io-auth-action%40&type=code).

```yaml
on:
  push:
    tags: ["v*"]

jobs:
  publish:
    runs-on: ubuntu-latest

```

Note that you can trigger this workflow from other GitHub events, such as:

* on release:
  ```yaml
  on:
    release:
      types: [created]
  ```
* manually with workflow dispatch:
  ```yaml
  on:
    workflow_dispatch:
  ```

### 3b. release-plz from the default branch

If you want more automation, instead of running `cargo publish` directly in the
workflow, check out [release-plz].

You can find examples of `release-plz` in Rust Project repositories by using
[this GitHub search](https://github.com/search?q=org%3Arust-lang+AND+%28path%3A*.yml+OR+path%3A*.yaml%29++release-plz%2Faction%40&type=code).

[crates.io trusted publishing]: https://crates.io/docs/trusted-publishing
[release-plz]: https://release-plz.ieni.dev/
[crates-io-auth-action]: https://github.com/rust-lang/crates-io-auth-action
[`rust-lang/team`]: https://github.com/rust-lang/team
