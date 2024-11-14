# PJB: Use the VENV in Development/HayStack
# PJB: Path to python can be set in .vscode/settings.json!
#
# -----------------------------------------------------------------------------
# We could extract names (and other meta data) from the query
# using a LLM and use it in the retrieval.
# -----------------------------------------------------------------------------
#
import sys
import ollama
from haystack import Pipeline
from haystack import Document
from haystack.document_stores.in_memory import InMemoryDocumentStore
from haystack.components.retrievers.in_memory import InMemoryBM25Retriever
from haystack.components.converters import TextFileToDocument
from haystack.components.preprocessors import DocumentCleaner
from haystack.components.preprocessors import DocumentSplitter
from haystack.document_stores.types import DuplicatePolicy
from haystack.components.writers import DocumentWriter
from haystack.components.rankers import LostInTheMiddleRanker
from haystack.components.rankers import TransformersSimilarityRanker
from haystack.components.rankers import SentenceTransformersDiversityRanker
from haystack_integrations.components.generators.ollama import OllamaGenerator
from haystack.components.retrievers.in_memory import InMemoryEmbeddingRetriever
from haystack.components.embedders import SentenceTransformersDocumentEmbedder
from haystack.components.embedders import SentenceTransformersTextEmbedder
from haystack.components.builders import PromptBuilder
import argparse

parser = argparse.ArgumentParser()
parser.add_argument("-m", "--model", help="Model for text generation.", default="llama3.1")
parser.add_argument("-e", "--extractionmodel", help="Model for text exxtraction.", default="mistral")
parser.add_argument("-q", "--query", help="query.", default=None)
parser.add_argument("-r", "--research", help="Research file.", default=None) #"research_docs.txt"
parser.add_argument("-R", "--reranker", action='store_true', help="Run re-ranker.", default=False)
parser.add_argument("-p", "--showprompt", action='store_true', help="Show LLM prompts.", default=False)
args = parser.parse_args()

store_filename = "docs_research.store"

# Create a on-disk database if True.
if False:
    document_store = InMemoryDocumentStore()

    pipeline = Pipeline()
    pipeline.add_component("converter", TextFileToDocument())
    pipeline.add_component("cleaner", DocumentCleaner(ascii_only=True,
                                                      remove_empty_lines=True,
                                                      remove_extra_whitespaces=True,
                                                      remove_repeated_substrings=True)
                           )
    pipeline.add_component("splitter", DocumentSplitter(split_by="sentence", split_length=3))
    pipeline.add_component("writer", DocumentWriter(document_store=document_store))
    pipeline.connect("converter", "cleaner")
    pipeline.connect("cleaner", "splitter")
    pipeline.connect("splitter", "writer")

    file_names = ["research_docs.txt"] # Generate using lucris-rs (out of date.)
    pipeline.run({"converter": {"sources": file_names}})

    document_store.save_to_disk(store_filename)
    
# -----------------------------------------------------------------------------

#RESEARCH: a06df509-b7e0-474a-b84a-3376a72f9e56
#PERSON0: Karin Johansson e1b388c9-685a-41d6-84cc-b217e14bbff3
#PERSON1: Nisse Johansson e1b388c9-685a-41d6-84cc-b217e14bbff4
#TITLE: A randomized study ...
#ABSTRACT: We compared manual ...
#RESEARCH: 740c676d-7ab4-4975-a1c6-d4d0d2976092

# We could have defaults for the other values as well.
def get_new_meta() -> dict:
    return {"persons":[]}

# Document(id=a364dddc7b9dfdefbbc7584920912d6c7dfe8c30c646cf7c586d4906525f2cf6, content: 'We ...', meta: {'uuid': 'f05eb79e-ac84-4d33-b6b1-55d650621132', 'title': 'A self-consistent ...'})
def read_research(a_file) -> [Document]:
    current_content = None
    current_meta = get_new_meta()
    documents = []
    with open(a_file, "r") as f:
        for line in f:
            line = line.strip()
            if line.startswith("RESEARCH:"):
                bits = line.split(":")
                if len(bits) == 2:
                    uuid = bits[1]
                    print("RESEARCH", uuid)
                    # If we have current contents, we save it first.
                    if current_content and current_meta:
                        doc = Document(content=current_content, meta=current_meta)
                        documents.append(doc)
                        print("ADDED", current_meta)
                    current_meta = get_new_meta()
                    current_meta["uuid"] = uuid.strip()
                    current_content = None
            if line.startswith("PERSON"):
                bits = line.split(":")
                if len(bits) == 2:
                    person = bits[1].strip()
                    # get the number from PERSONn
                    number = bits[0][6:] # bits[0] == "PERSON"
                    person = person.split() # You got to love untyped languages...
                    person_uuid = person[-1]
                    person_name = " ".join(person[:-1])
                    print("PERSON", number, person_name, person_uuid)
                    current_meta["persons"].append(person_name) # Note assumes "persons" is present.
            if line.startswith("TITLE:"):
                bits = line.split(":")
                if len(bits) == 2:
                    title = bits[1]
                    current_meta["title"] = title.strip()
            if line.startswith("ABSTRACT:"):
                bits = line.split(":")
                if len(bits) == 2:
                    current_content = bits[1].strip()
    # Left overs.
    if current_content and current_meta:
        doc = Document(content=current_content, meta=current_meta)
        documents.append(doc)
        print("ADDED", current_meta)
    return documents

