# Windows

1. Acquire LLD somehow. Either your distro provides it or you have to build it
   from source.
2. You'll need an lld-link wrapper, which is just lld using the link flavor so
   it accepts the same flags as link.exe. You may either have a binary called
   lld-link, or you may have to write some sort of script to wrap lld.
3. If you want to be able to cross compile C/C++ as well, you will need to
   obtain clang-cl, which is clang pretending to be cl.
4. You'll need libraries from an existing msvc installation on Windows to link
   your Rust code against. You'll need the VC++ libraries from either VS 2015 or
   VS 2017, and the system libraries from either the Windows 8.1 or Windows 10
   SDK. Here are some approximate paths which may vary depending on the exact
   version you have installed. Copy them over to your non-windows machine.
    * VS 2015: `C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\lib`
    * VS 2017: `C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.10.24728\lib`
    * Windows 10 SDK: `C:\Program Files (x86)\Windows Kits\10\Lib\10.0.14393.0`
    * Windows 8.1 SDK: `C:\Program Files (x86)\Windows Kits\8.1\Lib\winv6.3`
5. If you want to cross compile C/C++ you'll also need headers. Replace `lib` in
   the above paths with `include` to get the appropriate headers.
6. Set your LIB and INCLUDE environment variables to semicolon separated lists
   of all the relevant directories for the correct architecture.
7. In your .cargo/config add `[target.x86_64-pc-windows-msvc] linker = "lld-link"`
   or whatever your lld pretending to be link.exe is called.
8. For cross compiling C/C++, you'll need to get the gcc crate working
   correctly. I never tested it to cross compile, I have no idea whether it will
   even do anything sane.
9. Install the appropriate target using rustup and pass
   `--target=x86_64-pc-windows-msvc` while building. Hopefully it works. If it
   doesn't, well... I don't know.

