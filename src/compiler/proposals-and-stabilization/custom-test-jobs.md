# Adding custom test jobs or adding custom test components to rust-lang/rust CI

## Scope

This policy is applicable to proposals for adding new test jobs or adjusting existing test jobs to
build additional custom things that are *not* shipped to the end users, but may still cause
[rust-lang/rust] PR CI or Full CI (the "CI") to fail, or produce a failure message, which may impact
compiler/library/tools contributors who also use the [rust-lang/rust] CI.

For example, this includes but is not limited to:

- Ecosystem test jobs, such as Rust for Linux or Fuchsia;
- Custom codegen backend test components, such as `codegen_gcc`, which are not part of the "main"
  distributed toolchain.

## Background

[rust-lang/rust] runs a small set of test (and build) jobs on PR CI (faster, less
expensive/exhaustive), and runs a much larger set of test (and build) jobs on Full CI. PR CI usually
takes around an hour, while Full CI usually takes around 3 hours. Having test jobs that:

- may spuriously fail; or
- may genuinely fail but it's not clear who should be consulted about the failure or who is
  responsible for fixing the failure; or
- otherwise do not have well documented test job maintainers and failure protocol;

can introduce a lot of friction and frustration for other contributors who now will have to deal
with the failure or understand why the test job failed in the first place, which may now be blocking
their PR. Everyone utilizing the [rust-lang/rust] CI is expected to use it responsibly.

To help with this, please follow the process described below if you'd like to propose adding a
custom test job/component to the [rust-lang/rust] CI, and copy and fill out the Custom Test
Job/Component Policy.

## Process for adding a custom test job to rust-lang/rust CI

![Flowchart of the process for adding a custom test job/component to rust-lang/rust
CI](./custom-test-jobs/test-job-flow.svg)

## Custom Test Job/Component Template

*Please copy and fill out this template as part of the MCP.*

The proposed custom test job or custom test component for the [rust-lang/rust] CI must:

- Be approved by the compiler team through a proposed Major Change Proposal (MCP), where the MCP is
  seconded by a compiler team member, and the MCP being accepted with no blocking concerns.
- Have no blocking concerns from the library team by creating an issue on [rust-lang/rust] proposing
  to add the custom test job/component to [rust-lang/rust] CI linking to the MCP, and nominating the
  issue for review by the library team via `@rustbot label +I-libs-nominated`.
- Have the implementation PR be reviewed and approved by the infrastructure team, linking to the MCP
  and issue.
- Be properly documented on [rustc-dev-guide] (preferably as part of the implementation PR), linking
  to the MCP and issue.

### Test job/component rationale

- What does this custom test job/component do?
- What [rust-lang/rust] changes can potentially break the custom test job/component? E.g. changes to
  rustc, standard library, bootstrap or tools (like clippy/rustfmt/cargo).
- Why does this custom test job/component need to be part of the [rust-lang/rust] PR and/or Full
  Merge CI?
- If the custom test job/component will block on failure, why does it need to block?
- If the custom test job/component will not block on failure initially but is intended to eventually
  become blocking, why so? And when will it become blocking?

### Test job/component maintainers

The proposed custom test job/component for [rust-lang/rust] CI must have at least one dedicated
maintainer. The test job/component maintainers understand that they will be pinged or otherwise
contacted about the custom test job/component, particularly for (but not limited to) its failures.
Please list who will be maintaining this test job/component here.

**We, the custom test job/component maintainers, acknowledge and consent that we will be pinged
about the custom test job/component failures or be consulted about it in general.**

### Features and implementation details

- Does the proposed test job/component intend to use any unstable features?
    - If so, are the unstable features ready for exposure (e.g. must an unstable feature be
      completely reworked)?
    - For ecosystem test jobs/components, are the unstable features ready for such exposure, and are
      the feature stakeholders ready for such usage?
- Does the proposed test job/component intend to intentionally depend on any implementation details?
  This may include but is not limited to: unstable/internal compiler/tool flags and behaviors,
  `RUSTC_BOOTSTRAP` usages, standard library implementation details, etc.
    - If so, are there plans to shrink or expand such dependencies in the future?

### Failure protocol: what to do if the job/component breaks/fails?

- How can the test job/component maintainers be contacted in case of failure?
- If the addition of a custom test job is being proposed:
    - How can the test job be run in CI? If so, is there a try job (`try-job: ...`) invocation?
      What's the job name?
    - Can the test job be run locally? If so, how?
- If the addition of a custom test component is being proposed:
    - Which existing CI jobs will be building and testing this custom test component? Will they be
      built and ran as part of a try job? If so, what are the job names and the try job (`try-job:
      ...`) invocation?
    - Can the test component be built and run locally? If so, how?
- How can the custom test job/component be disabled in the event of spurious failures that are
  blocking PR and/or Full Merge CI?
- Will the `rust-log-analyzer` failure message include a link to the custom test job/component page
  in [rustc-dev-guide]?

For both custom test jobs/components:

- Is there a ping group or zulip channel/topic that can be used to notify the test job/component
  maintainers about a test job/component failure?
- If a PR breaks the test job/component:
    - If the breakage seems **spurious** and retrying does not resolve the spurious breakage, the
      test job will be **temporarily disabled** (see below).
    - If the breakage is **intentional**, how will this be resolved?
    - If the breakage is **unintentional**, is the PR author expected to fix the breakage?

**We, the test job/component maintainers, acknowledge and understand that the custom test
job/component may be temporarily disabled to unblock [rust-lang/rust] PR CI or Full Merge CI without
receiving prior approval from the test job/component maintainers, but the test job/component
maintainers will be pinged or otherwise notified about the test job/component being disabled.**

### Dependencies, build/test environments and reliability

- Does the test job involve any custom build systems that are not used in the regular
  [rust-lang/rust] CI jobs?
- Does the test job depend on external resources (e.g. external servers) that may be subject to
  network connectivity?
    - If so, does the infrastructure team need to help maintain a mirror of the required assets?
- Are there any other unusual requirements (build environment, dependencies, etc.)?

### CI infrastructure considerations

- Does the custom test job/component require substantial CI resources (storage and/or CI time)?


[rust-lang/rust]: https://github.com/rust-lang/rust
[rustc-dev-guide]: https://github.com/rust-lang/rustc-dev-guide
