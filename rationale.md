# Rationale

POSIX Shell Command Language is obtuse, archaic, inefficient, difficult to maintain, nearly impossible to secure, and extremely widely used.  It is also the only programming language that can be written in near-real-time, ex: shell sessions are frequently continously extended and shell history over time remains a fully valid executable. Text based computer interfaces are arcane, through a combination of factors, some of which were specific to the moment in time Shell Command Language was stabilized, and some of which were due to the extremely portable design of POSIX operating systems. As the landscape of computing, human computer interacton, hardware, and interface design has changed significantly since then, we see an opportunity to improve on the efforts of Thompson, Bourne, Joy, Korn, Almquist, Fox and Herbert Xu.