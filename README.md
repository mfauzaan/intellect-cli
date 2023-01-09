# intellect-cli

## Get started

### Install cargo
Once you have it installed, run `help` to view the available commands:

```
cargo run -- help
```

The intellect-cli has two main commands: `weather` and `crypto-currency`. 
To view the options for each command, you can use the help command followed by the name of the command, like this:
```
cargo run -- help weather
```

### Example usage of weather

```
cargo run -- weather --city malaysia --api-key 29c5cc6856a72a94b080356e78d3c88c
```

### Example usage of crypto-currency

```
cargo run -- crypto-currency --api-key cee95a22-46fb-49f6-9a39-3fe012283984
```
