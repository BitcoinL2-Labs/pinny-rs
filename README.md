# Pinny

**Pinny** is a procedural macro crate that enables test tagging for better test organization and filtering, providing fine grain control over test execution.

The main idea is to provide a tagging mechanism that:
- supports `cargo test`
- supports `cargo nextest` (and takes advantage of filteset DSL)
- allows test filtering to work at `runtime` (read as: avoid re-com
pilation when trying to run tests with different filter)
- allows to use third-party `#[test]` attribute, other than the rust build-in one.
- dev-friendly to setup and maintain, having a clear vision on what tags are being used and also protect against wrong usage (configuration driven tagging)

## Table of Contents

* [Usage](#usage)
    * [Configuration](#step1-configuration)
    * [Test Tagging](#step2-test-tagging)
    * [Test Execution](#step3-test-execution)
* [Appendix](#appendix)
    * [Insights](#insights)
    * [Drawbacks](#drawbacks)

## Usage
To use `pinny` you need to follow this steps:
- list the available tag labels in `Cargo.toml`
- use the labels to tag your tests with `#[tag]` attribute
- then use your preferred test runner for tests filtering and execution

### Step1: Configuration
In your package `Cargo.toml` define the list of allowed tags for your tests, by configuring `package.metadata.pinny.allowed`attribute:

```toml
# Cargo.toml

...

[dev-dependencies]
pinny = ...

[package.metadata.pinny] 
allowed = ["tag1", "tag2", "tag3"]
```

### Step2: Test Tagging

Implement your test as usual and use `#[tag]` attribute to assign relevants labels to them.

> NOTE: `#[tag]` must precede `#[test]` attribute, and possibly any kind of attribute attached to the test

```rust
#[cfg(test)]
mod tests {
    use pinny::tag;
    #[tag(tag1)]
    #[test]
    fn test_1() { assert!(true); }

    #[tag(tag1, tag2)]
    #[test]
    fn test_12() { assert!(true); }

    #[tag(tag2, tag3)]
    #[test]
    fn test_23() { assert!(true); }

    #[tag(unexistent)]  // Compilation Error due to `unexistent` tag not configured in `Cargo.toml`
    #[test]
    fn test_unexistent() { assert!(true); }
}
```

### Step3: Test execution

For test filtering and execution, one can use the preferred test runner.

Here few examples using rust built-in and nextest.
> Take a look at the [Insights](#insights) for further details on how it works under the hood and then properly build filtering expression for your tests. 

- cargo test:
    ```bash
    cargo test :tag1:

    running 2 tests:
    test tests::test_1::t::tag1::t ... ok
    test tests::test_12::t::tag1::tag2::t ... ok
    ```

- cargo nextest commandline:
    ```bash
    cargo nextest --filter-expr 'test(:tag2:) and test(:tag3:)'     

    Starting 1 tests across 1 binary (3 tests skipped)
     Running [ 00:00:00] 0/1: 0 running, 0 passed, 0 skipped
        PASS [   0.009s] your-package tests::test_23::t::tag2::tag3::t

     Summary [   0.009s] 1 tests run: 1 passed, 2 skipped
    ```

- cargo nextest config:
    ```toml
    [profile.t1]
    default-filter = "test(:tag1:)"                      # simple

    [profile.t2]
    default-filter = "test(/:t::(?:.*::)?tag2:/)"        # anti-clash

    [profile.t23]
    default-filter = "test(:tag2:) and test(:tag3:)"     # expression
    ```
    
    ```bash
    cargo nextest --profile t23

    Starting 1 tests across 1 binary (3 tests skipped)
     Running [ 00:00:00] 0/1: 0 running, 0 passed, 0 skipped
        PASS [   0.009s] your-package tests::test_23::t::tag2::tag3::t

     Summary [   0.009s] 1 tests run: 1 passed, 2 skipped
    ```

## Appendix

### Insights
The `#[tag]` procedural macro does some mangling on the test function, basically enriching it with the list of labels associated.

So a test defined like this:
```rust
#[cfg(test)]
mod tests {
    use pinny::tag;
    #[tag(tag1)]
    #[test]
    fn test_1() { assert!(true); }
``` 
where its path is `tests::test_1`, becomes `tests::test_1::t::tag1::t`

Basically, the function name is trasformed in a module and the real test function is named `t` (the last in the sequence). 
Even each tag label associated to the function become a module and the list of the labels is preceded by a `t` (the first in the sequence). 

This ensure specific delimiter pattern enclosing the list of labels `t::<labels>::t`, that can help specific test search or anti-clash filter scenario.

### Drawbacks
Known or potential drawbacks:
- Exact test match: filtering test by "exact" path match cannot more be used (e.g. `--exact` option).  Anyhow, considering that original test path is preserved is still possibile filter by it. So this should be a very minor issue.
- Interoperability with other crates: considering the test function mangling,  potentially this can conflict with other testing lbraries that do test mangling as well(like: `rstest` or `test_case`). Depending on the case, it could be addressed with proper ordering of the related attributes, or not at all.
- Using `super::` from a parent module: within a tagged test, referencing symbols from a parent module (for instance `super::parent_func`) won't compile. This can mitigated importing the symbol outside the test (`use super::parent_func`) and then directly use the symbol within the test (`parent_func()`);
    ```rust
    // mod.rs
    //////////////////////////////////
    #[cfg(test)]
    mod test_module;
    
    fn parent_func() {}
    //////////////////////////////////

    
    // test_module.rs
    //////////////////////////////////
    #[tag(tag1)]
    #[test]
    fn test_fail() {
        // won't compile: symbol not found
        super::parent_func();
    }

    use super::parent_func;

    #[tag(tag1)]
    #[test]
    fn test_ok() {
        parent_func();
    }
    //////////////////////////////////
    ```
