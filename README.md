# Overview

This is a simple maze generator implemented in Rust. The generator takes advantage of the Depth First Search Algorithmn to generate a simple square maze.

## Usage

To use this maze, simply follow the prompts on the command line interface. The first prompt will try to generate the longest maze possible in a given time frame while the second is more useful for generating a large amount of mazes. 

Output will be stored in a file (With a name of your choosing) with subdirectories "problems" and "solutions". Mazes are sorted by the lengths of the solution path, with longer more complex paths occuring in the mazes farther down the list. Problems contains blank mazes while solutions contains the solutions. 

The naming convention is as follows

problems
- Maze_<number>

solutions
- Maze_<number>_<number of steps in the solution>

[Software Demo Video](https://youtu.be/eyDfqUUxHxE)

# Development Environment

Developed in visual studio code. I used the image creator library to make the images.

# Useful Websites

{Make a list of websites that you found helpful in this project}

- no boiler plate youtube channel covers several aspects of rust development
- https://github.com/rust-lang/rustlings <- this repository teaches you rust

# Future Work

{Make a list of things that you need to fix, improve, and add in the future.}

- Deploy a web version of this project
- Generate other kinds of mazes
- Show real time generation in a graphic
