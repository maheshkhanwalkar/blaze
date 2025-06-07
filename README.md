# blaze

### Project Structure

The `blaze` source is made up of `main.rs` (the main entry point) and packages
for each subcommand. The subcommand entry point in each package is `runner.rs`,
while the other files implement the core logic

### diff
Implements file diff functionality

```shell
blaze diff <file-1> <file-2>
```

### merge
Implements merge functionality

```shell
blaze merge <original-file> <v1-file> <v2-file>
```
