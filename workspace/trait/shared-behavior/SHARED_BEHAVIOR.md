[[TRAIT_BOUND]]

[[NONE_OBJECT_SAFE]]

[[SB_AGGRIGATOR]]

[[SB_TRAIT_INBUILT]]

[[DEFAULT_CONCRETE_TYPE]]

---

## Generic traits
> Two such examples are is the **From<T> and Into<T> traits**, which allow from conversion from a type to a type T and vice versa.
> generic traits can get quite **verbose** when they are declared with three or four generic types.

```rust,compile_fail,no_run,ignore
pub trait From<T> {
    fn from(T) -> Self;
}
```


## Orphan Rule-Coherence

> The idea of trait coherence is that there **should be exactly one implementation of a trait on a type that implements it**. This should be quite obvious since, with two implementations there would be ambiguity in what to choose between the two.

> Another rule that might confuse many about traits is the orphan rule. The orphan rule, in simple words, **states that we cannot implement external traits on external types.**
To word it in another way, *either the trait must be defined by you* if you are implementing something on an external type, or your type should be defined by you when you are implementing an external trait. This rules out the possibility of having conflicts in overlapping trait implementations across crates.


## Glossery

> `fully qualified syntax` : <Type as Trait>::function(receiver_if_method, next_arg, ...);

> `bound`  : 'where Self: Sized'


---

> `tags` [[orphan_rule]] [[coherence]]
