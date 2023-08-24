Step 2: Idioms
==============

These steps describe common idioms required for writing well-designed and idiomatic [Rust] code.

> ❗️Before completing this step you should complete all its sub-steps.

After doing them you should be able to answer the following questions:
- Why should I care about types and expressing things in types? How do types help to increase guarantees of a program being correct?
- What is essential for writing well-designed and ergonomic APIs in [Rust] and why?


> The essential thing is to use Idiomatic code, thus using special Idioms developed by Rust community and team.
> It's something like "best practices" - the best ways to layout your code in specific and general cases.
> This also makes implicit/explicit convention among developers and allows to avoid misunderstanding in meaning of written code.


- Why `mem::replace` exists and what purpose does it serve?
> `mem::replace` allows us to extract some data directly in the memory,
> and put another of the same type in its place, returning original one.
> The purpose of `mem::replace` is to give a more fast and direct way
> to manipulate whole values/data without introducing additional allocations manually.

- When and why is it really helpful?
> It is really helpful in cases where some data is a part of some data structure,
> what introduces some Borrow Checker restrinctions when we need to move the value out.
> `mem::replace` and a few other `mem::`-family items allow us to get around such edge cases without actually violating OBRM rules.


- How input type polymorphism is usually organized in [Rust] APIs?
> Using generics and bounds.


- What cost does it have?
> Increased binary size.


- Which ways and tools do exist for future-proofing source code in [Rust]?
> As for tools - those are usually crates from crates.io. (like `sealed` , )
> As for direct Rust-lang features those are
    - `non_exhaustive`, // for cases when some type can be extended in future
    - `exhausiveness` // making sure that structure does not used if it was changed somehow
    - Trait sealing Jutsu! // to prevent implementing trait methods/traits from implementing custom behaviour on custom types


## Task

__Estimated time__: 2 days




Design and implement a `VendingMachine` type, which behaves like a [vending machine][1]:
- `Product` should has a price and a name;
- `VendingMachine` should have a limited capacity of `Product`s;
- `VendingMachine` should be able to give a rest;
- `VendingMachine` should reject purchase if it cannot give a rest;
- `Coin`s could have only `1`, `2`, `5`, `10`, `20` and `50` nominals.

Make its usage API as convenient as you're capable to.




[Rust]: https://www.rust-lang.org

[1]: https://en.wikipedia.org/wiki/Vending_machine
[2]: https://doc.rust-lang.org/book/ch11-03-test-organization.html
[3]: https://youtu.be/Vw8BFScm0K0
