
> `tags` [[proptest_test]] 

## Proptest
Proptest is a property testing framework (i.e., the QuickCheck family) inspired by the Hypothesis framework for Python. It allows to test that certain properties of your code hold for arbitrary inputs, and if a failure is found, automatically finds the minimal test case to reproduce the problem. Unlike QuickCheck, generation and shrinking is defined on a per-value basis instead of per-type, which makes it more flexible and simplifies composition.

## What is property testing?
Property testing is a system of testing code by checking that certain properties of its output or behaviour are fulfilled for all inputs. These inputs are generated automatically, and, critically, when a failing input is found, the input is automatically reduced to a minimal test case.
Property testing is best used to complement traditional unit testing (i.e., using specific inputs chosen by hand). Traditional tests can test specific known edge cases, simple inputs, and inputs that were known in the past to reveal bugs, whereas property tests will search for more complicated inputs that cause problems.