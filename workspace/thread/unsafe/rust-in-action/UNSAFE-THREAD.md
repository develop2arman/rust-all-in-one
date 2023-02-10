
> if the Rust compiler doesn’t have enough information to be confident, it will reject the code. In these cases,
you can use unsafe code to tell the compiler, “Trust me, I know what I’m doing.” The downside is that you use it at your own risk: 

> if you use unsafe code incorrectly, problems due to memory unsafety, such as null pointer dereferencing, can occur.

> You can take five actions in unsafe Rust, called unsafe superpowers, that you can’t in safe Rust. Those superpowers 
> include the ability to:
- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of unions

> calling unsafe() would crash the program.

> consider unsafe to be a warning sign rather than an indicator that you’re embarking on anything illegal. Unsafe means “the same level of safety offered by C at all times.” 

> If you still had access to (via unsafe) they might still look like valid S, but any attempt to use them as valid S is undefined behavior. ↓
`https://cheats.rs/#unsafe-unsound-undefined-dark` side of force

>Try to avoid "unsafe {}", often safer, faster solution without it. Exception: FFI.

> People are fallible, and mistakes will happen, but by requiring these five unsafe operations to be inside blocks annotated with unsafe you’ll know that any errors related to memory safety must be within an unsafe block. Keep unsafe blocks small; you’ll be thankful later when you investigate memory bugs.

> To isolate unsafe code as much as possible, it’s best to enclose unsafe code within a safe abstraction and provide a safe API, which we’ll discuss later in the chapter when we examine unsafe functions and methods.

> Parts of the standard library are implemented as safe abstractions over unsafe code that has been audited.Wrapping unsafe code in a safe abstraction prevents uses of unsafe from leaking out into all the places that you or your users might want to use the functionality implemented with unsafe code, because using a safe abstraction is safe.
