# blaze

### Project Structure

The `blaze` source is made up of `Main.kt` (the main entry point) and packages
for each subcommand. The subcommand entry point in each package is `Runner.kt`,
while the other files implement the core logic

### diff
Implements file diff functionality

```shell
blaze diff <file-1> <file-2>
```
