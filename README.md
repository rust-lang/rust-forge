# Rust Forge

Welcome to the [Rust Forge]! Rust Forge serves as a repository of supplementary
documentation useful for members of [The Rust Programming Language].

[the rust programming language]: https://rust-lang.org
[rust forge]: https://forge.rust-lang.org

# Development

You can build a local version by running the following command.

```console
mdbook build
```

This will build and run the `blacksmith` tool automatically. When developing
it's recommended to use the `serve` command to launch a local server to allow
you to easily see and update changes you make.

```console
mdbook serve
```

## JavaScript

Forge uses JavaScript to display dates for releases and "no tools breakage
week". When making modifications to the JavaScript make sure it matches the
[standard] style. You can install `standard` and automatically format the code
using the following commands.

[standard]: https://standardjs.com/index.html

### Install

```console
# With Yarn
yarn global add standard
# With NPM
npm install --global standard
```

### Formatting

```console
standard --fix js/
```

# Contributing

## Adding Teams

Any Rust team can have a section in the Rust Forge. If you'd like to add your team you first need to add them as an item to `src/SUMMARY.md`, like so replacing `TEAM NAME` with your respective team's name to show on forge, and `<TEAM_NAME>` with a filesystem and url friendly version of that name where your documentation will be stored.

```markdown
- [TEAM NAME](src/<TEAM_NAME>/README.md)
<!-- or -->
- [TEAM NAME](src/<TEAM_NAME>.md)
```

 If you run `mdbook build`, `mdbook` will automatically create the folder and file for your team.
 
 It's recommended that you put general team information in `src/<TEAM_NAME>/README.md` such as where the meetings happen, repositories that the team manages, links to chat platforms, etc. Larger topics should be made as a subpage e.g. (`src/release/topic.md`).

```markdown
- [TOPIC](src/<TEAM_NAME>/TOPIC.md)
```
