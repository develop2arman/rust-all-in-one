
> `tags` [[test_fixture]] [[test]]

## Fixture
A fixture is something that you can use in your tests to encapsulate a test’s dependencies.

The general idea is to have smaller tests that only describe the thing you’re testing while you hide the auxiliary utilities your tests make use of somewhere else. For instance, if you have an application that has many *tests with users, shopping baskets, and products, you’d have to create a user, a shopping basket, and product every single time in every test which becomes unwieldy quickly.* In order to cut down on that repetition, you can instead use fixtures to declare that you need those objects for your function and the fixtures will take care of creating those by themselves. Focus on the important stuff in your tests!

In rstest a fixture is a function that can return any kind of valid Rust type. This effectively means that your fixtures are not limited by the kind of data they can return. **A test can consume an arbitrary number of fixtures at the same time.**


```rust, no_run, compile_fail
#[fixture]
fn alice_and_bob(mut empty_repository: impl Repository) -> impl Repository {
    empty_repository.add("Bob", 21);
    empty_repository.add("Alice", 22);
    empty_repository
}

#[rstest]
fn should_process_two_users(alice_and_bob: impl Repository,
                            string_processor: FakeProcessor) {
    string_processor.send_all("Good Morning");

    assert_eq!(2, string_processor.output.find("Good Morning").count());
    assert!(string_processor.output.contains("Bob"));
    assert!(string_processor.output.contains("Alice"));
}
```

Injecting fixtures as function arguments

rstest functions can receive fixtures by using them as input arguments. A function decorated with [rstest] will resolve each argument name by call the fixture function. Fixtures should be annotated with the [fixture] attribute.

Fixtures will be resolved like function calls by following the standard resolution rules. Therefore, an identically named fixture can be use in different context.

```rust, no_run, compile_fail
mod empty_cases {
    use super::*;

    #[fixture]
    fn repository() -> impl Repository {
        DataSet::default()
    }

    #[rstest]
    fn should_do_nothing(repository: impl Repository) {
        //.. test impl ..
    }
}

mod non_trivial_case {
    use super::*;

    #[fixture]
    fn repository() -> impl Repository {
        let mut ds = DataSet::default();
        // Fill your dataset with interesting case
        ds
    }

    #[rstest]
    fn should_notify_all_entries(repository: impl Repository) {
        //.. test impl ..
    }
}
```

```rust, no_run, compile_fail
#[cfg(test)]
mod fixt {

    #[rstest]
    #[case("1.2.3.4:8080", 8080)]
    #[case("127.0.0.1:9000", 9000)]
    fn check_port(#[case] addr: SocketAddr, #[case] expected: u16) {
        assert_eq!(expected, addr.port());
    }
}
```