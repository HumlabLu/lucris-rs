import os, sys
from haystack.document_stores.in_memory import InMemoryDocumentStore
from datasets import load_dataset, load_from_disk
from haystack import Document
from haystack.components.writers import DocumentWriter
from haystack.components.embedders import SentenceTransformersDocumentEmbedder
from haystack.components.preprocessors.document_splitter import DocumentSplitter
from haystack import Pipeline
from haystack.utils import ComponentDevice
from haystack.components.retrievers.in_memory import InMemoryBM25Retriever, InMemoryEmbeddingRetriever
from haystack.components.embedders import SentenceTransformersTextEmbedder
from haystack.components.joiners import DocumentJoiner
from haystack.components.rankers import TransformersSimilarityRanker
from haystack import Pipeline
from haystack.document_stores.types import DuplicatePolicy
from haystack import Pipeline
from haystack.document_stores.in_memory import InMemoryDocumentStore
from haystack.components.converters import PyPDFToDocument
from haystack.components.preprocessors import DocumentCleaner
from haystack.components.preprocessors import DocumentSplitter
from haystack.components.writers import DocumentWriter
from haystack.components.builders import PromptBuilder 
from haystack_integrations.components.generators.ollama import OllamaGenerator 
import re
import argparse


'''
This reads a HF dataset and creates a document store.
The other stuff is for reading and testing.
'''

# embedding_model = "sentence-transformers/all-MiniLM-L6-v2"
embedding_model = "sentence-transformers/all-MiniLM-L12-v2"

# see https://huggingface.co/BAAI/bge-m3
reranker_model = "BAAI/bge-reranker-base"

parser = argparse.ArgumentParser()
parser.add_argument("-c", "--create_store", help="Create a new data store.", default=None)
parser.add_argument("-r", "--read_store", help="Read a data store.", default=None)
parser.add_argument("--top_k", type=int, help="Retriever top_k.", default=8)
parser.add_argument("-q", "--query", help="Query DBs.", default=None)
args = parser.parse_args()


# Read the PDF(s) in file_names, turn them into Documents.
def read_pdf() -> [Document]:
    file_names = ["paper_I.pdf", "paper_II.pdf", "paper_III.pdf", "paper_IV.pdf"]
    pipeline = Pipeline()
    pipeline.add_component("converter", PyPDFToDocument())
    pipeline.add_component("cleaner", DocumentCleaner())
    # pipeline.add_component("splitter", DocumentSplitter(
    #                                                     split_by="sentence",
    #                                                     split_length=3,
    #                                                     split_overlap=1))
    pipeline.connect("converter", "cleaner")
    # pipeline.connect("cleaner", "splitter")
    result = pipeline.run({"converter": {"sources": file_names}})
    return result['cleaner']['documents']

# Take the Documents, saves them into a database, returns a hybrid retriever.
def prepare_retriever(docs, doc_store, split_length=5, split_overlap=1):
    document_splitter = DocumentSplitter(
                split_by="sentence",
                split_length=split_length,
                split_overlap=split_overlap
            )
    document_embedder = SentenceTransformersDocumentEmbedder(
        model=embedding_model, #"BAAI/bge-small-en-v1.5" #), device=ComponentDevice.from_str("cuda:0")
    )
    document_writer = DocumentWriter(doc_store,
                                     policy=DuplicatePolicy.SKIP)

    indexing_pipeline = Pipeline()
    indexing_pipeline.add_component("document_splitter", document_splitter)
    indexing_pipeline.add_component("document_embedder", document_embedder)
    indexing_pipeline.add_component("document_writer", document_writer)

    indexing_pipeline.connect("document_splitter", "document_embedder")
    indexing_pipeline.connect("document_embedder", "document_writer")

    indexing_pipeline.run({"document_splitter": {"documents": docs}})

    text_embedder = SentenceTransformersTextEmbedder(
        model=embedding_model, #"BAAI/bge-small-en-v1.5" #, device=ComponentDevice.from_str("cuda:0")
    )
    embedding_retriever = InMemoryEmbeddingRetriever(doc_store)
    bm25_retriever = InMemoryBM25Retriever(doc_store)

    document_joiner = DocumentJoiner()
    ranker = TransformersSimilarityRanker(model=reranker_model) #"BAAI/bge-reranker-base")

    hybrid_retrieval = Pipeline()
    hybrid_retrieval.add_component("text_embedder", text_embedder)
    hybrid_retrieval.add_component("embedding_retriever", embedding_retriever)
    hybrid_retrieval.add_component("bm25_retriever", bm25_retriever)
    hybrid_retrieval.add_component("document_joiner", document_joiner)
    hybrid_retrieval.add_component("ranker", ranker)

    hybrid_retrieval.connect("text_embedder", "embedding_retriever")
    hybrid_retrieval.connect("bm25_retriever", "document_joiner")
    hybrid_retrieval.connect("embedding_retriever", "document_joiner")
    hybrid_retrieval.connect("document_joiner", "ranker")

    return hybrid_retrieval

