# So you want to add a new (stable) option to rustc

So you want to add a new command-line flag to rustc. What is the procedure?

## Is this a perma-unstable option?

The first question to ask yourself is:

* Is this a "perma-unstable" option meant only for debugging rustc (e.g., `-Ztreat-err-as-bug`)?

If so, you can just add it in a PR, no check-off is required beyond ordinary review.

## Other options

If this option is meant to be used by end-users or to be exposed on the stable channel, however, it represents a "public commitment" on the part of rustc that we will have to maintain, and hence there are a few more details to take care of.

There are two main things to take care of, and they can proceed in either order, but both must be completed:

* Proposal and check-off
* Implementation and documentation

Finally, some options begin as unstable and only get stabilized over time, in which case you will also need:

* Tracking issue and stabilization

### Proposal and check-off

The "proposal" part describes the motivation and design of the new option you wish to add. It doesn't necessarily have to be very long. It takes the form of a [Major Change Proposal][MCP].

[MCP]: https://forge.rust-lang.org/compiler/mcp.html

The proposal should include the following:

* **Motivation:** what is this flag used for?
* **Design:** What input does the flag take and what is its observable effect?
* **Implementation notes:** You don't have to talk about the implementation normally, but if there are any key things to note (i.e., it was very invasive to implement), you night note them here.
* **Precedent, links, and related material:** Are similar flags available on other compilers/linkers/tools, like clang or lld?
* **Alternatives, concerns, and key decisions:** Were there any alernatives considered? If so, why did you pick this design?

Note that it is fine if you don't have any implementation notes, precedent, or alternatives to discuss.

Also, one good approach to writing the MCP is basically to write the documentation you will have to write anyway to explain to users how the option works, and then add any additional notes on alternatives and so forth that are required.

Once you've written up the proposal, you can [open a MCP](https://github.com/rust-lang/compiler-team/issues/new?assignees=&labels=major-change%2C+T-compiler&template=major_change.md&title=%28My+major+change+proposal%29) issue. But note that since this MCP is promoting a permanent change, a full compiler-team FCP is required, and not just a "second". This can be done by `@rfcbot fcp merge` by a team member.

### Implementation, documentation

Naturally your new option will also have to be implemented. You can implement the option and open up a PR. Often, this implementation work actually happens **before** the MCP is created, and that's fine -- we'll just ask you to open an MCP with the write-up.

A few notes that are sometimes overlooked:

* Many options begin as "unstable" options, either because they use `-Z` or because they require `-Zunstable-options` to use.
* You should document the option. Often this documentation can just be copied from the MCP text. Where you add this documentation depends on whether the option is available on stable Rust:
    * If it is unstable, then document the option in the [Unstable Book](https://doc.rust-lang.org/nightly/unstable-book/index.html), whose sources are in [src/doc/unstable-book](https://github.com/rust-lang/rust/tree/master/src/doc/unstable-book).
    * Once the option is stabilized, it should be documented in the [Rustc book](https://doc.rust-lang.org/rustc/index.html), whose sources as in [src/doc/rustc](https://github.com/rust-lang/rust/tree/master/src/doc/rustc). 


## Stabilization and tracking issue

Typically options begin as unstable, meaning that they are either used with `-Z` or require `-Zunstable-options`.

Once the issue lands we should create a tracking issue that links to the MCP and where stabilization can be proposed.

Stabilization generally proceeds when the option has a seen a bit of use and the implementation seems to be working as expected for its intended purpose.

Remember that when stabilization occurs, documentation should be moved from the Unstable Book to the Rustc Book.
