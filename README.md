# ts-arena

`ts-arena` is yet another Rust crate for performing [arena allocation][], where
a large number of instances of the same type can be allocated with low overhead.

[arena allocation]: https://en.wikipedia.org/wiki/Region-based_memory_management

## Homogeneous, ID-based, no deletion

Similar to the [`id-arena`][] crate, each `Arena` is used to allocate instances
of a single type.  When you allocate a new object, what you get back is an
_identifier_ or _handle_ to the object, and not a reference to it.

Moreover, you cannot individually delete objects that are allocated via an
`Arena`.  Instead, _all_ objects allocated via an `Arena` are freed when the
arena itself is freed.

[`id-arena`]: https://docs.rs/id-arena/

## Thread-safe

Unlike [`id-arena`][], the `Arena` class defined in this crate is fully (and
efficiently) thread-safe.  The `Arena` itself only provides access to existing
objects in the arena.  To allocate new instances, you must use a `Context`.  You
create separate `Context` objects for each thread / task / worker / etc.  Most
of the time, allocating a new object via a `Context` requires no thread
synchronization; we only have to synchronize when when a `Context`'s "current
chunk" fills up, and it needs to acquire a new chunk from its `Arena`.

## Optional interning

We also provide an `InternedArena`, which provides all of the same capabilities
as above, but moreover ensures that there are no "duplicate" objects in the
arena.  This allows you to use the handles themselves as stable identifiers for
your objects — equal objects will be guaranteed to have equal handles.