# Index the documents.
def create_index_nosplit(docs, doc_store):
    document_embedder = SentenceTransformersDocumentEmbedder(
        model=embedding_model, #"BAAI/bge-small-en-v1.5", #), device=ComponentDevice.from_str("cuda:0")
        meta_fields_to_embed=["title", "researcher_name"]
    )
    document_writer = DocumentWriter(doc_store,
                                     policy=DuplicatePolicy.SKIP)

    indexing_pipeline = Pipeline()
    indexing_pipeline.add_component("document_embedder", document_embedder)
    indexing_pipeline.add_component("document_writer", document_writer)
    indexing_pipeline.connect("document_embedder", "document_writer")
    print("Running indexing_pipeline.")
    indexing_pipeline.run({"document_embedder": {"documents": docs}})

    hybrid_retrieval = create_hybrid_retriever(doc_store)
    return hybrid_retrieval

# Just the retriever pipeline on a document store.
def create_hybrid_retriever(doc_store):
    text_embedder = SentenceTransformersTextEmbedder(
        model=embedding_model, #"BAAI/bge-small-en-v1.5" #, device=ComponentDevice.from_str("cuda:0")
    )
    embedding_retriever = InMemoryEmbeddingRetriever(doc_store)
    bm25_retriever = InMemoryBM25Retriever(doc_store)

    document_joiner = DocumentJoiner()
    ranker = TransformersSimilarityRanker(model=reranker_model) #"BAAI/bge-reranker-base")

    hybrid_retrieval = Pipeline()
    hybrid_retrieval.add_component("text_embedder", text_embedder)
    hybrid_retrieval.add_component("embedding_retriever", embedding_retriever)
    hybrid_retrieval.add_component("bm25_retriever", bm25_retriever)
    hybrid_retrieval.add_component("document_joiner", document_joiner)
    hybrid_retrieval.add_component("ranker", ranker)

    hybrid_retrieval.connect("text_embedder", "embedding_retriever")
    hybrid_retrieval.connect("bm25_retriever", "document_joiner")
    hybrid_retrieval.connect("embedding_retriever", "document_joiner")
    hybrid_retrieval.connect("document_joiner", "ranker")

    return hybrid_retrieval
    
def _read_dataset(filename) -> [Document]:
    dataset = load_from_disk(filename)
    print(dataset)

    docs = []
    for doc in dataset:
        docs.append(
            Document(
                    content=doc["contents"],
                    meta={"title": doc["title"], "abstract": doc["contents"]}
                )
        )
    return docs

