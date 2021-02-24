# vm1
A very minimal virtual machine, inspired by Apple 2 era computers, intended for retro game programming. Very WIP.

## features
- simple instructionset (i am lazy)
- built-in assembler
- windowed output

## todo
- command line parameters to compile/run from text files
- binary i/o
- subroutines
- keyboard input system
- ??

## example

```
set r0 1
inc r0 64
store r0 1
set r1 31167
set r2 32767
set r4 91
store r0 r1
inc  r1 1
inc r0 1
jlt r0 r4 11
set r0 65
jlt r1 r2 6
draw
```

![a screenshot](https://github.com/C34A/vm1/blob/master/res/screenshot.png?raw=true)
