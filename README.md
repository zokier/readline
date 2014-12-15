readline
========

This library wraps readline on Linux, editline on Mac, and implements the same
functionality on Windows (without add\_history()).

I'll probably add add\_history() later for Windows. add\_history() will be
automatically called by readline(); if you want to not have that, tell me. I
personally don't see the use, though.
