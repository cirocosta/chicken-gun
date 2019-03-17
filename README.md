### chicken-gun 

> A **chicken gun** is a large-diameter, compressed-air cannon used to fire dead chickens at aircraft components in order to simulate high-speed bird strikes during the aircraft's flight. (source: [*Wikipedia*](https://en.wikipedia.org/wiki/Chicken_gun))

Here you can find `cg`, a tool aimed at providing very targetted load at specific parts of a machine to verify:

- what happens when specific problematic scenarios occur, and
- if we're properly collecting telemetry from our systems.

```
chicken-gun 0.1.0
Throwing chickens into the system

USAGE:
    cg [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --pid <pid>    file to write the PID of the current execution to [default: /tmp/cg.pid]

SUBCOMMANDS:
    cpu            drive user cpu utilization to the top
    help           Prints this message or the help of the given subcommand(s)
    memory         tries to allocate a lot of memory
    memory-wave    keeps allocating and deallocating memory in intervals
```
