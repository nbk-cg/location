# location-cli.rs


Building a cli that convert postal address formats and save them in local files (on a json file as a local database).

## Clone the project
```
git clone https://github.com/yonnwii/location-cli.git
```
## Run the project
```
make start
```
## Example of usage command
```
make setup
```
## Run community-svc locally


### To add a French Address Format
```
cargo run -- store <ID> "<addr1>" "<addr2>" "<zipCode City>" "<Country>"
cargo run -- store 1 "30 Rue Des Mesanges" "" "78300 Poissy" "France"

```

### To add an Iso Address Format
```
cargo run -- store-iso <ID> "<addr1>" "<addr2>" "<zipCode City>" "<Country>"
cargo run -- store-sio 1 "30 Rue Des Mesanges" "" "78300 Poissy" "France"

```

### Fetch French Address Format by ID
```
cargo run -- <COMMAND> <ID> 
cargo run -- fetch 1

```


### Fetch Iso 20022 Address Format by ID
```
cargo run -- <COMMAND> <ID>
cargo run -- fetch-iso 1

```


### Delete any Address Format by ID
```
cargo run -- <COMMAND> <ID>
cargo run -- delete 2

```


### Fetch all Addresses
```
cargo run -- <COMMAND> <ID>
cargo run -- fetch-all

```
