# Value Union

A proof-of-concept for my idea of a "value union", where a single variable can hold multiple values
of the same type.

It's distinct from a list in that the value of `a: list<int>` is not the integers themselves but
the list holding the integers. On the other hand, `a: value union<int>` does hold the integer values
directly and acts as if it were an integer, not a list.
