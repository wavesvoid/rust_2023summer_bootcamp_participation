Step 0: Building Up an Vocabulary
========================================

__Estimated time__: 4 days

Read through [Rust Book], [Rust FAQ], and become familiar with basic [Rust] concepts, syntax, memory model, type and module systems.
Polish your familiarity by completing [Rust By Example] and [rustlings].
Read through [Cargo Book] and become familiar with [Cargo] and its workspaces.
After completing these steps, you should be able to answer (and understand why) the following questions:

- What memory model [Rust] has? Is it single-threaded or multiple-threaded? Is it synchronous or asynchronous? What is the memory layout of the box and vector? What are heap and stack? Where, but on heap and stack data could live in RAM?
- What runtime [Rust] has? Does it use a GC (garbage collector)?
- What statically typing means? What is a benefit of using it? Weak typing vs strong typing? Implicit / explicit?
- What are generics and parametric polymorphism? Which problems do they solve?
- What is nominative typing and structural typing? What is difference?
- What are traits? How are they used? How do they compare to interfaces? What are an auto trait and a blanket impl? Uncovered type? What is a marker trait?
- What are static and dynamic dispatches? Which should I use, and when? What is monomorphisation?
- What is a crate, module and package in Rust? How do they differ? How are the used? What is workspace?
- What is cloning? What is copying? How do they compare? What is for trait drop? What is special about the trait?
- What is immutability? What is the benefit of using it? What is the difference between immutability and const?
- What are move semantics? What are borrowing rules? What is the benefit of using them?
- What is RAII? How is it implemented in [Rust]? What is the benefit of using it?
- What are lifetimes? Which problems do they solve? Which benefits do they give?
- What is an iterator? What is a collection? How do they differ? How are they used?
- What are macros? Which problems do they solve? What is the difference between declarative and procedural macro?
- How code is tested in [Rust]? Where should you put tests and why?
- What is special about slice? What is layout of Rust standard data types? Difference between fat and thin pointers?
- Why [Rust] has `&str` and `String` types? How do they differ? When should you use them? Why str slice coexist with slice? What is differnece between `String` and `Vec`?
- Is [Rust] OOP language? Is it possible to use SOLID/GRASP? Does it have an inheritance? Is Rust functional language? What variance rules does Rust have?



_Additional_ articles, which may help to understand the above topic better:
- [Chris Morgan: Rust ownership, the hard way][1]
- [Adolfo Ochagavía: You are holding it wrong][23]
- [Vikram Fugro: Beyond Pointers: How Rust outshines C++ with its Borrow Checker][30]
- [HashRust: A guide to closures in Rust][24]
- [Ludwig Stecher: Rusts Module System Explained][2]
- [Tristan Hume: Models of Generics and Metaprogramming: Go, Rust, Swift, D and More][3]
- [Jeff Anderson: Generics Demystified Part 1][4]
- [Jeff Anderson: Generics Demystified Part 2][5]
- [Bradford Hovinen: Demystifying trait generics in Rust][25]
- [Brandon Smith: Three Kinds of Polymorphism in Rust][6]
- [Jeremy Steward: C++ & Rust: Generics and Specialization][7]
- [x] [cooscoos: &stress about &Strings][8]
- [x] [Jimmy Hartzell: RAII: Compile-Time Memory Management in C++ and Rust][9]
- [x] [Trait Drop][10]
- [Common Lifetime Misconception][11]
- [x] [Visualizing Memory Layout][12]
- [x] [Package vs. Crate terminology (r/rust)][13]
- [x] [Packages and crates (Rust wiki)][14]
- [x] [Full list of available crates on Rust Playground][16]
- [x] [Blanket impl definition][17]
- [x] [Auto-trait definition][18]
- [Georgios Antonopoulos: Rust vs Common C++ Bugs][21]
- [Yurii Shymon: True Observer Pattern with Unsubscribe mechanism using Rust][22]

Additional:
- [Rust API guidline checklist][19]
- [Interview Questions on Rust Programming][20]
- [Step-by-step instruction to start development in Rust][26]
- [Awesome collection of crates for productive development in Rust][27]
- [Awesome Collection of Materials to Learn Rust][28]

