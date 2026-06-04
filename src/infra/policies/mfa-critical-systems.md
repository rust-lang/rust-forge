# Multi-factor authentication in the Rust Project

The Rust infrastructure team adopts multi-factor authentication to secure access to different
systems, and in particular, it enforces stricter rules for services considered critical
Rust infrastructure.

## Multi-factor authentication and assurance levels

The Rust infrastructure team uses NIST's [Authentication Assurance Levels] to
score different MFA methods according to the security expectations they bring. Thus, we consider
as secure and approved MFA methods (in this order of preference): 

1. Hardware security keys compatible with FIDO2 / Webauthn (e.g. YubiKeys)  as AAL-3 
2. Hardware enabled with Webauthn passkeys (e.g. Apple TouchId) as AAL-2 
3. TOTP apps (e.g. Google Authenticator) as AAL-2

## MFA and critical infrastructure access

As a rule of thumb, when different MFA methods are supported by a service considered critical
infrastructure, Project members with *privileged* or *administrator* access **must** use the most
secure MFA method that the service provider supports. That means using hardware security keys whenever
possible, and if hardware keys are not an option, Passkeys or TOTP apps must be used otherwise.

Some of these services include:

- Google Workspace and GCP (`rust-lang.org`)
- AWS (through AWS SSO sessions)
- Azure
- Github
- Datadog
- Fastly
- Heroku
- 1password

The Rust Infrastructure team officially supports [Yubico YubiKeys Series-5] as AAL-3 tested and
approved devices. Project members may bring hardware keys from other vendors if they want, but
the Rust infrastructure team won't be able to offer support regarding bugs or compatibility issues.

In addition to that, when multiple secure MFA methods and devices are supported by a service, Project
members **should** configure at least one additional MFA method for redundancy purposes, as long as additional
MFA devices or methods are in the same AAL. For example, when setting up MFA for a `heroku` account, one may
configure additional YubiKey (AAL-3) for redundancy purposes, but **should not** configure `1password` as TOTP
(AAL-2) with the same intent, since this _could_ potentially decrease security, especially if TOTP the
backup is configured in a way that makes it reachable to attack vectors during admin operations

Finally, when a Project member with access to critical infrastructure loses access to a hardware device
used for MFA (e.g. a laptop was stolen or a YubiKey was lost), this must be disclosed with the Rust
Infrastructure team, and that device **must** be immediately revoked from all systems it was configured as
an allowed MFA device/method.

## Yubico Hardware Key grants

As part of the [Yubico Secure it Forward Program], The Rust Foundation will provide YubiKeys to Rust
Project members with critical infrastructure access. If you are eligible for such a grant and would
like to get the recommended YubiKeys for free, get in touch with the [T-infra team in Zulip].

The members of the following teams are eligible for this grant:

- `infra`
- `crates.io`
- `docs.rs`
- `release`
- `triagebot`
- `bors`

[Authentication Assurance Levels]: https://pages.nist.gov/800-63-3/sp800-63b.html#sec3
[Yubico YubiKeys Series-5]: https://www.yubico.com/products/yubikey-5-overview
[Yubico Secure it Forward Program]: https://www.yubico.com/why-yubico/secure-it-forward
[T-infra team in Zulip]: https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra
