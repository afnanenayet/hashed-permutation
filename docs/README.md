# hashed-permutation

[![Build Status](https://dev.azure.com/afnanenayet/hashed-permutation/_apis/build/status/afnanenayet.hashed-permutation?branchName=master)](https://dev.azure.com/afnanenayet/hashed-permutation/_build/latest?definitionId=7&branchName=master)
[![crates badge](https://meritbadge.herokuapp.com/hashed-permutation)](https://crates.io/crates/hashed-permutation)
[![Documentation](https://docs.rs/hashed-permutation/badge.svg)](https://docs.rs/hashed-permutation)
![License](https://img.shields.io/crates/l/hashed-permutation/1.0.0.svg)

## Synopsis

This is an implementation of Andrew Kensler's hashed permutation, which allows
you to take an array of the elements [0 ... n) and shuffle it with no memory
overhead and very little computational overhead. This works by using a clever
hash function to effectively permute all of the elements in the array.

Basically, you get a nearly free method to shuffle a bunch of numbers that
doesn't require you to allocate a vector of size `n`, letting you sample the
set without replacement.

You can find the paper here: https://graphics.pixar.com/library/MultiJitteredSampling/paper.pdf.
I have a little writeup of how the algorithm works [here](https://afnan.io/posts/2019-04-05-explaining-the-hashed-permutation),
and [Timothy Hobbs](https://github.com/timthelion) made a nice writeup explaining how to use the library itself
[here](https://timothy.hobbs.cz/rust-play/hashed-permutation.html).
