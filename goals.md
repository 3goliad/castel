# Goals

At time of writing *none* of these goals have been stabilized. The scope of this project is extremely unclear and we should not proceed with implementation until the *entire* language spec, standard library, shell, and execution environment have been designed. We run the risk of embarking on an journey we cannot complete, and the objective of this design project should remain the same as the objective of this language: any unneeded feature, any technical debt, any obscurity, is a failure.


## Language Design Goals
### all goals are secondary to ease of use and microscripting

- the unnamed language will not expect familiarity with established programming techniques or theoretical computer science knowledge, only basic computing knowledge
    - ex:c-style for loops
- the unnamed language will be complex enough to reconstruct the most advanced shell command language programs
- the unnamed language will be simple enough to support extensive static analysis
- a novice user of the unnamed language will be able to strongly reason about the behavior of programs of moderate size without extensive study
- the foreign function interface will be practical for novice users of the unnamed language who have experience in the language they are linking to
    - eeeehhhhh, an ffi might be out of scope
- the syntax will not depend on whitespace
- correct style will be specified as part of the language
- the syntax will not rely on symbol delimiters, nor will it overload standard symbols to mean anything other than their use in modern english literature
- keywords will be selected in a manner consistent with a later translation, keyword for keyword, to most major spoken languages
- utf-8 by default
- easily translated to bytecode or assembly
- all individual modules of code will require clearly specified interfaces
    - also possibly not
- there could possibly be a module  system
- YAML support
- full string processing
- regex engine inside the standard library

## Unnamed Shell Design Goals

- interpreted performance competitive with the most advanced interpreters of high level languages today
- interactive onboarding process that can be completed in an hour and provide a computer-literate user with all the tools neccesary to reconstruct simple workflows from Windows 10, macOS 10.0, or Debian Jessie
- native support for directly translating Shell Command Language
- functionally run the majority of common existing programs for the shell
- no dependence on llvm
- no dependencies on *any* external libraries, excepting only the most stable portions of the Rust standard library
- the shell will terminate on any error
- the shell will terminate on any warning unless a directive exists stating otherwise
- the shell will warn on style errors deemed indicative of lack of structure or style errors severe enough to make the application unmaintainable
- the shell will warn on scripts with sufficient complexity in a module to warrant separation into multipe modules