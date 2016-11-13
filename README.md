Design Notes on Castel
======================

truly an effort so far described as "a tinkertoy version of a grad
project" tentatively picking castel for a name [see:
here](https://www.youtube.com/watch?v=UFnmq5PPSc)

Rationale
=========

POSIX Shell Command Language is obtuse, archaic, inefficient, difficult
to maintain, nearly impossible to secure, and extremely widely used. It
is also the only programming language that can be written in
near-real-time, ex: shell sessions are frequently continously extended
and shell history over time remains a fully valid executable. Text based
computer interfaces are arcane, through a combination of factors, some
of which were specific to the moment in time Shell Command Language was
stabilized, and some of which were due to the extremely portable design
of POSIX operating systems. As the landscape of computing, human
computer interacton, hardware, and interface design has changed
significantly since then, we see an opportunity to improve on the
efforts of Thompson, Bourne, Joy, Korn, Almquist, Fox and Herbert Xu.

Goals
=====

At time of writing *none* of these goals have been stabilized. The scope
of this project is extremely unclear and we should not proceed with
implementation until the *entire* language spec, standard library,
shell, and execution environment have been designed. We run the risk of
embarking on an journey we cannot complete, and the objective of this
design project should remain the same as the objective of this language:
any unneeded feature, any technical debt, any obscurity, is a failure.

Language Design Goals
---------------------

### all goals are secondary to ease of use and microscripting

-   the unnamed language will not expect familiarity with established
    programming techniques or theoretical computer science knowledge,
    only basic computing knowledge

    -   ex:c-style for loops
-   the unnamed language will be complex enough to reconstruct the most
    advanced shell command language programs
-   the unnamed language will be simple enough to support extensive
    static analysis
-   a novice user of the unnamed language will be able to strongly
    reason about the behavior of programs of moderate size without
    extensive study
-   the foreign function interface will be practical for novice users of
    the unnamed language who have experience in the language they are
    linking to

    -   eeeehhhhh, an ffi might be out of scope
-   the syntax will not depend on whitespace
-   correct style will be specified as part of the language
-   the syntax will not rely on symbol delimiters, nor will it overload
    standard symbols to mean anything other than their use in modern
    english literature
-   keywords will be selected in a manner consistent with a later
    translation, keyword for keyword, to most major spoken languages
-   utf-8 by default
-   easily translated to bytecode or assembly
-   all individual modules of code will require clearly specified
    interfaces

    -   also possibly not
-   there could possibly be a module system
-   YAML support
-   full string processing
-   regex engine inside the standard library

Unnamed Shell Design Goals
--------------------------

-   interpreted performance competitive with the most advanced
    interpreters of high level languages today
-   interactive onboarding process that can be completed in an hour and
    provide a computer-literate user with all the tools neccesary to
    reconstruct simple workflows from Windows 10, macOS 10.0, or Debian
    Jessie
-   native support for directly translating Shell Command Language
-   functionally run the majority of common existing programs for the
    shell
-   no dependence on llvm
-   no dependencies on *any* external libraries, excepting only the most
    stable portions of the Rust standard library
-   the shell will terminate on any error
-   the shell will terminate on any warning unless a directive exists
    stating otherwise
-   the shell will warn on style errors deemed indicative of lack of
    structure or style errors severe enough to make the application
    unmaintainable
-   the shell will warn on scripts with sufficient complexity in a
    module to warrant separation into multipe modules

Why not X Language?
===================

Many scripting languages exist today, all of which have been more
carefully developed and tested. Any language that could be used to
create system scripts should be listed here, along with the reasons why
it is not fit for native system scripting.

JavaScript
==========

-   Almost as weird as Shell

Python
======

-   verbose
-   whitespace-sensitive

Lua
===

Lua is the language that comes closest to being appropriate for system
scripting, but at the moment the system shell primarily deals with the
chaining of commands and the redirection of input and output. A system
shell would be best served by a language extension of Lua, but any
shell-tweaked Lua would not be cross-compatible with standard Lua. If
compatibility is broken, there are opportunities to reconsider language
design choices.
