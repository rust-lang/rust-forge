---
layout: default
title: So you want to implement a feature?
---

When you want to implement a new significant feature in the compiler, you need to go through this process to make sure everything goes smoothly:

## Process Overview

### Small Features - @rfcbot

If the feature is more of a large bugfix rather than a feature, and the relevant team doesn't think it needs to be iterated on on nightly, but it's still too user-visible or controversial for a normal code review, you can call for a final comment period using @rfcbot.

Someday, someone will write a guide for using @rfcbot and I'll link to it, but currently what I know is that you do write the following line:

```
@rfcbot fcp merge
```

in the feature's PR, and that marks the PR for an RFC-style FCP.

### Warning Cycles

In some other cases, a feature or bugfix might break some existing programs in some edge cases. In that case, you might want to do a crater run to assess the impact and possibly add a warning cycle.

MOAR DETAILS HERE

### Stability

For most features, code review is not enough. We want to gain experience in using that feature on nightly, and possibly change it on the basis of that experience.

If we want to change a feature, we must make sure users don't accidentally depend on that new feature - otherwise, especially if experimentation takes time or is delayed and the feature takes the trains to stable, it would end up *de facto* stable and we'll not be able to make changes in it without breaking people's code.

The way we do that is that we make sure all new features are *feature gated* - they can't be used without a enabling a *feature gate* (`#[feature(foo)]`), which can't be done in a stable/beta compiler. See the [stability in code](#stability-in-code) section for more details.

Eventually, after we gain the experience using the feature and are satisfied, we expose it to the world using the stabilization process described [here](stabilization-guide.html). Until then, nothing about the feature is set in stone. Features are not supposed to become "common law stable" after being unstable and unchanged for a year.

### Tracking Issues

For features that have an RFC, you should use the RFC's tracking issue for the feature.

For other features, MOAR DETAILS HERE

## Stability in code

In order to implement a new unstable feature, you need to do the following steps:

1. Open a [tracking issue](#tracking-issues) - if you have an RFC, you can use the tracking issue for the RFC.
2. Pick a name for the feature gate (for RFCs, use the name in the RFC).
3. Add a feature gate declaration to `libsyntax/feature_gate.rs`
    In the active `declare_features` block:
    ```
    // description of feature
    (active, $feature_name, "$current_nightly_version", Some($tracking_issue_number))
    ```
    For example
    ```
    // allow '|' at beginning of match arms (RFC 1925)
    (active, match_beginning_vert, "1.21.0", Some(44101)),
    ```
    
    The current version is not actually important - the important version is when you are *stabilizing* a feature.
4. Prevent usage of the new feature unless the feature gate is set. You can check it in most places in the compiler using `tcx.sess.features.borrow().$feature_name`, and you should either maintain the pre-feature behavior or raise an error, depending on what makes sense.
5. Add a test that the feature can't be used without a feature gate, under `src/test/compile-fail/feature-gate-$feature_name.rs`.
6. Add a section to the unstable book, in `src/doc/unstable-book/src/language-features/$feature_name.md`.
7. Write a lots of tests for the new feature. PRs without tests will not be accepted!
8. Get your PR reviewed and land it. You have now successfully implemented a feature in Rust!
