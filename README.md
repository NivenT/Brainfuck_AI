# Brainfuck_AI
Using genetic algorithms to evolve programs written in brainfuck 

This project was inspired by [primaryobjects's](https://github.com/primaryobjects) [AI-Programmer](https://github.com/primaryobjects/AI-Programmer) repository. The goal of the project is to use genetic algorithms to allow the program to write brainfuck code to perform different tasks.

## The Language
[Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) is a relatively simple language. It stores data in an "infinitely" long array of (unsigned) bytes and has only 8 instructions. In addition, it has an input stream and an output stream for basic I/O. An interpreter updates the table after reading an instruction and has a data pointer to keep track of where in the array it is.

Instruction | Description
--- | ---
+ | Increment byte at data pointer
- | Decrement byte at data pointer
> | Increment data pointer
< | Decrement data pointer
. | Output byte at data pointer (as ASCII encoded char)
, | Read byte from input and store in current location
[ | If byte at data pointer is 0, move to the instruction after the matching ']'
] | If byte at data pointer is not 0, move to the instruction after the matching '['

### The Extension
For the purposes of this project, I have extended the original brainfuck language. In addition to the table, the language now has a single byte of memory that can be written to or read from at any time. Along with this, new instructions have been added.

Instruction | Description
--- | ---
0-F | 16 times the hexadecimal digit is stored at the current location (Ex. 5 writes 80 to current position)
! | Output the value at data pointer (as an integer)
@ | Store the value at data pointer in memory
* | Replace value at data pointer with value in memory

## Outputted Programs
### hi
This program outputs the word hi

```
6++++++++.+.[>6
```

### hello
This program outputs the word hello

```
7--------.---.7----..[]]D77-.-3D-
```

### loop
This program continuously outputs the word loop (The target was "looplooploop").

```
[E++[[4[[7----.+++..+.]]D,]DD
````

## Details
* The interpreter only runs a program for a maximum of 2 milliseconds (in order to prevent infinite loops)
* The parser ignores mismatched brackets (Ex. ++>[--<. is the same as ++>--<.)
* Empty loops are ignored (Ex. ++[].-. is the same as ++.-.)
