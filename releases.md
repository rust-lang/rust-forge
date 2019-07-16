---
layout: default
title: Rust release history &middot; The Rust Forge
---

# Rust release history

This is an archive of all the existing Rust release artifacts. Each release is
signed with the Rust [GPG signing key][key] ([older key][pre-0.8-key], [even
older key][pre-0.5-key]).

[key]: https://www.rust-lang.org/rust-key.gpg.ascii
[pre-0.8-key]: https://www.rust-lang.org/rust-key-old.gpg.ascii
[pre-0.5-key]: https://www.rust-lang.org/rust-key-very-old.gpg.ascii

TODO: Update me

## 1.7.0

- [Announcement][1.7.0-announce]
- [Release notes][1.7.0-notes]
- [Source code][1.7.0-tar] ([signature][1.7.0-tar-sig])
- [Windows x86_64 .exe gnu installer][1.7.0-windows-x64]
  ([signature][1.7.0-windows-x64-sig])
- [Windows x86_64 .msi gnu installer][1.7.0-windows-msi-x64]
  ([signature][1.7.0-windows-msi-x64-sig])
- [Windows x86_64 .exe MSVC installer][1.7.0-windows-msvc-exe-x64]
  ([signature][1.7.0-windows-msvc-exe-x64-sig])
- [Windows x86_64 .msi MSVC installer][1.7.0-windows-msvc-msi-x64]
  ([signature][1.7.0-windows-msvc-msi-x64-sig])
- [Windows i686 .exe gnu installer][1.7.0-windows-x32]
  ([signature][1.7.0-windows-x32-sig])
- [Windows i686 .msi gnu installer][1.7.0-windows-msi-x32]
  ([signature][1.7.0-windows-msi-x32-sig])
- [Windows i686 .exe MSVC installer][1.7.0-windows-msvc-exe-x32]
  ([signature][1.7.0-windows-msvc-exe-x32-sig])
- [Windows i686 .msi MSVC installer][1.7.0-windows-msvc-msi-x32]
  ([signature][1.7.0-windows-msvc-msi-x32-sig])
- [Linux x86_64 tarball][1.7.0-linux-x64] ([signature][1.7.0-linux-x64-sig])
- [Linux i686 tarball][1.7.0-linux-x32] ([signature][1.7.0-linux-x32-sig])
- [Mac OS X i686 pkg][1.7.0-osx-x32-pkg] ([signature][1.7.0-osx-x32-pkg-sig])
- [Mac OS X i686 tarball][1.7.0-osx-x32-tar]
  ([signature][1.7.0-osx-x32-tar-sig])
- [Mac OS X x86_64 pkg][1.7.0-osx-x64-pkg] ([signature][1.7.0-osx-x64-pkg-sig])

[1.7.0-announce]: http://blog.rust-lang.org/2016/03/02/Rust-1.7.html
[1.7.0-notes]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-170-2016-03-03
[1.7.0-tar]: https://static.rust-lang.org/dist/rustc-1.7.0-src.tar.gz
[1.7.0-tar-sig]: https://static.rust-lang.org/dist/rustc-1.7.0-src.tar.gz.asc
[1.7.0-windows-x64]: https://static.rust-lang.org/dist/rust-1.7.0-x86_64-pc-windows-gnu.exe
[1.7.0-windows-x64-sig]: https://static.rust-lang.org/dist/rust-1.7.0-x86_64-pc-windows-gnu.exe.asc
[1.7.0-windows-msi-x64]: https://static.rust-lang.org/dist/rust-1.7.0-x86_64-pc-windows-gnu.msi
[1.7.0-windows-msi-x64-sig]: https://static.rust-lang.org/dist/rust-1.7.0-x86_64-pc-windows-gnu.msi.asc
[1.7.0-windows-msvc-exe-x64]: https://static.rust-lang.org/dist/rust-1.7.0-x86_64-pc-windows-msvc.exe
[1.7.0-windows-msvc-exe-x64-sig]: https://static.rust-lang.org/dist/rust-1.7.0-x86_64-pc-windows-msvc.exe.asc
[1.7.0-windows-msvc-msi-x64]: https://static.rust-lang.org/dist/rust-1.7.0-x86_64-pc-windows-msvc.msi
[1.7.0-windows-msvc-msi-x64-sig]: https://static.rust-lang.org/dist/rust-1.7.0-x86_64-pc-windows-msvc.msi.asc
[1.7.0-windows-x32]: https://static.rust-lang.org/dist/rust-1.7.0-i686-pc-windows-gnu.exe
[1.7.0-windows-x32-sig]: https://static.rust-lang.org/dist/rust-1.7.0-i686-pc-windows-gnu.exe.asc
[1.7.0-windows-msi-x32]: https://static.rust-lang.org/dist/rust-1.7.0-i686-pc-windows-gnu.msi
[1.7.0-windows-msi-x32-sig]: https://static.rust-lang.org/dist/rust-1.7.0-i686-pc-windows-gnu.msi.asc
[1.7.0-windows-msvc-exe-x32]: https://static.rust-lang.org/dist/rust-1.7.0-i686-pc-windows-msvc.exe
[1.7.0-windows-msvc-exe-x32-sig]: https://static.rust-lang.org/dist/rust-1.7.0-i686-pc-windows-msvc.exe.asc
[1.7.0-windows-msvc-msi-x32]: https://static.rust-lang.org/dist/rust-1.7.0-i686-pc-windows-msvc.msi
[1.7.0-windows-msvc-msi-x32-sig]: https://static.rust-lang.org/dist/rust-1.7.0-i686-pc-windows-msvc.msi.asc
[1.7.0-linux-x32]: https://static.rust-lang.org/dist/rust-1.7.0-i686-unknown-linux-gnu.tar.gz
[1.7.0-linux-x32-sig]: https://static.rust-lang.org/dist/rust-1.7.0-i686-unknown-linux-gnu.tar.gz.asc
[1.7.0-linux-x64]: https://static.rust-lang.org/dist/rust-1.7.0-x86_64-unknown-linux-gnu.tar.gz
[1.7.0-linux-x64-sig]: https://static.rust-lang.org/dist/rust-1.7.0-x86_64-unknown-linux-gnu.tar.gz.asc
[1.7.0-osx-x32-pkg]: https://static.rust-lang.org/dist/rust-1.7.0-i686-apple-darwin.pkg
[1.7.0-osx-x32-pkg-sig]: https://static.rust-lang.org/dist/rust-1.7.0-i686-apple-darwin.pkg.asc
[1.7.0-osx-x32-tar]: https://static.rust-lang.org/dist/rust-1.7.0-i686-apple-darwin.tar.gz
[1.7.0-osx-x32-tar-sig]: https://static.rust-lang.org/dist/rust-1.7.0-i686-apple-darwin.tar.gz.asc
[1.7.0-osx-x64-pkg]: https://static.rust-lang.org/dist/rust-1.7.0-x86_64-apple-darwin.pkg
[1.7.0-osx-x64-pkg-sig]: https://static.rust-lang.org/dist/rust-1.7.0-x86_64-apple-darwin.pkg.asc

