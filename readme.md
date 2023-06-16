# Typestates (in Rust)

A personal exploration\*<sup>+</sup> of the [typestate](https://en.wikipedia.org/wiki/Typestate_analysis)
concept in Rust v1.70.

###### \*exploration means this repo focuses on the writeup on top of the code, and this README serves as a long-ish report on the results

###### <sup>+</sup>it also means there may be some inaccurate understanding on my part as i'm still familiarising myself with the topic

## Table of contents

- [Motivation and context](#motivation-and-context)
  - [Why](#why)
  - [What](#what)
- [The code](#the-code)
- [Results](#results)
  - [Valid state transitions](#valid-state-transitions)
  - [Invalid state transitions](#invalid-state-transitions)
- [Readings](#readings)

## Motivation and context

### Why?

When I first encountered [SPARK Ada](https://www.adacore.com/about-spark) in my uni class on high-integrity systems
engineering, I was amazed by some of the property guarantees that its static analysis could provide with its "ghost
code" annotations. Around this time I was also starting to look into Rust, notorious for its explicit ownership model
and strong type system, and these experiences solidified my belief that we should be able to encode as many desirable
properties and assumptions of a system as possible into source code that our tools (type checker, linter, compiler etc.)
can verify for us statically: either during write time or build/compile time.

Fast forward to a more recent time, I was reading about
[how to link Rust code with C++ code](https://docs.rust-embedded.org/book/interoperability/rust-with-c.html)
and noticed the ["Static Guarantees"](https://docs.rust-embedded.org/book/static-guarantees/index.html) section of
the document. I then read the section on "typestates" and how the pattern can be used to enforce valid state
transitions... at compile time!! Being the fan of static guarantees that I am, I of course had to play around
with it :\)

### What?

The state machine that we are representing in this exploration is a simple model of an online shopping flow.
A customer visits an online store, adds some items to the cart if they want, and then finally checks out to
finalise the purchase. A rough sketch of the state machine is depicted below.

![state machine model](./online_store_state_machine.png)

The main objective here is to explore using typestates to implement the model, such that we would be able
to **statically** validate that we are only using valid transitions between the different states.

## The code

The source code is available in [`src/`](./src/), but if you'd prefer to read everything
here on this document instead, feel free to expand the sections below. More bite-sized
chunks of code with explanation are also available in the [Results](#results) section.

<details>
<summary>The library (implementation of the state machine)</summary>

```rust
pub mod online_shop {
    use std::marker::PhantomData;

    // The different states the customer can be in throughout the shopping flow.
    // We can model a "Left" state if we want, but we don't have to.
    pub struct Browsing;
    pub struct Shopping;
    pub struct Checkout;

    // Representation of the online shop customer (the domain entity).
    // The fields are private so we can't instantiate it directly and would have
    // to use the exposed `visit_site()` func as the entry point.
    pub struct Customer<S> {
        shopping_cart: Vec<u8>,
        _inner: PhantomData<S>,
    }

    // This contains the only transitions allowed from the "Browsing" state.
    impl Customer<Browsing> {
        // This is the only entry point to the flow, starting with "Browsing".
        pub fn visit_site() -> Self {
            println!("Hi site!");
            Customer {
                shopping_cart: vec![],
                _inner: PhantomData,
            }
        }

        // This consumes `self`, so after calling this func we shouldn't be able
        // to use the `Customer` value anymore, which is why we don't need to
        // model the "Left" (end) state explicitly.
        pub fn leave(self) {
            println!("Not buying anything, bye site!");
        }

        // "Browsing" -> "Shopping"
        pub fn add_item(mut self, item: u8) -> Customer<Shopping> {
            self.shopping_cart.push(item);
            println!("Added {} to cart ({:?})", item, self.shopping_cart);
            Customer {
                shopping_cart: self.shopping_cart,
                _inner: PhantomData,
            }
        }
    }

    // This contains the only transitions allowed from the "Shopping" state.
    impl Customer<Shopping> {
        // "Shopping" -> "Shopping"
        pub fn add_item(mut self, item: u8) -> Self {
            self.shopping_cart.push(item);
            println!("Added {} to cart ({:?})", item, self.shopping_cart);
            self
        }

        // "Shopping" -> "Shopping"
        pub fn pop_item(mut self) -> Self {
            if let Some(popped) = self.shopping_cart.pop() {
                println!("Removed {} from cart ({:?})", popped, self.shopping_cart);
            }
            self
        }

        // "Shopping" -> "Browsing"
        pub fn clear_cart(mut self) -> Customer<Browsing> {
            self.shopping_cart.clear();
            println!("Cart has been cleared.");
            Customer {
                shopping_cart: self.shopping_cart,
                _inner: PhantomData,
            }
        }

        // "Shopping" -> "Checkout"
        pub fn proceed_to_checkout(self) -> Customer<Checkout> {
            println!("Proceeding to checkout.");
            Customer {
                shopping_cart: self.shopping_cart,
                _inner: PhantomData,
            }
        }
    }

    // This contains the only transitions allowed from the "Checkout" state.
    impl Customer<Checkout> {
        // "Checkout" -> "Shopping"
        pub fn cancel_checkout(self) -> Customer<Shopping> {
            println!("Cancelling checkout, continue shopping.");
            Customer {
                shopping_cart: self.shopping_cart,
                _inner: PhantomData,
            }
        }

        // This, like `leave()`, also consumes `self` and returns nothing, so
        // this transition leads to the end of the flow.
        pub fn finalise_payment(self) {
            println!("Done paying for the items, bye site!");
        }
    }
}
```

</details>

<details>
<summary>The application (how we use the state machine)</summary>

```rust
use stated::online_shop::Customer;

fn main() {
    // This enables the transition `Browsing` -> `Left` via `.leave()`
    let has_sudden_change_of_plan = false;

    // This enables the transition `Shopping` -> `Browsing` via `.clear_cart()`
    let is_using_mums_credit_card = false;

    // This enables the transition `Checkout` -> `Shopping` via `.cancel_checkout()`
    let forgot_my_wallet = false;

    let catalogue: Vec<u8> = vec![20, 42, 36, 13, 71, 100];
    let (first, rest_of_items) = catalogue.split_first().unwrap();

    // Entry point of the flow: start with "Browsing".
    let mut browsing = Customer::visit_site();

    if has_sudden_change_of_plan {
        // One possible transition from "Browsing".
        browsing.leave();
        return;
    }

    // The other possible transition from "Browsing".
    let mut shopping = browsing.add_item(*first);

    for item in rest_of_items {
        // This is just some arbitrary logic to exhibit using both `add_item()`
        // and `pop_item()`.
        if item % 2 == 0 {
            shopping = shopping.add_item(*item);
        } else {
            shopping = shopping.pop_item();
        }
    }

    if is_using_mums_credit_card {
        // One possible "ending" to the flow, via clearing the cart and just leaving.
        browsing = shopping.clear_cart();
        browsing.leave();
        return;
    }

    // The other possible "ending" to the flow, where we actually proceed with
    // checkout and then leave.
    let checkout = shopping.proceed_to_checkout();

    if forgot_my_wallet {
        // This demonstrates another branch where instead of just going forwards,
        // we backtrack.
        shopping = checkout.cancel_checkout();
        browsing = shopping.clear_cart();
        browsing.leave();
        return;
    }

    checkout.finalise_payment();

    // This "default" flow results in this output:
    // Hi site!
    // Added 20 to cart ([20])
    // Added 42 to cart ([20, 42])
    // Added 36 to cart ([20, 42, 36])
    // Removed 36 from cart ([20, 42])
    // Removed 42 from cart ([20])
    // Added 100 to cart ([20, 100])
    // Proceeding to checkout.
    // Done paying for the items, bye site!
}
```

</details>

## Results

Earlier we mentioned that the main objective of this exploration is to _statically_ validate
state transitions. To be more precise, that means the following two conditions have to hold:

- if all state transitions used are valid, the program should compile, and
- if at least one invalid state transation is used, the program should _not_ compile

Let's start with the first one, as it's more straightforward.

### Valid state transitions

The main program (in the [code](#the-code) section) only uses valid transitions and it
compiles — our job here is done 😎

No, really — even if we flip around the boolean values at the top of the main function,
the `return;` statements in the `if` blocks ensure that we only ever:

- `leave()` **OR** `add_item()` to leave the "Browsing" state,
- `clear_cart()` **OR** `proceed_to_checkout()` to leave the "Shopping" state, and
- `cancel_checkout()` **OR** `finalise_payment()` to leave the "Checkout" state

### Invalid state transitions

## Readings

- https://docs.rust-embedded.org/book/static-guarantees/typestate-programming.html
- https://willcrichton.net/rust-api-type-patterns/typestate.html
- https://blog.yoshuawuyts.com/state-machines-3/