[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Rust]: https://www.rust-lang.org
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust FAQ]: https://prev.rust-lang.org/faq.html
[rustlings]: https://rustlings.cool

[1]: https://chrismorgan.info/blog/rust-ownership-the-hard-way
[2]: https://aloso.github.io/2021/03/28/module-system.html
[3]: https://thume.ca/2019/07/14/a-tour-of-metaprogramming-models-for-generics
[4]: https://web.archive.org/web/20220525213911/http://jeffa.io/rust_guide_generics_demystified_part_1
[5]: https://web.archive.org/web/20220328114028/https://jeffa.io/rust_guide_generics_demystified_part_2
[6]: https://www.brandons.me/blog/polymorphism-in-rust
[7]: https://www.tangramvision.com/blog/c-rust-generics-and-specialization#substitution-ordering--failures
[8]: https://cooscoos.github.io/blog/stress-about-strings
[9]: https://www.thecodedmessage.com/posts/raii
[10]: https://vojtechkral.github.io/blag/rust-drop-order/
[11]: https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md
[12]: https://www.youtube.com/watch?v=rDoqT-a6UFg
[13]: https://www.reddit.com/r/rust/comments/lvtzri/confused_about_package_vs_crate_terminology/
[14]: https://rustwiki.org/en/book/ch07-01-packages-and-crates.html
[16]: https://github.com/integer32llc/rust-playground/blob/master/compiler/base/Cargo.toml
[17]: https://doc.rust-lang.org/reference/glossary.html#blanket-implementation
[18]: https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits
[19]: https://rust-lang.github.io/api-guidelines/checklist.html
[20]: https://iq.opengenus.org/questions-on-rust/
[21]: https://geo-ant.github.io/blog/2022/common-cpp-errors-vs-rust
[22]: https://web.archive.org/web/20230319015854/https://ybnesm.github.io/blah/articles/true-observer-pattern-rust
[23]: https://ochagavia.nl/blog/you-are-holding-it-wrong
[24]: https://hashrust.com/blog/a-guide-to-closures-in-rust
[25]: https://gruebelinchen.wordpress.com/2023/06/06/demystifying-trait-generics-in-rust
[26]: https://github.com/rust-lang-ua/learn_rust_together/blob/master/introduction.md
[27]: https://github.com/rust-lang-ua/learn_rust_together/blob/master/toolbox_general.md
[28]: https://github.com/rust-lang-ua/learn_rust_together/blob/master/learn.md
[29]: https://github.com/Learn-Together-Pro/ComputerScience/blob/master/cheatsheets.md#asynchronous-vs-multithreading
[30]: https://dev.to/vikram2784/beyond-pointers-how-rust-outshines-c-with-its-borrow-checker-1mad

---


# QA
### 1. What memory model Rust has? Is it single-threaded or multiple-threaded? Is it synchronous or asynchronous?
#### 1.1. What memory model does [Rust] have? (/has Rust)
According to Wikipedia and other sources on the web, the term "memory model" can be assembled as:

> A rules and capabilities of language that describe how memory is used and shared in parallel computations.

