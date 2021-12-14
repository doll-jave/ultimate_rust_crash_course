const STARTING_MISSILES: i32 = 8;
const READY_MOUNT: i32 = 2;

fn main() {
    // Initializing variables in separate statements
    // let ready: i32 = READY_MOUNT;
    // let mut missiles: i32 = STARTING_MISSILES;

    // Initializing variables in one statement
    let (ready, mut missiles) = (READY_MOUNT, STARTING_MISSILES);

    // If I wanted both variables to be mutable...
    // I would put mut in front of both variable names
    // let (mut ready, mut missiles) = (READY_MOUNT, STARTING_MISSILES);

    missiles = missiles - ready;
    println!("Firing {} of my {} missiles...", ready, missiles);

    // When I comment this...
    // missiles = missiles - ready;
    // And change to this...
    // println!("Firing {} of my {} missiles...", ready, missiles - ready);
    // With these two statements, I get a warning instead of an error!
    // The warning is: variable does not need to be mutable!

    // let not_used = 10;
    // When I don't use a variable, I get an "unused variable:" warning

    // READY_MOUNT = 5;
    // When I try to change the value of a constant, I get the following error,
    // "error[E0070]: invalid left-hand side of assignment"
}
