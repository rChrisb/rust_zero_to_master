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
// #[derive(Debug, Clone, Copy)]
// struct NeverZero(i32);
// impl NeverZero {
//     fn new(i: i32) -> Result<Self, String> {
//         if i == 0 { Err("can NOT be zero".to_owned()) } else { Ok(Self(i)) }
//     }
// }
// fn divide(a: i32, b: NeverZero) -> i32 {
//     let b = b.0;
//     a / b
// }
struct Employee<State> {
    name: String,
    state: State,
}
impl<State> Employee<State> {
    fn transition<NextState>(self, state: NextState) -> Employee<NextState> {
        Employee {
            name: self.name,
            state: state,
        }
    }
}
struct Agreement;
struct Signature;
struct Training;
struct FailedTraining {
    score: u8,
}
#[allow(dead_code)]
struct OnboardingComplete {
    score: u8,
}

// impl Employee<Agreement> {
//     fn new(name: &str) -> Self {
//         Self {
//             name: name.to_string(),
//             state: Agreement,
//         }
//     }
//     fn read_agreement(self) -> Employee<Signature> {
//         self.transition(Signature)
//     }
// }
// impl Employee<Signature> {
//     fn sign(self) -> Employee<Training> {
//         self.transition(Training)
//     }
// }
// #[rustfmt::skip]
// impl Employee<Training> {
//     fn train(self, score: u8) -> Result<Employee<OnboardingComplete>, Employee<FailedTraining>> {
//         if score >= 7 {
//             Ok(self.transition(OnboardingComplete {score}))
//         } else {
//             Err(self.transition(FailedTraining {score}))
//         }
//     }
// }
enum Status {
    Error(i32),
    Info,
    Warn,
}
enum Species {
    Finch,
    Hawk,
    Parrot,
}
struct Bird {
    age: usize,
    species: Species,
}
fn main() {
    let status = status::Error(5);
    match status {
        Status::Error(s @ 2) => println!("error three"),
        Status::Error(s @ 5..=10) => println!("error 5 through 10"),
        Status::Error(s @ 18 | s @ 20) => println!("error 18 or 20"),
        Status::Error(s) => println!("error"),
        Status::Info => println!("info"),
        Status::Warn => println!("warn"),
    }
    let hawk = Bird { age: 21, species: Species::Parrot };
    match hawk {
        Bird { age: 4, .. } => println!("4 year old bird"),
        Bird { species: Species::Parrot, .. } => println!("this bird is a Parrot"),
        Bird { .. } => println!("a bird"),
    }
    // let employee = Employee::new("Sarah");
    // let onboarded = employee.read_agreement().sign().train(10);
    // match onboarded {
    //     Ok(emp) => println!("onboarding complete, score: {}", emp.state.score),
    //     Err(emp) => println!("training failed, score: {}", emp.state.score),
    // }
    // // match NeverZero::new(10) {
    //     Ok(nevzero) => println!("{:?}", divide(10, nevzero)),
    //     Err(e) => println!("{:?}", e),
    // }
    // let pass_status = swipe_card().and_then(|mut pass| use_pass(&mut pass, 3));
    // match pass_status {
    //     Ok(_) => println!("Ok to board!"),
    //     Err(e) => println!("error: {}", e),
    // }
}