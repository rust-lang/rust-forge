# Autolabels

Auto labels will automatically apply labels to GitHub issues and PRs based on the `[autolabel]` configuration in `triagebot.toml`.

## Usage

Auto labels have no manual control.
See [labeling](labeling.md) for manually changing labels.

## Configuration

### Triggered by labels

Labels can be added when another label is added.
The `trigger_labels` config option specifies which labels will cause this to trigger.

```toml
# Automatically applies the `I-prioritize` label whenever one of the labels
# listed below is added to an issue (unless the issue already has one of the
# labels listed in `exclude_labels`).
[autolabel."I-prioritize"]
trigger_labels = [
    "regression-untriaged",
    "regression-from-stable-to-stable",
    "regression-from-stable-to-beta",
    "regression-from-stable-to-nightly",
    "I-unsound",
]
exclude_labels = [
    "P-*",
    "T-infra",
    "T-release",
    "requires-nightly",
]
```

Exclude labels support shell-like `*` glob patterns.

### Triggered by files

Labels can be added based on which files are modified in a PR.
The `trigger_files` config option specifies which files will cause the label to be added.
Paths are matched with `starts_with`.

```toml
# Adds the `T-compiler` label to any PR that touches `compiler` or
# `src/test/ui` unless it already has a `T-*` label.
[autolabel."T-compiler"]
trigger_files = [
    "compiler",
    "tests/ui",
]
exclude_labels = [
    "T-*",
]
```

### Triggered by new PRs

Labels can be added to any PR in a non-draft state, either when opened or later when they switch status.
The labels are removed when they don't meet those conditions anymore.

Set the `new_pr = true` config option to enable this.
For example:

```toml
[autolabel."S-waiting-on-review"]
new_pr = true
```

### Triggered by new draft PRs

Labels can be added to any PR in a draft state, either when opened or later when they switch status.
The labels are removed when they don't meet those conditions anymore.

Set the `new_draft = true` config option to enable this.
For example:

```toml
[autolabel."S-waiting-on-author"]
new_draft = true
```

### Triggered by new issues

Labels can be added to any issue when it is opened.
Set the `new_issue = true` config option to enable this.
For example:

```toml
[autolabel."new-issue"]
new_issue = true
```

### Triggered by merged PRs

Labels can be added to any PRs when it is merged.
Set the `pr_merged = true` config option to enable this.
For example:

```toml
[autolabel."needs-relnotes-triage"]
pr_merged = true
```

## Implementation

See [`src/handlers/autolabel.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/autolabel.rs).
