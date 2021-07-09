What is this?
=============

This is a thought experiment made manifest. What would happen if you had an
emulated CPU that used the same address space as the host machine. What would it
look like to use a RISCV CPU emulation to run scripting code?

Why?
====

Because I can!

But seriously, this means that the host environment can allocate
structures and pass pointers to them into the emulation and no address
translation is necessary.

Why Not?
========

A bad pointer dereference in the emulated environment **will** bring down
the host. If there is any chance of running untrusted code then this is not 
the way to do things. On the other hand if you are certain that the code you 
are going to run will not be doing anything naughty then you should be just fine.

This is the idea I'm playing with.

Results
=======

It seems to work, and work remarkably well especially considering I have not
gone to any effort to optimize the code.

Prior Art
=========

This code is based in part on the code found here https://github.com/takahirox/riscv-rust
