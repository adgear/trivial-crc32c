There's a more comprehensive CRC32C crate available at:

  - https://github.com/lemonrock/crc32c-sse42

But I had a few reasons for writing this one:

 - the workload I need crc32c for is many very small messages, and
   according to the original Intel paper, around 96 bytes is the point
   at which it even makes sense to start using tricks like `CLMUL`;
   (of course, don't trust me -- benchmark!)

 - I wanted something that didn't have a dozen C files and
   dependencies on yasm and so on to build.  This crate takes a
   simpler approach: you get an SSE4.2 implementation using inline
   assembly (and thus, requiring nightly rust).  If your CPU does not
   support these instructions, you lose.

I might add an explicitly simple fallback for debug builds running on
VMs or similar.

## License

BSD
