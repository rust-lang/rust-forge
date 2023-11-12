# GitHub Releases

Triagebot can be used to automatically create releases on GitHub when a tag is pushed, using the relevant section of the changelog as the release body.
No artifacts are uploaded when doing this.

## Usage

Any time you push a git tag, or update the contents of the changelog, triagebot will synchronize *all* tags with the releases.
That is, any tag that doesn't have a release will create a new release.
Additionally, the text of all the releases will be synchronized with the text in the changelog.

Tags that don't have entries in the changelog will not create a release.

## Configuration

To enable automatically creating GitHub Releases, add this to the `triagebot.toml` at the root of your repository:

```toml
[github-releases]
format = "rustc"
project-name = "Rust"
changelog-path = "RELEASES.md"
changelog-branch = "master"
```

The `format` defines which format the changelog file adheres to, and it's used to properly extract the relevant section from it.
You can add another format by changing triagebot's [`src/changelogs/`](https://github.com/rust-lang/triagebot/tree/master/src/changelogs).
The currently supported formats are:

* `rustc`: follows the custom style of rustc's [RELEASES.md](https://github.com/rust-lang/rust/blob/master/RELEASES.md).

The `project-name` defines what the title of the release should be.
The final title will be `{project-name} {tag}`.

The `changelog-path` and `changelog-branch` keys define where triagebot should look at when searching for the changelog.

## Implementation

See [`src/handlers/github_releases.rs`](https://github.com/rust-lang/triagebot/blob/HEAD/src/handlers/github_releases.rs) and [`src/changelogs/`](https://github.com/rust-lang/triagebot/tree/HEAD/src/changelogs).
