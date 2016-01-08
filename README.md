# Brainfuck_AI
Using genetic algorithms to evolve programs written in brainfuck 

This project was inspired by [primaryobjects's](https://github.com/primaryobjects) [AI-Programmer](https://github.com/primaryobjects/AI-Programmer) repository. The goal of the project is to use genetic algorithms to allow the program to write brainfuck code to perform different tasks.

## The Language
[Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) is a relatively simple language. It stores data in an "infinitely" long array of bytes and has only 8 instructions. An interpreter updates the table after reading an instruction and has a data pointer to keep track of where in the array it is.

Instruction | Description
--- | ---
+ | Increment byte at data pointer
- | Decrement byte at data pointer
> | Increment data pointer
< | Decrement data pointer
. | Output byte at data pointer (as char)
, | Read byte of input and store in byte at data pointer
[ | If byte at data pointer is 0, move to the instruction after the matching ']'
] | If byte at data pointer is not 0, move to the instruction after the matching '['

### The Extension
For the purposes of this project, I have extended the original brainfuck language. Instead of having an array of bytes, the extended language keeps an array of signed 64-bit integers. Furthermore, in addition to the original 8 instructions, 4 new instructions have been added.

Instruction | Description
--- | ---
integer literal | The integer is stored in the value at data pointer
# | Read integer from input and store in value at data pointer
! | Output the value at data pointer (as an integer)
? | Reads a string from input and stores its chars in successive cells. The data pointer is not changed

## Outputted Programs
Currently, the project is set up to only write programs that output some target word. (Hopefully) In the future, more complicated will be possible.
### hi
These programs outputs the word hi (possibly follwed by additional output)

```
-]362--[.-+[[[[+.70<
```

```
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++[+++++++++++++++++++++++++++++++++++.----------------------------------]
```

### hello
This program outputs the word hello

```
877-----.---.+++++++..+++.++>[7773+<+7<77777[>7>>++7
```

## Relevant Details
* The interpreter only runs a program for a max of 10 milliseconds (in order to prevent infinite loops)
* The parser ignores mismatched brackets (Ex. ++>[--<. is the same as ++>--<.)
* Programs can be evolved without the ability to contains certain characters (Ex. the second "hi" program was unable to contain numbers)
* The evaluation function for the genetic algorithm only checks the output up to the length of the target word
