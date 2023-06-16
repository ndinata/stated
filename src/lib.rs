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
    // The methods take `self` and not `&self` to disable reusing of the value
    // after the method call. If the value is meant to be reused, the methods can
    // return an instance of `Self`.
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
    // The methods take `self` and not `&self` to disable reusing of the value
    // after the method call. If the value is meant to be reused, the methods can
    // return an instance of `Self`.
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
    // The methods take `self` and not `&self` to disable reusing of the value
    // after the method call. If the value is meant to be reused, the methods can
    // return an instance of `Self`.
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
