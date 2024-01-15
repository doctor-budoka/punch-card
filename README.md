# Punch Card

This is a CLI tool for tracking time done. 

With the move to remote working and flexible working hours, it can be hard to track your time at work. This CLI tool allows you to track the hours you have done each day. You just need to punch in and punch out. It also tracks how much time you need to catch up on if you don't do your full time some day.

## How to Use

Once it's installed you can start your day by running `punch in`. The following subcommands can be run once the you have "punched in" for the day:

- `pause`: To take a break.
- `resume`: To resume after you come back from a break. You should give it a new task name for the black about to start.
- `out`: Ends the day. If you end the day while on a break, the break is automatically ended. This also works if you end up working after midnight too.
- `task`: Used to start a new time-block for working on a new task. Used for task time-tracking.
- `view`: Allows you to see a string representation of your day.
- `edit`: Allows you to edit your day so far.
- `summary`: Prints a summary of your day. Tells you how many minutes you have worked, how many minutes you have left and how far behind on time you have fallen (for instance, if you finished early one of the days and need to make that time back). 
- `note`: Used to add a note at the current time.
- `edit-config`: Used to edit the configuration file for `punch`.
- `view-config`: Used to view the configuration file for `punch`.
- `add-summary`: Used to add a summary for what's been done for a particular task.

The config file will be stored at `~/.punch-card/punch.cfg`. This stores the length of your day in minutes (480 minutes or 8 hours by default) as well as storing how many minutes you have fallen behind.


## Installation

At the moment, the only way to install is to build the program locally. You'll need to have Rust and Cargo installed as well as Vim. In addition, this has only been tested on a Mac (though it should work on Linux and Windows too, with different instructions).

1. Clone this repository to your computer.
2. Run 'cargo build -- release'. The executable will then appear in `/target/release/punch`
3. Copy it to somewhere on your PATH

Alternatively, you can run the included `install.sh` after you have cloned your repository, provided you have a `/usr/local/bin/` directory. You will also need to add `usr/local/bin/` to your PATH if it hasn't been added already.
