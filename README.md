# Substreams Ethereum Token Transfers
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Show cases Substreams extracting all ERC20/ERC721/ERC1155 transfers from Ethereum events for the full chain with sink output type ready to be consumed by https://github.com/streamingfast/substreams-sink-files in JSONL or CSV format.

## Quick Start

Substreams knowledge is expected in the steps below, if you haven't, ensure you are familiar with [Substreams Installation](https://substreams.streamingfast.io/getting-started/installing-the-cli) and [Substreams Quickstart](https://substreams.streamingfast.io/getting-started/quickstart)

To see the results ready to be consumed by [substreams-sink-files](https://github.com/streamingfast/substreams-sink-files) in JSONL format:

```
make stream_jsonl
```

To see the results ready to be consumed by [substreams-sink-files](https://github.com/streamingfast/substreams-sink-files) in CSV format:

```
make stream_csv
```

Refer to tutorial https://substreams.streamingfast.io/developers-guide/sink-targets/substreams-sink-files to see how you can leverage [substreams-sink-files](https://github.com/streamingfast/substreams-sink-files) to consume this Substreams and have those files written to your computer.
