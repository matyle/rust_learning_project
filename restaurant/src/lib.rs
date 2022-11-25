pub use crate::front_of_house::hosting;
use front_of_house;

mod front_of_house;

fn serve_order() {
    self::back_of_houst::cook_order()
}

mod back_of_houst {
    fn fix_incorrect_order() {
        cook_order(); //同一个crate中
        super::serve_order(); //private
    }

    pub fn cook_order() {}
}
// pub use crate::font_of_house;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