If we refer to the Rust Reference, in [chapter 13](https://doc.rust-lang.org/stable/reference/memory-model.html?highlight=memory%20model#memory-model) we would find that, actually Rust, surprisingly has no "Memory ModeL" defined. (still).
Though it does not exactly mean it it does not use one.
Rust's std lib exposes tools to make concurrent/parallel programming possible and safe: such as threads and atomic pointers.


#### 1.2. Is it single-threaded or multiple-threaded?
We have the ability to use threads thanks to std.
We "may" as well use single-threaded approach **by default**.

The main principles are not that different from broad approach:
- shared memory is stored on the heap and is managed by Smart Pointers in safe Rust.
- Pointers istself are stored on the stack
- each thread has its own stack being allocated on creation


#### 1.3. Is it synchronous or asynchronous?
Rust has "interface" implemented for this purposes, so there kinda "is support" for the asynchronous code.
Still this is duty lies on the developer shoundelrs:
though Rust has interface defined it does not have a literal implementation.

There are libraries like `tokio` that do implement **asynchronous runtime**.
Using such libraries you are able to facilitate lib's features to run code asynchronously.


< -----------------------------------------------------------------------------------------

### 2. What runtime Rust has? Does it use a GC (garbage collector)?
#### 2.1. What runtime Rust has?
Rust has pretty much so small runtime it is often said "that it does not".
In fact, any language has to have some runtime to implement some basic features it is designed with.

Returning back to Rust, you can eliminate runtime impact in such ways as:
- excluding std lib with `#[no_std]`
- setting some configuration options for cargo/rustc to simplify resulting binary
    - like disable stack unwinding
    - enable link-time optimisations
    - exclude debugging facilities

#### 2.2. Does it use a GC (garbage collector)?
Rust does not have a conception of GC in its terminology, meaning there is no such thing as GC in Rust.

Rust have a different approach to memory management called OBRM - ownership-based resource management.

We could say that it is some kind of superset over the C++'s RAII.
Rust OBRM "borrows"))) some safe stuff from the RAII.
As well as introduces its unique features that make Rust so awesome and safe.


< -----------------------------------------------------------------------------------------


### 3. What statically typing means? What is a benefit of using it? Weak typing vs strong typing? Implicit / explicit?
#### 3.1. What statically typing means?
A "static typing" describes a kind of "fixated" type system behaviour if i may say.
Meaning that types checks (verifying of proper usage of types) are performed at compile time.


#### 3.2. What is a benefit of using it?
The benefits of such type system would be:
- safety, 
- readability/clarity (it is more comprehensive i.e. when)
- less errors and bugs in production (at runtime)


#### 3.3. Weak typing vs strong typing?
Well, actually (as far as i know) there is no well-defined standartised explanation
and every can express meaning from his unique perspective.

But generally what is meant by "strongly" or "weakly" considering a context of "statically typed" lang:
- strongly-typed system would limit a possible set of workarounds and manipulations with types.
Imposing a strict rules on how and where concrete types can be used.

- weakly-typed system has no such restrctions and does allow some flexibility:
   such as implicit type-casting, even at runtime

#### 3.4. Implicit / explicit?
By explicit/implicit typing we specfify (from my view) how is it obvious where and what type is being used.

Rust does have some implicit shenaningans concerning explicitness of typing.
This can be called **"type inference"**.

That means Rust is capable of infering type in some cases, i.e.:
```rust
fn main() {
    // will be inferred as &str
    let me = "leave";
}
```


< -----------------------------------------------------------------------------------------


### 4. What are generics and parametric polymorphism? Which problems do they solve?
#### 4.1. What are generics and parametric polymorphism?
Generics are the way to use generalized type aliases in arguments, parameters and implementations over types.

By "generalized" i mean that types are not undefined at the time of **declaration**, 
but are substituted with a concrete types **in place** **where they are used** - at compile time.

Such a feature of compiler in Rust is called "monomorphisation":
- the compiler performs checks of places where generic is used and analyses concrete types,
- after that it generates inplace a concrete declaration with each found concrete type.


**Parametric polymorphism** is about being able to define single implementation/declaration different types.
In other words - ablility to use different types in at place of where some specific type has to be.


#### 4.2. Which problems do they solve?
They reduce the amount of handwritten boilerplate.
They add more flexibility, readability, code comprehension by a programmer.

That's how far i'm capable to foresee their usefulness and usage at this moment.


< -----------------------------------------------------------------------------------------


### 5. What is nominative typing and structural typing? What is difference?
#### 5.1. What is nominative typing and structural typing?
* **Nominative typing** system
    goes by what name type has. It is guided by type declarations and thus deteermines a specific type.

* **Structural typing** system on the other hand
    goes by internal type structure. It determines a type from what a type actually is (what structure does it have).

Rust has Nominal Typing system. It won't allow differnt-named types with same structure to be used where type with same structure is required.
More than that there is no such thing as "required structure" in context of types.
A similar in some way would be a concept of **traits** and **trait bounds**,
with their help along with generics we can limit or extend types which can be passed in particular place.
Still this is only available until code is expanded by compiler frontend.


