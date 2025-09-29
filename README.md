# vortex-vm

## description

vortex-vm is a simple stackbased vm wich executes code(duh). it is 32 bit based

## instructions

**instructions are case insensitve**
1. null          -         just increases the instruction pointer
1. push          -         pushes a value to the stack
1. pop           -         pops a value from the stack
1. ret           -         returns with the top most value from the stack
1. jnz           -         jumps if the top most value is not zero
1. jiz           -         jumps if the top most value is zero
1. add           -         adds the specified value to the top of the stack
1. sub           -         subtracts the specified value from the top of the stack
1. mult          -         multiplies the second most value with the top most value
1. mults         -         multiplies the top most value with a specified value
1. div           -         divides the second top most value with the top most value
1. divs          -         divides the top most value with a specified value
1. dup           -         duplicates the top element
1. swap          -         swaps the top two elements of the satck

see examples in the examples folder for more information
