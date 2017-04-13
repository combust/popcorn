# Popcorn

Popcorn is a library for executing parallel computation across different
hardware devices. Just think of all the kernels you'll be cooking up.

## Design Principles

* Buffer: Used to store your data across several devices. All operations
  on a buffer are asynchronous and return a Future. This means we never
block, even for buffer synchronization across devices.

## Thank You Collenchyma

The [Collenchyma](https://github.com/autumnai/collenchyma) codebase provided a great starting point for
Popcorn. The folks at Autumn.ai did a wonderful job pushing Rust forward
in the machine learning community.
