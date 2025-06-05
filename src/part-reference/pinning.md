# Pinning

Pinning is a notoriously difficult concept and has some subtle and confusing properties. This section will go over the topic in depth (arguably too much depth). Pinning is key to the implementation of async programming in Rust[^design], but it's possible to get far without ever encountering pinning and certainly without having to have a deep understanding.

The first section will give a summary of pinning, which hopefully is enough for most async programmers to know. The rest of this chapter is for implementers, others doing advanced or low-level async programming, and the curious.

After the summary, this chapter will give some background on move semantics before getting into pinning. We'll cover the general idea, then the `Pin` and `Unpin` types, how pinning achieves it goals, and several topics about working with pinning in practice. There are then sections on pinning and async programming, and some alternatives and extensions to pinning (for the really curious). At the end of the chapter are some links to alternative explanations and reference material.

[^design]: It's worth noting that pinning is a low-level building block designed specifically for the implementation of async Rust. Although it is not directly tied to async Rust and can be used for other purposes, it was not designed to be a general-purpose mechanism, and in particular is not an out-of-the-box solution for self-referential fields. Using pinning for anything other than async code generally only works if it is wrapped in thick layers of abstraction, since it will require lots of fiddly and hard to reason about unsafe code. 


## TL;DR

`Pin` marks a pointer as pointing to an object which will not move until it is dropped. Pinning is not built-in to the language or compiler; it works by simply restricting access to mutable references to the pointee. It is easy enough to break pinning in unsafe code, but like all safety guarantees in unsafe code, it is the responsibility of the programmer not to do so.

By guaranteeing that an object won't move, pinning makes it safe to have references from one field of a struct to another (sometimes called self-references). This is required for the implementation of async functions (which are implemented as data structures where variables are stored as fields, since variables may reference each other, fields of a future implementing an async function must be able to reference each other). Mostly, programmers don't have to be aware of this detail, but when dealing with futures directly, you might need to be because the signature of `Future::poll` requires `self` to be pinned.

If you're using futures by reference, you might need to pin a reference using `pin!(...)` to ensure the reference still implements the `Future` trait (this often comes up with the `select` macro). Likewise, if you want to manually call `poll` on a future (usually because you are implementing another future), you will need a pinned reference to it (use `pin!` or ensure arguments have pinned types). If you're implementing a future or if you have a pinned reference for some other reason, and you want mutable access to the object's internals, you'll need to understand the section below on pinned fields to know how to do so and when it is safe.


## Move semantics

A useful concept for discussing pinning and related topics is the idea of *place*s. A place is a chunk of memory (with an address) where a value can live. A reference doesn't really point at a value, it points at a place. That is why `*ref = ...` makes sense: the dereference gives you the place, not a copy of the value. Places are well-known to language implementers but usually implicit in programming languages (they are implicit in Rust). Programmers usually have a good intuition for places, but may not think of them explicitly.

As well as references, variables and field accesses evaluate to places. In fact, anything that can appear on the left-hand side of an assignment must be a place at runtime (which is why places are called 'lvalue's in compiler jargon).

In Rust, mutability is a property of places, as is being 'frozen' as a result of borrowing (we might say the place is borrowed).

