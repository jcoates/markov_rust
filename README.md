# markov_rust [![Build Status](https://travis-ci.org/jcoates/markov_rust.svg?branch=master)](https://travis-ci.org/jcoates/markov_rust)
mini markov chain project created for senior seminar class

This was made as [part of a quick demo](http://slides.com/jcoates/markov#/). 

I then wanted to update some piece of code to rust 1.0, and then wanted to test travis-ci, so I did both with this repo.

#How to use this stuff

You need rust installed to build it, it was developed for rust 1.0.

From within this directory you can `cargo run` and get a command line style prompt.

It accepts the following commands:
* create <uint> - creates a markov chain of the specified order.
* train <path> - trains the markov chain on the text located at the path
* generate <uint> - generates the specified number of sentences using the current chain
* exit / quit / exit() - all allow you to exit the program
