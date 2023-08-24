Step 1: Concepts
================

These steps describe common and necessary-to-know concepts for everyday programming in [Rust].

> ❗️Before completing this step you should complete all its sub-steps.

After doing them you should be able to answer the following questions:
- How do I recognize that data is allocated at the heap rather than at the stack? When data should be allocated at the heap?
    Data allocated in heap is usually behind the Box smart pointer and alike.
    All other values are stack-allocated.

    Data is usually heap-allocated when:
    - the data is large, so it is more effectively to keep data in single place and refer to it by pointer
    - it is useful when the size of the data is unknown at compile time (such as Cons List)
    - as for Rust specifically there may be some cases where you need to 

- What is copying and cloning data in [Rust]? What's the difference? When and why should I use them?
    - Copying is a simple bitwise copy. It usually happens on the stack. It is used for primitives and simple data structures.
    Clonning is more heavy concept in a sense that it implies more expensive semantics.
    in other words it means that clonning is used for more complex data structures, where a simple copy won't do

    - the difference is when copying is cheap, clonning maybe (but not necessary) can be more expensive in computations and data resources
    also copying is performed implicitly, while clonning need to be used manually in an explicit way
    clonning may require from programmer a custom clonning behaviour implementation

    - As it was said for simple structures use copy, for more complex - use clone.
    The thing is that Clone can be implemented without Copy, whether to implement Copy you also need clone.

    Clone is to be simple `*self` for copy. but for complicated reasons it is not forbidden to provide custom implementation in this case.
    
- How can a single piece of data be owned by multiple parts of program? When and why is this commonly required?
    - The concept referred here in question is `Shared Ownership`.
    It utilises a special smart pointer `Rc` for Reference Count.
    This pointer keeps tracking of reference counts, so the data behind lives as long as a single reference exists.
    This pointer can be `clonned` / duplicated and used as owned value in different parts.
    STill under the hood no additional allocations happen aside from increasing Reference Counts
    
    - I think it is often used in some linked data tree-like stuctures,
    where one node can be referenced multiple times
     
- How borrowing rules may be violated? In what price? When and why is this commonly required?
    - This concept is called "Interior Mutability" and i would say that it is rather "extended" not "violated"
    Borrowing rules cannot be actually violated, cause there is no known way to get around the Borrow Checker

    - This is Runtime implementation, so there is a little bit of overhead to keep access tracking.
    To be more specific there are two mutually-exclusive `<word>` flags which are set when a value is borrowed or exclusively accessed
    Also this introduces some indirections which are not the case for static borrowing.

    - This is most of the time used internally for specific types, when there is only IMMUtABLE access to a value
    and the Developer needs to enforce specific API without exposing it to the user.
    So the user cannot make something mutable and mess up.

    This is widely also applied in multithreading.

- How to deal with owned and borrowed data simultaneously? When and why is this commonly required?
    - There is a `Cow` - Copy on Write.
    It is designed to keep a reference to a value and only create a copy/clone when you try to modify the value.
    Then it becomes Owned.
    This is actually enum with `Borrowed<'_'` and `Owned` variants.

    - There is only one case i know : when the data is very often tossed around for reading,
    and very rarely under specific conditions it is mutated
    
- How to share values between threads? What is `Send` and `Sync` markers? Why are they required, when should be used?
    - You can use `Arc` pointer or use values that have `Sync` behaviour enforced out-of-the-box.
    - `Send` and `Sync` are marker traits that 
    tell the compiler that a specific type can be shared (`Sync`) or moved (`Send`) to another thread.
    It does not actually enforces this behaviour. So the developer must use these markers very carefully and manually enforce invariants.

    - They are a part of Rust's `Fearless Concurrency` feature.
    The types from std lib that have these markers are guaranteed to behave properly when used concurrently.
    So the compiler can ensure this safety by looking at markers.
    Only in rare cases they should be used , when a dev knows and really needs them to be there to implement some custom multithreading types.

- How do static and dynamic dispatches differ? Why do they exist? When and why should I choose between them?
    The `static` dispatch is perfomed at compile-time.
    The `dynamic` dispatch is a part of Runtime and has overhead as well as limitations.

    Static dispatch uses monomorphisation and can perform `inlining` (substitution optimizations where a call is replaced with computed value or a simpler code).
    Though it has a limitation that there may no be type variations, if specific type is once replaced it is there till the end.
    
    Dynamic dispatch allows to use different types and performs indirection at Runtime.
    At compile time data types `are erased` and at runtime indirection is performed to match the needed data and vtable.

    - A dynamic dispatch exists so it is used in some cases where more flexibility is needed.
    (for example to rerurn different types which still implement common trait)

    - When the concrete type to be used is unknown at compile time you need a dynamic dispatch.
    When you have a defined behaviour and you know you will be using only a sinlgle type in one specific place.

- Why `?Sized` types exist? How are they used? Why should I care about them?
    - `?Sized` marker or `size widening` or `relaxed size` is designed to be used in cases when no specific size can be known at compile time
    or you know you will be storing an `Unsized` type (such as slice `[T]` or `str`) and refer through reference later.

    They may be used to shift type sizedness to the runtime.
    They can be only used through fat-pointers.

    - I dont know why you should.
    But Rust developers may care because there is `str` and `[T]`
    and they are often used.
    Basically `str` is unsized, because you cannot know a concrete size of the str type,
    it can vary in length. Same for the slices and Trait objects.
    Trait objects are also "unsized"/DST/?Sized...

- Why phantom types exist? What problems do they solve?
    - Phantom types are types that actually do not introduce any overhead.
    Otherwise they are ZST (Zero-Sized-Types).
    They are used to introduce some compile-time invariants,
    so the compiler can verify correctness of the program.

    Such a case is to simulate some behaviour of some type without laying out any overhead that would give usual type usage.
    Phantom types can be also used somewhat in a way like a `marker` traits are used.

    - As it was said they introduce some constraints on a type or where they are used
    to simulate behaviour or properties of the Phantom type.


The following articles may help you to sum up your experience:
- [Wrapper Types in Rust: Choosing Your Guarantees][1]
- [Rust, Builder Pattern, Trait Objects, `Box<T>` and `Rc<T>`][2]
- [Rust's Built-in Traits, the When, How & Why][3]
- [Learn Rust With Entirely Too Many Linked Lists][4]
- [Rustonomicon: Subtyping and Variance][13]
- [Crust of Rust: Subtyping and Variance][14]



## Task

__Estimated time__: 2 days




Provide your own implementation of [doubly linked list][11] data structure. It should be [thread safe][12] without a necessity to use explicit synchronization primitives (like `Arc<Mutex<T>>`) on top of it.

Prove your implementation correctness with tests. Provide both single-threaded and multi-threaded examples of usage.  




[Rust]: https://www.rust-lang.org

[1]: https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees
[2]: https://abronan.com/rust-trait-objects-box-and-rc
[3]: https://llogiq.github.io/2015/07/30/traits.html
[4]: https://rust-unofficial.github.io/too-many-lists/
[11]: https://en.wikipedia.org/wiki/Doubly_linked_list
[12]: https://en.wikipedia.org/wiki/Thread_safety
[13]: https://doc.rust-lang.org/nomicon/subtyping.html
[14]: https://www.youtube.com/watch?v=iVYWDIW71jk
