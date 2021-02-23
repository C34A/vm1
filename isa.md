
reg: a register. 0-256 i guess.
addr: a 16 bit address.
caddr: a 16 bit code address. differentiated since code is stored in ROM


| code | op | description |
| ---- | -  | ----------- |
| 0    | nop | |
| 1    | add reg reg | add and store in former | 
| 2    | sub reg reg | sub and store in former |
| 3    | mult reg reg | " |
| 4    | div reg reg | " |
| 5    | rem reg reg | remainder and store in former |
| 6    | load addr reg | load from address into reg |
| 7    | store reg addr | store from reg into address |
| 8    | jmp caddr | jump to code address (line) |
| 9    | jeq reg reg caddr | jump to code adress if equal |
| 10   | jgt reg reg caddr | jump if reg > reg |
| 11   | jlt reg reg caddr | jump if reg < reg |
| 12   | print addr | print? |
| 13   | set reg val | set a register to a literal value |
