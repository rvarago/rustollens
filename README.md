# RusTollens - Type-level Boolean for Logic

A small and experimental Rust library with a type-level representation of booleans with companion connectives for compile-time sort-of fun with logic.

```rust
type Tollens<P, Q> = Imp<And<Imp<P, Q>, Not<Q>>, Not<P>>;

fn what_is_reality<T: Truth>() {}
what_is_reality::<Or<True, Not<True>>>();
```

The intuition behind it is as follows:

Consider the trait `Negation`:

```rust
trait Negation {
    type Output;
}
```

> Notice the lack of associated functions, it only has an associated *type*.

And the implementation of `Negation` for the types `True` and `False`:

```rust
struct True;
struct False;

impl Negation for True {
    type Output = False
}

impl Negation for False {
    type Output = True
}
```

We may treat `Negation` as a piece-wise function from types to types, where `Self` would be the input and `Output` the output, each combination of input types gives rise to a function (`impl`). In pseudo-code:

```
negation(True) = False
negation(False) = True
```

A function with more inputs would correspond to a trait with generic parameters, such as `Conjunction` defined below:

```rust
trait Conjunction<L> {
    type Output;
}
```

Where we would now need four `impl`s to exhaust the four combinations of `True` and `False`, for instance:

```rust
impl Conjunction<True> for False {
    // False AND True IS False
    type Output = False;
}
```

RusTollens builds on top of these ideas to compute simple logical statements at the type-level, statically at compile-time.

Fairly likely not an amazingly applicable library, still the idea of encoding invariants at the type-level such that the compiler can verify them on our behalf is incredibly powerful. Therefore, this small experiment aims to give some more food for thought and invite the reader to explore and appreciate Rust's type-system to its maximum.