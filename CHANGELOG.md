# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.4.0]

* Bumped to `substreams-ethereum` version `0.9`.

* Refactored output type to be friendlier to JSON format by removing usage of `oneof` in the Protobuf
  and by generating a common `Transfer` model instead.

## [0.1.0] - 2022-11-25

* Initial release, offers extractions of ERC20, ERC721 and ERC1155 transfers.