< -----------------------------------------------------------------------------------------


### 6. What are traits? How are they used? How do they compare to interfaces? What are an auto trait and a blanket impl? Uncovered type? What is a marker trait?
#### 6.1. What are traits?
Traits is the Rust's inherent concept (at least at my opinion).
It allows to define a behaviour of types.

A trait by itself is simply an aggregation of method signatures along with their optional default implementations.
A trait has to be implemented for a specific/generic type.
Than it is said the type implements that particular trait.


#### 6.2. How are they used?
They are used specifically to expose a specifically described behaviour on types.
Then, after implementation implemented methods/associated functions can be called on a Type instance/Type.

The syntax is as follows:
```rust
trait MyTrait {
    fn assoc_function(no_self_here: bool) {
        println!("Default implementation");
    }
    fn non_default_method();
}

struct MyType;

impl MyTrait for MyType {
    fn non_default_method() {
        println!("METHOD");
    }
}

fn main() {
    let my_type = MyType;
    my_type.non_default_method();

    MyType::non_default_method(&my_type);
    MyType::assoc_function();
}
```


#### 6.3. How do they compare to interfaces?
They are pretty much like interfaces in other languages.
But traits can also be implemented for foreign types (types not implemented by you).
In contrast interfaces can be implemented only for 


#### 6.4. What are an auto trait and a blanket impl?
- **Auto Traits**
An auto traits are specific traits that are implemented automatically by compiler for specific types under specific conditions.
These traits are:
- Sync
- Send
- Unpin
- UnwindUnsafe
- RefUnwindUnsafe

They are automatically implemented for (yes i remembered this):
- `[T; N], [T], &T, &mut T, *const T, *mut T` - if `T` does implement such trait
- function items types and function pointers
- Structs, tuples, enums, unions - if each of their fields implement a particular trait
- closures, if all the captured values types do implement a particular trait


- **Blanket implementation**
A blanked is called an implementation where a generic type is met uncovered.
Meaning it "covers" all the possible types with this implementation, not a specific one.

This type can have or have no bounds.
I.E. `impl<T: CustomType> String for T`

A type can only have a single implementation.
Trying to to define another implementation will raise error.


#### 6.5. Uncovered type?
It's a type that is not a part of other type (it is not covered by other type, such as `Vec<T>`).
It is worth to elaborate that i.e.
`impl<T> MyStruct<T> for Vec<T>` is also a blanket implementation

`T` here is cosidered uncovered, because:
- it is specified as generic type for `MyStruct`
- thus it is not actually a part of that type in this context but rather as a specific independent type.
    - it is passed as `T`, not as `<T>`, this is a reason why it is uncovered in this context


#### 6.6. What is a marker trait?
Marker traits are as they sound like: they are used to mark a type to have a specific properties.
Marker trait is not obligated to implement any behaviour.
It is just as it is : a marker.

They act as hints for the compiler that a particular type has some special properties that a compiler can take into consideration and change its behaviour accordingly.


< -----------------------------------------------------------------------------------------


### 7. What are static and dynamic dispatches? Which should I use, and when? What is monomorphisation?
#### 7.1. What are static and dynamic dispatches?
Static dispatch is a compile-time function addressation.
Dynamic dispatch is when an address of the function is resolved at runtime dynamically.
This of course lays out some performans overhead.


#### 7.2. Which should I use, and when?
A static dispatch can be used when you have a determined logic of calls in your code which is knonw at compile time.
Whether a dynamic dispatch (like in case of trait objects) can be used in case where it is not clear at compile time which method/function should be invoked at specific place but it is defined dynamically at runtime.


#### 7.3. What is Monomorphization?
Monomorphization is a process of reduction of generic form to its specific forms within concrete types.
In context of Rust it is a process of generating a code with type replaced for all types that are used in code.

Generic code with generic type turns into a code with concrete type replaced for every type where a generic is used.


< -----------------------------------------------------------------------------------------


