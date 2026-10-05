# Overview

descent is a command-line tool written in Rust that learns the relationship between pizza diameter and price using linear regression, trained with gradient descent written from scratch. No ML libraries: the prediction math, the error measurement, and the parameter updates are all hand-written.

How it works: the program starts with a flat line (slope 0, intercept 0) and improves it one epoch at a time. Each epoch predicts every pizza's price with y = mx + b, measures how wrong the line is with mean squared error, then nudges the slope and intercept a little against the error, scaled by the learning rate. After hundreds of epochs the line threads through the data. The evolving line and the shrinking loss are drawn live in the terminal as ASCII art. A stretch mode trains the same data with three learning rates side by side to show slow convergence, good convergence, and divergence.

My purpose in writing this was to understand gradient descent from the inside out, and to learn Rust by building something real with it, instead of calling a library and trusting the magic.

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

* VS Code with the rust-analyzer extension
* Rust (cargo) command-line tools, edition 2021
* Git and GitHub for version control

# Useful Websites

* [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - the official guide, how I learned ownership, structs, and error handling
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - quick runnable examples of syntax

# Future Work

* Let the user pass the learning rate and epoch count as command-line arguments instead of hardcoding them
* Read the pizza data from a CSV file instead of hardcoding the lists
* Try a second feature (for example, number of toppings) and watch the math grow from a line to a plane
