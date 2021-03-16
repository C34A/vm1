
reg: a register. 0-256 i guess.
addr: a 15 bit address, top bit is 0.
caddr: a 15 bit code address. differentiated since code is stored in ROM by top bit being 1.


| code | op | description |
| ---- | -  | ----------- |
| 0    | nop | do nothing |
| 1    | add reg reg | add and store in former |
| 2    | sub reg reg | sub and store in former |
| 3 | inc reg val | add value to register value |
| 4 | dec reg val | decrement register by value |
| 5   | mult reg reg | mult, store in reg1 |
| 6   | div reg reg | div, store in reg1 |
| 7   | rem reg reg | remainder and store in former |
| 8   | load addr reg | load from address into reg |
| 9 | load reg reg  | load from address stored in reg1 and store data in reg2 |
| 10  | store reg addr | store from reg into address |
| 11 | store reg reg  | get address from reg 2, store reg 1 at that address in ram |
| 12  | jmp caddr | jump to code address (line) |
| 13 | jeq reg reg caddr | jump to code adress if equal |
| 14 | jgt reg reg caddr | jump if reg > reg |
| 15  | jlt reg reg caddr | jump if reg < reg |
| 16 | jsr caddr | jump to code address, push the current address to the stack |
| 17 | rsr | return to the last address on the stack + 1 |
| 18 | print addr | debug print? |
| 19 | print reg | debug print |
| 20 | set reg val | set a register to a literal value |
| 21 | draw | draw and update input |
