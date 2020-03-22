# advent_of_code_2019

my code for advent of code 2019 (https://adventofcode.com/2019)

This is my second year doing aoc. Last year I used python (the language I'm most familiar with).

This year, I'm trying in rust. I'm quite new at rust, so the code quality is likely to be poor/slow/inefficient.

## Status Update - End of Advent

As of boxing day, I have 36/50 stars.

This is not as good as last year, when I had (I think) 45/50 by boxing day; but I don't attribute the difference to using Rust. Last was my first year, so I was much more eager and determined. This year, towards the end, I found myself looking at the day's description and thinking "ugh, I'll do it later"

I do intend to go back and finish off those remaining stars 'eventually'

[UPDATE; 2020-03-22] - I probably won't

As for Rust, I actually found it really pleasant to work with. Going into this advent, I expected it might take more effort than python (especially since I haven't used it much). But actually, I didn't find it to be that much harder than doing it in python.

I think using VS Code with RLS was a big help in that regard. Seeing potential issues highlighted as you go is very useful. I think if I'd had to do 'write some code, compile, fix errors, repeat', I probably would have gotten frustrated very quickly.

Admittedly, the code itself is quite sloppy. I used isize/usize for all the integers, did error 'handling' via panic/unwrap, and was pretty reckless with memory usage. Not that any of that mattered here, it still worked perfectly well.

One thing I liked in particular was using the 'type state' pattern in the IntCode computer. This is pattern that is unique to Rust. For those unfamiliar, you essentially construct a state-machine where each state is its own type; but the clever part is using Rust's ownership system when 'trasforming' between states to effectively invalidate the previous state.

In the context of IntCode, I used type-states to encode the 'waiting for input', 'output available', and 'halt' states. The result is, if the computer is in e.g. 'waiting for input' state, then the only thing you can do (the only method available) is provide an input value, and the compiler itself enforces that the computer can't continue until an input has been provided. This is very useful for eliminating a mistake that would be very easy to miss.

I also had much fun with iterators, which is 'possible' in python, but much less ergonomic (without introducing a 3rd party lib)

All in all, I have enjoyed using Rust, and would love to use it for a more 'serious' project.
