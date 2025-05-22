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
CI](./ecosystem-integration-tests/test-job-flow.svg)

1. Ask the Infrastructure Team on zulip
  ([#t-infra](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra)) to check if there
  would be capacity for the proposed test job/component.
2. Propose using a [Major Change Proposal
   (MCP)](../proposals-and-stabilization.md#how-do-i-submit-an-mcp).
    - Including the filled out *Ecosystem and Integration Test Job/Component Policy* (see below).
3. Once the MCP is seconded and accepted, create a new issue on [rust-lang/rust] about the proposal,
   linking to the MCP, and then nominate the proposal for library team review via `@rustbot label
   +I-libs-nominated`.
    - Link to the MCP.
4. Once the library team reviews the proposal and has no blocking concerns, submit the
   implementation PR to [rust-lang/rust].
    - The PR should include an accompanying Ecosystem/Integration Test Job/Component support page in
      the in-tree [rustc-dev-guide], ensuring that the *Failure Protocol* is well-documented (see
      below). Link to the MCP and issue.
5. The Infrastructure Team will review the implementation PR. Once approved, the
   ecosystem/integration test job/component will run in [rust-lang/rust] PR CI or Full Merge CI.

## Ecosystem and Integration Test Job/Component Policy

Please copy and fill out this template (below the separator) as part of the MCP. The policy
questions/explanation themselves will be quoted blocks. Information that the MCP author is expected
to provide are indicated by italicized sentences whose content should be replaced.

NOTE: It is not the intention of this policy to make it frustrating annoying to add an ecosystem
test job/component. We simply wish to gather the necessary background information upfront
(especially working together to figure out a viable *Failure Protocol*) to minimize potential
frustrations from other [rust-lang/rust] contributors if and when the the test job/component do
fail, potentially blocking PR / Full Merge CI in completely unrelated PRs.

---

```markdown
## Ecosystem and Integration Test Job/Component Policy

The ecosystem/integration test job/component ("test job/component") proposed for the
[rust-lang/rust] CI must:

- Be approved by the compiler team through a proposed MCP, where the MCP is seconded by a compiler
  team member, and the MCP is accepted with no blocking concerns.
- Have no blocking concerns from the library team.
- Have the implementation PR be reviewed and approved by the infrastructure team.
- Be properly documented on [rustc-dev-guide] (preferably as part of the implementation PR).

Please complete the sections below so [rust-lang/rust] teams can have sufficient context about the 
proposed test job/component.

### Test job/component rationale

> What does this test job/component do?
>
> - If an ecosystem test job/component is being proposed, can you briefly describe the intended
>     ecosystem users?

*Please provide responses here, replacing this sentence.*

> What [rust-lang/rust] changes can potentially break the test job/component?
>
> E.g. changes to rustc, standard library, bootstrap or tools (like clippy/rustfmt/cargo).

*Please provide responses here, replacing this sentence.*

> Why does this test job/component need to be part of the [rust-lang/rust] PR and/or Full Merge CI?

*Please provide responses here, replacing this sentence.*

> If the test job/component will block on failure, why does it need to block?

*Please provide responses here, replacing this sentence.*

> If the test job/component will not block on failure initially but is intended to eventually become
> blocking:
>
> - Why will it become blocking?
> - When will it become blocking?

*Please provide responses here, replacing this sentence.*

### Test job/component maintainers

> The proposed test job/component for [rust-lang/rust] CI must have at least one dedicated test
> job/component maintainer. The test job/component maintainers understand that they will be pinged
> or otherwise contacted about the ecosystem/integration test job/component, particularly for (but 
> not limited to) its failures.
>
> **Please list who will be maintaining this ecosystem/integration test job/component here. Please
> format the github handles in the style:**
>
> ```
> [@github_handle_1](https://github.com/github_handle_1)
> [@github_handle_2](https://github.com/github_handle_2)
> ```
>
> NOTE: For future readers, you can paste the usernames without formatting them as links via
> **ctrl-shift-v**.

*Please list test job/component maintainers here with the formatting advice above, replacing this
sentence.*

> **NOTE: If an ecosystem/integration test job/component no longer has an active dedicated
> maintainer (or maintainers), and if [rust-lang/rust] teams find the ecosystem/integration test
> job/component causes significant burden or becomes irrelevant, then the ecosystem/integration test
> job/component may be removed.**

### CI infrastructure considerations

> You should ask the Infrastructure Team on the
> [`#t-infra`](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra) zulip channel when
> proposing a new ecosystem/integration test job/component to check if there's capacity for the test
> job/component.
>
> - Does the ecosystem/integration test job/component require substantial CI resources (storage and/
>   or CI time)? In particular, will it require large runners?

*Please provide responses here, replacing this sentence. If there is a zulip topic discussing it
with the Infrastructure Team, please include the zulip topic link here.*

### Features and implementation details

> Does the proposed test job/component intend to use any unstable features?
>
> - If so, are the unstable features ready for exposure (e.g. must an unstable feature be completely
>   reworked)?
> - For ecosystem test jobs/components, are the unstable features ready for such exposure to the
>   ecosystem, and are the feature stakeholders ready for such usage?

*Please provide responses here, replacing this sentence.*

> Does the proposed test job/component intend to intentionally depend on any implementation details?
> This may include but is not limited to: unstable/internal compiler/tool flags and behaviors,
> `RUSTC_BOOTSTRAP` usages, standard library implementation details, etc.
>
> - If so, are there plans to shrink or expand such dependencies in the future?

*Please provide responses here, replacing this sentence.*

### Failure protocol: what to do if the job/component breaks/fails?

> **NOTE: If the artifacts of an ecosystem/integration test job/component are not shipped as part of
> a distribution component/toolchain, the test job/component may be temporarily disabled to unblock
> [rust-lang/rust] PR CI or Full Merge CI without receiving prior approval from the test
> job/component maintainers. The test job/component maintainers will be pinged or otherwise notified
> about the test job/component being disabled.**

> How can the test job/component maintainers be contacted in case of failure? By default, it is
> assumed that the test job/component maintainer can be pinged via their GitHub handles.

*Please provide responses here, replacing this sentence.*

> (If applicable) If the addition of an ecosystem/integration test job is being proposed:
>
> - How can the test job be run in CI? If so, is there a try job (`try-job: ...`) invocation? What's
>   the job name?
> - Can the test job be run locally? If so, how?

*If applicable, please provide responses here, replacing this sentence. Otherwise, you can ignore
this question.*

> (If applicable) If the addition of an ecosystem/integration test component is being proposed:
>
> - Which existing CI jobs will be building and testing this test component?
> - Can they be built and ran as part of a try job? If so, what are the job names and the try job
>   (`try-job: ...`) invocations?
> - Can the test component be built and run locally? If so, how?

*If applicable, please provide responses here, replacing this sentence. Otherwise, you can ignore
this question.*

> How can the test job/component be disabled in the event of spurious failures that are blocking PR
> and/or Full Merge CI?

*Please provide responses here, replacing this sentence.*

> If a PR breaks the test job/component:
>
> - If the breakage seems **spurious** and retrying does not resolve the spurious breakage, the test
>   job may be **temporarily disabled** (see below).
> - If the breakage is **intentional**, how will this be resolved?
> - If the breakage is **unintentional**, is the PR author expected to fix the breakage?

*Please provide responses here, replacing this sentence.*

### Dependencies, build/test environments and reliability

> Does the test job/component involve any custom build systems that are not used in the regular
> [rust-lang/rust] CI jobs?

*Please provide responses here, replacing this sentence.*

> Does the test job/component depend on external resources (e.g. external servers) that may be
> subject to network connectivity?
>
> - If so, does the infrastructure team need to help maintain a mirror of the required assets?

*Please provide responses here, replacing this sentence.*

> Are there any potential sources of spurious failures due to the test job/component?

*Please provide responses here, replacing this sentence.*

> Are there any other unusual requirements (build environment, dependencies, etc.)?

*Please provide responses here, replacing this sentence.*


[rust-lang/rust]: https://github.com/rust-lang/rust
[rustc-dev-guide]: https://github.com/rust-lang/rustc-dev-guide
```

[rust-lang/rust]: https://github.com/rust-lang/rust
[rustc-dev-guide]: https://github.com/rust-lang/rustc-dev-guide
