# Triage meeting procedure

This page documents how to run a lang team triage meeting,
should you have the misfortune of being forced to do so.

## Attending a meeting

If you would just like to **attend** a lang-team triage meeting, all
you have to do is join the zoom call (the URL is attached to the
calendar invite below).

## Scheduling

Note that the scheduling for all meetings is recorded in the [team
calendar], links to which can be found on the [rust-lang/lang-team]
repository.

[team calendar]: https://github.com/rust-lang/lang-team/#meeting-calendar
[rust-lang/lang-team]: https://github.com/rust-lang/lang-team/

## Pre-triage

To start, we have a pre-triage meeting which occurs before the main
meeting. This is not recorded. It is boring.

To execute this meeting you:

* Open the [Current Meeting] dropbox paper document
* Skim down the action items and look to see if there are any you know have been handled
    * they can be checked off and removed
* Skip down to the Triage section
* For each Triage section, click on the link and populate it with what you find
    * typically it is best to copy-and-paste the title of the issue, so that links remain intact
* For each item, click in and try to add a few notes as to the main topic
    * look for things where there isn't much discussion needed, or just reminders
    * these can be handled quickly in the meeting, or perhaps not at all
    * items that require more discussion will need time alotted for them

[Current Meeting]: https://paper.dropbox.com/doc/T-Lang-Meeting-Current-meeting--AmyXFNnryXTNzBsSWjbdJcVSAg-nRfrSxCbfeo9q7fEYogZQ

## Main meeting

* Ping the team on discord `@lang-team`
* Begin the recording on Zoom, if you have acccess 
    * If nobody has access to the recording, oh well, we don't do it every week
* Discuss item by item and take some notes on what was said
    * Add specific actions to the action items section above
    * If a consensus arises, make sure to create an action item to document it!
    * The goal should be that we leave some comment on every issue
    
## After meeting

* Export the meeting file to markdown
    * you will need to cleanup "check boxes" -- I usually search and replace 
      `^(\s*)[ ]` with `\1* [ ]` or something like that to insert a
      `*` before them, which makes them valid markdown
* Upload video to youtube if applicable and get the URL
* Add the file to the [minutes] directory of [rust-lang/lang-team] repository
  with a file name like `YYYY-MM-DD.md`

[minutes]: https://github.com/rust-lang/lang-team/tree/master/minutes
