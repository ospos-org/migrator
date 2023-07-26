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
odm parse <DIRECTORY>
```

The output of which will be `output.os`.

`migrator` will automatically determine the origin of the files in the directory and apply the most appropriate parsing for it. This means, you may place all exported files in this directory and `migrator` will decode (to the best of its ability) each one, as long as it exists in the [currently supported standards list](#supported-standards).

### Supported Standards
> Please note this is an active area of development, changes should occur quickly.

We currently support:
| Standard Name | Products | Customers | Transactions | Stores |
|--------------|-----------|-----------|-----------|-----------|
| Shopify  | ✅ | ✅ | ✅ | 🔌 |
| Lightspeed Retail  | ❌ | ❌ | ❌ | ❌ |
| SquarePOS | 🚧 | 🚧 | 🚧 | 🚧 |
| Retail Pro  | 🚧 | 🚧 | 🚧 | 🚧 |

*🚧  Have yet to investigate file formatting*

*🔌  Source lacks implementation/export*