# mistral seems to be better than llama, at least on the test cases.
def extract_persons(a_text) -> str:
    prompt = "Your task is to extract the names of the people mentioned in the users input after TEXT:\n"\
        "Only reply with the json structure.\n"\
        "Do not repeat the input text.\n"\
        "Remove titles like Mr. or Mrs.\n"\
        "If you cannot find any persons, reply with an empty structure like this: [{}].\n"\
        "If the text is empty, reply with an empty structure like this: [{}].\n"\
        "Format your output as a list of json with the following structure.\n"\
        "[{\n"\
        "   \"person\": The name of the person\n"\
        "}]\n"\
        "Example user input: \"TEXT: What is Mr. John Doe working on?\n"\
        "Example output: [{\"person\": \"John Doe\"}]\n"
    prompt = prompt + "TEXT:" + a_text + ".\n"
    if args.showprompt:
        print(prompt)
    output = ollama.generate(
        model=args.extractionmodel,
        options={
            'temperature': 0.0,
            'top_k': 10, # ?
            'num_ctx': 8096,
            'repeat_last_n': -1,
        },
        prompt=prompt
    )
    return output['response']

# -----------------------------------------------------------------------------

# Specifying a research file reads it and save the resulting
# documents to disk.
# Pipeline example: https://docs.haystack.deepset.ai/docs/documentwriter
if args.research:
    docs = read_research("research_docs.txt")
    print("Doc count:", len(docs))
    print(docs[0])

    doc_embedder = SentenceTransformersDocumentEmbedder(
        model="sentence-transformers/all-MiniLM-L6-v2", # Dim depends on model.
        meta_fields_to_embed=["title", "persons"]
    )
    doc_embedder.warm_up()
    docs_with_embeddings = doc_embedder.run(docs)
    document_store = InMemoryDocumentStore()
    document_writer = DocumentWriter(
        document_store=document_store,
        policy=DuplicatePolicy.SKIP
    )
    document_writer.run(documents=docs_with_embeddings["documents"])
    document_store.save_to_disk(store_filename)

# -----------------------------------------------------------------------------

# Test name extraction.
if False:
    print(extract_persons("What is Quinten Berck working on?"))
    print(extract_persons("Tell me what John and Nisse Nissesson are researching?"))
    print(extract_persons("I did my shopping at ICAs"))
    print(extract_persons(""))
    print(extract_persons("We used site-directed mutagenesis by Van den Bosch and Mr. Smith to do this."))
    sys.exit(0)
# -----------------------------------------------------------------------------

if not args.query:
    sys.exit(0)
    
# -----------------------------------------------------------------------------

print("Loading...")
document_store_new = InMemoryDocumentStore().load_from_disk(store_filename)
print(f"Number of documents: {document_store_new.count_documents()}.")
retriever = InMemoryBM25Retriever(document_store=document_store_new)
#retriever = InMemoryEmbeddingRetriever(document_store_new)
query = args.query
print(f"Query: {query}")

retrieve_top_k = 9
rank_top_k = 3

# Filter of meta-data?
res = retriever.run(
    query=query,
    top_k=retrieve_top_k,
    #scale_score=True
)
print("Retriever")
for i, r in enumerate(res["documents"]):
    print(f"{i:02n}", f"{r.score:.4f}", r.content[0:78])
    #print(r)
print()
print("=" * 78)

if False:
    print("Running LostInTheMiddleRanker()")
    ranker = LostInTheMiddleRanker()
    res = ranker.run(
        documents=res["documents"],
        top_k=rank_top_k
    )
    for i, r in enumerate(res["documents"]):
        print()
        print(f"{i:02n}", f"{r.score:.4f}", r.content[0:78])
    print()
    print("=" * 78)

if False:
    print("Running TransformersSimilarityRanker()")
    ranker = TransformersSimilarityRanker(model="BAAI/bge-reranker-base")
    ranker.warm_up()
    res = ranker.run(
        query=query,
        documents=res["documents"],
        top_k=rank_top_k
    ) 
    for i, r in enumerate(res["documents"]):
        print()
        print(f"{i:02n}", f"{r.score:.4f}", r.content[0:78])
    print()
    print("=" * 78)

if args.reranker:
    ranker = SentenceTransformersDiversityRanker(
        model="sentence-transformers/all-MiniLM-L6-v2",
        #model="cross-encoder/ms-marco-MiniLM-L-6-v2",
        similarity="cosine",
    )
    ranker.warm_up()
    res = ranker.run(
        query=query,
        documents=res["documents"],
        top_k=rank_top_k
    ) 
    for i, r in enumerate(res["documents"]):
        print()
        print(f"{i:02n}", f"{r.score:.4f}", r.content[0:78])
    print()
    print("=" * 78)

template = """
Given the following context, answer the question.
Do not make up facts. Do not use lists. When referring to research
mention the researchers names from the meta-data.

Context:
{% for document in documents %}
    {{ document.content }}
{% endfor %}

Question: {{question}}
"""

prompt_builder = PromptBuilder(template=template)
generator = OllamaGenerator(
    model="llama3.1",
    #model="gemma2",
    url = "http://localhost:11434",
    generation_kwargs={
        "num_predict": 2000,
        "temperature": 1.5, # Higher is more "creative".
        'num_ctx': 8096,
        'repeat_last_n': -1,
    }
)

basic_rag_pipeline = Pipeline()
basic_rag_pipeline.add_component("prompt_builder", prompt_builder)
basic_rag_pipeline.add_component("llm", generator)
basic_rag_pipeline.connect("prompt_builder", "llm")

print() 
print(query)
print()
response = basic_rag_pipeline.run(
    {
        "prompt_builder": {"question": query,
                           "documents": res["documents"]
                           },
    },
    include_outputs_from={"prompt_builder"},
)
print("-" * 78)
print(response["llm"]["replies"][0])
print("-" * 78)
print()

if args.showprompt:
    print()
    print("Prompt builder:")
    print(response["prompt_builder"]["prompt"])
    print("=" * 78)

