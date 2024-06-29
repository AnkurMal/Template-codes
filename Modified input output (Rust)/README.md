# Modified-Input-Output
A modified io library for rust, only in some aspects.

## For input
Modified the ```print!``` macro to ```printf!``` marco. Similar to ```print!```, just flushes the buffer as well.

## For output
Just simplified the input of a number to be a oneliner i.e. just call the ```input_num()``` function.

## How to use
Just add this lines at the start of your ```main.rs```:
``` rust
mod modio;
use modio::input_num;
```
