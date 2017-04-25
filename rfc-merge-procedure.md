---
layout: default
title: RFC Merge Procedure
---

Once an RFC has been accepted (i.e., the final comment period is
complete), it must be merged. Right now this is a manual process, and
can only be done by a subteam member with push rights (though others
could open a PR to do it, it would just have to be merged by a subteam
member). Here is the complete set of steps to merge an RFC -- in some
cases, not all the steps will be applicable.

### Step 1: Open tracking issue

Open a tracking issue over on rust-lang/rust. Here is a
template for the issue text. You'll have to adjust the various places
labeled XXX with some suitable content (e.g., the name of the RFC, or
the most appropriate team).

```
This is a tracking issue for the RFC "XXX" (rust-lang/rfcs#NNN).

**Steps:**

- [ ] Implement the RFC (cc @rust-lang/XXX -- can anyone write up mentoring instructions?)
- [ ] Adjust documentation ([see instructions on forge][doc-guide])
- [ ] Stabilization PR ([see instructions on forge][stabilization-guide])

[stabilization-guide]: https://forge.rust-lang.org/stabilization-guide.html
[doc-guide]: https://forge.rust-lang.org/stabilization-guide.html#updating-documentation

**Unresolved questions:**

XXX --- list all the "unresolved questions" found in the RFC
        to ensure they are not forgotten
```   

Add the following labels to the issue:

- `B-rfc-approved`
- `B-unstable`
- the approriate `T-XXX` label

### Step 2: Merge the RFC PR itself

In your local git checkout:

- Merge the RFC PR into master, locally
- Add a commit that moves the file name from 0000- to its RFC number
- Edit the new file to include links to the RFC PR and the tracking issue you just created
  in the header
- Push directly to the master branch  
  
### Step 3: Leave a comment

Leave a final comment on the PR directing everyone to the tracking
issue. Something like this, but feel free to add your own personal
flavor (and change the team):

```
**Huzzah!** The @rust-lang/lang team has decided **to accept** this RFC. 

To track further discussion, subscribe to the tracking issue here: rust-lang/rust#41517
```

### That's it, you're done!

