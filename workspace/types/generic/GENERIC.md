[[rust-in-action/ria-types-generic]]

[[rd-types-generic]]

!!! ria
The fragment <T: std::ops::Add<Output = T>> says that T must implement trait std::ops::Add. Using a single type variable T with the trait bound ensures that arguments i and j, as well as the result type, are the same type and that their type supports addition.
