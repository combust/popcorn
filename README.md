# Popcorn

Popcorn is a library for executing parallel computation across different
hardware devices. Just think of all the kernels you'll be cooking up.

## Design Principles

Some basic design choices of Popcorn.

### Buffers

A buffer is a piece of memory that can be stored in one or more devices.
Depending on the different backend APIs used, buffers will be
synchronized between different devices as needed. Buffers always have a
"latest copy", which indicates the most up-to-date version of the buffer
and which device it is on. This is used to determine if an actual sync
is needed or not when executing an operation on the data.

### Future-Based API

Never block if we can help it. OpenCL, CUDA and your CPU offer
asynchronous memory synchronization between the various devices. This
means we can abstract synchronization events as futures. This means a
synchronization event between devices won't block the calling thread, so
it can continue to queue other commands for different buffers.

Buffers are consumed into a future by every operation. This ensures that
you can only ever access the most up-to-date version of the buffer via
the future that is returned for every operation.

### Generic

Popcorn is generic across a set of supported devices: OpenCL, CUDA, CPU,
and Raspberry Pi GPU. We focus on generic use first and then on
device-specific optimizations.

## Thank You Collenchyma

The [Collenchyma](https://github.com/autumnai/collenchyma) codebase provided a great starting point for
Popcorn. The folks at Autumn.ai did a wonderful job pushing Rust forward
in the machine learning community.