# Reads the lucris-rs output and creates haystack documents.
def read_research_nta(a_file) -> [Document]:
    current_content = None
    current_meta = {}
    documents = []
    with open(a_file, "r") as f:
        for line in f:
            line = line.strip()
            if line.startswith("NAME"):  # Matches NAME: and NAMES:
                bits = line.split(":", 1)
                if len(bits) > 0:
                    name = bits[1]
                    #print("NAME", name)
                    # If we have current contents, we save it first.
                    if current_content and current_meta:
                        doc = Document(content=current_content, meta=current_meta)
                        documents.append(doc)
                        #print("ADDED", current_meta)
                    current_meta = {}
                    current_meta["researcher_name"] = name.strip()
                    current_content = None
            elif line.startswith("TITLE:"):
                bits = line.split(":", 1)
                if len(bits) > 0:
                    title = bits[1]
                    #print("TITLE", title)
                    current_meta["title"] = title.strip()
            elif line.startswith("ABSTRACT:"):
                bits = line.split(":", 1)
                if len(bits) > 0:
                    # abstract can be empty... mirror title?
                    abstract = bits[1].strip()
                    #print(current_meta)
                    if len(abstract) < 2: #some arbitrary small value
                        try:
                            abstract = current_meta["title"] # We assume we have read it... FIXME
                        except KeyError:
                            abstract = "no abstract"
                    current_content = abstract
                    current_meta["abstract"] = abstract
    # Left overs.
    if current_content and current_meta:
        doc = Document(content=current_content, meta=current_meta)
        documents.append(doc)
        #print("ADDED", current_meta)
    return documents

# Not used at the moment.
def _retrieve(query):
    hybrid_r = prepare_retriever()
    result = hybrid_r.run(
        {"text_embedder": {"text": query}, "bm25_retriever": {"query": query}, "ranker": {"query": query}}
    )
    return result

def pretty_print_results(prediction):
    for doc in prediction["documents"]:
        # print(doc.meta["title"][:60], "...\t", doc.score)
        print(doc.meta["content"], "\t", doc.score)
        #print(doc.meta["abstract"])
        print("\n")

def dump_docs(docs):
    for doc in docs:
        print(doc.id[0:8], doc.content[0:80], "...")
        
def print_res(doc, width=0):
    try:
        txt = doc.meta["researcher_name"]+":"+" ".join(doc.content.split())
    except KeyError:
        txt = " ".join(doc.content.split())
    if width > 0:
        txt_width = width - 8 - 3 - 1 # float and ... and LF
        txt = txt[0:txt_width]+"..."
    print("{:.5f}".format(doc.score), txt)
        
# Run the pre-defined retrievers, returns the top_k best documents.
def retrieve(retriever, query, top_k=8):
    result = retriever.run(
        {
            "text_embedder": {"text": query},
            "bm25_retriever": {"query": query, "top_k": top_k, "scale_score": True},
            "embedding_retriever": {"top_k": top_k, "scale_score": True},
            "ranker": {"query": query, "top_k": top_k, "scale_score": True}
        }
    )
    #print(result)
    #pretty_print_results(result["ranker"])
    return result['ranker']['documents']

def run_rag_pipeline(query, docs, model, temp):
    template = """
    Given the following context, answer the question at the end.
    Do not make up facts. Do not use lists. When referring to research
    mention the researchers names from the context. The name of the researcher will be given
    first, followed by an abstract of the relevant research. The question will follow the context.
    Reference the index numbers in the context when replying.

    Context:
    {% for document in documents %}
        Researcher: {{ document.meta.researcher_name }}. Research: {{ document.content }}
    {% endfor %}

    Question: {{question}}
    """

    prompt_builder = PromptBuilder(template=template,
                                   required_variables=["question"])
    generator = OllamaGenerator(
        model=model,
        url="http://localhost:11434",
        generation_kwargs={
            "num_predict": 8000,
            "temperature": temp,
            "num_ctx": 12028,
            "repeat_last_n": -1,
        },
        # streaming_callback=lambda chunk: print(chunk.content),
    )

    basic_rag_pipeline = Pipeline()
    basic_rag_pipeline.add_component("prompt_builder", prompt_builder)
    basic_rag_pipeline.add_component("llm", generator)
    basic_rag_pipeline.connect("prompt_builder", "llm")
    response = basic_rag_pipeline.run(
        {
            "prompt_builder": {
                "question": query,
                "documents": docs,
            },
        },
        include_outputs_from={"prompt_builder"},
    )
    #logger.debug(f"Context len: {len(response['llm']['meta'][0]['context'])}")
    #logger.info(f"Prompt length: {len(response['prompt_builder']['prompt'])}")
    #logger.info("-" * 78)
    # logger.info(response["llm"]["replies"][0])
    answer = response["llm"]["replies"][0]
    # Remove deepseek's tags.
    answer = re.sub(r"<think>.*?</think>", "", answer, flags=re.DOTALL)
    #logger.info(answer)
    #logger.info("-" * 78)

    if False: # or args.showprompt:
        print("Prompt builder prompt:")
        print(response["prompt_builder"]["prompt"])
        print("=" * 78)
    return answer
    
