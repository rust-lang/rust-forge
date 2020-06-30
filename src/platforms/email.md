# Email

While most of Rust's discussion happens on other platforms, email is eternal and
we occasionally need a way to approach individuals or groups privately. Our
email is hosted through Mailgun (provided by Mozilla). We create and edit the
mailing lists for teams through the [rust-lang/team] repository. Our email
domain is `rust-lang.org`, e.g. `ferris@rust-lang.org`.

## Sending a public broadcast
If your teams need to reach everyone in Rust organisation they can send a
email to `all@`. It is recommended that you only use this mailing list when you
know that you need contact every member, such as for organising a members event
like the All Hands, or security alerts.

### Keeping responses private
When sending a message to `all@`, do not put `all@` in `To`. This will mean that
any replies to your broadcast will also be sent to everyone. Instead put your
team's email address in `To` field, and place `all@` in the `Bcc` field. Then
any replies will be sent to just your team.


[rust-lang/team]: https://github.com/rust-lang/team
