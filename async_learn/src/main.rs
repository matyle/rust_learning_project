mod asynclearn;
mod futurelearn;
use futures::executor::block_on;
fn main() {
    let f = asynclearn::do_some();
    block_on(f)
}
