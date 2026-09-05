# CI runners on AWS

We manage self-hosted GitHub actions runners on AWS compute today. This page
describes the moving pieces we've built at a high level to help someone
navigate where to go if something is broken.

## Bors

Bors is responsible for listening for GitHub-sent webhooks and spinning up EC2
instances on which to run the jobs. It will also spin up instances if it
detects a job is pending. It also handles last-resort termination if an
instance doesn't self-terminate after hitting a systemd timeout (e.g., if the
instance OOMs the wrong thing this may not happen).

Bors is configured via `rust-bors.toml` in each repository (currently
rust-lang/rust and rust-lang/bors-kindergarten).

## GitHub configuration

We have configured two runner groups at the organization level. bors issues a
one time use "JIT" token via its GitHub app associated with these groups, which
is passed into the launched instance. The GitHub-owned runner code uses this
token to authenticate and authorize access to executing jobs. These groups
currently allow executing from a subset of workflows and branches in their
respective repositories, but note that PRs targeting those branches will be
allowed to run (at the GitHub layer).

Note the settings URLs are not accessible to non-infra-admins today:

* [rust-lang/rust runner group](https://github.com/organizations/rust-lang/settings/actions/runner-groups/8)
  * <https://github.com/rust-lang/rust/blob/546f07f6ee18fa92e586e659ef888bbf0f99e6e7/rust-bors.toml#L82>
* [rust-lang/bors-kindergarten runner group](https://github.com/organizations/rust-lang/settings/actions/runner-groups/11)
  * <https://github.com/rust-lang/bors-kindergarten/blob/583ebb666a34a2472bfa13d1c0378cd6bd577eee/rust-bors.toml#L6>

## AMIs

Each repository owns configuring both the instance type and the AMI (disk
image) to run on the instance via `rust-bors.toml`. The AMI is indirectly
specified as an SSM parameter name. Those SSM parameters currently correspond
to AMIs we build ourselves.

We build AMIs daily from a AWS-cron-job, invoking a [Lambda function](https://github.com/rust-lang/simpleinfra/tree/master/terragrunt/modules/ci-runners/lambda).

The Lambda function takes as input a JSON blob with a single key (`arch`),
either `aarch64` or `x86_64` today. It performs the following tasks (in this
order):

1. Downloads a packer release, verifying the hash against a hard-coded checksum
1. Checks if the most recent AMI is more than 2 days old. If yes, we post to Zulip (#t-infra/private > ci-runner-build-notifications).
   * This is a simple alerting mechanism that lets us know if the builds stop working.
1. Calls GitHub API (unauthenticated) to get latest actions/runner release version and download URL.
1. Invokes packer to build according to the contained [template], specifying the architecture and GitHub URL.
1. Once build is complete, we persist new AMI ID to an SSM parameter based on
   the architecture we built. This will be used immediately on next EC2 runner
   start in the relevant repository (ci-staging - bors-kindergarten, ci-prod -
   rust-lang/rust).
   * Note that there's no validation today that the new AMI "works", and each
     account builds AMIs entirely separately. So far we haven't hit issues with
     this approach.
   * If CI breaks after an AMI release, rolling back the SSM version by
     specifying the previous AMI ID (look it up in the EC2 console) should be
     an easy hot fix.
1. After a successful build we additionally delete AMIs >14 days old.

Pre-building AMIs is done so that we:

* Have fast instance start-up (dependencies are all installed, we start executing within <1 minute after GitHub creates the job)
* Can customize instance configuration, including boot-time configuration
  * For example, we tune the mount options for `/` for (theoretically) faster builds
* In theory, easy-ish ability to rollback an update to the base image. In
  practice, this still requires infra intervention so it's not perfect.
* Give a convenient hook point to fetch latest runner installation.
  * See [docs](https://docs.github.com/en/actions/reference/runners/self-hosted-runners#runner-software-updates-on-self-hosted-runners).
    If we don't do this we'd start seeing self-updates and/or breakage roughly
    30 days after a new release, which isn't ideal.

[template]: https://github.com/rust-lang/simpleinfra/blob/master/terragrunt/modules/ci-runners/lambda/ubuntu.pkr.hcl

## Security

### Hostile code within the runner

We aim to consider runners 'fully untrusted'. We have these protections in place:

1. EC2 instances only execute one GitHub Actions run/job. They get access via a
   JIT token, which GitHub promises can only be used once. Caveat: today there
   is only weak enforcement *which* job is executed on the instance. In the
   happy path, the JIT token embeds the GHA run + workflow job ID and we
   configure CI to execute on that particular label. But there's no enforcement
   available in the GitHub control plane to uniquely bind a JIT token to a
   particular workflow. There's no real risk from executing on the wrong
   instance though.
1. The EC2 instance is launched with no associated IAM role (enforced by IAM
   permissions -- bors can't associate a role even if it wanted to). The only
   secret present on the instance at startup is the JIT token, but it is used
   up by calling GitHub before any code on the instance starts running.
1. Instances currently have zero inbound network access (enforced by EC2 security
   group), and outbound access to port 80 and 443 anywhere on the internet.
   Restricting outbound access to a limited set of destinations would be ideal,
   but is complicated. Today GitHub requires SNI/DNS allowlisting rather than IP
   filtering, see [docs](https://docs.github.com/en/actions/reference/runners/self-hosted-runners#communication).

Note that today, most of these runners end up accessing secrets from GitHub. So
while the AWS environment should generally be safe from compromised code in the
runners, that code can still do bad things.

### Hostile actor in the account

We have a weaker goal of defending against access into the runners while
they're running. The ci-prod account has Admin (infra-admins) + ReadOnly
(infra) [access](https://github.com/rust-lang/simpleinfra/blob/00a678c6ea843d87e345a9b896982bc0d1ef9d39/terragrunt/modules/aws-organization/groups.tf#L439)
today. The production hosts intentionally don't have remote access into them
(e.g., ssh) and have a limited install of Ubuntu (minimal base image, installed
just what is needed to run GHA) to limit risk.

One known threat here is JIT tokens being exposed via EC2 User Data. These are
only useful for a short period of time between instance launch and the usage of
the token to GitHub. Hiding this token from users in the AWS account is
complicated. One possible solution is encrypting under a KMS key with a grant
issued to the EC2 instance's security credentials (which are always available).
This could also be used to prove that the instance has booted from an expected
prebuilt AMI (e.g., via [TPM attestation]). For now we think mitigating this is
not worth the added complexity.

[TPM attestation]: https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/nitrotpm-attestation.html

## Policy compliance

Our [external runner policy] is not fully met by these runners. Since we
control the compute, we are on the hook for (minimal) maintenance needed.

Security compliance is partially met:

* Access controls: CloudTrail provides a record of any attempts to access the compute.
* Ephemeral builds: fully ephemeral compute, one build/instance.
* Endpoint protection: none currently, but the instances are rebuilt daily from latest Ubuntu AMIs.

[external runner policy]: https://github.com/rust-lang/infra-team/blob/main/service-catalog/rust-ci/external-runners/README.md#security-requirements-optional

## Why not CodeBuild?

We moved away from CodeBuild to speed up (and reduce costs) for CI. See [evaluation issue](https://github.com/rust-lang/simpleinfra/issues/1132) for additional context.
