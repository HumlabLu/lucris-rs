# Lucris

## Intro

Terminal tool to process files for the AI Lund project.

Takes the LUCRIS `jsonl` files and extract information in plain text.
Allows extraction of the Swedish and English texts, and optionally filters out "opt-out" UUIDs.

Tries to connect people UUIDs from `persons.jsonl` to research from `research.jsonl`. 
Also reads the other files (fingerprints, concepts and orgunits), but these are not processed yet.

Just dumps plain text to standard-out at the moment. Output can be
used in `haystack_research.py` for LLM querying.

## Parameters

```shell
Process files for the AI Lund project.

Usage: lucris-rs [OPTIONS]

Options:
  -r, --research <RESEARCH>          The file containing the cleaned research-outputs.
  -p, --persons <PERSONS>            The file containing the cleaned persons.
  -f, --fingerprints <FINGERPRINTS>  The file containing the cleaned fingerprints.
  -c, --concepts <CONCEPTS>          The file containing the cleaned concepts.
  -o, --orgunits <ORGUNITS>          The file containing the cleaned organisational-units.
  -u, --optout <OPTOUT>              The file containing the opt-out uuids.
  -l, --locale <LOCALE>              Sets the locale for the extracted texts [default: en_GB]
      --ll <LOG_LEVEL>               Sets the level of logging; error, warn, info, debug, or trace [default: warn]
  -h, --help                         Print help (see more with '--help')
  -V, --version                      Print version
```
## Example

```text
lucris-rs -p cleaned/persons.clean.jsonl -r cleaned/research-outputs.clean.jsonl

NAMES:...
TITLE:...
ABSTRACT:...
```
## Installation

### cargo

If you have the rust toolchain, you can install from git.
```shell
cargo install --git https://github.com/HumlabLu/lucris-rs.git
```

## Workflow

Run the Go-code first to scrape the LUCRIS website.

Run the Rust extractor.

```shell
cargo run --release -- -p persons.clean.jsonl -r research-outputs.clean.jsonl > all_text_se.txt
```