Assignment in Rust *moves* data (mostly, some simple data has copy semantics, but that doesn't matter too much). When we write `let b = a;`, the data that was in memory at a place identified by `a` is moved to the place identified by `b`. That means that after the assignment, the data exists at `b` but no longer exists at `a`. Or in other words, the address of the object is changed by the assignment[^compiler].

If pointers existed to the place which was moved from, the pointers would be invalid since they no longer point to the object. This is why borrowed references prevent moving: `let r = &a; let b = a;` is illegal, the existence of `r` prevents `a` being moved.

The compiler only knows about references from outside an object into the object (such as the above example, or a reference to a field of an object). A reference entirely within an object would be invisible to the compiler. Imagine if we were allowed to write something like:

```rust,norun
struct Bad {
    field: u64,
    r: &'self u64,
}
```

We could have an instance `b` of `Bad` where `b.r` points to `b.field`. In `let a = b;`, the internal reference `b.r` to `b.field` is invisible to the compiler, so it looks like there are no references to `b` and therefore the move to `a` would be ok. However if that happened, then after the move, `a.r` would not point to `a.field` as we'd like, but to invalid memory at the old location of `b.field`, violating Rust's safety guarantees.

Moving data isn't limited to values. Data can also be moved out of a unique reference. Dereferencing a `Box` moves the data from the heap to the stack. `take`, `replace`, and `swap` (all in [`std::mem`](https://doc.rust-lang.org/std/mem/index.html)) move data out of a mutable reference (`&mut T`). Moving out of a `Box` leaves the pointed-to place invalid. Moving out of a mutable reference leaves the place valid, but containing different data.


[^compiler]: We're conflating source code and runtime a bit here. To be absolutely clear, variables don't exist at runtime. The (compiled) snippet might be executed multiple times (e.g., if it's in a loop or in a function called multiple times). For each execution the variables in the source code will be represented by different addresses at runtime.

Abstractly, a move is implemented by copying the bits from the origin to the destination and then erasing the origin bits. However, the compiler can optimise this is many ways.


## Pinning

Important note: I'm going to start by discussing an abstract concept of pinning, which is not exactly what is expressed by any particular type. We'll make the concept more concrete as we go on, and end up with precise definitions of what different types mean, but none of these types mean exactly the same as the pinning concept we'll start with.

An object is pinned if it will not be moved or otherwise invalidated. As I explained above, this is not a new concept - borrowing an object prevents the object being moved for the duration of the borrow. Whether an object can be moved or not is not explicit in Rust's types, though it is known by the compiler (which is why you can get "cannot move out of" error messages). As opposed to borrowing (and the temporary restriction on moves caused by borrowing), being pinned is permanent. An object can change from being not pinned to being pinned, but once it is pinned then it must remain pinned until it is dropped[^inherent].

Just as pointer types reflect the ownership and mutability of the pointee (e.g., `Box` vs `&`, `&mut` vs `&`), we want to reflect pinned-ness in pointer types too. This is not a property of the pointer - the pointer is not pinned or movable - it is a property of the pointed-to place: whether the pointee can be moved out of its place.

Roughly, `Pin<Box<T>>` is a pointer to an owned, pinned object and `Pin<&mut T>` is a pointer to a uniquely borrowed, mutable, pinned object (c.f., `&mut T` which is a pointer to a uniquely borrowed, mutable, object which may or may not be pinned).

The pinning concept was not added to Rust until after 1.0 and for reasons of backwards compatibility, there is no way to explicitly express whether an *object* is pinned or not. We can only express that a reference points to a pinned or not-pinned object.

Pinning is orthogonal to mutability. An object might be mutable and either pinned (`Pin<&mut T>`) or not (`&mut T`) (i.e., the object can be modified, and either it is pinned in place or can be moved), or immutable and either pinned (`Pin<&T>`) or not (`T`) (i.e., the object can't be modified, and either it can't be moved or can be moved but not modified). Note that `&T` cannot be mutated or moved, but is not pinned because its immovability is only temporary.


[^inherent]: Permanence is not a fundamental aspect of pinning, it is part of the framing of pinning in Rust and the safety guarantees around it. It would be ok for pinning to be temporary if this could be safely expressed and the temporal scope of pinning could be relied upon by consumers of the pinning guarantees. However, that is not possible with Rust today or with any reasonable extension.


### `Unpin`

Although moving and not moving is how we introduced pinning and is somewhat suggested by the name, `Pin` does not actually tell you much about whether the pointee will actually move or not.

What? Sigh.

Pinning is actually a contract about validity, not about moving. It guarantees that *if an object is address-sensitive, then* its address will not change (and thus addresses derived from it, such as the addresses of its fields, will not change either). Most data in Rust is not address-sensitive. It can be moved around and everything will be ok. `Pin` guarantees that the pointee will be valid with respect to it's address. If the pointee is address-sensitive, then it can't be moved; if it's not address-sensitive, then it doesn't matter whether it is moved.

`Unpin` is a trait which expresses whether objects are address-sensitive. If an object implements `Unpin`, then it is *not* address-sensitive. If an object is `!Unpin` then it is address-sensitive. Alternatively, if we think of pinning as the act of holding an object in its place, then `Unpin` means it is safe to undo that action and allow the object to be moved.

`Unpin` is an auto-trait and most types are `Unpin`. Only types which have an `!Unpin` field or which explicitly opt-out are not `Unpin`. You can opt-out by having a [`PhantomPinned`](https://doc.rust-lang.org/std/marker/struct.PhantomPinned.html) field or (if you're using nightly) with `impl !Unpin for ... {}`.

For types which implement `Unpin`, `Pin` essentially does nothing. `Pin<Box<T>>` and `Pin<&mut T>` can be used just like `Box<T>` and `&mut T`. In fact, for `Unpin` types, the `Pin`ed and regular pointers can be freely-interconverted using `Pin::new` and `Pin::into_inner`. It's worth restating: `Pin<...>` does not guarantee that the pointee will not move, only that the pointee won't move if it is `!Unpin`.

The practical implication of the above is that working with `Unpin` types and pinning is much easier than with types which are not `Unpin`, in fact the `Pin` marker has basically no effect on `Unpin` types and pointers to `Unpin` types, and you can basically ignore all the pinning guarantees and requirements.

`Unpin` should not be understood as a property of an object alone; the only thing `Unpin` changes is how an object interacts with `Pin`. Using an `Unpin` bound outside of the pinning context doesn't affect the compiler's behaviour or what can be done with the object. The only reason to use `Unpin` is in conjunction with pinning, or to propagate the bound to where it is used with pinning.


### `Pin`

[`Pin`](https://doc.rust-lang.org/std/pin/struct.Pin.html) is a marker type, it is important for type checking, but is compiled away and does not exist at runtime (`Pin<Ptr>` is guaranteed to have the same memory layout and ABI as `Ptr`). It is a wrapper of pointers (such as `Box`), so it behaves like a pointer type, but it does not add an indirection, `Box<Foo>` and `Pin<Box<Foo>>` are the same when a program is run. It is better to think of `Pin` as a modifier to the pointer rather than a pointer itself.

`Pin<Ptr>` means that the pointee of `Ptr` (not `Ptr` itself) is pinned. That is, `Pin` guarantees that the pointee (not the pointer) will remain valid with respect to its address until the pointee is dropped. If the pointee is address-sensitive (i.e., is `!Unpin`), then the pointee will not be moved.


### Pinning values

Objects are not created pinned. An object starts unpinned (and may be freely moved), it becomes pinned when a pinning pointer is created which points to the object. If the object is `Unpin`, then this is trivial using `Pin::new`, however, if the object is not `Unpin`, then pinning it must ensure that it cannot be moved or invalidated via an alias.

To pin an object on the heap, you can create a new pinning `Box` by using [`Box::pin`](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.pin), or convert an existing `Box` into a pinning `Box` using [`Box::into_pin`](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.into_pin). In either case, you'll end up with `Pin<Box<T>>`. Some other pointers (such as `Arc` and `Rc`) have similar mechanisms. For pointers which don't, or for your own pointer types, you'll need to use [`Pin::new_unchecked`](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked) to create a pinned pointer[^box-pin]. This is an unsafe function and so the programmer must ensure that `Pin`'s invariants are maintained. That is, that the pointee will, under every circumstance, remain valid until it's destructor is called. There are some subtle details to ensuring this, refer to the function's [docs](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked) or the below section [how pinning works](#how-pinning-works) for more.

`Box::pin` pins an object to a place in the heap. To pin an object on the stack, you can use the [`pin`](https://doc.rust-lang.org/std/pin/macro.pin.html) macro to create and pin a mutable reference (`Pin<&mut T>`)[^not-stack].

Tokio also has a [`pin`](https://docs.rs/tokio/latest/tokio/macro.pin.html) macro which does the same thing as the std macro and also supports assigning into a variable inside the macro. The futures-rs and pin-utils crates have a `pin_mut` macro which used to be commonly used, but is now deprecated in favor of the std macro.

You can also use `Pin::static_ref` and `Pin::static_mut` to pin a static reference.

[^box-pin]: There is no special treatment for `Box` (or the other std pointers) either in the pinning implementation or the compiler. `Box` uses the unsafe functions in `Pin`'s API to implement `Box::pin`. The safety requirements of `Pin` are satisfied due to the safety guarantees of `Box`.

[^not-stack]: This is only strictly pinning to the stack in non-async functions. In an async function, all locals are allocated in the async pseudo-stack, so the place being pinned is likely to be stored on the heap as part of the future underlying the async function.


### Using pinned types

In theory, using pinned pointers is just like using any other pointer type. However, because it is not the most intuitive abstraction, and because it has no language support, using pinned pointers tends to be pretty unergonomic. The most common case for using pinning is when dealing with futures and streams, we'll cover those specifics in more detail below.

Using a pinned pointer as an immutably borrowed reference is trivial because of `Pin`'s implementation of `Deref`. You can mostly just treat `Poll<Ptr<T>>` as `&T`, using an explicit `deref()` if necessary. Likewise, getting a `Pin<&T>` is pretty easy using `as_ref()`.

The most common way to work with pinned types is using `Pin<&mut T>` (e.g., in [`Future::poll`](https://doc.rust-lang.org/std/future/trait.Future.html#tymethod.poll)), however, the easiest way to produce a pinned object is `Box::pin` which gives a `Pin<Box<T>>`. You can convert the latter to the former using [`Pin::as_mut`](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.as_mut). However, without the language support for reusing references (implicit reborrowing), you have to keep calling `as_mut` rather than reusing the result. E.g. (from the `as_mut` docs),

```rust,norun
impl Type {
    fn method(self: Pin<&mut Self>) {
        // do something
    }

    fn call_method_twice(mut self: Pin<&mut Self>) {
        // `method` consumes `self`, so reborrow the `Pin<&mut Self>` via `as_mut`.
        self.as_mut().method();
        self.as_mut().method();
    }
}
```

If you need to access the pinned pointee in some other way, you can do so via [`Pin::into_inner_unchecked`](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.into_inner_unchecked). However, this is unsafe and you must be *very* careful about ensuring the safety requirements of `Pin` are respected.


### How pinning works

`Pin` is a simple wrapper struct (aka, a newtype) for pointers. It is enforced to work only on pointers by requiring the `Deref` bound on it's generic parameter to do anything useful, however, this is just for expressing intention, rather than for preserving safety. As with most newtype wrappers, `Pin` exists to express an invariant at compile-time rather than for any runtime effect. Indeed, in most circumstances, `Pin` and the pinning machinery will completely disappear during compilation.

To be precise, the invariant expressed by `Pin` is about validity, not just movability. It is also a validity invariant which only applies once a pointer is pinned - before that `Pin` has no effect and makes no requirements on what happens before something is pinned. Once a pointer is pinned, `Pin` requires (and guarantees in safe code) that the pointed-to object will remain valid at the same address in memory until the object's destructor is called.

For immutable pointers (e.g., borrowed references), `Pin` has no effect - since the pointee cannot be mutated or replaced, there is no danger of it being invalidated.

For a pointer that allows mutation (e.g., `Box` or `&mut`), having direct access to that pointer or access to a mutable reference (`&mut`) to the pointee could allow for mutation or moving the pointee. `Pin` simply does not provide any (non-`unsafe`) way to get direct access to the pointer or a mutable reference. The usual way for a pointer to provide a mutable reference to its pointee is by implementing [`DerefMut`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html), `Pin` only implements `DerefMut` if the pointee is `Unpin`.

This implementation is incredibly simple! To summarize: `Pin` is a wrapper struct around a pointer which provides only immutable access to the pointee (and mutable access if the pointee is `Unpin`). Everything else is details (and subtle invariants for unsafe code). For convenience, `Pin` provides a facility to convert between `Pin` types (always safe since the pointer cannot escape a `Pin`), etc.

`Pin` also provides unsafe functions for creating pinned pointers and accessing the underlying data. As with all `unsafe` functions, maintaining the safety invariants is the responsibility of the programmer rather than the compiler. Unfortunately, the safety invariants for pinning are somewhat scattered, in that they are enforced in different places and are hard to describe in a global, unified manner. I won't describe them in detail here and refer you to the docs, but I'll attempt to summarize (see the [module docs](https://doc.rust-lang.org/std/pin/index.html) for a detailed overview):

- Creating a new pinned pointer [`new_unchecked`](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked). The programmer must ensure that the pointee is pinned (that is, abides by the pinning invariants). This requirement may be satisfied by the pointer type alone (e.g., in the case of `Box`) or may require participation of the pointee type (e.g., in the case of `&mut`). This includes (but is not limited to):
  - Not moving out of `self` in `Deref` and `DerefMut`.
  - Properly implementing `Drop`, see [the drop guarantee](https://doc.rust-lang.org/std/pin/index.html#subtle-details-and-the-drop-guarantee).
  - Opting out of `Unpin` (by using [`PhantomPinned`](https://doc.rust-lang.org/std/marker/struct.PhantomPinned.html)) if you require the pinning guarantees.
  - The pointee may not be `#[repr(packed)]`.
- Accessing the pinned value [`into_inner_unchecked`](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.into_inner_unchecked), [`get_unchecked_mut`](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.get_unchecked_mut), [`map_unchecked`](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.map_unchecked), and [`map_unchecked_mut`](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.map_unchecked_mut). It becomes the programmer's responsibility to enforce the pinning guarantees (including not moving the data) from the moment data is accessed until it's destructor runs (note that this scope of responsibility extends beyond the unsafe call and applies whatever happens to the underlying data).
- Not providing any other way to move data out of a pinned type (which would need an unsafe implementation).


#### Pinning pointer types

We said earlier that `Pin` wraps a pointer type. It is common to see `Pin<Box<T>>`, `Pin<&T>`, and `Pin<&mut T>`. Technically, the only requirement of the pinning pointer type is that it implements `Deref`. However, there are no ways to create a `Pin<Ptr>` for any other pointer types other than using unsafe code (via `new_unchecked`). Doing so has requirements on the pointer type to ensure the pinning contract:

- The pointer's implementations of `Deref` and `DerefMut` must not move out of their pointee.
- It must not be possible to obtain an `&mut` reference to the pointee at any time after the `Pin` is created, even after the `Pin` has been dropped (this is why you can't safely construct a `Pin<&mut T>` from an `&mut T`). This must remain true via multiple steps or via references (which prevents using `Rc` or `Arc`).
- The pointer's implementation of `Drop` must not move (or otherwise invalidate) it's pointee. 

See the `new_unchecked` [docs](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked) for more detail.

### Pinning and `Drop`

The pinning contract applies until the pinned object is dropped (technically, that means when its `drop` method returns, not when it is called). This is usually fairly straightforward since `drop` is called automatically when objects are destroyed. If you are doing things manually with an object's lifecycle, you might need to give it some extra thought. If you have an object which is (or might be) pinned and that object is not `Unpin`, then you must call it's `drop` method (using [`drop_in_place`](https://doc.rust-lang.org/std/ptr/fn.drop_in_place.html)) before deallocating or reusing the object's memory or address. See the [std docs](https://doc.rust-lang.org/std/pin/index.html#drop-guarantee) for details.

If you are implementing an address-sensitive type (i.e., one that is `!Unpin`), then you must take extra care with the `Drop` implementation. Even though the self-type in `drop` is `&mut Self`, you must treat the self-type as `Pin<&mut Self>`. In other words, you must ensure the object remains valid until the `drop` function returns. One way to make this explicit in the source code is to follow the following idiom:

```rust,norun
impl Drop for Type {
    fn drop(&mut self) {
        // `new_unchecked` is okay because we know this value is never used
        // again after being dropped.
        inner_drop(unsafe { Pin::new_unchecked(self)});

        fn inner_drop(this: Pin<&mut Self>) {
            // Actual drop code goes here.
        }
    }
}
```

Note that the validity requirements will be dependent on the type being implemented. Precisely defining these requirements, especially concerning object destruction is recommended, especially if multiple objects could be involved (e.g., an intrusive linked list). Ensuring correctness here is likely to be interesting!

### Pinned self in methods

Calling methods on pinned types leads to thinking about the self-type in these methods. If the method does not need to mutate `self`, then you can still use `&self` since `Pin<...>` can dereference to a borrowed reference. However, if you need to mutate `self` (and your type is not `Unpin`) then you need to choose between `&mut self` and `self: Pin<&mut Self>` (although pinned pointers can't be implicitly coerced to the latter type, they can be easily converted using `Pin::as_mut`).

Using `&mut self` makes the implementation easy, but means the method cannot be called on a pinned object. Using `self: Pin<&mut Self>` means considering pin projection (see the next section) and can only be called on a pinned object. Although this is all a bit confounding, it makes sense intuitively when you remember that pinning is a phased concept - objects start unpinned, and at some point undergo a phase change to become pinned. `&mut self` methods are ones which can be called in the first (unpinned) phase and `self: Pin<&mut Self>` methods are ones which can be called in the second (pinned) phase.

Note that `drop` takes `&mut self` (even though it might be called in either phase). This is due to a limitation of the language and the desire for backwards compatibility. It requires special treatment in the compiler and comes with safety requirements.


### Pinned fields, structural pinning, and pin projection

Given that an object is pinned, what does that tell us about the 'pinned'-ness of its fields? The answer depends on choices made by the implementer of the datatype, there is no universal answer (indeed it can be different for different fields of the same object). 

If the pinned-ness of an object propagates to a field, we say the field exhibits 'structural pinning' or that pinning is projected with the field. In this case there should be a projection method `fn get_field(self: Pin<&mut Self>) -> Pin<&mut Field>`. If the field is not structurally pinned, then a projection method should have signature `fn get_field(self: Pin<&mut Self>) -> &mut Field`. Implementing either method (or implementing similar code) requires `unsafe` code and either choice has safety implications. Pin-propagation must be consistent, a field must always be structurally pinned or not, it is nearly always unsound for a field to be structurally pinned at some times and not at others.

Pinning should project to a field if the field is an address-sensitive part of the aggregate datatype. That is, if the aggregate being pinned depends on the field being pinned, then pinning must project to that field. For example, if there is a reference from another part of the aggregate into the field, or if there is a self-reference within the field, then pinning must project to the field. On the other hand, for a generic collection, pinning does not need to project to it's contents since the collection does not rely on their behaviour (that's because the collection cannot rely on the implementation of the generic items it contains, so the collection itself cannot rely on the addresses of its items).

When writing unsafe code, you can only assume that the pinning guarantees apply to the fields of an object which are structurally pinned. On the other hand, you can safely treat non-structurally pinned fields as moveable and not worry about the pinning requirements for them. In particular, a struct can be `Unpin` even if a field is not, as long as that field is always treated as not being structurally pinned.

If a field is structurally pinned, then the pinning requirements on the aggregate struct extend to the field. Under no circumstance can code move the contents of the field while the aggregate is pinned (this would always require unsafe code). Structurally pinned fields must be dropped before they are moved (including deallocation) even in the case of panicking, which means care must be taken within the aggregate's `Drop` impl. Furthermore, the aggregate struct cannot be `Unpin` unless all of its structurally-pinned fields are.


#### Macros for pin projection

There are macros available for helping with pin projection.

The [pin-project](https://docs.rs/pin-project/latest/pin_project/) crate provides the `#[pin_project]` attribute macro (and the `#[pin]` helper attribute) which implements safe pin projection for you by creating a pinned version of the annotated type which can be accessed using the `project` method on the annotated type.

[Pin-project-lite](https://docs.rs/pin-project-lite/latest/pin_project_lite/) is an alternative using a declarative macro (`pin_project!`) which works in a very similar way to pin-project. Pin-project-lite is lightweight in the sense that it is not a procedural macro and therefore does not add dependences for implementing procedural macros to your project. However, it is less expressive than pin-project and does not give custom error messages. Pin-project-lite is recommended if you want to avoid adding the procedural macro dependencies, and pin-project is recommended otherwise.

Pin-utils provides the [`unsafe_pinned`](https://docs.rs/pin-utils/latest/pin_utils/macro.unsafe_pinned.html) macro to help implement pin projection, but the whole crate is deprecated in favor of the above crates and functionality now in std.


### Assigning to a pinned pointer

It is generally safe to [assign into a pinned pointer](https://doc.rust-lang.org/std/pin/index.html#assigning-pinned-data). Although this can't be done in the usual way (`*p = ...`), it can be done using [`Pin::set`](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.set). More generally, you can use unsafe code to assign into fields of the pointee.

Using `Pin::set` is always safe since the previously pinned pointee will be dropped, fulfilling the pin requirements and the new pointee is not pinned until the move into the pinned place is complete. Assigning into individual fields does not automatically violate the pinning requirements, but care must be taken to ensure that the object as a whole remains valid. For example, if a field is assigned into, then any other fields which reference that field must still be valid with the new object (this is not part of the pinning requirements, but might be part of the object's other invariants).

Copying one pinned object into another pinned place can only be done in unsafe code, how safety is maintained depends on the individual object. There is no general violation of the pinning requirements - the object being replaced is not moving and nor is the object being copied. However, the validity of the object being replaced may have safety requirements which are usually protected by pinning, but in this case must be established by the programmer. For example, if we have a struct with two fields `a` and `b` where `b` refers to `a`, that reference requires pinning to be remain valid. If such a struct is copied into another place, then the value of `b` must be updated to point to the new `a` rather than the old one.


## Pinning and async programming

Hopefully, you can do all you ever want to do with async Rust and never worry about pinning. Sometimes you'll hit a corner case which requires using pinning and if you want to do implement futures, a runtime, or similar things, you'll need to know about pinning. In this section, I'll explain why.

Async functions are implemented as futures (see section TODO - this is a summary overview, make sure we explain more deeply and with examples elsewhere). At each await point execution of the function may be paused and during that time the values of live variables must be saved. They essentially become fields of a struct (which is part of an enum). Such variables may refer to other variables which are saved in the future, e.g., consider,

```rust,norun
async fn foo() {
  let a = ...;
  let b = &a;
  bar().await;
  // use b
}
```

The generated future object here will be something like:

```rust,norun
struct Foo {
  a: A,
  b: &'self A,  // Invariant `self.b == &self.a`
}
```

(I'm simplifying a bit, ignoring the state of execution, etc., but the important bit is the variables/fields).

This makes intuitive sense, unfortunately `'self` does not exist in Rust. And for good reason! Remember that Rust objects can be moved, so code like the following would be unsound:

```rust,norun
let f1 = Foo { ... }; // f1.b == &f1.a
let f2 = f1; // f2.b == &f1.a, but f1 no longer exists since it moved to f2
```

Note that this is not just an issue of not being able to name the lifetime, even if we use raw pointers, such code would still be incorrect.

However, if we know that once it is created, then an instance of `Foo` will never move, then everything Just Works. (The compiler has a concept similar to `'self` internally for such cases, as a programmer, we would have to use raw pointers and unsafe code). This concept of not moving is exactly what pinning describes.

We see this requirement in the signature of `Future::poll`, where the type of `self` (the future) is `Pin<&mut Self>`. Mostly, when using async/await, the compiler takes care of pinning and unpinning, and as a programmer you don't need to worry about it.


### Manual pinning

There are some places where pinning leaks through the abstraction of async/await. At its root, this is due to the `Pin` in the signature of `Future::poll` and `Stream::poll_next`. When using futures and streams directly (rather than through async/await), we might need to consider pinning to make things work. Some common reasons to need pinned types are:

- Polling a future or stream - either in application code or when implementing your own future.
- Using boxed futures. If you're using boxed futures (or streams) and therefore writing out future types rather than using async functions, you'll likely see a lot of `Pin<...>` in those types and need to use `Box::pin` to create the futures.
- Implementing a future - inside `poll`, `self` is pinned and therefore you need to work with pin projection and/or unsafe code to get mutable access to fields of `self`.
- Combining futures or streams. This mostly just works, but if you need to take a reference to a future and then poll it (e.g., defining a future outside a loop and using it in `select!` inside the loop), then you will need to pin the reference to the future in order to use the reference like a future.
- Working with streams - there is currently less abstraction in Rust around streams than futures, so you're more likely to use combinator methods (which don't technically require pinning, but seems to make issues around referencing or creating futures/streams more prevalent) or even `poll` manually than when working with futures.


## Alternatives and extensions

This section is for those with a curiosity about the language design around pinning. You absolutely don't need to read this section if you just want to read, understand, and write async programs.

Pinning is difficult to understand and can feel a bit clunky, so people often wonder if there is a better alternative or variation. I'll cover a few alternatives and show why they either don't work or are more complex than you might expect.

However before that, it's important to understand the historical context for pinning. If you are designing a brand new language and want to support async/await, self-references, or immovable types there are certainly better ways to do so than Rust's pinning. However, async/await, futures, and pinning were added to Rust after it's 1.0 release and designed in the context of a strong backwards-compatibility guarantee. Beyond that hard requirement, there was a requirement of wanting to design and implement this feature in a reasonable time frame. Some solutions (e.g., those involving linear types) would require fundamental research, design, and implementation that would realistically be measured in decades when considering the resources and constraints of the Rust project.


### Alternatives

First, lets consider the class of solutions which make Rust types non-movable by default. Note that this is a significant change to the fundamental semantics of Rust; any solution in this class would likely need significant effort to achieve backwards-compatibility (I won't speculate on if that's even possible for specific solutions, but with techniques like auto-traits, derive attributes, editions, migration tooling, etc., it is possibly possible).

One proposal (really, a group of proposals since there are various ways to define the semantics) is to have a `Move` marker trait (similar to `Copy`) which marks objects as movable and all other types would be immovable. In contrast to `Pin`, this is a property of values, not of pointers, so the effect is much more far-reaching, e.g., `let a = b;` would be an error if `b` does not implement `Move`.

The fundamental problem with this approach is that pinning today is a phased concept (a place starts unpinned and becomes pinned) and types apply to the whole lifetime of values. (Pinning is also best understood as a property of places rather than values, but types apply to values, whether this is a fundamental problem for any trait-based approach, I don't know). This is explored in these two blog posts: [Two Ways Not to Move](https://theincredibleholk.org/blog/2024/07/15/two-ways-not-to-move/) and [Ergonomic Self-Referential Types for Rust](https://blog.yoshuawuyts.com/self-referential-types/#immovable-types).

Furthermore, any `Move` trait is likely to have problems with [backwards-compatibility](https://without.boats/blog/pin/) and lead to 'infectious bounds' (i.e., `Move` or `!Move` would be required in many, many places).

Another proposal is to support move constructors similar to C++. However, this breaks the fundamental invariant of Rust that objects can always be bit-wise moved. That would make Rust much less predictable and therefore make Rust programs more difficult to understand and debug. This is a backwards-incompatible change of the worst kind because it would silently break unsafe code because it changes a fundamental assumption that authors of the code may have made. Furthermore, the design and implementation effort required for such a fundamental change would be huge. On top of those practical issues, it's unclear if it would even work: move constructors could be used to fix-up references in the object being moved, but there might be references to the object being moved from outside the object which could not be fixed up.

A potential solution of a different kind is the idea of offset references. This is a reference which is relative rather than absolute, i.e., a field which is an offset reference to another field would always point within the same object, even if the object is moved in memory. The issue with offset pointers is that a field must be either an offset pointer or an absolute pointer. But references in async function become fields which sometimes reference memory internal to the future object and sometimes reference memory outside it.


### Extensions

There are multiple proposals for making pinning more powerful and/or easier to work with. These are mostly proposals to make pinning a more first-class part of the language in various ways, rather than a purely library concept (they often include extensions to std as well as the language). I'll cover a few of the more developed ideas, they are related to each other and all have the general goal of improving pinning ergonomics by making creating and using pinned places easier, in particular around structural pinning and `drop`.

[Pinned places](https://without.boats/blog/pinned-places/) runs with the idea that pinning is property of places rather than values or types, and adds a `pin`/`pinned` modifier to references similar to `mut`. This integrates with reborrowing and method resolution to improve the ergonomics of method calls with pinned `self`.

[`UnpinCell`](https://without.boats/blog/unpin-cell/) extends the pinned places idea to support native pin projection of fields. [MinPin](https://smallcultfollowing.com/babysteps/blog/2024/11/05/minpin/) is a more minimal (and backwards-compatible) proposal for native pin projection and better `drop` support.

The [`Overwrite` trait](https://smallcultfollowing.com/babysteps/series/overwrite-trait/) is a proposed trait which makes explicit the distinction between permission to modify a part of an object (`foo.f = ...`) and permission to overwrite the whole object (`*foo = ...`), both of which are currently allowed for all mutable references. The proposal also includes immutable fields. `Overwrite` is a sort-of-replacement for `Unpin` which (together with some of the ideas from pinned places) could improve working with pinning. Unfortunately, although it could be adopted backwards-compatibly, the transition would be a lot more work than for the other extensions.


## References

- [std docs](https://doc.rust-lang.org/std/pin/index.html) source of truth for behaviour and guarantees of `Pin`, etc. Good docs.
  - [`Pin`](https://doc.rust-lang.org/std/pin/struct.Pin.html), [`Unpin`](https://doc.rust-lang.org/std/marker/trait.Unpin.html), [`pin` macro](https://doc.rust-lang.org/std/pin/macro.pin.html)
- [RFC 2349](https://rust-lang.github.io/rfcs/2349-pin.html) the RFC which proposed pinning. The stabilized API is a bit different from the one proposed here, but there is a good explanation of the core concept and rationale in the RFC.
- Some blog posts or other resources explaining pinning:
  - [Pin](https://without.boats/blog/pin/) by WithoutBoats (the primary designer of pinning) on the history, context, and rationale of pinning, and why it is a difficult concept.
  - [Why is std::pin::Pin so weird?](https://sander.saares.eu/2024/11/06/why-is-stdpinpin-so-weird/) deep dive into the rationale of the pinning design and using pinning in practice.
  - [Pin, Unpin, and why Rust needs them](https://blog.cloudflare.com/pin-and-unpin-in-rust/)
  - [Pinning section of async/await](https://os.phil-opp.com/async-await/#pinning)
  - [Pin and suffering](https://fasterthanli.me/articles/pin-and-suffering) thorough blog post in a very conversational style about understanding async code and pinning with lots of examples.
  - The book *Rust for Rustaceans* by Jon Gjengset has an excellent description of why pinning is necessary for the implementation of async/await and how pinning works.
