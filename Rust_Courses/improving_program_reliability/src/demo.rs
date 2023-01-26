// use chrono::{ DateTime, Duration, Utc };
// use thiserror::Error;

// struct SubwayPass {
//     id: usize,
//     funds: isize,
//     expires: DateTime<Utc>,
// }
// #[derive(Debug, Error)]
// enum PassError {
//     #[error("expired pass")]
//     PassExpired,
//     #[error("insufficient funds: {0}")] InsufficientFunds(isize),
//     #[error("pass read error")] ReadError(String),
// }

// fn swipe_card() -> Result<SubwayPass, PassError> {
//     Ok(SubwayPass {
//         id: 0,
//        funds: 200,
//         expires: Utc::now() + Duration::weeks(52),
//     })
// }

// fn use_pass(pass: &mut SubwayPass, cost: isize) -> Result<(), PassError> {
//     if Utc::now() > pass.expired {
//         Err(PassError::PassExpired)
//     } else {
//         if pass.funds - cost < 0 {
//             Err(PassError::InsufficientFunds(pass.funds))
//         } else {
//             pass.funds = pass.funds - cost;
//             Ok(())
//         }
//     }
// }
#[derive(Debug, Clone, Copy)]
struct NeverZero(i32);
impl NeverZero {
    fn new(i: i32) -> Result<Self, String> {
        if i == 0 { Err("can NOT be zero".to_owned()) } else { Ok(Self(i)) }
    }
}
fn divide(a: i32, b: NeverZero) -> i32 {
    let b = b.0;
    a / b
}

fn main() {
    match NeverZero::new(10) {
        Ok(nevzero) => println!("{:?}", divide(10, nevzero)),
        Err(e) => println!("{:?}", e),
    }
    // let pass_status = swipe_card().and_then(|mut pass| use_pass(&mut pass, 3));
    // match pass_status {
    //     Ok(_) => println!("Ok to board!"),
    //     Err(e) => println!("error: {}", e),
    // }
}