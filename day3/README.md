# Challenge 3

The first challenge that requires a bit of thought. There are two problems:

1. Extract the non-corrupted operations
2. Evaluate the operations

There's a decent chance part 2 will involve other operations, so let's keep it vaguely generic.

> I was right. Extending was nice and easy as a result

I can parse as normal using nom, using a alt(parse_op1, parse_op2, parse_op3, parse_corrupted).
