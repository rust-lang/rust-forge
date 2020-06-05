# How to run the planning meeting

## Week of the meeting

* Announce the meeting in the triage meeting
* Skim over the list of proposals and ping people who have open
  proposals to get their availability over the next few weeks

## Day of the meeting

* Create a `design meeting YYYY.MM.DD` topic
    * Ping `@t-compiler/meeting`, ideally 1h or so before the meeting actually starts,
      to remind people
* At the time of the meeting, return to the topic
    * Ping `@t-compiler/meeting` to let people know the meeting is starting
* We typically begin with a 5min announcement period
* Visit the [compiler-team] repository to get a list of proposed meetings

To actually make the final selection, we recommend

* First, try to identify topics that are clear non-candidates
    * for example, sometimes more investigative work (e.g., data gathering) is needed
    * try to identify people to do those tasks
    * other issues may be out of date, or clear non-starters, and they can be closed
* Next tackle technical design meetings, then non-technical
    * Typical ratio is 2 technical, 1 non-technical, but this is not set in stone
    * It's ok to have fewer than 3 meetings

[compiler-team]: XXX

## Announce the meetings

For each scheduled meeting, create a calendar event:

* invite key participants to the meeting
* set the location to `#t-compiler, Zulip`
* include a link to the design meeting issue in the event

In the relevant issues, add the `meeting-scheduled` label and add a
message like:

```
In today's [planning meeting], we decided to schedule this meeting for **DATE**.

[Calendar event]

[planning meeting]: XXX link to Zulip topic
[Calendar event]: XXX link to calendar event
```

You can get the link to the calendar event by clicking on the event in
google calendar and selecting "publish".

## Publish a blog post

Add a blog post to the Inside Rust blog using the [template found on
the compiler-team repository][blog-template].

[blog-template]: https://github.com/rust-lang/compiler-team/blob/master/templates/planning-meeting-blog-post.md
