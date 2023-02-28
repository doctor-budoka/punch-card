use std::env;

enum SubCommand {
    In,
    Out,
    Pause,
    Resume,
}

impl SubCommand {
    fn from_string(name: &String) -> Self {
        return match name.to_owned().trim() {
            "in" => Self::In,
            "out" => Self::Out,
            "pause" => Self::Pause,
            "resume" => Self::Resume,
            other => panic!("{other} is not a valid subcommand!"),
        }
    }
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let command_name: &String = &env_args[1];
    let command: SubCommand = SubCommand::from_string(command_name);

    match command {
        SubCommand::In => punch_in(),
        SubCommand::Out => punch_out(),
        SubCommand::Pause=> take_break(),
        SubCommand::Resume => resume(),
    }

    // Continued program logic goes here...
}

fn punch_in() {
    println!("Clocking in for the day");
}

fn punch_out() {
    println!("Clocking out for the day");
}

fn take_break() {
    println!("Taking a break")
}

fn resume() {
    println!("Getting back to work")
}
