# blaze

### Project Structure

The project consists of two main crates:

- **blaze**: Main executable crate, which the end-user interacts with
- **catalyst**: Core library crate that provides the underlying functionality

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

### kv
Implement key-value (k-v) store functionality

```shell
blaze kv put [--global/--partition <partition>] <key> <value>
```

```shell
blaze kv get [--global/--partition <partition>] <key>
```

```shell
blaze kv hash <key>
```
