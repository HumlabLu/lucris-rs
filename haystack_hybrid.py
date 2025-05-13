from haystack import Pipeline
from haystack.components.retrievers.in_memory import InMemoryBM25Retriever, InMemoryEmbeddingRetriever
from haystack.components.embedders import SentenceTransformersTextEmbedder
from haystack.components.joiners import DocumentJoiner
from haystack.components.rankers import TransformersSimilarityRanker
from haystack.components.writers import DocumentWriter
from haystack.components.embedders import SentenceTransformersDocumentEmbedder
from haystack.components.preprocessors.document_splitter import DocumentSplitter
from haystack import Pipeline
from haystack.utils import ComponentDevice
from datasets import load_dataset
from haystack import Document
from haystack.document_stores.in_memory import InMemoryDocumentStore

document_store = InMemoryDocumentStore()
store_filename = "TEMPdocs_research.store" #"hybrid_documents.store"

if False:
    dataset = load_dataset("anakin87/medrag-pubmed-chunk", split="train")
    docs = []
    for doc in dataset:
        docs.append(
            Document(
                content=doc["contents"],
                meta={"title": doc["title"], "abstract": doc["content"], "pmid": doc["id"]}
            )
        )
    document_splitter = DocumentSplitter(split_by="word", split_length=512, split_overlap=32)
    document_embedder = SentenceTransformersDocumentEmbedder(
        model="BAAI/bge-small-en-v1.5" #, device=ComponentDevice.from_str("cuda:0")
    )
    document_writer = DocumentWriter(document_store)
    indexing_pipeline = Pipeline()
    indexing_pipeline.add_component("document_splitter", document_splitter)
    indexing_pipeline.add_component("document_embedder", document_embedder)
    indexing_pipeline.add_component("document_writer", document_writer)
    indexing_pipeline.connect("document_splitter", "document_embedder")
    indexing_pipeline.connect("document_embedder", "document_writer")
    indexing_pipeline.run({"document_splitter": {"documents": docs}})
    document_store.save_to_disk(store_filename)
else:
    print("Loading...")
    document_store = InMemoryDocumentStore().load_from_disk(store_filename)
    print(f"Number of documents: {document_store.count_documents()}.")

text_embedder = SentenceTransformersTextEmbedder(
    model="BAAI/bge-small-en-v1.5" #, device=ComponentDevice.from_str("cuda:0")
)
embedding_retriever = InMemoryEmbeddingRetriever(document_store)
bm25_retriever = InMemoryBM25Retriever(document_store)

document_joiner = DocumentJoiner()

ranker = TransformersSimilarityRanker(model="BAAI/bge-reranker-base")

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

#hybrid_retrieval.draw("hybrid-retrieval.png")

query = "apnea in infants"
result = hybrid_retrieval.run(
    {"text_embedder": {"text": query}, "bm25_retriever": {"query": query}, "ranker": {"query": query}}
)
def pretty_print_results(prediction):
    for doc in prediction["documents"]:
        print(doc.meta["title"], "\t", doc.score)
        print(doc.meta["abstract"])
        print("\n", "\n")

pretty_print_results(result["ranker"])