def run_rag_pipeline_stream(query, docs, model, temp):
    template = """
    Given the following context, answer the question at the end.
    Do not make up facts. Do not use lists. When referring to research
    mention the researchers names from the context. The name of the researcher will be given
    first, followed by an abstract of the relevant research. The question will follow the context.
    Reference the index numbers in the context when replying.

    Context:
    {% for document in documents %}
        Researcher: {{ document.meta.researcher_name }}. Research: {{ document.content }}
    {% endfor %}

    Question: {{question}}
    """
    prompt_builder = PromptBuilder(template=template, required_variables=["question"])
    prompt = prompt_builder.run(question=query, documents=docs)
    prompt = prompt["prompt"]

    # print(prompt)
    
    partial = ""
    def _cb(chunk):
        nonlocal partial
        partial += chunk.content
        #print(f"[{partial}]")
        return chunk.content #partial

    generator = OllamaGenerator(
        model=model,
        url="http://localhost:11434",
        generation_kwargs={
            "num_predict": 8000,
            "temperature": temp,
            "num_ctx": 12028,
            "repeat_last_n": -1,
        },
        streaming_callback=_cb
    )

    generator.run(prompt)
    # for _ in generator.run(prompt):  
    #     yield partial

if __name__ == '__main__':
    terminal_width = os.get_terminal_size().columns
    query = args.query
    
    if args.create_store:
        print("Loading research dataset")
        dataset = load_from_disk("research_docs.dataset")
        print(dataset)
        docs = []
        for doc in dataset:
            docs.append(
                Document(
                        content=doc["contents"],
                        meta={
                            "researcher_name": doc["researcher_name"],
                            "title": doc["title"],
                            "abstract": doc["contents"]
                        }
                    )
            )
        print(dataset[0])
        print(docs[0])
        rs_doc_store = InMemoryDocumentStore()
        #rs_hybrid_r = prepare_retriever(docs, rs_doc_store, split_length=5, split_overlap=0)
        #rs_hybrid_r = prepare_retriever_nosplit(docs, rs_doc_store)
        print("Starting create_index_nosplit()")
        create_index_nosplit(docs, rs_doc_store)
        print("Ready create_index_nosplit()")
        rs_hybrid_r = create_hybrid_retriever(rs_doc_store)

        documents = retrieve(rs_hybrid_r, query)
        for doc in documents:
            #print(doc.id, doc.meta["names"], ":", doc.meta["title"])
            print_res(doc, terminal_width)

        print("Saving document store")
        rs_doc_store.save_to_disk(args.create_store)

    if not args.query:
        sys.exit(0)
        
    print("Loading document store...")
    if not args.read_store and not args.create_store:
        args.read_store = "research_docs_ns.store"
    elif not args.read_store and args.create_store:
        args.read_store = args.create_store
    doc_store = InMemoryDocumentStore().load_from_disk(args.read_store)
    print(f"Number of documents: {doc_store.count_documents()}.")

    # Docs are already indexed/embedded in the sotre.
    hybrid_retrieval = create_hybrid_retriever(doc_store)
    
    documents = retrieve(hybrid_retrieval, query, top_k=args.top_k)
    for doc in documents:
        #print(doc.id, doc.meta["names"], ":", doc.meta["title"])
        print_res(doc, terminal_width)

    model = "llama3.1:latest"
    answer = run_rag_pipeline(query, documents, model, 0.1)
    print(answer)

    print("=" * 80)

    run_rag_pipeline_stream(query, documents, model, 0.1)
    # for answer in run_rag_pipeline_stream(query, documents, model, 0.1):
    #     print(answer)
    
