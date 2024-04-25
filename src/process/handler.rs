use crate::state::Counter;

pub fn handle_increment(counter_account: &mut Counter, value: u32) {
    counter_account.counter += value;
}

pub fn handle_decrement(counter_account: &mut Counter, value: u32) {
    counter_account.counter = if counter_account.counter > value {
        counter_account.counter - value
    } else {
        0
    };
}

pub fn handle_update(counter_account: &mut Counter, value: u32) {
    counter_account.counter = value;
}

pub fn handle_reset(counter_account: &mut Counter) {
    counter_account.counter = 0;
}
