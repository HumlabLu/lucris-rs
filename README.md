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
cargo run --release -- -p persons.clean.jsonl -r research-outputs.clean.jsonl > research_docs.txt
```

Create a virtual environment using the `requirements.txt` (which probably contains more than necessary).

Create the HayStack document store.
```shell
python haystack_store.py -r research_docs.txt -s docs_research.store
```

Run queries like this.
```shell
python haystack_research.py -s docs_research.store
```

Enter 'bye' to quit.

## Example

Running : `python haystack_research.py -s docs_research.store`

Eample output.
```shell
Enter Query:
explain eye-tracking research
2025-03-05 14:20:40 - explain eye-tracking research
2025-03-05 14:20:45 -  [{}]
2025-03-05 14:20:45 -  ResearchQuestion
Batches: 100%|███████████████████████████████████████████████████████████████████████████████████████████████████████████████| 1/1 [00:00<00:00,  1.94it/s]
2025-03-05 14:20:59 - 00 0.9835 Marcus Nyström,Diederick C Niehorster,Roy S Hessels,Antje Nuthmann <p>Eye tracking technology has become increasingly prevalent in scientific res
2025-03-05 14:20:59 - 01 0.9533 Halszka Jarodzka,S. Brand-Gruwel <p>Eye tracking has helped to understand the process of reading a word or a se
2025-03-05 14:20:59 - 02 0.9223 Jana Holsanova,Roger Johansson,Sven Strömqvist The research group from Humanities laboratory at Lund University, Sweden, pres
2025-03-05 14:20:59 - 03 0.8898 Diederick C Niehorster,Raimondas Zemblys <p>Eye trackers are sometimes used to study the miniature eye movements such a
2025-03-05 14:20:59 - 04 0.8711 Philipp Stark,Efe Bozkir,Patricia Goldberg,Gerrit Meixner,Enkelejda Kasneci,Richard Gollner <p>Currently, VR technology is increasingly being used in applications to enab
2025-03-05 14:20:59 - 05 0.8703 Linnéa Larsson This doctoral thesis has signal processing of eye-tracking data as its main th
2025-03-05 14:20:59 - 06 0.8337 Diederick C Niehorster,Roy S Hessels,Chantal Kemner,Ignace T C Hooge <p>Eye-tracking research in infants and older children has gained a lot of mom
2025-03-05 14:20:59 - 07 0.8263 Jana Holsanova The chapter presents a new perspective that concerns reception of multimodalit
2025-03-05 14:20:59 - 08 0.7963 Arantxa Villanueva,R Cabeza,S Porta,Martin Böhme,Detlev Droege Report on New Approaches to Eye Tracking
2025-03-05 14:20:59 - 09 0.7864 Peng Kuang,Emma Söderberg,Diederick C Niehorster Eye tracking has been used as part of software engineering and computer scienc
2025-03-05 14:20:59 -
2025-03-05 14:20:59 - ==============================================================================
2025-03-05 14:20:59 - Answering: explain eye-tracking research
2025-03-05 14:21:19 - Prompt length: 11558
2025-03-05 14:21:19 - ------------------------------------------------------------------------------
2025-03-05 14:21:19 - Based on the provided context, eye-tracking
research is a scientific method that uses technology to track and
record eye movements, providing insights into oculomotor and cognitive
processes. This research aims to understand how people perceive,
process, and respond to visual information, such as reading, scene
perception, and task execution.

According to Marcus Nyström, Diederick C Niehorster, Roy S Hessels,
and Antje Nuthmann (Researcher: Marcus Nyström, Diederick C
Niehorster, Roy S Hessels, Antje Nuthmann), eye-tracking technology
has become increasingly prevalent in scientific research, offering
unique insights into oculomotor and cognitive processes. The
researchers provide examples from various studies, including
oculomotor control, reading, scene perception, task execution, visual
expertise, and instructional design, to illustrate the connection
between theory and eye-tracking data.

Furthermore, Halszka Jarodzka and S. Brand-Gruwel (Researcher: Halszka
Jarodzka, S. Brand-Gruwel) propose structuring eye-tracking research
in reading into three levels: level 1 research on reading single words
or sentences, level 2 research on reading and comprehending a whole
text, and level 3 research on reading and processing involving several
text documents.

Eye-tracking research has also been applied in various fields, such as
language and cognition (Researcher: Jana Holsanova, Roger Johansson,
Sven Strömqvist), expertise assessment (Researcher: Philipp Stark, Efe
Bozkir, Patricia Goldberg, Gerrit Meixner, Enkelejda Kasneci, Richard
Gollner), and multimodality (Researcher: Jana Holsanova).

In summary, eye-tracking research is a scientific method that uses
technology to track and record eye movements, providing insights into
oculomotor and cognitive processes. This research has been applied in
various fields, including reading, language and cognition, expertise
assessment, and multimodality, and has led to a better understanding
of how people perceive, process, and respond to visual information.
2025-03-05 14:21:19 - ------------------------------------------------------------------------------

Enter Query:
```

## Web app

The `app_lucris.py` script provides a web interface to a 'chatbot' answering questions about
the research-data. 'Chatbot' between quotation marks because it only answers single questions
without looking at the previous questions and answers. 

It is built using the `Gradio` chat bot framework (making it relatively easy to host it on
HuggingFace).

It uses the Ollama framework to run an LLM locally.

Screenshot of the web interface.
![Schat bot screenshot](chatbot00.png?raw=true "Chat bot example")

### Preparing the web app

The web app reads the same lucris data produced by the `lucris-rs` scripts. The
`lucris2dataset.py` and `hybrid.py` scripts read and prepare the data for the web app.
They prepare a HayStack document store for hybrid (embeddings and BM25) retrieval.

So the worktflow is as follows:
 - run the scraper
 - run `lucris-rs`´on its output
 - run `lucris2dataset.py` to create a data set
 - run `hybrid.py` to convert the data set to a data store
 - run `app_lucris.py`



