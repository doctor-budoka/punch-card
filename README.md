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
- `edit`: Allows you to edit your day so far. It opens up the day as a file in an editor (vim by default) so you can make any changes needed.
- `summary`: Prints a summary of your day. Tells you how many minutes you have worked, how many minutes you have left and how far behind on time you have fallen (for instance, if you finished early one of the days and need to make that time back). It also gives a summary of the tasks you've done and the time spent too.
- `note`: Used to add a note at the current time.
- `add-summary`: Used to add a summary for what's been done for a particular task.

In addition, once you've "punched out" you can run:
- `back-in`: If you end the day accidentally/learn later that you need to punch back in, use this command.

The following commands don't require you to have "punched in" for the day yet:
- `view-past`: Allows you to see a string representation of some day in the past. It takes one argument: A date string in yyyy-mm-dd format.
- `summary-past`: Does the same as `summary` except you can pick some day in the past. It takes as argument a date string in yyyy-mm-dd format.
- `summarise-week`: This prints a similar summary to the last two commands except it does it for a week's worth of days. Ran without argument, it summarises the last 7 days including today. Otherwise, you can provide it with a single date string argument, which allows you to summarise the week ending on that date. In addition, you can provide it with a second argument that specifies the time behind when starting the week, which adds an extra summary line.
- `summarise-days`: This does the same as the previous command except you have to specify the start and end dates. If only one date is provided, it will just summarise that one day, if two date strings are provided, it summarises those days (inclusive). You can also provide a third argument indicating the time behind at the start of the period.
- `edit-config`: Used to edit the configuration file for `punch`. It opens it up the config file in an editor (vim by default).
- `view-config`: Used to view the configuration file for `punch`.

The config file will be stored at `~/.punch-card/punch.cfg`. This stores the length of your day in minutes (480 minutes or 8 hours by default) as well as storing how many minutes you have fallen behind.

## Installation

At the moment, the only way to install is to build the program locally. You'll need to have Rust and Cargo installed. In addition, you'll need some sort of text editor installed to use commands such as `punch edit`. The following instructions should work for any *NIX OS (though something like it should work on pretty much any OS including Windows).

1. Clone this repository to your computer.
2. Run 'cargo build --release'. The executable will then appear in `/target/release/punch`
3. Copy it to somewhere on your PATH

Alternatively, you can run the included `install.sh` after you have cloned your repository, provided you have a `/usr/local/bin/` directory. You will also need to add `usr/local/bin/` to your PATH if it hasn't been added already.

## Using your own favourite editor

As stated above, we assume Vim as the default editor for use with `punch edit` and `punch edit-config`. However, if you prefer a different editor, this can be changed in one of two ways:

1. You can set the `editor_path` in the config
2. You can set the `EDITOR` environment variable

If the editor in question is already on the `PATH`, then you just need to provide the name for the editor, otherwise, you need to provide the full path for it.
In addition, most GUI based editors won't work exactly as is: `punch-card` will open the editor and not wait for the user to make the changes. 
A lot of popular GUI based editors provide a "wait" flag that will work instead. Some examples (though these depend on what version and sometimes what OS you have too):

1. gedit has `gedit -w` or `gedit --wait`
2. VS Code has `code --wait`
3. Sublime has `subl -n -w` (`-n` for a new window and then `-w` for wait as with the others)

If you can't find a way to do this from your editor's documentation, try searching for how to use that editor as the default editor for git. 
The answer that works for git will likely work for this too.
