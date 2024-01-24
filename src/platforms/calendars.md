# Calendars

Many Rust teams and working groups have regular meetings, and it can get
challenging quickly to manage all the calendar events.

That's why we have automation available for generating both one-time
and recurring calendar events. It can be found in the
[calendar](https://github.com/rust-lang/calendar) repository, which also
contains a guide for its usage.

You can use it to create and update calendar invites declaratively
using a TOML file, and the tool will then generate `.ics` files from them,
which can be imported into various calendar tools.
