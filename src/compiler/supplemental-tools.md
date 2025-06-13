# Supplemental Tool Policy
T-compiler's mandate is to do what is necessary to ship a functioning Rust compiler (`rustc`), but `rustc` has never worked in isolation.
It has always required additional tools to use, whether provided with the compiler toolchain or expected in the environment.
The following is an attempt to clarify some of the expectations around `rustc` and the tools it may be used with or require,
and make explicit some expectations that are already implicit.


## Definitions
- **compilation environment** - the host system that a **provided tool** is invoked on,
  including its transient state like the filesystem and environment variables.
- **invoke** - running the code of any artifact, whether via process spawning, library loading, or
  other means that the **compilation environment** may support.
- **intermediates** - artifacts output by **invoking** a **compiler tool** that will be
  provided as inputs to compiler tools.
- **compiler tool** - an artifact, especially one shipped as part of the Rust toolchain, that is
  **invoke**d as part of the process of compiling and linking code into binary code objects.
- **binary tool** - a **compiler tool** that is used to emit or edit binary code objects.
- **provided tool** - a **binary tool** or **compiler tool** shipped as part of the Rust toolchain,
  with the support of T-compiler and T-release.
  This especially includes **binary tools** like `rustc`, `rust-lld`, and `rust-objcopy`.
- **supplemental tool** - a **compiler tool** that is not shipped by T-compiler and T-release,
  but that may be used alongside the **provided tool**s.
  This especially includes **binary tools** like `ld.bfd`, `link.exe`, `llvm-bolt`, and "linker scripts"
  (programs written in the Link Editor Command Language).


## Our Use of Supplemental Tools

Any provided tool, including `rustc`, may *implicitly* invoke a supplemental tool,
**without** requiring explicit user input.
A provided tool may examine the compilation environment to determine what tools to invoke.[^10]
The compilation environment may be expected to be configured in a customary and/or idiosyncratic
way when a supplemental tool is invoked.

Examples of customary expectations include
- an executable named `link.exe` being expected to perform the task of linking binary code objects
  located at paths given as arguments to it, or
- a Unix tool may be expected to obey arguments in a way specified by POSIX, or
- an environment variable such as `$CC` may be expected to contain a path to a C compiler.

Idiosyncratic expectations may resemble a customary expectation but apply it unreasonably,
such as assuming a Unix system, even macOS, resembles a Linux system with a GNU userland.
Alternately they may be purely invented, such as assuming data in the compilation environment
was created or modified by another provided tool in the Rust toolchain.

These expectations extend to not being required to determine whether the contents of the
compilation environment are safe to use, correctly functioning, or correctly named.
The filesystem may be expected to be organized in a certain way, and
any names provided for files may be trusted implicitly as correct labels.
Reasonable attempts *should* be made to find supplemental tools at their expected paths,
to document what tools we may rely on, and in what ways, but this is not *required*.
It is not even required to continue to invoke a supplemental tool from the environment,
as the provided tools may be expanded at the discretion of T-release and T-compiler and
used for the explicit purpose of replacing supplemental tools found in the environment.

If a provided tool determines a supplemental tool must be invoked and its outputs
used as intermediates for any other part of the compilation process, then
the resulting output may be expected to be strictly adherent to the format output by that tool.
They may also be expected to fulfill other requirements, such as specifically being
linkable, loadable, or even executable binary code objects, or simply
retaining additional metadata sections the Rust toolchain expects.

Assuming that no expectations of the provided tools have been violated, then
provided tools **shall**, when emitting executable or dynamically loadable binary code objects,
emit objects that at least minimally adhere to the object's binary format specification.
Intermediates may not be correctly formatted, even if they usually adhere to a binary format.
Adherence may be interpreted as T-compiler pleases for underdocumented binary formats.

For the compiler team, this mostly means to invoke things from the environment as you like.
Just try to get outputs right assuming, for example, there's no undefined behavior in the source.
Ideally, we adhere enough that the "native" toolchain, including debuggers and the like,
works well with our code, but there is an infinite list of possible tools one can want to use.

## Triaging Supplemental Tool Issues

Before resolving an issue concerning supplemental tools, expectations our toolchain depends on
should be documented if they are not "obvious". For example, dependence on a minimum tool version,
especially if it is higher than the one shipped with the operating system by default.
The reason to only document non-obvious expectations is the verbose definition-of-definitions
at the start of this document: it can take a long time to define sufficient vocabulary to
describe how compilers work if you assume absolutely nothing.

If someone actively includes a supplemental tool via compiler flags or configuration, then
files a bug, try to determine if the object is correctly-formatted without the supplemental tool.
If that is the case, then we should usually close the issue without trying to fix it

Sometimes a supplemental tool is involved because it was invoked implicitly by `rustc`.
If the toolchain features a provided tool that could functionally replace it,
then preferably that should be done before resolving the issue.

If the matter is about conformance to the local customs of a given operating system, then
as long as we have a distinct target for it, some effort can be made to respond to that.
We should usually try to participate, assuming the resources are open and available for doing so.

[^10]: This is a functional consequence from the OS-provided execution of any tool by name
being possible or preferable, as that means implicitly or explicitly rummaging through `$PATH`.
