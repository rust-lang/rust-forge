# Triaging Crater Runs

## Running crater

We regularly run Crater runs, and this documents the procedure for triaging a beta run; it may also
be applicable to non-release team runs (e.g., PR crater runs) with minor modifications.

First, file a new issue titled "Crater runs for 1.x" ([example](https://github.com/rust-lang/rust/issues/66244))

A crater run for beta should be started as soon as we have beta out. Use the following craterbot
invocations.

$BETA_VERSION is e.g. 1.40.0-1, increment the 1 if it's not the first beta crater run, you can also
use the auto-incremented counter on the beta `rustc --version`.

$STABLE is e.g. 1.39.0 (the stable release)
$BETA is beta-YYYY-MM-DD, get the date by looking at https://static.rust-lang.org/manifests.txt and
get the date of the most recent channel-rust-beta.toml.

```
@craterbot run name=beta-$BETA_VERSION start=$STABLE end=$BETA mode=build-and-test cap-lints=warn p=10
@craterbot run name=beta-rustdoc-$BETA_VERSION start=$STABLE end=$BETA mode=rustdoc cap-lints=warn p=5
```

Once the runs complete, you want to triage them

## Triaging

These steps should generally be done for the normal rustc run, and then followed up by a triage of
the rustdoc run. Ignore failures in rustdoc that look to be rooted in rustc (i.e., duplicate
failures).

There will usually be quite a few regressions -- there are a couple tools that can help reduce the
amount of work that you need to do. It's mostly a matter of personal preference which is more
helpful.

 * https://github.com/Mark-Simulacrum/crater-generate-report/
   * This groups regressions by 'root' by parsing the logs to look for the compilation failed
     messages printed by Cargo
 * https://github.com/Centril/crater-cat-errors
   * This groups regressions by the "error" message, also by parsing logs

If you've written a tool, feel free to add it here! We're still figuring out what the best UI for
this is.

Regardless of the tool you've run, you ultimately need to read through a bunch of logs and try to
quickly determine if they're genuine failures or spurious. Most of the time, a compiler failure is
genuine, and test failures are mostly spurious, but this usually requires some level of guessing.

Once you've determined that something is a genuine failure, add it to a list somewhere (local file,
HackMD, whatever) with the error "category." Mostly, you're trying to group things such that the
regressions in a single group are all caused by the same set of commits, and different groups have
different causes.

Once this is done, and you have all the regressions triaged into their separate groups, you want to
file a new issue for each group. It should have the `regression-from-stable-to-beta` and
`T-compiler` label by default, possibly `T-libs` if it's a standard library regression, but that's
relatively rare. If you happen to think you know the PR that caused the failure, cc the PR author in
a separate comment and link to the PR; otherwise compiler team will triage the issue soon.

Leave a comment on the original issue with the crater runs linking to all the issues you just
opened, ideally with the issue titles as well.

You're done!

## Re-running rustc on a crate

For the crates which we're not sure about, you can try running crater locally, or build the crate
directly ([`cratesio-curl`] can be helpful). Be careful -- regardless of what you do, you are running arbitrary code locally. It's
also fine to file issues for the crates you're not sure about and let the triage process naturally
categorize the error, though it's not good to do this for *all* the crates. Once you've triaged a
crater run a couple times you get a pretty good sense of what is spurious and what isn't, too.

You can run crater on just a single crate by doing something like this (at least, as of now).
Note that this will download several gigabytes (on first use) and requires Docker to be running.

```
git clone https://github.com/rust-lang/crater
cd crater
cargo run -- prepare-local
CRATES="crates-io-crate-0.4.0,owner/repository-name" # Edit this.
cargo run -- define-ex --crate-select=list:$CRATES --cap-lints=forbid 1.38.0 beta # Edit the stable version.
cargo run -- run-graph --threads 4
cargo run -- gen-report work/ex/default/
# view report for this crate
```

It's also possible to re-queue a subset of crates onto the official builders, for which that take a
look at: https://gist.github.com/ecstatic-morse/be799bfa4d3b3d6e163fa61a9c30706f

[`cratesio-curl`]: https://gist.githubusercontent.com/lqd/4a8af10389d10840d90655c109df5eac/raw/1bb8ac86e211b745b1674041bc725a859b390c3c/cratesio-curl

## Determining the root cause of the regression

It's not always apparent why a crate stopped building. This isn't generally something done as part
of crater triage -- but can be a good followup. Here, [`cargo-bisect-rustc`] and Felix's
[minimization guide] are excellent tools to apply.

[`cargo-bisect-rustc`]: https://github.com/rust-lang/cargo-bisect-rustc
[minimization guide]: http://blog.pnkfx.org/blog/2019/11/18/rust-bug-minimization-patterns/
