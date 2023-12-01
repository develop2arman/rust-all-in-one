
Binary crates cannot be tested in the same way as library crates, because the items in binary crates cannot to be imported by other crates. Instead, we can test the binary crate by running it with the assert_cmd crate. This crate allows us to run the binary crate as a subprocess and assert that it exits successfully and prints the expected output.

> `Tags` [[test]] #binary [[test_binary]] [[test-integration]] [[assert_cmd]] [[fold]] [[map]] [[cmd]]
