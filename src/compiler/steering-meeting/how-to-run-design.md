# How to run the design meeting

## Week of the meeting

* Announce the meeting in the triage meeting
* Skim over the list of proposals and ping people who have open
  proposals to get their availability over the next few weeks
* Make sure that a write-up is available and nag the meeting person otherwise

## Day of the meeting

* Create a `design meeting YYYY.MM.DD` topic
    * Ping `@t-compiler/meeting`, ideally 1h or so before the meeting actually starts,
      to remind people
    * Include a link to the design meeting write-up
* At the time of the meeting, return to the topic
    * Ping `@t-compiler/meeting` to let people know the meeting is starting
    * Include a link to the design meeting write-up
* We typically begin with a 5min announcement period

To guide the meeting, create a shared hackmd document everyone can
view (or adapt an existing one, if there is a write-up). Use this to
help structure the meeting, document consensus, and take live
notes. Try to ensure that the meeting ends with sort of consensus
statement, even if that consensus is just "here are the problems, here
is a space of solutions and their pros/cons, but we don't have
consensus on which solution to take".

## After the meeting

* Post the final contents of the summary hackmd as minutes to the
  [`minutes/design-meeting` directory in the compiler-team
  repository][ct]
* (Optional) Create an Inside Rust blog post pointing people at the
  minutes and maybe giving a few notes

[ct]: https://github.com/rust-lang/compiler-team/tree/master/content/minutes/design-meeting

