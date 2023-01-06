# Nody
A lisp-looking programming language for fun in Rust.

It is made to be pretty easy to understand and be rewritten in other programming lanuages.
Nody uses function overloading, meaning that a function can have different definitions depending on the arguments passed in.

# Content
- [Guide](#guide)
    - [Syntax](#syntax)
    - [Values and Types](#values-and-types)
- [Contact](#contact)

# Guide

You can look at some samples in the `samples/` directory.

## Syntax

Nody has a lisp-like syntax as already mentioned, having every function call wrapped in `(` and `)` to mark the end. Most programming languages have two parts for grammatical analysis, *lexing* and then *parsing*, but because the syntax of Nody is so simple, I decided to combine the two parts which I just called *scanning*. Instead of scanning the file for so called *tokens*, which the lexer does, the *scanner* instantly converts the source file to a tree-node structure. This saves some time on the pre-interpretation stage.

| name    | representation
|---------|---------------
| none    | `()`
| int     | `1`, `2`, ...
| float   | `1.5`, `0.1`, ...
| char    | `'a'`, `'b'`, ...
| bool    | `true` / `false`
| string  | `"..."`
| type    | `int`, `float`, ... see [types](#types)
| word    | `name`, `age123`, `<`, ... *any chars ended by white space*
| key     | `@name`, `@age123`, `@<`, ... *a word with `@` in front*
| node    | `(anything ...)` a head node with following arguements which can be any type of node
| body    | `{...}` a collection of any kind of node
| vector  | `[...]` a collection of any kind of node that return a value
| closure | `#...` a container for any kind of node saved as a closure value used for functions
| params  | `$(word type ...)` a collection of pairs of words and types or nodes that return types used for functions
| object  | `${word anything ...}` a collection of pairs of words and values

**Comments** are made by starting with a `;`. Until a new line starts, everything in-between will be ignored by *scanner*.

## Values and Types

The language is pretty strict with it's types, even though it's interpreted.

| name      | examples
|-----------|----------
| int       | `1`, `2`, ... any natural number
| float     | `1.5`, `0.1`, ... any decimal point number
| char      | `1.5`, `0.1`, ... any decimal point number
| bool      | `true` / `false`
| str       | `"..."` a string of chars
| vec       | `[...]` a sequence of values
| key       | `@...` a word as a value used for referencing
| index     | a index of a vector as a value used for referencing
| path      | a key of an object as a value used for referencing
| closure   | `#...` a container for any kind of node as a value used for functions
| params    | a key of an object as a value used for referencing
| fn        | a procedure that takes in parameters and a closure to execute that might return a value
| native-fn | like a `fn` but written in the interpreters programming language
| object    | `${age 18 ...}` a collection of pairs of words and values as a value
| type      | `int`, `float`, ... any type name in this list
| any       | a special type as it has no value of it's type, used for parameters as it matches with any other type

# Contact
**Discord**: `sty#8189`
**Gmail**: `reutervincent6@gmail.com`
Help is always welcome!