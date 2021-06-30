
use std::io::*;

pub fn input(mut waiter: String) -> String {
    stdin().read_line(&mut waiter).expect("input error => ?");

    return waiter
}