### 8. What is a crate, module and package in Rust? How do they differ? How are the used? What is workspace?
#### 8.1. What is a crate, module and package in Rust?
- A crate is a basic (single) compilation unit in rustc and cargo. 
We can say it represents all codebase that will then gets compiled into a binary or used elsewhere as a library.
It can be as much as `main.rs` with "hello, world" or some large codebase splitted into libraries, with many imports, dependencies, etc..

lib.rs is also considered a crate root. Cargo has special treatment for library crate. 
It does not getting compiled directly, but can be used as a part of another binary crate.

Term "crate" is used in context of refering to a library codebase, dependency package, binary codebase (binary crate).

- module is logical unit of division chunks of code within a crate.
You can have as many modules as you need in a single crate.
There is also a module hierarchy (they can be splitted into files or declared as blocks inside a `.rs` file)

- packages is a set of crates combined to represent a specific functionality unveiling the purpose of the whole project


#### 8.2. How do they differ?
They differ in the way that they are different. Duh

But describing structural properties we could say that:
- module is the least structuring unit, that allowing to separate the code itself
- crate are one level up structuring unit, which combines modules and code into a single codebase
- package is more broad term and is also level up structuring uint, which withholds a set of crates (binary, library) and all required configurations, supplements, etc...


#### 8.5. How are the used?
Their usage can be inferred from their descriptions.


#### 8.4. What is workspace?
Workspace is a special feature of Cargo, which allows to combine several packages under a single structure and share dependencies, configurations, etc...
When dependenices are built, they are built once, and are reused for every package (unless otherwise specified).


< -----------------------------------------------------------------------------------------


### 9. What is cloning? What is copying? How do they compare? What is for trait drop? What is special about the trait?
#### 9.1. What is cloning?
Cloning is a way to duplicate data that has non-primitive type, which are stored on the heap.
Well, at least that is by convention.
You are free to implement a `Clone` trait on your considerations.

This operation is considered to be expensive (though it does not have to actually be so).
It adds some semantic meaning to the code where something expensive would take place.


#### 9.2. What is copying?
The copying is a bitwise duplication. That is copying data bit-by-bit.
In Rust it is performed with the help of memcpy.

The type that is `Copy` is implicitly made a copy of in places where it seems it should be moved.
Move does not occure cause the type implements a `Copy` trait
thus moving))) from "move semantics" to "copy semantics".


#### 9.3. How do they compare?
`Copy` is used where non-expensive bitwise copy is required.
`Clone` is used when something more complicated and expensive is needed to be done.

`Copy` does have `Clone` as its supertrait.
The reason is possibly that it would not make much sense if `Copy` type could not be cloned.
It is very possible that some code would use `clone` semantics in all cases, but will work with `Copy`-types as well.

There is one more thing to be aware of:
- if the `Copy` is derived: the `Copy` bounds are also added for all type fields (like in case of struct, enum).


#### 9.4. What is for trait drop?
You mean "What is Drop trait for?"?.

The `Drop` trait can be implemented when a custom behaviour is needed before some data gets destructed/dropped.
It does not replace the built-in `drop` behaviour of Rust.

For example if you need to release some sort of resource like socket, DB connection or other thing properly.


#### 9.5. What is special about the trait?
- You can call the `mem::drop(value)`, but not `value.drop()`.
    This trait can be implemented but is called automatically
    The `mem::drop` is similar to value going out of scope.
  Why? - I dare to presume there is just  no sense to implement a custom destructor for a `Copy` type:
  what will you be freeing? A children from slave labour at Apple corp.??
  The resource that can be bitwise copied by the semantics of `Copy` does not make sese to have complex destructor logic to be implemented on it.

- There can be either `Copy` or `Drop` implemented for type, not both.
- Values are dropped in reverse order (with built-in deconstructor of rust, not particulary Drop trait)
- Values are cleaned up recursively
    (this is part of OBRM - Rust calls destructors recursively for all the contained within data structures for a current value.
      Thus, all the resources are relieved properly)

There is a specific order the value is being dropped in for each data type, and that may be surprising sometimes.

 
< -----------------------------------------------------------------------------------------


