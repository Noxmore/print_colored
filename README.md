A small crate to help me print colored text.

As an example here's the `cprint!` macro's doc:

# cprint!(...)

Prints something out with the specified formatting.

## Formatting

If you put the color formatting character in your input, followed by a formatting code, the text after that point will be formatted with that color.

The default formatting color character is `'§'` (if you have a num-pad type right-alt-down > 0167 > right-alt-up) but you can use an alternative character with `print_colored_with!`.

A formatting code is either an 'r' for resetting the color, or 2 hex digits, regarding the foreground and background colors respectively. The case of the formatting code does not matter.

### Digits
0 = Black, 1 = Dark Blue, 2 = Dark Green, 3 = Dark Cyan, 4 = Dark Red, 5 = Dark Magenta, 6 = Gold, 
7 = Gray, 8 = Dark Gray, 9 = Blue, a = Green, b = Cyan, c = Red, d = Magenta, e = Yellow, f = White

### Attributes
A formatting code can also be prefixed with attribute characters, the list of which are defined below.

L = Bold, M = Crossed Out, U = Underlined, I = Italic

## Examples

```rust
use print_colored::*;

fn main()
{
  cprint!("§ub0This is blue against black, underlined! §rAnd this is the terminal's default color!\n");

  let foo = 32;
  cprint!("§07This is Black against Gray! §rThis macro also supports arguments like these: {foo}\n");
}
```
