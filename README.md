Design Notes on Castel
======================

[see: here](https://www.youtube.com/watch?v=UFnmq5PPSc) "Emacs: a great
operating system, lacking only a decent editor" Castel aims to be a
tightly integrated working environment for personal use. Operating
systems tend toward the byzantine, but users spend the majority of their
time in only a few programs. User experience is fundamentally fragmented
by weak application coordination and crufty legacy abstractions. The
majority of the user experience should be centralized into a single
program, with a shared UI, interface and experience.

Core
----

### Scripting Language

1.  Rationale

    POSIX Shell Command Language is obtuse, archaic, inefficient,
    difficult to maintain, nearly impossible to secure, and extremely
    widely used. It is also the only programming language that can be
    written in near-real-time, ex: shell sessions are frequently
    continously extended and shell history over time remains a fully
    valid executable. Text based computer interfaces are arcane, through
    a combination of factors, some of which were specific to the moment
    in time Shell Command Language was stabilized, and some of which
    were due to the extremely portable design of POSIX
    operating systems. As the landscape of computing, human computer
    interacton, hardware, and interface design has changed significantly
    since then, we see an opportunity to improve on the efforts of
    Thompson, Bourne, Joy, Korn, Almquist, Fox and Herbert Xu.

2.  Goals

    At time of writing *none* of these goals have been stabilized. The
    scope of this project is extremely unclear and we should not proceed
    with implementation until the *entire* language spec, standard
    library, shell, and execution environment have been designed. We run
    the risk of embarking on an journey we cannot complete, and the
    objective of this design project should remain the same as the
    objective of this language: any unneeded feature, any technical
    debt, any obscurity, is a failure.

3.  Language Design Goals

    1.  all goals are secondary to ease of use and microscripting

        -   the unnamed language will not expect familiarity with
            established programming techniques or theoretical computer
            science knowledge, only basic computing knowledge

            -   ex:c-style for loops
        -   the unnamed language will be complex enough to reconstruct
            the most advanced shell command language programs
        -   the unnamed language will be simple enough to support
            extensive static analysis
        -   a novice user of the unnamed language will be able to
            strongly reason about the behavior of programs of moderate
            size without extensive study
        -   the foreign function interface will be practical for novice
            users of the unnamed language who have experience in the
            language they are linking to

            -   eeeehhhhh, an ffi might be out of scope
        -   the syntax will not depend on whitespace
        -   correct style will be specified as part of the language
        -   the syntax will not rely on symbol delimiters, nor will it
            overload standard symbols to mean anything other than their
            use in modern english literature
        -   keywords will be selected in a manner consistent with a
            later translation, keyword for keyword, to most major spoken
            languages
        -   utf-8 by default
        -   easily translated to bytecode or assembly
        -   all individual modules of code will require clearly
            specified interfaces

            -   also possibly not
        -   there could possibly be a module system
        -   YAML support
        -   full string processing
        -   regex engine inside the standard library

4.  Unnamed Shell Design Goals

    -   interpreted performance competitive with the most advanced
        interpreters of high level languages today
    -   interactive onboarding process that can be completed in an hour
        and provide a computer-literate user with all the tools
        neccesary to reconstruct simple workflows from Windows 10, macOS
        10.0, or Debian Jessie
    -   native support for directly translating Shell Command Language
    -   functionally run the majority of common existing programs for
        the shell
    -   no dependence on llvm
    -   no dependencies on *any* external libraries, excepting only the
        most stable portions of the Rust standard library
    -   the shell will terminate on any error
    -   the shell will terminate on any warning unless a directive
        exists stating otherwise
    -   the shell will warn on style errors deemed indicative of lack of
        structure or style errors severe enough to make the application
        unmaintainable
    -   the shell will warn on scripts with sufficient complexity in a
        module to warrant separation into multipe modules

5.  Why not X Language?

    Many scripting languages exist today, all of which have been more
    carefully developed and tested. Any language that could be used to
    create system scripts should be listed here, along with the reasons
    why it is not fit for native system scripting.

6.  JavaScript

    -   Almost as weird as Shell

7.  Python

    -   verbose
    -   whitespace-sensitive

8.  Lua

    Lua is the language that comes closest to being appropriate for
    system scripting, but at the moment the system shell primarily deals
    with the chaining of commands and the redirection of input
    and output. A system shell would be best served by a language
    extension of Lua, but any shell-tweaked Lua would not be
    cross-compatible with standard Lua. If compatibility is broken,
    there are opportunities to reconsider language design choices.

### Window System

-   layout engine and low-level css analogue
-   standard content markup format
-   castel language UI toolkit

### Document Editor

Applications
------------

### Data Package

eve looked interesting, otherwise something like R+Excel

### Browser

probably want to wait on servo, vimperator is the inspiration here

### Communications Package

host a decentralized image board that stores state over time as whatever
clients have in cache posts effectively as long as people keep visiting
them, but rarely viewed posts will drop out of the cache use a rolling
blockchain type thing to agree on postings/encode additional data try to
avoid the existence of full nodes but ensure that the platform is
effectively hosted nowhere regen user uuids automatically on every
communication for anonynimity but allow clients to send whatever uuid
they want, allowing for optional namig services that could also be
impersonated all the system apps should share a layout enginge a custom
layout engine implements some kind of abstraction for basic gui
elements, a useful language for describing documents and content, and a
minimal markup language that people can compile too the ideal is to
provide the flexibility of electron based apps in a more basic but more
performant situation a castel window toolkit should be modular enough
that builds of castel apps can be run on linux systems one chain per
s/mime type and namespace, which contains subchannels that can be
filtered? pay for post with gas? if all miners are pooled, even tiny
miners will be able to get new posting gas blocks should be lists of
file references and filesizes? need some way to implement content
reference counting alternately, chain per thread the basic foundation is
effectively a public anonymous forum atm i think the best way to do it
is a blockchain per thread i realized that blockchains broadcasting
transactions looked a lot like message posting using variable length
blocks, the block rewards would be composed out of a fee each client
pays per byte per post one post follows another in a distributed,
publicly available, censorship resistant format you can set the client
to randomize the identity key on every post, and it's tough to track
which node recieved a posting first ethereum has a neat scheme where
they ensure that contract code gets executed by putting a cost on each
computational operation I think the same would work for effectively
leasing disk space on other computers threads people stop subscribing to
die and cease to be publicly available ideally some kind of referencing
scheme could be more granular about only preserving posts people visit
hosting images inside the chain is only barely feasible though so i need
to figure out how to integrate file hosting in general so you pay to
post and miners get that reward by mining? and some kind of distributed
discovery mechanism yeah but text is so small compared to files that
effectively file posters fund the system how much you think it would be
for an image? ideally an average client, even a mobile one would be able
to mine enough for text posting no idea, theoretically it would end up
being a function of how cheap hard drives are i've always liked the
4chan structure a lot, because it cultivates pools of local knowledge
about specific subjects multi-topic image boards were effectively the
successor to usenet-type networks and from a public registry you can
build a lot of ancillary services if you dont regenerate your keys on
each post, you have optional secure identity, that could also be coupled
to the name registry system private rooms and threads are also a
critical feature a functional combination of distributed name discovery
and distributed file hosting allows a whole host of currently federated
applications to go full distributed those are both the tough parts
though chain threading requires many nodes to care about a thread for it
to continue existing which works fine for short lived topic threads and
a few superpopular fat threads it's also single commit, with no post
editing or content updating what do you think?

honestly single commit short lived could be a good thing. how easily
would you be able to search it? also how would privacy work with the
block chain? encryption? currently i think searching falls into two
stages, first is a public registry that all clients have to be connected
to that advertises thread chains and whatever information they want to
display then once you subscribe to a thread you keep a rolling copy on
your client and can search whatever content you have downloaded locally
or pay an archiver node to search its content for you i don't think it
can be done without two classes of node, client nodes that contribute
just enough to the network to keep posting and archiver nodes that are
run for profit ideally there would be no archiver nodes but who knows
also there isn't really a privacy goal, just an attempt at anonymity
message transactions are posted by randomly regenerated addresses, but
if someone picked up the message leaving your computer it would
obviously be you otoh as soon as the message hits the first node and
starts getting replicated it would be very difficult to figure out where
it came from and if you're remaining anonymous by regenerating keys you
can just trash whatever you used to sign the message and it can't be
attributed to you after the fact 3 things i can't figure out very well
yet: ethereum as a unit of exchange is dependent on ethereum not being
shit, and transaction costs have to remain really low to keep posting
price down

public name registry is an open problem, with something like namecoin
looking like a good contender, but namecoin is weird and not lightweight

this system would be pretty extensible and i want to see if the same
model could support other kinds of internet community models

-   pub/sub would allow single author content streams for blogging, paid
    content sites, and microblogging
-   the big one is making a large amount of updatable data available
    without forcing clients to save all of it, which would give
    distributed web hosting capability

fees for keeping something massively available on the archiver nodes
would need to be pretty cheap for people to switch from single server
hosting a low amount, low transaction cost system solves the problem of
incentivizing permanent hosts for distributed content, but only if the
economics are balanced very carefully
