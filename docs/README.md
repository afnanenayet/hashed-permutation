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

Basically, you get a nearly free method to shuffle a bunch of numbers. This
allows you to sample with no replacement, without needing to save all of the
numbers in the range (0...n).

You can find the paper here: https://graphics.pixar.com/library/MultiJitteredSampling/paper.pdf
