
> `tags` [[test_proptest_runner]] [[test]] #runner #proptest


## Test Compound Strategies
Testing functions that take single arguments of primitive types is nice and all, but is kind of underwhelming. Back when we were writing the whole stack by hand, extending the technique to, say, two integers was clear, if verbose. But TestRunner only takes a single Strategy; 
> How can we test a function that needs inputs from more than one?

## How
Other compound strategies include fixed-sizes arrays of strategies and Vecs of strategies (which produce arrays or Vecs of values parallel to the strategy collection), as well as the **various strategies** provided in the **collection module**.