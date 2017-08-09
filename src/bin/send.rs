extern crate home_easy;

use home_easy::scheduler_realtime;

fn main() {
    unsafe { scheduler_realtime() };
}
