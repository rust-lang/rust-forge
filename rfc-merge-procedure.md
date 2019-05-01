---
layout: default
title: RFC Merge Procedure
---

Once an RFC has been accepted (i.e., the final comment period is
complete, and no major issues were raised), it must be merged. Right now this is a manual process,
though just about anyone can do it (if you're not a subteam member,
though, you'll have to open a PR rather than merge the RFC
manually). Here is the complete set of steps to merge an RFC -- in
some cases, not all the steps will be applicable.

### Step 1: Open tracking issue

Open a tracking issue over on rust-lang/rust. Here is a
template for the issue text. You'll have to adjust the various places
labeled XXX with some suitable content (e.g., the name of the RFC, or
the most appropriate team).

```markdown
This is a tracking issue for the RFC "XXX" (rust-lang/rfcs#NNN).

**Steps:**

- [ ] Implement the RFC (cc @rust-lang/XXX -- can anyone write up mentoring instructions?)
- [ ] Adjust documentation ([see instructions on rustc-guide][doc-guide])
- [ ] Stabilization PR ([see instructions on rustc-guide][stabilization-guide])

[stabilization-guide]: https://rust-lang.github.io/rustc-guide/stabilization_guide.html#documentation-prs
[doc-guide]: https://rust-lang.github.io/rustc-guide/stabilization_guide.html#stabilization-pr

**Unresolved questions:**

XXX --- list all the "unresolved questions" found in the RFC
        to ensure they are not forgotten
```   

Add the following labels to the issue:

- `B-rfc-approved`
- the approriate `T-XXX` label

(If you don't have permissions to do so, leave a note cc'ing the
appropriate team and asking them to do so.)

### Step 2: Merge the RFC PR itself

In your local git checkout:

- Merge the RFC PR into master in your fork
- Add a commit that moves the file name from 0000- to its RFC number
- Edit the new file to include links to the RFC PR and the tracking issue you just created
  in the header
- Open a PR or push directly to the master branch on rust-lang/rfcs, as appropriate
 
### Step 3: Leave a comment

Leave a final comment on the PR directing everyone to the tracking
issue. Something like this, but feel free to add your own personal
flavor (and change the team):

```markdown
**Huzzah!** The @rust-lang/lang team has decided **to accept** this RFC. 

To track further discussion, subscribe to the tracking issue here: rust-lang/rust#41517
```

### Step 4: Update the rendered link

Update the rendered link in the first post of the PR to the permanent home under
`https://github.com/rust-lang/rfcs/blob/master/text/`.

(This way future visitors can open it easily after the PR branch is deleted.)

### That's it, you're done!

