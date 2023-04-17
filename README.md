### `open-retail migrator`

The open source data format migrator utility for OpenRetail.

## Getting Started
You can install the utility locally using:

```
cargo install odm
``` 

Running `odm` will provide helpful information.

### Converting

To convert toward the open-retail standard, run the following command. 

```
odm parse --format <FORMAT> --type <TYPE> --input <FILE>

```
Where:
-  `<FILE>` represents the path to your product `.csv` export from your existing solution.
- `<FORMAT>` represents the name of the existing solution. For supported standards, see [here]
(#supported-standards).
- `<TYPE>` represents the type being converted, i.e. `Products`, `Customers`, ...


### Supported Standards
> Please note this is an active area of development, changes should occur quickly.

We currently support:
| Standard Name | Products | Customers | Transactions | Stores |
|--------------|-----------|-----------|-----------|-----------|
| Shopify  | ✅ | ❌ | ❌ | ❌ |
| Lightspeed  | ❌ | ❌ | ❌ | ❌ |