### 10. What is immutability? What is the benefit of using it? What is the difference between immutability and const?
#### 10.1. What is immutability?
An immutability is a concept meaning that a specific value declared as immutable cannot be changed/mutated.
It prevents undesired and casual value mutations at compile time.

#### 10.2. What is the benefit of using it?
It is useful for a developer as a semantic sign that a value should not be mutated.
Thus preventing accidental mutations.

#### 10.3. What is the difference between immutability and const?
There are several differences:
    - const cannot be defined at runtime, they have to have its value computed at compile time
    - const cannot be changed/mutated ever.
    - const has to have its type explicitly specified when it's defined
    - const can be declared at global scope (outside the main or other function)
    - const are stored directly in program binary
immutability on the other hand is a compiler feature that prevents undesired mutations of the value

The immutable value can be redeclared as mutable (variable shadowing).


< -----------------------------------------------------------------------------------------


### 11. What are move semantics? What are borrowing rules? What is the benefit of using them?
#### 11.1. What are move semantics?
It is a set of rules of OBRM (specifically - Borrow Checker) that enforce a single ownership of a value at a time.   
That is when one value is assigned to another variable/field or passed as parameter and it does not implement neither Clone or Copy behaviour,
it will be moved out from current container/variable disabling the ability to used it after.


#### 11.2. What are borrowing rules?
Borrowing rules are also the rules enforced by the Borrow Checker.
They imply that a value:
- can have only a single owner at a time
- there can be multiple **immutable** references to the value at a time
- there can be only a single **mutable** reference and zero immutable references at a time
- value is dropped at the end of its scope and no longer available


#### 11.3. What is the benefit of using them?
These limitations help to catch hard-to-debug errors early-on at compile time (in some cases at runtime with the help of smart pointers).
Such errors can be :
- use-after-free
- double free


< -----------------------------------------------------------------------------------------


### 12. What is RAII? How is it implemented in Rust? What is the benefit of using it?
#### 12.1.  What is RAII?
RAII - Resource Acquisition Is Initialization
In C++ it is the compiler feature that automatically manages allocation and destruction of resources.


#### 12.2. How is it implemented in Rust?
In Rust it is implemented more robustly and is rather called OBRM - ownership-based resource management.
Though it is based/inherited on/from RAII, it only takes the best and safe concepts from there.
Also implementing its own unique features and implementation details.

Mostly it is performed at compile time, thus avoiding runtime overhead.


#### 12.3. What is the benefit of using it?
The benefit would be avoiding unnecessary headaches that C++ developers encounter very often with.
That is eliminating problems such as:
    - unnecessary overcomplicated memory management (if compiler can do, why you have to?)
    - use-after-free errors
    - double-free errors
    - dereferencing null pointers
    - enforcing ownership rules (wich therefore deals with double-free errors for complex types)


< -----------------------------------------------------------------------------------------


### 13. What are lifetimes? Which problems do they solve? Which benefits do they give?
#### 13.1. What are lifetimes?
A lifetimes is a Rust's Borrow Checker's conception.
Inside of the Rust code it's a special generic annotation which is applied to reference types.
Generally it is used to help the compiler to adhere borrowing rules, because not always (yet) they can be precisely inferred automatically.

So you specify lifetimes to specify how different types' longevity is related to each other.
(Thus the compiler can be sure what you mean to do and detect if you violate borrowing rules)


#### 13.2. Which problems do they solve?
As was said above they help compiler to be better at adhering borrowing rules at compile time.


#### 13.3. Which benefits do they give?
They add more clarity to the code not only for compiler but also for developer who reads the code.
It will be more explicit how lifetimes of different parameters/types are related.


< -----------------------------------------------------------------------------------------


### 14. What is an iterator? What is a collection? How do they differ? How are they used?
#### 14.1. What is an iterator?
An iterator is a special Rust's std trait that implements a semantics for iterating/walking over a set of values within collections.
In more general sense an iterator is a tool/function/facility that allows to iterate over some set of values within a container.

