# hashed-permutation

[![Build Status](https://dev.azure.com/afnanenayet/hashed-permutation/_apis/build/status/afnanenayet.hashed-permutation?branchName=master)](https://dev.azure.com/afnanenayet/hashed-permutation/_build/latest?definitionId=7&branchName=master)

## Synopsis

This is an implementation of Andrew Kensler's hashed permutation, which allows
you to take an array of the elements [0 ... n) and shuffle it with no memory
overhead and very little computational overhead. This works by using a clever
hash function to effectively permute all of the elements in the array.

You can find the paper here: https://graphics.pixar.com/library/MultiJitteredSampling/paper.pdf
