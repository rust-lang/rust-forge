---
layout: default
title: So you want to stabilize a feature?
---

Once we have decided to **stabilize** a feature, we need to have a PR that actually makes that stabilization happen. These kinds of PRs are a great way to get involved in Rust, as they take you on a little tour through the source code. Here is a general guide to how to stabilize a feature -- every feature is different, of course, so some features may require steps beyond what this guide talks about.

**IMPORTANT:** Before we stabilize any feature, note that we also have a rule that it should appear in the documentation. This is often overlooked. =)

### Updating the feature-gate listing

There is a central listing of feature-gates in `src/libsyntax/feature_gate.rs`. Search for the `declare_features!` macro. In there, you should find an entry for the feature you are aiming to stabilize, something like (this example is taken from [rust-lang/rust#32409](https://github.com/rust-lang/rust/issues/32409):

```rust
    // pub(restricted) visibilities (RFC 1422)
    (active, pub_restricted, "1.9.0", Some(32409)),
```

You want to move this line down to the area for "accepted" features, declared below in a separate call to `declare_features!`. So when you're done it should look like:

```rust
    // pub(restricted) visibilities (RFC 1422)
    (accepted, pub_restricted, "1.17.0", Some(32409)),
    //                          ^^^^^^ note that we changed this
```

Note that we will change the version number to be the version number of the stable release where this feature will appear. This can be found by consulting http://rusty-dash.com/, which will tell you the next stable release number. You want to add 1 to that, because the code that lands today will become go into beta on that date, and then become stable after that. So, at the time of this writing, the next stable release (what is currently beta, iow) was 1.16.0, hence I wrote 1.17.0 above.

### Removing existing uses of the feature-gate

Next you will want to search for the feature string (in this case, `pub_restricted`) in the codebase to find where it appears. You can change uses of `#![feature(XXX)]` from the stdlib and rustc crates to be `#![cfg_attr(stage0, feature(XXX))]`. This includes the feature-gate only for stage0, which is built using the current beta (this is needed because the feature is still unstable in the current beta).

Similarly, you can remove those strings from any tests. If there are tests specifically targeting the feature-gate (i.e., testing that the feature-gate is required to use the feature, but nothing else), you can simply remove the test.

### Do not require the feature-gate to use the feature

Most importantly, you want to remove the code which flags an error if the feature-gate is not present (since the feature is now considered stable). If the feature can be detected because it employs some new syntax, then a common place for that code to be is in the same `feature_gate.rs`. For example, you might see code like this:

```rust
gate_feature_post!(&self, pub_restricted, span, "`pub(restricted)` syntax is experimental");
```

This `gate_feature_post!` macro prints an error if the `pub_restricted` feature is not enabled. It is not needed now that `#[pub_restricted]` is stable. 

For more subtle features, you may find code like this:

```rust
if self.tcx.sess.features.borrow().pub_restricted { /* XXX */ }
```

This `pub_restricted` field (obviously named after the feature) would ordinarily be false if the feature flag is not present, and true if it is. So you can transform the code to assume that the field is true. In this case, that would mean removing the `if` and leaving just the `/* XXX */`.

Some examples:

```rust
if self.tcx.sess.features.borrow().pub_restricted { /* XXX */ }
= becomes ==>
/* XXX */

if self.tcx.sess.features.borrow().pub_restricted && something { /* XXX */ }
= becomes ==>
if something { /* XXX */ }
```