#### 14.2. What is a collection?
A collection is a more general concept describing a set of all container-like data types that adhere to specific semantics and rules and mainly are capable of holding a set of multiple values within it (such as tuples, HashMaps, Vecs, arrays).
You are free to implement your own type of collection-like data type.


#### 14.3. How do they differ?
Collection is about data type.
Iterator is about facilities to iterate over collection.
Collection implements an iterator trait, thus adhering a conventions of syntax and semantics on iterating in Rust.


#### 14.4. How are they used?
That is said. Iterator allows us to iterate over a set of values of collection making some modifications/transformations to the collection or returning a new collection.
A collection is a type instance containing a set of values and implementing a special behaviour and special structure of data.


< -----------------------------------------------------------------------------------------


### 15. What are macros? Which problems do they solve? What is the difference between declarative and procedural macro?
#### 15.1. What are macros?
A macro is a special feature of Rust that allows 
to utilize a compiler facilities to generate and transorm the code itself,
enable/disable useful compiler features, settings, etc...
This is otherwise called metaprogramming, that is writing code by writing code.


#### 15.2. Which problems do they solve?
`macro_rules!` for example acts like a `match` construction, matching different call signatures,
but exposing metaprogramming features (such as parsing and substitutioning identifiers etc..).

- This helps a developer to reduce amount of manually-written code, take variable number of arguments.
- Mutating functions and methods by applying attribute-like macros extending thus the function/method capabilities.
- Parsing extracting, modifying ASTs and working with TokenStreams, thus facilitating a developer with compile-time reflection.


#### 15.3. What is the difference between declarative and procedural macro?
Declarative `macro_rules!` macro and three types of procedural macros are different in a way that:
- declarative macro works, well in declarative way, that is defining and manipulating in more "write-in-place" maneer
- whether a procedural macro is more function-like, it manipulates on input TokenStream and gives output result
but it works like a function and has no special constructions in its toolbox

in the same time declarative macro has some special syntax constructions that can be used in a declarative way and used in place, wheter procedural macro will perform some operations on variables, invoke functions, like a regular function...


< -----------------------------------------------------------------------------------------


### 16. How code is tested in Rust? Where should you put tests and why?
Code is tested with the help of the Cargo build tool.
It has built-in functionality for performing / executing tests.

There are two base kinds of tests: Unit Tests and Integration Tests.

- Unit tests are responsible for testing a specific units of code (such as specific functions).


- Integration tests are tests that are devoted for testing workflows of the programs or some separate complex functionality.
Or in other words some different parts of the program that perform a specific complex tasks.


All tests are run concurrently/in parallel so it must be considered,
when writing integration tests, as the execution order is not guaranteed.


#### 16.2. Where should you put tests and why?

