pub mod hosting;
mod serving; 
pub fn serve() {
    hosting::seat_at_table();
    serving::take_order();
    serving::take_payment();
    serving::notify_parent();
}
