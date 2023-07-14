### `migrator`

The open source data format migrator utility for OpenRetail.

## Getting Started
You can install the utility locally using:

```
cargo install odm
``` 

Running `odm` will provide helpful information.

### Converting

To convert toward the open-retail standard, run the following command. 

```python
odm parse --format <FORMAT> --type <TYPE> --input <FILE>
```

or 

```python
odm parse --solution <SOLUTION> --input <DIRECTORY>
```

Where:
-  `<FILE>` represents the file path (or directory) to your `.csv` export(s) from your existing solution, this is often broken into multiple files, the one you wish to migrate can be specified by `--type`.
- `<FORMAT>` represents the name of the existing solution. For supported standards, see [here](#supported-standards).
- `<TYPE>` represents the type being converted, the following options are available: `Products`, `Customers`, `Transactions`, `Stores`.
- `<SOLUTION>` represents the ability to convert all files of a particular standard, the following options are available: `Shopify`.

### Supported Standards
> Please note this is an active area of development, changes should occur quickly.

We currently support:
| Standard Name | Products | Customers | Transactions | Stores |
|--------------|-----------|-----------|-----------|-----------|
| Shopify  | ✅ | ✅ | ✅ | 🔌 |
| Lightspeed Retail  | ❌ | ❌ | ❌ | ❌ |
| SquarePOS | 🚧 | 🚧 | 🚧 | 🚧 |
| Retail Pro  | 🚧 | 🚧 | 🚧 | 🚧 |

*🚧  Havent Investigated Yet*

*🔌  Source lacks implementation*
