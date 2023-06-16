use stated::online_shop::Customer;

fn main() {
    // This enables the transition `Browsing` -> `Left` via `leave()`
    let has_sudden_change_of_plan = false;

    // This enables the transition `Shopping` -> `Browsing` via `clear_cart()`
    let is_using_mums_credit_card = false;

    // This enables the transition `Checkout` -> `Shopping` via `cancel_checkout()`
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
