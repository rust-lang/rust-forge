# Friends of the Tree

[The Rust Team](http://www.rust-lang.org/team.html) likes to occasionally
recognize people who have made outstanding contributions to The Rust Project,
its ecosystem, and its community. These people are 'Friends of the Tree',
archived here for eternal glory.

## 2016-02-26 @mitaa

This week we would like to nominate @mitaa as Friend of the Tree. Recently @mitaa
[has][rd1] [sent][rd2] [a][rd3] [wave][rd4] [of][rd5] [fixes][rd6] [to][rd7]
[rustdoc][rd8] (yes those are all separate links) with even more on the way!
Rustdoc has historically been a tool in need of some love, and the extra help in
fixing bugs is especially appreciated. Thanks @mitaa!

[rd1]: https://github.com/rust-lang/rust/pull/31837
[rd2]: https://github.com/rust-lang/rust/pull/31835
[rd3]: https://github.com/rust-lang/rust/pull/31839
[rd4]: https://github.com/rust-lang/rust/pull/31715
[rd5]: https://github.com/rust-lang/rust/pull/31626
[rd6]: https://github.com/rust-lang/rust/pull/31614
[rd7]: https://github.com/rust-lang/rust/pull/31602
[rd8]: https://github.com/rust-lang/rust/pull/31596

## 2016-02-12 Jeffrey Seyfried (@jseyfried)

**This week's friend of the tree is Jeffrey Seyfried (@jseyfried)!**

Jeffrey Seyfried (@jseyfried) has made some awesome contributions to name
resolution. He has fixed a ton of bugs, reported previously unknown edge cases,
and done some big refactorings, all of which have helped improve a complex and
somewhat neglected part of the compiler.

## 2015-12-04 Vadim Petrochenkov @petrochenkov

This week we'd like to nominate @petrochenkov for Friend of the Tree. Vadim has
been doing some absolutely amazing compiler work recently such as [fixing
privacy bugs][bug1], [fixing hygiene bugs][bug2], [fixing pattern bugs][bug3],
[paving the way][deprecated1] and [implementing `#[deprecated]`][deprecated2],
[fixing and closing many privacy holes][privacy1], [refactoring][hir1]
[and][hir2] [improving][hir3] the HIR, and reviving [the old type ascription
PR][ascribe]. The list of outstanding bugs and projects in the compiler is
growing ever smaller now; thanks @petrochenkov!

[bug1]: https://github.com/rust-lang/rust/pull/29726
[bug2]: https://github.com/rust-lang/rust/pull/29748
[bug3]: https://github.com/rust-lang/rust/pull/29383
[deprecated1]: https://github.com/rust-lang/rust/pull/29952
[deprecated2]: https://github.com/rust-lang/rust/pull/30206
[privacy1]: https://github.com/rust-lang/rust/pull/29973
[hir1]: https://github.com/rust-lang/rust/pull/30087
[hir2]: https://github.com/rust-lang/rust/pull/30095
[hir3]: https://github.com/rust-lang/rust/pull/30145
[ascribe]: https://github.com/rust-lang/rust/pull/30184

## 2015-11-16 Peter Atashian (WindowsBunny, retep998)

In his own words, WindowsBunny is "a hopping encyclopedia of all the issues
windows users might run into and how to solve them." One of the heroes that make
Rust work on Windows, he actively pushes the frontiers of what Rust can do on
the platform. He is also notably the maintainer of the
[winapi](https://crates.io/crates/winapi) family of crates, a comprehensive set
of bindings to the Windows system APIs. You da bunny, WindowsBunny. Also, a
friend of the tree.

[Source](https://internals.rust-lang.org/t/subteam-reports-2015-11-16/2910).

## 2015-10-31 Marcus Klaas

Today @nrc would like to nominated @marcusklaas as Friend of the Tree:

Marcus is one of the primary authors of
[rustfmt](https://github.com/nrc/rustfmt). He has been involved since the early
days and is now the
[top contributor](https://github.com/nrc/rustfmt/graphs/contributors). He has
fixed innumerable bugs, implemented new features, reviewed a tonne of PRs, and
contributed to the design of the project. Rustfmt would not be the software it
is today without his hard work; he is indeed a Friend Of The Tree.

## 2015-10-16 Ryan Prichard

nmatsakis would also like to declare Ryan Prichard a **Friend of the Tree**.
Over the last few months, Ryan has been comparing the Rust compiler's parsing
behavior with that of the rust-grammar project, which aims to create a LALR(1)
grammar for parsing Rust. Ryan has found a number of inconsistencies and bugs
between the two. This kind of work is useful for two reasons: it finds bugs,
obviously, which are often hard to uncover any other way. Second, it helps pave
the way for a true Rust reference grammar outside of the compiler source itself.
So Ryan Prichard, thanks!

## 2015-10-02 Vikrant Chaudhary

[Vikrant Chaudhary (nasa42)](https://github.com/nasa42) is an individual who
believes in the Rust community. Since June he has been contributing to
[This Week in Rust](http://this-week-in-rust.org), coordinating its publication
on
[urlo](https://users.rust-lang.org/t/this-week-in-rust-editors-thread/1806/51),
and stirring up contributions. He recently rolled out an overhaul to the site's
design that brings it more inline with the main website. Today Vikrant is the
main editor on the weekly newsletter, assisted by
[llogiq](http://github.com/llogiq) and other contributors. Thanks for keeping
TWiR running, Vikrant, you friend of the tree.

[Source](https://internals.rust-lang.org/t/subteam-reports-2015-10-02/2716).

## 2015-07-24 Tshepang Lekhonkhobe

[@Gankro](http://github.com/Gankro) has nominated
[@tshepang](http://github.com/tshepang) for Friend of the Tree this week:

Over the last year Tshepang has landed over 100 improvements to our
documentation. Tshepang saw where documentation was not, and said "No. This will
not do."

We should all endeavor to care about docs as much as Tshepang.

[Source](https://internals.rust-lang.org/t/subteam-reports-2015-07-24/2397).

## 2015-05-19 Chris Morgan

I'd like to nominate Chris Morgan (@chris-morgan) for Friend of the Tree today.
Chris recently redesigned the play.rust-lang.org site for the 1.0 release,
giving the site a more modern and rustic feel to it. Chris has been contributing
to Rust for quite some time now, his first contribution dating back to July 2013
and also being one of the early pioneers in the space of HTTP libraries written
in Rust. Chris truly is a friend of the tree!

## 2015-03-24 Andrew Gallant (BurntSushi)

[BurntSushi] is an individual who practically needs no introduction. He's
written many of the world's most popular crates, including [docopt.rs], [regex],
[quickcheck], [cbor], and [byteorder]. Don't forget his CSV swiss-army-knife,
[xsv], built on [rust-csv]. Feedback from his early work on libraries helped
informed the evolution of Rust during a critical time in its development, and
BurntSushi continues to churn out the kind of Rust gems that can only come from
someone who is a skilled friendofthetree.

[burntsushi]: https://github.com/burntsushi
[docopt.rs]: https://github.com/docopt/docopt.rs
[regex]: https://github.com/rust-lang/regex
[quickcheck]: https://github.com/BurntSushi/quickcheck
[cbor]: https://github.com/BurntSushi/rust-cbor
[xsv]: https://github.com/BurntSushi/xsv
[rust-csv]: https://github.com/BurntSushi/rust-csv
[byteorder]: https://github.com/BurntSushi/byteorder

## 2015-03-03 Manish Goregaokar (Manishearth)

Manish started working on Servo as part of the GSoC program in 2014, where he
implemented XMLHttpRequest. Since then he's become in integral part of the Servo
team while finishing his university studies and organizing Rust community
events. In 2015 he took an interest in bors' queue and started making rollup PRs
to accelerate the integration process. Nursing the PR queue is the kind of
time-consuming labor that creates friends of the tree like Manish, the rollup
friend of the tree.

## 2015-02-17 Toby Scrace

Today I would like to nominate Toby Scrace as Friend of the Tree. Toby emailed
me over the weekend about a login vulnerability on crates.io where you could log
in to whomever the previously logged in user was regardless of whether the
GitHub authentication was successful or not. I very much appreciate Toby
emailing me privately ahead of time, and I definitely feel that Toby has earned
becoming Friend of the Tree.

## 2015-02-10 Jonathan Reem (reem)

Jonathan Reem has been making an impact on Rust since May 2014. His primary
contribution has been as the main author of the prominent [Iron][iron] web
framework, though he has also created several other popular projects including
the testing framework [stainless]. His practical experience with these projects
has led to several improvements in upstream rust, most notably his complete
rewrite of the `TaskPool` type. Reem is doing everything he can to advance the
Rust cause.

[iron]: https://github.com/iron/iron
[stainless]: https://github.com/reem/stainless

## 2015-01-20 Barosl Lee (barosl)

Today I would like to nominate Barosl Lee (@barosl) for Friend of the Tree.
Barosl has recently rewritten our bors cron job in a new project called [homu].
Homu has a number of benefits including:

- Zero "down time" between testing different PRs (compared to 30+ minutes for
  bors!)
- A new rollup button to create separate rollup PRs from other PRs.
- Multiple repositories are supported (Cargo and Rust are on the same page)

Homu was recently deployed for rust-lang/rust thanks to a number of issues being
closed out by Barosl, and it's been working fantastically so far! Barosl has
also been super responsive to any new issues cropping up. Barosl truly is a
Friend of the Tree!

[homu]: https://github.com/barosl/homu

## 2015-01-13 Kang Seonghoon (lifthrasiir, Yurume)

Seonghoon has been an active member of the Rust community since early 2013, and
although he has made a number of valuable contributions to Rust itself, his
greatest work has been in developing key libraries out of tree. [rust-encoding],
one of the most popular crates in Cargo, performs character encoding, and
[rust-chrono] date / time handling, both of which fill critical holes in the
functionality of the standard library. [rust-strconv] is a prototype of
efficient numerical string conversions that is a candidate for future inclusion
in the standard library. He maintains a [blog][rustlog] where he discusses his
work.

[rust-encoding]: https://github.com/lifthrasiir/rust-encoding
[rust-strconv]: https://github.com/lifthrasiir/rust-strconv
[rust-chrono]: https://github.com/lifthrasiir/rust-chrono
[rustlog]: https://lifthrasiir.github.io/rustlog/

## 2015-01-06 Jorge Aparicio (japaric)

I nominate Jorge Aparicio (japaric) for Friend of the Tree (for the second time,
no less!). japaric has done tremendous work porting the codebase to use the new
language features that are now available. First, he converted APIs in the
standard library to take full advantage of DST after it landed. Next, he
converted APIs to use unboxed closures. Then, he converted a large portion of
the libraries to use associated types. Finally, he removed boxed closures from
the compiler entirely. He has also worked to roll out RFCs changing the
overloaded operators and comparison traits, including both their definitions and
their impact on the standard library. And this list excludes a number of smaller
changes, like deprecating older syntax. The alpha release would not be where it
is without him; Japaric is simply one of the best friends the tree has ever had.

## 2014-12-30 Kevin Ballard (kballard, Eridius)

This is a belated recognition of Kevin Ballard (aka @kballard, aka Eridius) as a
friend of the tree. Kevin put a lot of work into Unicode issues in Rust,
especially as related to platform-specific constraints. He wrote the current
path module in part to accommodate these constraints, and participated in the
recent redesign of the module. He has also been a dedicated and watchful
reviewer. Thanks, Kevin, for your contributions!

## 2014-12-16 Gábor Lehel (glaebhoerl)

Gabor's major contributions to Rust have been in the area of language design. In
the last year he has produced a number of very high quality RFCs, and though
many of them of not yet been accepted, his ideas are often thought-provoking and
have had a strong influence on the direction of the language. His [trait based
exception handling RFC][tbeh] was particularly innovative, as well that [for
future-proofing checked arithmetic][checked]. Gabor is an exceedingly clever
Friend of the Tree.

[tbeh]: https://github.com/rust-lang/rfcs/pull/243
[checked]: https://github.com/rust-lang/rfcs/pull/146

## 2014-11-11 Brian Koropoff (unwound)

In the last few weeks, he has fixed many, many tricky ICEs all over the
compiler, but particularly in the area of unboxed closures and the borrow
checker. He has also completely rewritten how unboxed closures interact with
monomorphization and had a huge impact on making them usable. Brian Koropoff is
truly a Friend of the Tree.

## 2014-10-07 Alexis Beingessner (Gankro)

Alexis Beingessner (aka @Gankro) began contributing to Rust in July, and has
already had a major impact on several library-related areas. His main focus has
been collections. He completely rewrote BTree, providing a vastly more complete
and efficient implementation. He proposed and implemented the new Entry API.
He's written extensive new documentation for the collections crate. He pitched
in on collections reform.

And he added collapse-all to rustdoc!

Alexis is, without a doubt, a FOTT.

## 2014-09-02 Jorge Aparicio (japaric)

Jorge has made several high-impact contributions to the wider Rust community. He
is the primary author of rustbyexample.com, and last week published "eulermark",
a comparison of language performance on project Euler problems, which happily
showed Rust performing quite well. As part of his benchmarking work he has
ported the 'criterion' benchmarking framework to Rust.

## 2014-07-29 Björn Steinbrink (dotdash, doener)

Contributing since April 2013. Björn has done many optimizations for Rust,
including removing allocation bloat in iterators, fmt, and managed boxes;
optimizing `fail!`; adding strategic inlining in the libraries; speeding up data
structures in the compiler; eliminating quadratic blowup in translation, and
other IR bloat problems.

He's really done an amazing number of optimizations to Rust.

Most recently he earned huge kudos by teaching LLVM about the lifetime of
variables, allowing Rust to make much more efficient use of the stack.

Björn is a total FOTT.

## 2014-07-22 Jonas Hietala (treeman)

Jonas Hietala, aka @treeman, has been contributing a large amount of
documentation examples recently for modules such as hashmap, treemap,
priority_queue, collections, bigint, and vec. He has also additionally been
fixing UI bugs in the compiler such as those related to format!

Jonas continues to add new examples/documentation every day, making
documentation more approachable and understandable for all newcomers. Jonas
truly is a friend of the tree!

## 2014-07-08 Sven Nilson (bvssvni, long_void)

Sven Nilson has done a great deal of work to build up the Rust crate ecosystem,
starting with the well-regarded rust-empty project that provides boilerplate
build infrastructure and - crucially - integrates well with other tools like
Cargo.

His Piston project is one of the most promising Rust projects, and its one that
integrates a number of crates, stressing Rust's tooling at just the right time:
when we need to start learning how to support large-scale external projects.

Sven is a friend of the tree.

## 2014-06-24 Jakub Wieczorek (jakub-)

jakub-, otherwise known as Jakub Wieczorek, has recently been working very hard
to improve and fix lots of match-related functionality, a place where very few
dare to venture! Most of this code appears to be untouched for quite some time
now, and it's receiving some well-deserved love now.

Jakub has fixed 10 bugs this month alone, many of which have been long-standing
problems in the compiler. He has also been very responsive in fixing bugs as
well as triaging issues that come up from fun match assertions.

Jakub truly is a friend of the tree!

## 2014-04-22 klutzy

klutzy has been doing an amazing amount of Windows work for years now. He picks
up issues that affect our quality on Windows and picks them off 1 by 1. It's
tedious and doesn't get a ton of thanks, but is hugely appreciated by us. As
part of the Korean community, he has also done a lot of work for the local
community there. He is a friend of the tree. Thank you!

- Rust on Windows crusader
- Fixed issues with x86 C ABI struct arguments
- Fixed multiple issues with non-US locales

## 2014-03-18 Clark Gaebel (cgaebel)

This week's friend of the tree is Clark Gaebel. He just landed a huge first
contribution to Rust. He dove in and made our hashmaps significantly faster by
implementing Robin Hood hashing. He is an excellent friend of the tree.

## 2014-02-25 Erick Tryzelaar (erickt)

- Contributing since May 2011
- Wrote the serialization crate
- Organizes the bay area Rust meetups
- Just rewrote the Hash trait

## 2014-02-11 Flavio Percoco (FlaPer87)

- Contributing since September
- Does issue triage
- Organizing community events in Italy
- Optimized the 'pow' function
- Recently been fixing lots of small but important bugs

## 2014-01-27 - Jeff Olson (olsonjefferey)

- Contributing since February 2012
- Did the original libuv integration
- Implemented our second attempt at I/O, first using libuv
- Ported parts of the C++ runtime to Rust
- Implemented file I/O for the newest runtime
- Last week published an article about file I/O on the Safari books blog

## 2014-01-21 - Steven Fackler (sfackler)

- Contributing since last May
- CMU grad
- Lots of library improvements, Base64, Bitv, I/O
- Rustdoc improvements
- Mut/RefCell
- std::io::util
- external module loading

## 2014-01-14 - Eduard Burtescu (eddyb)

- Contributing since October
- Working on the compiler, including trans
- Reduced rustc memory usage
- Optimized vector operations
- Helping refactor the compiler to eliminate use of deprecated features
- Cleaned up ancient code in the compiler
- Removed our long-standing incorrect use of the environment argument to pass
  the self param

## 2014-01-07 - Vadim Chugunov (vadimcn)

- Contributing since June
- Fixed numerous bugs on Windows
- Fixing broken tests
- Improved compatibility with newer mingw versions
- Eliminated our runtime C++ dependency by implementing unwinding through
  libunwind
