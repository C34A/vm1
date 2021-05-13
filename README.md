# vm1
A very minimal virtual machine, inspired by Apple 2 era computers, intended for retro game programming. Very WIP.

## features
- simple instructionset (i am lazy)
- built-in assembler
- windowed output
- subroutines
- keyboard input
- run from/save to file

## todo
- more inputs (time, ??)
- ??
- i'm honestly not sure

## usage

Either `cargo build --release` and use the binary in `target/release/vm1` or just use `cargo run`

```
$ vm1 run ./examples/game.vm1 # compile and run from source file
$ vm1 compile ./examples/game.vm1 # compile and write to "game.bm1"
$ vm1 run ./game.bm1 # execute an existing binary
```
Note that the `run` directive will use the file extension to determine if it is a binary. One can also give a more specific directive:
```
$ vm1 compilerun ./exampes/game.vm1 # compile and run from text file
$ vm1 exec ./game.bm1 # run from binary
```

## example
from `examples/alphabet.vm1`
```
set r0 65 ; 'A'
set r1 0xF9BF ; screen start addr
set r2 0xFFFF ; screen end + 1
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
set r1 0xF9BF ; back to start
jmp :loop
```

![a screenshot](https://github.com/C34A/vm1/blob/master/res/screenshot.png?raw=true)