- Unit tests are placed within each concrete module that is to be tested.
It can be placed in a single file or separately.
It can be aditionally wrapped into 

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_some_func() { /* ... */ }
}
```
This concrete layout is pretty much justified:
specific tests are placed along with the code they are testing and not in some other country.


- Integration tests are often big and are testing rather business logic of the application.
Thus it is better for them to reside somewhere closer to the root of the project.
Integration tests are under convention to be placed inside `tests` directory at root of the package.
Each file is a separate integration test.


< -----------------------------------------------------------------------------------------


### 17. What is special about slice? What is layout of Rust standard data types? Difference between fat and thin pointers?
#### 17.1. What is special about slice?
A slice is a Dynamically-Sized type (`str` or `[T]`).
A slice, whether it is a string slice or array slice can be interpreted abstractly as a view into some memory range.

As far as there is no fixed size, we cannot define slice as type in our code.
Thus we can only reference a slice type.


#### 17.2. What is layout of Rust standard data types?
- Primitive types like i<N> , f<N>, u<N>
are stored on the stack and taking up a respective amount of bytes in memory cell.

isize - takes a word on system the program is run under (on x64 - 8bytes, on x32 - 4bytes)
i128 - 16 bytes
i64  - 8 bytes
i32  - 4 bytes
i16  - 2 bytes


same for u<N>.


They all are stored on stack contiguously.
But there is a place for paddings (empty bits) being added to align data on the stack for faster access (the processor always works with words).

- DSTs are stored on Heap with its pointers stored on stack. The pointer can be thin or fat.
Thin pointer is just a regular usize pointer wheter a fat pointer is a pointer along with some additional metadata describing a particular DST on heap.
(such as a reference to a string slice `&str` : stack=[heap_ptr|length])

- Arrays are contiguous items aligned to T of [T; N].
- Tuples, except Unit-like tuple, are have default representation and are not guaranteed to have defined layout, but have some preconditions:
    - values are properly aligned (as well as the type as whole is properly aligned)
    - values do not overlap

- Closures have  no predetermined layout (on my thought cause of they are build/generated at compile-time for every special case thus having a different data structure)

- Trait object have a stack=[data_ptr|vtable_ptr] layout.

- Enums have also additional `tag` one-byte field which is what is in C called a Disscriminant.
There is special cases where such things are optimized out by the compiler like in case of Option<T> - None is zero-sized and Option does not have discriminants/tags.
Each enum variant is to take up the same size as the largest of the variants despite its literal size can be less. 

- char takes up 4 bytes, always. Cause it upholds the UTF-8 invariants.
- structs have no defined ordering and its fields can be stored in arbitraty order, but still contiguously with alignment/paddings
- slice types (`[T]` and `str`) are represented the same as part of array/string (not String).
the `str` and `[T]` cannot be used at compile time, cause they represent a fixed-size types:
    - the `str` has no defined size. it can be "string" or "this is string" in memory, so the size is only defined when it is actually stored in memory.


#### 17.3. Difference between fat and thin pointers?
Fat pointers is special type of pointers which we may describe as "extended".
The reason that this pointer actually consists of:
1. pointer to the data:
2. some additional metadata

That is the reason for its name: it's not just the pointer itself..

The examples would be the slice ref type (`&[T]`) and string ref slice (`&str`).
The essense is to have an ability to reference DST (dynamically-sized types).

Thin pointers on the other hand are "thin" just in contrast to "fat", under the hood they are just regular pointers.


< -----------------------------------------------------------------------------------------


### 18. Why Rust has `&str` and `String` types? How do they differ? When should you use them? Why str slice coexist with slice?
#### 18.1. Why Rust has `&str` and `String` types?
Because `&str` is faster built-in type and `String` is a data structure.


#### 18.2. How do they differ?
- `&str` is a fat-pointer to the slice to a string.
    it is is often described as a "view into string" whether the original string is literal, stack or heap-allocated.
- `String` is what is used to allocate a string on the heap and is owned type. It can be extended, changed, dropped.

String is allocated on the heap and is a smart pointer. Underthehood it is just Vec<u8>.
&str is fat pointer to a string slice and can also point to a part of String on a heap.


#### 18.3. When should you use them?
&str is used whether you want to reference to embedded string literal or a string allocated at a heap.
String is used in more complicated cases where a dynamic manipulations take place.


#### 18.4. Why str slice coexist with slice?
Because string is not that simple as it seems.
It is not ASCII, so there are some pitfalls in proper managing and using a UTF-8 encoded string.
So `str` slice is different from just a slice `[T]` which does not require special tretment.


< -----------------------------------------------------------------------------------------


### 19. Is Rust OOP language? Is it possible to use SOLID/GRASP? Does it have an inheritance? Is Rust functional language?
#### 19.1. Is Rust OOP language?
No it isn't. But it does allow you to implement some concepts from OOP.
Well, i've heard that pretty much any concept can be implemented, but there are of course some downsides.

#### 19.2. Is it possible to use SOLID/GRASP?
More than that it even has some similar conceptions, but implemented differently (such as traits).


#### 19.3. Does it have an inheritance?
Rust definitely have no support for INHERITANCE. Although that is not a big loss and can be as well hacked with the help of libraries etc...
Because inheritance is error-prone and is starting to be considered as bad practice.
Especially multiple inheritance (such as C++ does have).


#### 19.4. Is Rust functional language?
Pretty much it is!


---
After you're done notify your lead in an appropriate PR (pull request), and he will exam what you have learned.


