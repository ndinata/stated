# Typestates (in Rust)

An exploration of the [typestate](https://en.wikipedia.org/wiki/Typestate_analysis)
concept in Rust.

TODO: add ToC

## Motivation and context

### Why?

I was... TODO

### What?

The flow we are modelling... TODO

![state machine model](./online_store_state_machine.png)

## The Code

The source code is available in [`src/`](./src/), or if you'd prefer to read
everything here on this document instead, feel free to expand the sections below.

<details>
<summary>The application (how we use the state machine)</summary>

```rust
use stated::online_shop::Customer;

fn main() {
    // This enabls the transition `Browsing` -> `Left` via `.leave()`
    let has_sudden_change_of_plan = false;

    // This enables the transition `Shopping` -> `Browsing` via `.clear_cart()`
    let is_using_mums_credit_card = false;

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

<details>
<summary>The library (model of the state machine)</summary>

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

## Readings

- https://willcrichton.net/rust-api-type-patterns/typestate.html
- https://blog.yoshuawuyts.com/state-machines-3/