## 1.6.0

- [Announcement][1.6.0-announce]
- [Release notes][1.6.0-notes]

[1.6.0-announce]: http://blog.rust-lang.org/2016/01/21/Rust-1.6.html
[1.6.0-notes]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-160-2016-01-21

## 1.5.0

- [Announcement][1.5.0-announce]
- [Release notes][1.5.0-notes]

[1.5.0-announce]: http://blog.rust-lang.org/2015/12/10/Rust-1.5.html
[1.5.0-notes]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-150-2015-12-10

## 1.4.0

- [Announcement][1.4.0-announce]
- [Release notes][1.4.0-notes]

[1.4.0-announce]: http://blog.rust-lang.org/2015/10/29/Rust-1.4.html
[1.4.0-notes]: https://github.com/rust-lang/rust/blob/8ab8581f6921bc7a8e3fa4defffd2814372dcb15/RELEASES.md#version-140-october-2015

## 1.3.0

- [Announcement][1.3.0-announce]
- [Release notes][1.3.0-notes]

[1.3.0-announce]: http://blog.rust-lang.org/2015/09/17/Rust-1.3.html
[1.3.0-notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-130-september-2015

## 1.2.0

- [Announcement][1.2.0-announce]
- [Release notes][1.2.0-notes]

[1.2.0-announce]: http://blog.rust-lang.org/2015/08/06/Rust-1.2.html
[1.2.0-notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-120-2015-08-07

## 1.1.0

- [Announcement][1.1.0-announce]
- [Release notes][1.1.0-notes]

[1.1.0-announce]: http://blog.rust-lang.org/2015/06/25/Rust-1.1.html
[1.1.0-notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-110-2015-06-25

## 1.0.0

- [Announcement][1.0.0-announce]
- [Release notes][1.0.0-notes]

[1.0.0-announce]: http://blog.rust-lang.org/2015/05/15/Rust-1.0.html
[1.0.0-notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-100-2015-05-15

## 1.0.0-beta

- [Announcement][1.0.0-beta-announce]

[1.0.0-beta-announce]: http://blog.rust-lang.org/2015/04/03/Rust-1.0-beta.html

## 1.0.0-alpha.2

- [Announcement][1.0.0-alpha.2-announce]
- [Release notes][1.0.0-alpha.2-notes]
- [Source code][1.0.0-alpha.2-tar] ([signature][1.0.0-alpha.2-tar-sig])
- [Windows x86_64 .exe installer][1.0.0-alpha.2-windows-x64]
  ([signature][1.0.0-alpha.2-windows-x64-sig])
- [Windows i686 .exe installer][1.0.0-alpha.2-windows-x32]
  ([signature][1.0.0-alpha.2-windows-x32-sig])
- [Windows x86_64 .msi installer][1.0.0-alpha.2-windows-msi-x64]
  ([signature][1.0.0-alpha.2-windows-msi-x64-sig])
- [Windows i686 .msi installer][1.0.0-alpha.2-windows-msi-x32]
  ([signature][1.0.0-alpha.2-windows-msi-x32-sig])
- [Linux x86_64 tarball][1.0.0-alpha.2-linux-x64]
  ([signature][1.0.0-alpha.2-linux-x64-sig])
- [Linux i686 tarball][1.0.0-alpha.2-linux-x32]
  ([signature][1.0.0-alpha.2-linux-x32-sig])
- [Mac OS X x86_64 pkg][1.0.0-alpha.2-osx-x64-pkg]
  ([signature][1.0.0-alpha.2-osx-x64-pkg-sig])
- [Mac OS X i686 pkg][1.0.0-alpha.2-osx-x32-pkg]
  ([signature][1.0.0-alpha.2-osx-x32-pkg-sig])
- [Mac OS X x86_64 tarball][1.0.0-alpha.2-osx-x64-tar]
  ([signature][1.0.0-alpha.2-osx-x64-tar-sig])
- [Mac OS X i686 tarball][1.0.0-alpha.2-osx-x32-tar]
  ([signature][1.0.0-alpha.2-osx-x32-tar-sig])
- [Documentation][1.0.0-alpha.2-docs]

[1.0.0-alpha.2-announce]: http://blog.rust-lang.org/2015/02/20/Rust-1.0-alpha2.html
[1.0.0-alpha.2-notes]: https://github.com/mozilla/rust/blob/1.0.0-alpha.2/RELEASES.md
[1.0.0-alpha.2-tar]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2.tar.gz
[1.0.0-alpha.2-tar-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2.tar.gz.asc
[1.0.0-alpha.2-windows-x64]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-x86_64-pc-windows-gnu.exe
[1.0.0-alpha.2-windows-x64-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-x86_64-pc-windows-gnu.exe.asc
[1.0.0-alpha.2-windows-x32]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-i686-pc-windows-gnu.exe
[1.0.0-alpha.2-windows-x32-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-i686-pc-windows-gnu.exe.asc
[1.0.0-alpha.2-windows-msi-x64]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-x86_64-pc-windows-gnu.msi
[1.0.0-alpha.2-windows-msi-x64-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-x86_64-pc-windows-gnu.msi.asc
[1.0.0-alpha.2-windows-msi-x32]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-i686-pc-windows-gnu.msi
[1.0.0-alpha.2-windows-msi-x32-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-i686-pc-windows-gnu.msi.asc
[1.0.0-alpha.2-linux-x64]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-x86_64-unknown-linux-gnu.tar.gz
[1.0.0-alpha.2-linux-x64-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-x86_64-unknown-linux-gnu.tar.gz.asc
[1.0.0-alpha.2-linux-x32]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-i686-unknown-linux-gnu.tar.gz
[1.0.0-alpha.2-linux-x32-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-i686-unknown-linux-gnu.tar.gz.asc
[1.0.0-alpha.2-osx-x64-pkg]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-x86_64-apple-darwin.pkg
[1.0.0-alpha.2-osx-x64-pkg-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-x86_64-apple-darwin.pkg.asc
[1.0.0-alpha.2-osx-x32-pkg]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-i686-apple-darwin.pkg
[1.0.0-alpha.2-osx-x32-pkg-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-i686-apple-darwin.pkg.asc
[1.0.0-alpha.2-osx-x64-tar]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-x86_64-apple-darwin.tar.gz
[1.0.0-alpha.2-osx-x64-tar-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-x86_64-apple-darwin.tar.gz.asc
[1.0.0-alpha.2-osx-x32-tar]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-i686-apple-darwin.tar.gz
[1.0.0-alpha.2-osx-x32-tar-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.2-i686-apple-darwin.tar.gz.asc
[1.0.0-alpha.2-docs]: http://doc.rust-lang.org/1.0.0-alpha.2/index.html

## 1.0.0-alpha

- [Announcement][1.0.0-alpha-announce]
- [Release notes][1.0.0-alpha-notes]
- [Source code][1.0.0-alpha-tar] ([signature][1.0.0-alpha-tar-sig])
- [Windows x86_64 installer][1.0.0-alpha-windows-x64]
  ([signature][1.0.0-alpha-windows-x64-sig])
- [Windows i686 installer][1.0.0-alpha-windows-x32]
  ([signature][1.0.0-alpha-windows-x32-sig])
- [Linux x86_64 tarball][1.0.0-alpha-linux-x64]
  ([signature][1.0.0-alpha-linux-x64-sig])
- [Linux i686 tarball][1.0.0-alpha-linux-x32]
  ([signature][1.0.0-alpha-linux-x32-sig])
- [Mac OS X x86_64 pkg][1.0.0-alpha-osx-x64-pkg]
  ([signature][1.0.0-alpha-osx-x64-pkg-sig])
- [Mac OS X i686 pkg][1.0.0-alpha-osx-x32-pkg]
  ([signature][1.0.0-alpha-osx-x32-pkg-sig])
- [Mac OS X x86_64 tarball][1.0.0-alpha-osx-x64-tar]
  ([signature][1.0.0-alpha-osx-x64-tar-sig])
- [Mac OS X i686 tarball][1.0.0-alpha-osx-x32-tar]
  ([signature][1.0.0-alpha-osx-x32-tar-sig])
- [Documentation][1.0.0-alpha-docs]

[1.0.0-alpha-announce]: http://blog.rust-lang.org/2015/01/09/Rust-1.0-alpha.html
[1.0.0-alpha-notes]: https://github.com/mozilla/rust/blob/1.0.0-alpha/RELEASES.md
[1.0.0-alpha-tar]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.tar.gz
[1.0.0-alpha-tar-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha.tar.gz.asc
[1.0.0-alpha-windows-x64]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-x86_64-pc-windows-gnu.exe
[1.0.0-alpha-windows-x64-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-x86_64-pc-windows-gnu.exe.asc
[1.0.0-alpha-windows-x32]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-i686-pc-windows-gnu.exe
[1.0.0-alpha-windows-x32-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-i686-pc-windows-gnu.exe.asc
[1.0.0-alpha-linux-x64]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-x86_64-unknown-linux-gnu.tar.gz
[1.0.0-alpha-linux-x64-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-x86_64-unknown-linux-gnu.tar.gz.asc
[1.0.0-alpha-linux-x32]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-i686-unknown-linux-gnu.tar.gz
[1.0.0-alpha-linux-x32-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-i686-unknown-linux-gnu.tar.gz.asc
[1.0.0-alpha-osx-x64-pkg]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-x86_64-apple-darwin.pkg
[1.0.0-alpha-osx-x64-pkg-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-x86_64-apple-darwin.pkg.asc
[1.0.0-alpha-osx-x32-pkg]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-i686-apple-darwin.pkg
[1.0.0-alpha-osx-x32-pkg-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-i686-apple-darwin.pkg.asc
[1.0.0-alpha-osx-x64-tar]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-x86_64-apple-darwin.tar.gz
[1.0.0-alpha-osx-x64-tar-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-x86_64-apple-darwin.tar.gz.asc
[1.0.0-alpha-osx-x32-tar]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-i686-apple-darwin.tar.gz
[1.0.0-alpha-osx-x32-tar-sig]: https://static.rust-lang.org/dist/rust-1.0.0-alpha-i686-apple-darwin.tar.gz.asc
[1.0.0-alpha-docs]: http://doc.rust-lang.org/1.0.0-alpha/index.html

# Rust 0.x

In addition to the included short-form release notes, each 0.x release has a
longer explanation in the [[detailed release notes|Doc detailed release notes]].

## 0.12.0

- [Announcement][0.12.0-announce]
- [Release notes][0.12.0-notes]
- [Source code][0.12.0-tar] ([signature][0.12.0-tar-sig])
- [Windows x86_64 installer][0.12.0-windows-x64]
  ([signature][0.12.0-windows-x64-sig])
- [Windows i686 installer][0.12.0-windows-x32]
  ([signature][0.12.0-windows-x32-sig])
- [Linux x86_64 tarball][0.12.0-linux-x64] ([signature][0.12.0-linux-x64-sig])
- [Linux i686 tarball][0.12.0-linux-x32] ([signature][0.12.0-linux-x32-sig])
- [Mac OS X x86_64 pkg][0.12.0-osx-x64-pkg]
  ([signature][0.12.0-osx-x64-pkg-sig])
- [Mac OS X i686 pkg][0.12.0-osx-x32-pkg] ([signature][0.12.0-osx-x32-pkg-sig])
- [Mac OS X x86_64 tarball][0.12.0-osx-x64-tar]
  ([signature][0.12.0-osx-x64-tar-sig])
- [Mac OS X i686 tarball][0.12.0-osx-x32-tar]
  ([signature][0.12.0-osx-x32-tar-sig])
- [Documentation][0.12.0-docs]

[0.12.0-announce]: https://mail.mozilla.org/pipermail/rust-dev/2014-October/011267.html
[0.12.0-notes]: https://github.com/mozilla/rust/blob/0.12.0/RELEASES.md
[0.12.0-tar]: https://static.rust-lang.org/dist/rust-0.12.0.tar.gz
[0.12.0-tar-sig]: https://static.rust-lang.org/dist/rust-0.12.0.tar.gz.asc
[0.12.0-windows-x64]: https://static.rust-lang.org/dist/rust-0.12.0-x86_64-w64-mingw32.exe
[0.12.0-windows-x64-sig]: https://static.rust-lang.org/dist/rust-0.12.0-x86_64-w64-mingw32.exe.asc
[0.12.0-windows-x32]: https://static.rust-lang.org/dist/rust-0.12.0-i686-w64-mingw32.exe
[0.12.0-windows-x32-sig]: https://static.rust-lang.org/dist/rust-0.12.0-i686-w64-mingw32.exe.asc
[0.12.0-linux-x64]: https://static.rust-lang.org/dist/rust-0.12.0-x86_64-unknown-linux-gnu.tar.gz
[0.12.0-linux-x64-sig]: https://static.rust-lang.org/dist/rust-0.12.0-x86_64-unknown-linux-gnu.tar.gz.asc
[0.12.0-linux-x32]: https://static.rust-lang.org/dist/rust-0.12.0-i686-unknown-linux-gnu.tar.gz
[0.12.0-linux-x32-sig]: https://static.rust-lang.org/dist/rust-0.12.0-i686-unknown-linux-gnu.tar.gz.asc
[0.12.0-osx-x64-pkg]: https://static.rust-lang.org/dist/rust-0.12.0-x86_64-apple-darwin.pkg
[0.12.0-osx-x64-pkg-sig]: https://static.rust-lang.org/dist/rust-0.12.0-x86_64-apple-darwin.pkg.asc
[0.12.0-osx-x32-pkg]: https://static.rust-lang.org/dist/rust-0.12.0-i686-apple-darwin.pkg
[0.12.0-osx-x32-pkg-sig]: https://static.rust-lang.org/dist/rust-0.12.0-i686-apple-darwin.pkg.asc
[0.12.0-osx-x64-tar]: https://static.rust-lang.org/dist/rust-0.12.0-x86_64-apple-darwin.tar.gz
[0.12.0-osx-x64-tar-sig]: https://static.rust-lang.org/dist/rust-0.12.0-x86_64-apple-darwin.tar.gz.asc
[0.12.0-osx-x32-tar]: https://static.rust-lang.org/dist/rust-0.12.0-i686-apple-darwin.tar.gz
[0.12.0-osx-x32-tar-sig]: https://static.rust-lang.org/dist/rust-0.12.0-i686-apple-darwin.tar.gz.asc
[0.12.0-docs]: http://doc.rust-lang.org/0.12.0/index.html

## 0.11.0

- [Announcement][0.11.0-announce]
- [Release notes][0.11.0-notes]
- [Source code][0.11.0-tar] ([signature][0.11.0-tar-sig])
- [Windows installer][0.11.0-exe] ([signature][0.11.0-exe-sig])
- [Linux x86_64 tarball][0.11.0-linux-x64] ([signature][0.11.0-linux-x64-sig])
- [Linux i686 tarball][0.11.0-linux-x32] ([signature][0.11.0-linux-x32-sig])
- [Mac OS X x86_64 pkg][0.11.0-osx-x64-pkg]
  ([signature][0.11.0-osx-x64-pkg-sig])
- [Mac OS X i686 pkg][0.11.0-osx-x32-pkg] ([signature][0.11.0-osx-x32-pkg-sig])
- [Mac OS X x86_64 tarball][0.11.0-osx-x64-tar]
  ([signature][0.11.0-osx-x64-tar-sig])
- [Mac OS X i686 tarball][0.11.0-osx-x32-tar]
  ([signature][0.11.0-osx-x32-tar-sig])
- [Documentation][0.11.0-docs]

[0.11.0-announce]: https://mail.mozilla.org/pipermail/rust-dev/2014-July/010655.html
[0.11.0-notes]: https://github.com/mozilla/rust/blob/0.11.0/RELEASES.txt
[0.11.0-tar]: https://static.rust-lang.org/dist/rust-0.11.0.tar.gz
[0.11.0-tar-sig]: https://static.rust-lang.org/dist/rust-0.11.0.tar.gz.asc
[0.11.0-exe]: https://static.rust-lang.org/dist/rust-0.11.0-install.exe
[0.11.0-exe-sig]: https://static.rust-lang.org/dist/rust-0.11.0-install.exe.asc
[0.11.0-linux-x64]: https://static.rust-lang.org/dist/rust-0.11.0-x86_64-unknown-linux-gnu.tar.gz
[0.11.0-linux-x64-sig]: https://static.rust-lang.org/dist/rust-0.11.0-x86_64-unknown-linux-gnu.tar.gz.asc
[0.11.0-linux-x32]: https://static.rust-lang.org/dist/rust-0.11.0-i686-unknown-linux-gnu.tar.gz
[0.11.0-linux-x32-sig]: https://static.rust-lang.org/dist/rust-0.11.0-i686-unknown-linux-gnu.tar.gz.asc
[0.11.0-osx-x64-pkg]: https://static.rust-lang.org/dist/rust-0.11.0-x86_64-apple-darwin.pkg
[0.11.0-osx-x64-pkg-sig]: https://static.rust-lang.org/dist/rust-0.11.0-x86_64-apple-darwin.pkg.asc
[0.11.0-osx-x32-pkg]: https://static.rust-lang.org/dist/rust-0.11.0-i686-apple-darwin.pkg
[0.11.0-osx-x32-pkg-sig]: https://static.rust-lang.org/dist/rust-0.11.0-i686-apple-darwin.pkg.asc
[0.11.0-osx-x64-tar]: https://static.rust-lang.org/dist/rust-0.11.0-x86_64-apple-darwin.tar.gz
[0.11.0-osx-x64-tar-sig]: https://static.rust-lang.org/dist/rust-0.11.0-x86_64-apple-darwin.tar.gz.asc
[0.11.0-osx-x32-tar]: https://static.rust-lang.org/dist/rust-0.11.0-i686-apple-darwin.tar.gz
[0.11.0-osx-x32-tar-sig]: https://static.rust-lang.org/dist/rust-0.11.0-i686-apple-darwin.tar.gz.asc
[0.11.0-docs]: http://doc.rust-lang.org/0.11.0/index.html

## 0.10

- [Announcement][0.10-announce]
- [Release notes][0.10-notes]
- [Source code][0.10-tar] ([signature][0.10-tar-sig])
- [Windows installer][0.10-exe] ([signature][0.10-exe-sig])
- [Linux x86_64 tarball][0.10-linux-x64] ([signature][0.10-linux-x64-sig])
- [Linux i686 tarball][0.10-linux-x32] ([signature][0.10-linux-x32-sig])
- [Mac OS X x86_64 pkg][0.10-osx-x64-pkg] ([signature][0.10-osx-x64-pkg-sig])
- [Mac OS X i686 pkg][0.10-osx-x32-pkg] ([signature][0.10-osx-x32-pkg-sig])
- [Mac OS X x86_64 tarball][0.10-osx-x64-tar]
  ([signature][0.10-osx-x64-tar-sig])
- [Mac OS X i686 tarball][0.10-osx-x32-tar] ([signature][0.10-osx-x32-tar-sig])
- [Documentation][0.10-docs]

[0.10-announce]: https://mail.mozilla.org/pipermail/rust-dev/2014-April/009387.html
[0.10-notes]: https://github.com/mozilla/rust/blob/0.10/RELEASES.txt
[0.10-tar]: https://static.rust-lang.org/dist/rust-0.10.tar.gz
[0.10-tar-sig]: https://static.rust-lang.org/dist/rust-0.10.tar.gz.asc
[0.10-exe]: https://static.rust-lang.org/dist/rust-0.10-install.exe
[0.10-exe-sig]: https://static.rust-lang.org/dist/rust-0.10-install.exe.asc
[0.10-linux-x64]: https://static.rust-lang.org/dist/rust-0.10-x86_64-unknown-linux-gnu.tar.gz
[0.10-linux-x64-sig]: https://static.rust-lang.org/dist/rust-0.10-x86_64-unknown-linux-gnu.tar.gz.asc
[0.10-linux-x32]: https://static.rust-lang.org/dist/rust-0.10-i686-unknown-linux-gnu.tar.gz
[0.10-linux-x32-sig]: https://static.rust-lang.org/dist/rust-0.10-i686-unknown-linux-gnu.tar.gz.asc
[0.10-osx-x64-pkg]: https://static.rust-lang.org/dist/rust-0.10-x86_64-apple-darwin.pkg
[0.10-osx-x64-pkg-sig]: https://static.rust-lang.org/dist/rust-0.10-x86_64-apple-darwin.pkg.asc
[0.10-osx-x32-pkg]: https://static.rust-lang.org/dist/rust-0.10-i686-apple-darwin.pkg
[0.10-osx-x32-pkg-sig]: https://static.rust-lang.org/dist/rust-0.10-i686-apple-darwin.pkg.asc
[0.10-osx-x64-tar]: https://static.rust-lang.org/dist/rust-0.10-x86_64-apple-darwin.tar.gz
[0.10-osx-x64-tar-sig]: https://static.rust-lang.org/dist/rust-0.10-x86_64-apple-darwin.tar.gz.asc
[0.10-osx-x32-tar]: https://static.rust-lang.org/dist/rust-0.10-i686-apple-darwin.tar.gz
[0.10-osx-x32-tar-sig]: https://static.rust-lang.org/dist/rust-0.10-i686-apple-darwin.tar.gz.asc
[0.10-docs]: http://doc.rust-lang.org/doc/0.10/index.html

## 0.9

- [Announcement][0.9-announce]
- [Release notes][0.9-notes]
- [Source code][0.9-tar] ([signature][0.9-tar-sig])
- [Windows installer][0.9-exe] ([signature][0.9-exe-sig])
- [Documentation][0.9-docs]

[0.9-announce]: https://mail.mozilla.org/pipermail/rust-dev/2014-January/007753.html
[0.9-notes]: https://github.com/mozilla/rust/blob/0.9/RELEASES.txt
[0.9-tar]: https://static.rust-lang.org/dist/rust-0.9.tar.gz
[0.9-tar-sig]: https://static.rust-lang.org/dist/rust-0.9.tar.gz.asc
[0.9-exe]: https://static.rust-lang.org/dist/rust-0.9-install.exe
[0.9-exe-sig]: https://static.rust-lang.org/dist/rust-0.9-install.exe.asc
[0.9-rustpkg-manual]: http://doc.rust-lang.org/doc/0.9/rustpkg.html
[0.9-docs]: http://doc.rust-lang.org/doc/0.9/index.html

## 0.8

- [Announcement][0.8-announce]
- [Release notes][0.8-notes]
- [Source code][0.8-tar] ([signature][0.8-tar-sig])
- [Windows installer][0.8-exe] ([signature][0.8-exe-sig])
- [Tutorial][0.8-tutorial]
  - [borrowed pointers][0.8-tutorial-borrowed-ptr] |
    [conditions][0.8-tutorial-conditions] | [containers][0.8-tutorial-container]
    | [ffi][0.8-tutorial-ffi] | [macros][0.8-tutorial-macros] |
    [rustpkg][0.8-tutorial-rustpkg] | [tasks][0.8-tutorial-tasks]
- [Manual][0.8-manual] ([PDF][0.8-manual-pdf])
- [Rustpkg manual][0.8-rustpkg-manual]
- [Standard library docs][0.8-std]
- [Extra library docs][0.8-extra]

[0.8-announce]: https://mail.mozilla.org/pipermail/rust-dev/2013-September/005804.html
[0.8-notes]: https://github.com/mozilla/rust/blob/0.8/RELEASES.txt
[0.8-tar]: https://static.rust-lang.org/dist/rust-0.8.tar.gz
[0.8-tar-sig]: https://static.rust-lang.org/dist/rust-0.8.tar.gz.asc
[0.8-exe]: https://static.rust-lang.org/dist/rust-0.8-install.exe
[0.8-exe-sig]: https://static.rust-lang.org/dist/rust-0.8-install.exe.asc
[0.8-tutorial]: http://doc.rust-lang.org/doc/0.8/tutorial.html
[0.8-tutorial-borrowed-ptr]: http://doc.rust-lang.org/doc/0.8/tutorial-borrowed-ptr.html
[0.8-tutorial-conditions]: http://doc.rust-lang.org/doc/0.8/tutorial-conditions.html
[0.8-tutorial-container]: http://doc.rust-lang.org/doc/0.8/tutorial-container.html
[0.8-tutorial-ffi]: http://doc.rust-lang.org/doc/0.8/tutorial-ffi.html
[0.8-tutorial-macros]: http://doc.rust-lang.org/doc/0.8/tutorial-macros.html
[0.8-tutorial-rustpkg]: http://doc.rust-lang.org/doc/0.8/tutorial-rustpkg.html
[0.8-tutorial-tasks]: http://doc.rust-lang.org/doc/0.8/tutorial-tasks.html
[0.8-manual]: http://doc.rust-lang.org/doc/0.8/rust.html
[0.8-manual-pdf]: http://doc.rust-lang.org/doc/0.8/rust.pdf
[0.8-rustpkg-manual]: http://doc.rust-lang.org/doc/0.8/rustpkg.html
[0.8-std]: http://doc.rust-lang.org/doc/0.8/std/index.html
[0.8-extra]: http://doc.rust-lang.org/doc/0.8/extra/index.html

## 0.7

- [Announcement][0.7-announce]
- [Release notes][0.7-notes]
- [Source code][0.7-tar] ([signature][0.7-tar-sig])
- [Windows installer][0.7-exe] ([signature][0.7-exe-sig])
- [Tutorial][0.7-tutorial]
- [Manual][0.7-manual] ([PDF][0.7-manual-pdf])
- [Standard library docs][0.7-std]
- [Extra library docs][0.7-extra]

[0.7-announce]: https://mail.mozilla.org/pipermail/rust-dev/2013-July/004667.html
[0.7-notes]: https://github.com/mozilla/rust/blob/release-0.7/RELEASES.txt
[0.7-tar]: https://static.rust-lang.org/dist/rust-0.7.tar.gz
[0.7-tar-sig]: https://static.rust-lang.org/dist/rust-0.7.tar.gz.asc
[0.7-exe]: https://static.rust-lang.org/dist/rust-0.7-install.exe
[0.7-exe-sig]: https://static.rust-lang.org/dist/rust-0.7-install.exe.asc
[0.7-tutorial]: http://doc.rust-lang.org/doc/0.7/tutorial.html
[0.7-manual]: http://doc.rust-lang.org/doc/0.7/rust.html
[0.7-manual-pdf]: http://doc.rust-lang.org/doc/0.7/rust.pdf
[0.7-std]: http://doc.rust-lang.org/doc/0.7/std/index.html
[0.7-extra]: http://doc.rust-lang.org/doc/0.7/extra/index.html

## 0.6

- [Announcement][0.6-announce]
- [Release notes][0.6-notes]
- [Source code][0.6-tar] ([signature][0.6-tar-sig])
- [Windows installer][0.6-exe] ([signature][0.6-exe-sig])
- [Tutorial][0.6-tutorial]
- [Manual][0.6-manual] ([PDF][0.6-manual-pdf])
- [Core library docs][0.6-core]
- [Standard library docs][0.6-std]

[0.6-announce]: https://mail.mozilla.org/pipermail/rust-dev/2013-April/003427.html
[0.6-notes]: https://github.com/mozilla/rust/blob/release-0.6/RELEASES.txt
[0.6-tar]: https://static.rust-lang.org/dist/rust-0.6.tar.gz
[0.6-tar-sig]: https://static.rust-lang.org/dist/rust-0.6.tar.gz.asc
[0.6-exe]: https://static.rust-lang.org/dist/rust-0.6-install.exe
[0.6-exe-sig]: https://static.rust-lang.org/dist/rust-0.6-install.exe.asc
[0.6-tutorial]: http://doc.rust-lang.org/doc/0.6/tutorial.html
[0.6-manual]: http://doc.rust-lang.org/doc/0.6/rust.html
[0.6-manual-pdf]: http://doc.rust-lang.org/doc/0.6/rust.pdf
[0.6-core]: http://doc.rust-lang.org/doc/0.6/core/index.html
[0.6-std]: http://doc.rust-lang.org/doc/0.6/std/index.html

## 0.5

- [Announcement][0.5-announce]
- [Release notes][0.5-notes]
- [Source code][0.5-tar] ([signature][0.5-tar-sig])
- [Windows installer][0.5-exe] ([signature][0.5-exe-sig])
- [Tutorial][0.5-tutorial]
- [Manual][0.5-manual] ([PDF][0.5-manual-pdf])
- [Core library docs][0.5-core]
- [Standard library docs][0.5-std]

[0.5-announce]: https://mail.mozilla.org/pipermail/rust-dev/2012-December/002787.html
[0.5-notes]: https://github.com/mozilla/rust/blob/release-0.5/RELEASES.txt
[0.5-tar]: https://static.rust-lang.org/dist/rust-0.5.tar.gz
[0.5-tar-sig]: https://static.rust-lang.org/dist/rust-0.5.tar.gz.asc
[0.5-exe]: https://static.rust-lang.org/dist/rust-0.5-install.exe
[0.5-exe-sig]: https://static.rust-lang.org/dist/rust-0.5-install.exe.asc
[0.5-tutorial]: http://doc.rust-lang.org/doc/0.5/tutorial.html
[0.5-manual]: http://doc.rust-lang.org/doc/0.5/rust.html
[0.5-manual-pdf]: http://doc.rust-lang.org/doc/0.5/rust.pdf
[0.5-core]: http://doc.rust-lang.org/doc/0.5/core/index.html
[0.5-std]: http://doc.rust-lang.org/doc/0.5/std/index.html

## 0.4

- [Announcement][0.4-announce]
- [Release notes][0.4-notes]
- [Source code][0.4-tar] ([signature][0.4-tar-sig])
- [Windows installer][0.4-exe] ([signature][0.4-exe-sig])
- [Tutorial][0.4-tutorial]
- [Manual][0.4-manual] ([PDF][0.4-manual-pdf])
- [Core library docs][0.4-core]
- [Standard library docs][0.4-std]

[0.4-announce]: https://mail.mozilla.org/pipermail/rust-dev/2012-October/002489.html
[0.4-notes]: https://github.com/mozilla/rust/blob/release-0.4/RELEASES.txt
[0.4-tar]: https://static.rust-lang.org/dist/rust-0.4.tar.gz
[0.4-tar-sig]: https://static.rust-lang.org/dist/rust-0.4.tar.gz.asc
[0.4-exe]: https://static.rust-lang.org/dist/rust-0.4-install.exe
[0.4-exe-sig]: https://static.rust-lang.org/dist/rust-0.4-install.exe.asc
[0.4-tutorial]: http://doc.rust-lang.org/doc/0.4/tutorial.html
[0.4-manual]: http://doc.rust-lang.org/doc/0.4/rust.html
[0.4-manual-pdf]: http://doc.rust-lang.org/doc/0.4/rust.pdf
[0.4-core]: http://doc.rust-lang.org/doc/0.4/core/index.html
[0.4-std]: http://doc.rust-lang.org/doc/0.4/std/index.html

## 0.3.1

- [Announcement][0.3.1-announce]
- [Release notes][0.3.1-notes]
- [Source code][0.3.1-tar] ([signature][0.3.1-tar-sig])

[0.3.1-announce]: https://mail.mozilla.org/pipermail/rust-dev/2012-July/002152.html
[0.3.1-notes]: https://github.com/mozilla/rust/blob/release-0.3.1/RELEASES.txt
[0.3.1-tar]: https://static.rust-lang.org/dist/rust-0.3.1.tar.gz
[0.3.1-tar-sig]: https://static.rust-lang.org/dist/rust-0.3.1.tar.gz.asc

This was an OS X bugfix release.

## 0.3

- [Announcement][0.3-announce]
- [Release notes][0.3-notes]
- [Source code][0.3-tar] ([signature][0.3-tar-sig])
- [Windows installer][0.3-exe] ([signature][0.3-exe-sig])
- [Tutorial][0.3-tutorial]
- [Manual][0.3-manual] ([PDF][0.3-manual-pdf])
- [Core library docs][0.3-core]
- [Standard library docs][0.3-std]

[0.3-announce]: https://mail.mozilla.org/pipermail/rust-dev/2012-July/002087.html
[0.3-notes]: https://github.com/mozilla/rust/blob/release-0.3/RELEASES.txt
[0.3-tar]: https://static.rust-lang.org/dist/rust-0.3.tar.gz
[0.3-tar-sig]: https://static.rust-lang.org/dist/rust-0.3.tar.gz.asc
[0.3-exe]: https://static.rust-lang.org/dist/rust-0.3-install.exe
[0.3-exe-sig]: https://static.rust-lang.org/dist/rust-0.3-install.exe.asc
[0.3-tutorial]: http://doc.rust-lang.org/doc/0.3/tutorial.html
[0.3-manual]: http://doc.rust-lang.org/doc/0.3/rust.html
[0.3-manual-pdf]: http://doc.rust-lang.org/doc/0.3/rust.pdf
[0.3-core]: http://doc.rust-lang.org/doc/0.3/core/index.html
[0.3-std]: http://doc.rust-lang.org/doc/0.3/std/index.html

## 0.2

- [Announcement][0.2-announce]
- [Release notes][0.2-notes]
- [Source code][0.2-tar] ([signature][0.2-tar-sig])
- [Windows installer][0.2-exe] ([signature][0.2-exe-sig])

[0.2-announce]: https://mail.mozilla.org/pipermail/rust-dev/2012-March/001511.html
[0.2-notes]: https://github.com/mozilla/rust/blob/release-0.2/RELEASES.txt
[0.2-tar]: https://static.rust-lang.org/dist/rust-0.2.tar.gz
[0.2-tar-sig]: https://static.rust-lang.org/dist/rust-0.2.tar.gz.asc
[0.2-exe]: https://static.rust-lang.org/dist/rust-0.2-install.exe
[0.2-exe-sig]: https://static.rust-lang.org/dist/rust-0.2-install.exe.asc

## 0.1

- [Announcement][0.1-announce]
- [Release notes][0.1-notes]
- [Source code][0.1-tar] ([signature][0.1-tar-sig])

[0.1-announce]: https://mail.mozilla.org/pipermail/rust-dev/2012-January/001256.html
[0.1-notes]: https://github.com/mozilla/rust/blob/release-0.1/RELEASES.txt
[0.1-tar]: https://static.rust-lang.org/dist/rust-0.1.tar.gz
[0.1-tar-sig]: https://static.rust-lang.org/dist/rust-0.1.tar.gz.asc
