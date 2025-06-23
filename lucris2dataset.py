import os
import json
import datasets
import sys
import argparse


parser = argparse.ArgumentParser()
parser.add_argument("-f", "--filename", help="Filename.", default="research_docs_nta.txt")
parser.add_argument("-o", "--outputfilename", help="Output filename.", default="research_docs.dataset")
parser.add_argument("-q", "--query", help="Query DBs.", default="What is lichen?")
args = parser.parse_args()



# Reads lucris-rs output, create a HF dataset.

current_content = None
current_name = None
current_title = None
current_abstract = None
current_meta = {}
documents = []
ids = []
names= []
titles = []
contents = []
counter = 0
with open(args.filename, "r") as f:
    linecounter = 0
    for line in f:
        line = line.strip()
        if line.startswith("NAME:") or line.startswith("NAMES:"):
            # If we get a new name, the abstract is finished (could be
            # multi line).
            if current_name and current_title and current_abstract:
                ids.append("ID{:06n}".format(counter))
                counter += 1
                names.append(current_name)
                titles.append(current_title)
                contents.append(current_abstract)
                current_name = None
                current_title = None
                current_abstract = None
            bits = line.split(":", 1)
            if len(bits) > 0:
                name = bits[1]
                if not current_name:
                    current_name = name
                else:
                    print("Data seems wonky. Exit.", linecounter)
                    sys.exit(1)
        elif line.startswith("TITLE:"):
            if not current_name:
                print("Data out of order?", linecounter)
                sys.exit(2)
            bits = line.split(":", 1)
            if len(bits) > 0:
                title = bits[1]
                if not current_title:
                    current_title = title
                else:
                    print("Data seems wonky. Exit.", linecounter)
                    sys.exit(1)
        elif line.startswith("ABSTRACT:"):
            if current_abstract: # this happens, abstract containing "abstract"
                current_abstract += line
                continue
            if not current_name and not current_title:
                print("Data out of order?", linecounter)
                sys.exit(2)
            bits = line.split(":", 1)
            if len(bits) > 0:
                # abstract can be empty... mirror title?
                abstract = bits[1].strip()
                if len(abstract) < 2 or abstract == "no abstract" or abstract == "[abstract missing]": #some arbitrary small value
                    abstract = current_title
                if not current_abstract:
                    current_abstract = abstract
                else:
                    print("Data seems wonky. Exit.", linecounter)
                    sys.exit(1)
        linecounter += 1
    # Left over data
    if current_name and current_title and current_abstract:
        ids.append("ID{:06n}".format(counter))
        counter += 1
        names.append(current_name)
        titles.append(current_title)
        contents.append(current_abstract)
        current_name = None
        current_title = None
        current_abstract = None

data = {}
print(len(ids), len(names), len(titles), len(contents))
data['id'] = ids
data['researcher_name'] = names
data['title'] = titles
data['contents'] = contents
dataset = datasets.Dataset.from_dict(data)
print(dataset)
print(json.dumps(dataset[0], indent=4))
print(json.dumps(dataset[len(ids)-1], indent=4))
# for x in dataset:
#     print(json.dumps(x, indent=4))
    

dataset.save_to_disk(args.outputfilename)

# Reload to test.
test_dataset = datasets.load_from_disk(args.outputfilename)
print("\nReloaded dataset")
print(test_dataset)
print(json.dumps(test_dataset[0], indent=4))

