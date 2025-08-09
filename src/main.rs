mod demo;
mod exchange;
mod market;
mod order_book;
mod order;
mod price_level;
mod request;
mod state;

mod order_book_test;
mod order_test;
mod price_level_test;

fn main() -> Result<(), String> {
    demo::run();
    Ok(())
}
