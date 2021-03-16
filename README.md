# vm1
A very minimal virtual machine, inspired by Apple 2 era computers, intended for retro game programming. Very WIP.

## features
- simple instructionset (i am lazy)
- built-in assembler
- windowed output
- subroutines

## todo
- binary i/o
- keyboard input system
- more inputs (time, ??)
- ??

## example
from `examples/alphabet.vm1`
```
set r0 65 ; 'A'
set r1 31167 ; screen start addr
set r2 32767 ; screen end + 1
set r4 91 ; 'Z' + 1
loop:
store r0 @r1 ; write to "pixel"
inc r1 1 ; next addr
inc r0 1 ; next char
jlt r0 r4 :test ; don't go past 'Z'
set r0 65
test:
jlt r1 r2 :loop ; if at end, draw and reset drawing pointer
draw
set r1 31167
jmp :loop
```

![a screenshot](https://github.com/C34A/vm1/blob/master/res/screenshot.png?raw=true)
