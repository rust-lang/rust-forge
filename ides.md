---
layout: default
title: Rust and IDEs &middot; The Rust Forge
---

# Rust and IDEs

Modern IDEs can give developers a massive increase in
productivity. Good IDE support is an essential part of the tooling for
a new programming language. Several community projects have provided
an excellent start towards IDE support. Over the next few quarters,
providing better IDE support is a high priority for the Rust teams and
we hope to see big leaps in the power and usability of Rust IDEs.

Good IDE support requires a number of components - the compiler must
be modified to operate in a different mode, we must provide name and
type information from the compiler to the IDE, and we must write
plugins for the IDEs themselves so they know what to do with Rust
projects. We're going to work on all three of these components in
parallel and there is a lot of scope for useful input from the
community.


<a id="plan"></a>
## The plan

To work with an IDE, the compiler must respond to changes made by the
users in milliseconds and be able to cope with incomplete code. To
manage this we propose modifying the compiler to support incremental
and lazy compilation, and in the long-run, operate as a long-running
daemon process. The compiler must also be more robust and be able to
continue compilation in the face of parsing and name resolution
errors.

We propose a new tool, an 'oracle' which takes input from the
compiler, maintains a project-wide view of the code and it's type
information, and provides data about the project to IDEs and other
tools. The oracle is a long-running daemon and presents an API via
IPC.

We hope to provide support for IntelliJ IDEA, Eclipse, and Visual
Studio. There are existing projects started in the community for all
three of these IDEs. We want to support these efforts however we
can. Between these IDEs we should have good platform coverage and
cover most programmers' preferences. Depending on how things develop
we may focus or broaden our efforts in the future.


<a id="status"></a>
## Current status

There is [an RFC PR](https://github.com/rust-lang/rfcs/pull/1317) for
the compiler-side components. The RFC spells out in detail the changes
to the compiler and the plan for the 'oracle' tool. Comments are very
welcome.

Available IDE plugins:

* [Eclipse](https://github.com/RustDT/RustDT)
* [Visual Studio](https://github.com/PistonDevelopers/VisualRust)
* [IntelliJ IDEA](https://intellij-rust.github.io)

Editor plugins:

* [Emacs](https://github.com/rust-lang/rust-mode)
* [Vim](https://github.com/rust-lang/rust.vim)
* [Sublime Text](https://packagecontrol.io/packages/Rust)
* [Atom](https://atom.io/packages/language-rust)
* [Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=saviorisdead.RustyCode)

Code completion is available via
[Racer](https://github.com/phildawes/racer).  See
[areweideyet.com](http://areweideyet.com/) for more details on editor
plugins and integration with Racer.

The next steps are to finalise the plan as described in the RFC. Then
begin implementation work on the compiler and oracle, and continue
work on the IDE plugins.

Formatting source code is available via
[rustfmt](https://github.com/rust-lang-nursery/rustfmt)

<a id="help"></a>
## How to help

* Comment on the [RFC PR](https://github.com/rust-lang/rfcs/pull/1317),
* Use the IDEs (see links below), file issues,
* Contribute code:

### The compiler

The tracking issue for the IDE RFC will list useful issues to
tackle. Generally, these issues require some experience with the Rust
compiler. But some could be mentored bugs if you have an interest in
compilers. All written in Rust.

Repository: [https://github.com/rust-lang/rust](https://github.com/rust-lang/rust)

Tracking issue: TBA

Contact: Nick Cameron (nrc on GitHub and irc, nrc@mozilla.com)

### The Oracle

The oracle is also written in Rust, but should be a much easier project to get to grips with than the compiler, since it is in a much earlier stage of development and is smaller.

Repository: TBA

Issues: TBA

List of good first bugs: TBA

Contact: Nick Cameron (nrc on GitHub and irc, nrc@mozilla.com)

### IDE plugins

The Visual Studio plugin is written in C#, the other IDE plugins are written in Java. In any case, experience dealing with the IDE in question is probably more important than knowing the Rust compiler in depth, however, some knowledge of the AST, etc. will be useful.

**Eclipse (RustDT)**

Installation instructions: [https://github.com/RustDT/RustDT/blob/latest/documentation/Installation.md](https://github.com/RustDT/RustDT/blob/latest/documentation/Installation.md)

Download: [http://rustdt.github.io/releases](http://rustdt.github.io/releases) (via Eclipse)

Repository: [https://github.com/RustDT/RustDT](https://github.com/RustDT/RustDT)

Issues: [https://github.com/RustDT/RustDT/issues](https://github.com/RustDT/RustDT/issues)

Contact: Bruno Medeiros

**IntelliJ IDEA**

Documentation: [https://intellij-rust.github.io](https://intellij-rust.github.io)

Repository: [https://github.com/intellij-rust/intellij-rust](https://github.com/intellij-rust/intellij-rust)

Issues: [https://github.com/intellij-rust/intellij-rust/issues](https://github.com/intellij-rust/intellij-rust/issues)

Contact: [https://gitter.im/intellij-rust/intellij-rust](https://gitter.im/intellij-rust/intellij-rust)

**Visual Studio (Visual Rust)**

Download: [https://visualstudiogallery.msdn.microsoft.com/c6075d2f-8864-47c0-8333-92f183d3e640](https://visualstudiogallery.msdn.microsoft.com/c6075d2f-8864-47c0-8333-92f183d3e640)

Repository: [https://github.com/PistonDevelopers/VisualRust](https://github.com/PistonDevelopers/VisualRust)

Issues: [https://github.com/PistonDevelopers/VisualRust/issues](https://github.com/PistonDevelopers/VisualRust/issues)

Contact: [https://github.com/vosen/]() (vosen@vosen.pl)

### Editor Plugins

**Visual Studio Code (RustyCode)**

Installation instructions: [https://marketplace.visualstudio.com/items?itemName=saviorisdead.RustyCode]()

Download: [https://marketplace.visualstudio.com/items?itemName=saviorisdead.RustyCode]() (search rust via package manager)

Repository: [https://github.com/saviorisdead/RustyCode]()

Issues: [https://github.com/saviorisdead/RustyCode/issues]()

Contact: [https://github.com/saviorisdead/](Konstantin Akhantev)
