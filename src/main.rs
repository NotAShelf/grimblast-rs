// TODO: remove when all functions are properly referenced
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod utils;

fn main() {
    println!("{:?}", utils::actions::get_active_region());
    println!("{:?}", utils::actions::get_focused_monitor());
}
