# Why not X Language?
Many scripting languages exist today, all of which have been more carefully developed and tested.  Any language that could be used to create system scripts should be listed here, along with the reasons why it is not fit for native system scripting.

# JavaScript
- Almost as weird as Shell

# Python
- verbose
- whitespace-sensitive

# Lua
Lua is the language that comes closest to being appropriate for system scripting, but at the moment the system shell primarily deals with the chaining of commands and the redirection of input and output. A system shell would be best served by a language extension of Lua, but any shell-tweaked Lua would not be cross-compatible with standard Lua. If compatibility is broken, there are opportunities to reconsider language design choices.