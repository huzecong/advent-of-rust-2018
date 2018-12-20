### Day 9: Marble Mania

For both parts, simply simulate using a doubly-linked list.

Yes, I had to implement such a thing in Rust. I did some research and it turns out there's no way to implement a doubly-linked list (or anything with circular references) that satisfies these three criteria:

- Safe. Using `unsafe` code we can write raw pointers just like those in C.
- Pointer-based. We can simulate a linked list using an array by replacing pointers with indices. This way when you remove elements you won't be able to get your memory back, but otherwise it works.
- Without overhead. A safe implementation is to use `Option<Rc<RefCell<_>>>`, which gives us null pointers, ref-counted smart pointers, and internal mutability (with runtime borrow checking). Of course, these come at a cost.

In the end I chose to sacrifice safety. Hell, what could go wrong in a doubly-linked list?