pub fn calculate_price(quantity : i32) -> i32 {
    if quantity < 40 {
        quantity * 2
    } else {
        quantity
    }
}
