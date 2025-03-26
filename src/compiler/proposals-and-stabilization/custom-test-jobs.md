# Adding ecosystem/integration test jobs/components to rust-lang/rust CI

## Scope

This policy is applicable to proposals for adding new ecosystem and integration test jobs/components
that involve building and testing additional artifacts which may cause [rust-lang/rust] PR CI or
Full CI (the "CI") to fail, or produce a failure message, that may impact other [rust-lang/rust]
contributors who also use the [rust-lang/rust] CI.

For example (non-exhaustive):

- Ecosystem test jobs: Rust for Linux or Fuchsia.
- Integration test components: GCC codegen backend or Cranelift codegen backend.

## Background

[rust-lang/rust] runs a small set of build/test jobs on PR CI (faster, less expensive/exhaustive),
and runs a much larger set of build/test jobs on Full CI. PR CI usually takes around an hour, while
Full CI usually takes around 3 hours. Having ecosystem and integration test jobs/components that:

- may spuriously fail; or
- may genuinely fail but it's not clear who should be consulted about the failure or who is
  responsible for fixing the failure; or
- otherwise do not have well documented test job maintainers and failure protocol;

can introduce a lot of friction and frustration for other contributors who now will have to deal
with the failure or understand why the test job failed in the first place, which may now be blocking
their PR. Everyone utilizing the [rust-lang/rust] CI is expected to use it responsibly.

To help with this, please follow the process described below if you would like to propose adding an
ecosystem/integration test job/component to the [rust-lang/rust] CI.

## Process for adding an ecosystem/integration test job/component to rust-lang/rust CI

![Flowchart of the process for adding an ecosystem/integration test job/component to rust-lang/rust
CI](./custom-test-jobs/test-job-flow.svg)

1. Ask the Infrastructure Team on zulip
  ([#t-infra](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra)) to check if there
  would be capacity for the proposed test job/component.
1. Propose using a [Major Change Proposal
   (MCP)](../proposals-and-stabilization.md#how-do-i-submit-an-mcp).
    - Including the filled out *Ecosystem and Integration Test Job/Component Policy* (see below).
1. Once the MCP is seconded and accepted, create a new issue on [rust-lang/rust] about the proposal,
   linking to the MCP, and then nominate the proposal for library team review via `@rustbot label
   +I-libs-nominated`.
    - Link to the MCP.
1. Once the library team reviews the proposal and has no blocking concerns, submit the
   implementation PR to [rust-lang/rust].
    - The PR should include an accompanying Ecosystem/Integration Test Job/Component support page in
      the in-tree [rustc-dev-guide], ensuring that the *Failure Protocol* is well-documented (see
      below). Link to the MCP and issue.
1. The Infrastructure Team will review the implementation PR. Once approved, the
   ecosystem/integration test job/component will run in [rust-lang/rust] PR CI or Full Merge CI.

## Ecosystem and Integration Test Job/Component Policy

*Please copy and fill out this template as part of the MCP.*

The ecosystem/integration test job/component ("test job/component") proposed for the
[rust-lang/rust] CI must:

- Be approved by the compiler team through a proposed MCP, where the MCP is seconded by a compiler
  team member, and the MCP is accepted with no blocking concerns.
- Have no blocking concerns from the library team.
- Have the implementation PR be reviewed and approved by the infrastructure team.
- Be properly documented on [rustc-dev-guide] (preferably as part of the implementation PR).

### Test job/component rationale

- What does this test job/component do?
    - If an ecosystem test job/component is being proposed, can you briefly describe the intended
  ecosystem users?
- What [rust-lang/rust] changes can potentially break the test job/component? E.g. changes to rustc,
  standard library, bootstrap or tools (like clippy/rustfmt/cargo).
- Why does this test job/component need to be part of the [rust-lang/rust] PR and/or Full Merge CI?
- If the test job/component will block on failure, why does it need to block?
- If the test job/component will not block on failure initially but is intended to eventually become
  blocking:
    - Why will it become blocking?
    - When will it become blocking?

### Test job/component maintainers

The proposed test job/component for [rust-lang/rust] CI must have at least one dedicated test
job/component maintainer. The test job/component maintainers understand that they will be pinged or
otherwise contacted about the custom test job/component, particularly for (but not limited to) its
failures.

Please list who will be maintaining this ecosystem/integration test job/component here.

**If an ecosystem/integration test job/component no longer has an active dedicated maintainer (or
maintainers), and if [rust-lang/rust] teams find the ecosystem/integration test job/component causes
significant burden or becomes irrelevant, then the ecosystem/integration test job/component may be
removed.**

### CI infrastructure considerations

You should ask the Infrastructure Team on the
[`#t-infra`](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra) zulip channel when
proposing a new ecosystem/integration test job/component to check if there's capacity for the test
job/component.

- Does the custom test job/component require substantial CI resources (storage and/or CI time)? In
  particular, will it require large runners?

### Features and implementation details

- Does the proposed test job/component intend to use any unstable features?
    - If so, are the unstable features ready for exposure (e.g. must an unstable feature be
      completely reworked)?
    - For ecosystem test jobs/components, are the unstable features ready for such exposure to the
      ecosystem, and are the feature stakeholders ready for such usage?
- Does the proposed test job/component intend to intentionally depend on any implementation details?
  This may include but is not limited to: unstable/internal compiler/tool flags and behaviors,
  `RUSTC_BOOTSTRAP` usages, standard library implementation details, etc.
    - If so, are there plans to shrink or expand such dependencies in the future?

### Failure protocol: what to do if the job/component breaks/fails?

- How can the test job/component maintainers be contacted in case of failure?
- If the addition of an ecosystem/integration test job is being proposed:
    - How can the test job be run in CI? If so, is there a try job (`try-job: ...`) invocation?
      What's the job name?
    - Can the test job be run locally? If so, how?
- If the addition of an ecosystem/integration test component is being proposed:
    - Which existing CI jobs will be building and testing this test component?
    - Can they be built and ran as part of a try job? If so, what are the job names and the try job
      (`try-job: ...`) invocations?
    - Can the test component be built and run locally? If so, how?
- How can the test job/component be disabled in the event of spurious failures that are blocking PR
  and/or Full Merge CI?
- If a PR breaks the test job/component:
    - If the breakage seems **spurious** and retrying does not resolve the spurious breakage, the
      test job may be **temporarily disabled** (see below).
    - If the breakage is **intentional**, how will this be resolved?
    - If the breakage is **unintentional**, is the PR author expected to fix the breakage?

**If the artifacts of an ecosystem/integration test job/component are not shipped as part of a
distribution component/toolchain, the test job/component may be temporarily disabled to unblock
[rust-lang/rust] PR CI or Full Merge CI without receiving prior approval from the test job/component
maintainers. The test job/component maintainers will be pinged or otherwise notified about the test
job/component being disabled.**

### Dependencies, build/test environments and reliability

- Does the test job/component involve any custom build systems that are not used in the regular
  [rust-lang/rust] CI jobs?
- Does the test job/component depend on external resources (e.g. external servers) that may be
  subject to network connectivity?
    - If so, does the infrastructure team need to help maintain a mirror of the required assets?
- Are there any potential sources of spurious failures due to the test job/component?
- Are there any other unusual requirements (build environment, dependencies, etc.)?


[rust-lang/rust]: https://github.com/rust-lang/rust
[rustc-dev-guide]: https://github.com/rust-lang/rustc-dev-guide
