# Instructions

- Intcode program is a list of integers separated by commas.
- starting at Index 0 (opcode), which will be 1, 2, or 99
- Indicates what to do:
  - 1 = adds nums from two positions and stores result in 3rd position
    - 3 numbers after code are are the 3 positions - first two are the numbers to add, 3rd is the position to store the result at
  - 2 = same as 1 except it multiplies rather than adds
  - 99 = halt
  - Once code is complete, step forward 4 positions
