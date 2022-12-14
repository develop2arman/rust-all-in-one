   
   #![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://doc.rust-lang.org/")]

   % The title

   This is the example documentation.

   ///! you can add lines that start with #, and they will be hidden from the output, but will be used when compiling your code

    First, we set `x` to five:

    ```rust
    let x = 5;
    # let y = 6;
    # println!("{}", x + y);
    ```

    Next, we set `y` to six:

    ```rust
    # let x = 5;
    let y = 6;
    # println!("{}", x + y);
    ```

    Finally, we print the sum of `x` and `y`:

    ```rust
    # let x = 5;
    # let y = 6;
    println!("{}", x + y);
    ```

    /// should_panic tells rustdoc that the code should compile correctly, but not actually pass as a test.
    ```rust,should_panic
    assert!(false);
    ```

    ``` rust,should_fail
    // This code block is expected to generate a failure
    ```
    pub fn should_fail_func(){}


    /// The no_run attribute will compile your code, but not run it
    /// loop {
    ///     println!("Hello, world");
    /// }
    /// ```

    #[allow(missing_docs)]
    struct Undocumented;

    <script>alert(document.cookie)</script>

