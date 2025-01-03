# A basic todo CLI tool written in Rust.

Simple todo tool I wrote for some practice.

## Building
Literally just run cargo build.
<br>
```shell
cargo b
```


## Basic Commands
### Add
Adds entries to the list.
<br>
Usage:
```shell
todo add "fighting with the compiler"
# 1. fighting with the compiler
```
```shell
todo add fighting with the compiler
# 1. fighting
# 2. with
# 3. the
# 4. compiler
```

### Set
Sets the status of one or multiple entries in the list.
<br>
Usage:
```shell
todo set completed 1
# The first entry in the list will be marked as complete 
```
```shell
todo set in-progress 2 3
# The second and third entry in the list will be marked as in progress 
```

### Remove
Removes one or more entries from the todo list
<br>
Usage:
```shell
todo remove 1
# The first entry in the list will be removed
# To remove multiple entries in one command, just add multiple indicies seperated by spaces
```

### Clear
Clears the list of its entries
<br>
Usage:
```shell
todo clear
```
```shell
todo clear -w completed
# Only entries with the completed status are removed
```
