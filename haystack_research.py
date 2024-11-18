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

# -----------------------------------------------------------------------------

#RESEARCH: a06df509-b7e0-474a-b84a-3376a72f9e56
#PERSON0: Karin Johansson e1b388c9-685a-41d6-84cc-b217e14bbff3
#PERSON1: Nisse Johansson e1b388c9-685a-41d6-84cc-b217e14bbff4
#TITLE: A randomized study ...
#ABSTRACT: We compared manual ...
#RESEARCH: 740c676d-7ab4-4975-a1c6-d4d0d2976092

# We could have defaults for the other values as well.
def get_new_meta() -> dict:
    # return {"persons":[]} # initialise empty list
    return {}

# name, title abstract format.
def read_research_nta(a_file) -> [Document]:
    current_content = None
    current_meta = get_new_meta()
    documents = []
    with open(a_file, "r") as f:
        for line in f:
            line = line.strip()
            if line.startswith("NAME:"):
                bits = line.split(":")
                if len(bits) == 2:
                    name = bits[1]
                    #print("NAME", name)
                    # If we have current contents, we save it first.
                    if current_content and current_meta:
                        doc = Document(content=current_content, meta=current_meta)
                        documents.append(doc)
                        print("ADDED", current_meta)
                    current_meta = get_new_meta()
                    current_meta["researcher_name"] = name.strip()
                    current_content = None
            if line.startswith("TITLE:"):
                bits = line.split(":")
                if len(bits) == 2:
                    title = bits[1]
                    #print("TITLE", title)
                    current_meta["title"] = title.strip()
            if line.startswith("ABSTRACT:"):
                bits = line.split(":")
                if len(bits) == 2:
                    # abstract can be empty... mirror title?
                    abstract = bits[1].strip()
                    #print(current_meta)
                    if len(abstract) < 2: #some arbitrary small value
                        try:
                            abstract = current_meta["title"] # We assume we have read it... FIXME
                        except KeyError:
                            abstract = "no abstract"
                    current_content = abstract
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
    docs = read_research_nta("research_docs_nta.txt")
    print("Doc count:", len(docs))
    print(docs[0])

    doc_embedder = SentenceTransformersDocumentEmbedder(
        model="sentence-transformers/all-MiniLM-L6-v2", # Dim depends on model.
        meta_fields_to_embed=["title", "researcher_name"]
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
#print(retriever)
query = args.query
print(f"Query: {query}")

retrieve_top_k = 19
rank_top_k = 8
retriever_type = "embeddings"

# Filter of meta-data?
if retriever_type == "embeddings":
    retriever = InMemoryEmbeddingRetriever(document_store_new)
    doc_embedder = SentenceTransformersDocumentEmbedder(
        model="sentence-transformers/all-MiniLM-L6-v2", # Dim depends on model.
        meta_fields_to_embed=["title", "researcher_name"]
    )
    doc_embedder.warm_up()
    text_embedder = SentenceTransformersTextEmbedder(model="sentence-transformers/all-MiniLM-L6-v2")
    #text_embedder = SentenceTransformersTextEmbedder()
    #docs_with_embeddings = doc_embedder.run(docs)
    query_pipeline = Pipeline() 
    query_pipeline.add_component("text_embedder", text_embedder)
    result = query_pipeline.run({"text_embedder": {"text": query}})
    #print(result)
    q_embedding = result['text_embedder']['embedding']
    print(len(q_embedding))
    #print(q_embedding)
    res = retriever.run(
        query_embedding=q_embedding,
        top_k=retrieve_top_k,
        #scale_score=True
    )
else:
    retriever = InMemoryBM25Retriever(document_store=document_store_new)
    res = retriever.run(
        query=query,
        top_k=retrieve_top_k,
        #scale_score=True
    )
print("Retrieved")
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
mention the researchers names from the context. The name of the researcher will be given
first, followed by an abstract of the relevant research.

Context:
{% for document in documents %}
    Researcher: {{ document.meta.researcher_name }}. Research: {{ document.content }}
{% endfor %}

Question: {{question}}
"""

#   and: "{{ document.content if document.content is not none else 'NONE' }}"
#  {{ document.content if document.content.length() > 10 else 'NONE' }}

prompt_builder = PromptBuilder(template=template)
generator = OllamaGenerator(
    model="llama3.1",
    #model="gemma2",
    url = "http://localhost:11434",
    generation_kwargs={
        "num_predict": 2000,
        "temperature": 0.5, # Higher is more "creative".
        'num_ctx': 12028,
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


'''
(VENV) pberck@Peters-MacBook-Pro-2 lucris-rs % python haystack_research.py  -q "Anyone do research on cats?"
/Users/pberck/Development/HayStack/VENV/lib/python3.12/site-packages/haystack/core/errors.py:34: DeprecationWarning: PipelineMaxLoops is deprecated and will be remove in version '2.7.0'; use PipelineMaxComponentRuns instead.
  warnings.warn(
Loading...
Number of documents: 112497.
Query: Anyone do research on cats?
Retriever
00 24.7520 Reality TV is popular entertainment. And yet a common way to start a conversat
01 24.6157 We present the first evidence that cats experience visual illusions and that a
02 24.5765 Bilateral removal of the cerebral cortex was made in cats neonatally. Spontane
03 23.7246 Rather than using different therapies in isolation, many cancer patients use d
04 23.5214 The cat (Felis catus, Linneaus 1758) has lived around or with humans for at le
05 23.4702 Aim Vegetation dynamics and the competitive interactions involved are assumed
06 23.4620 This study compared acid-base and biochemical changes and quality of recovery
07 23.3391 We aim to introduce some new solutions in the studies involving the subjects w
08 22.8880 This study collected 257 vocalisations from three domestic cats when they were

==============================================================================

Anyone do research on cats?

------------------------------------------------------------------------------
Yes, numerous researchers have conducted studies on cats. For example, researchers such as E.C. Su and his colleagues have investigated the ability of cats to see illusory motion in a video, specifically the Rotating Snakes illusion (Su et al.). Researchers from the Meowsic project (which stands for Melody in human–cat communication) are studying the prosodic characteristics of cat vocalisations as well as the communication between humans and cats (Fleming et al.). Researchers such as T.E. Hermanson and his colleagues have studied the acid-base and biochemical changes in male cats with experimentally induced urethral obstruction (Hermanson et al.).
------------------------------------------------------------------------------
'''

'''
# With new NTA format on all research.
Loading...
Number of documents: 229679.
Query: Anyone do research on cats?
Retriever
00 26.1221 Bilateral removal of the cerebral cortex was made in cats neonatally. Spontane
01 24.6147 Aim Vegetation dynamics and the competitive interactions involved are assumed
02 24.6147 Aim Vegetation dynamics and the competitive interactions involved are assumed
03 24.4449 The Secret Language of Cats
04 24.3882 In this study, we investigated the prosody of domestic cat meows produced in d
05 24.3882 In this study, we investigated the prosody of domestic cat meows produced in d
06 24.3049 "Did anyone here recognize that?"
07 24.2851 <br/>The cat (Felis catus, Linneaus 1758) has lived around or with humans for
08 24.2851 <br/>The cat (Felis catus, Linneaus 1758) has lived around or with humans for

==============================================================================

Anyone do research on cats?

------------------------------------------------------------------------------
Yes, several researchers and research projects have studied cats. In the provided context, there are studies on cats that include:

* A study on cats with neonatal bilateral cerebral cortex removal, which found that despite their brain lesion, the cats were able to adapt and function normally in terms of spontaneous and imposed behavior, visual cue utilization, and learning (no researchers mentioned in this section).
* A study on tree migration in Europe, which used a post-process migration tool called LPJ-CATS that took into account vegetation dynamics, climate, environment, and local species dynamics such as succession and competition ( researchers not mentioned).
* A study by the research project Melody in human–cat communication (Meowsic) which aimed to investigate the prosodic characteristics of cat vocalisations as well as the communication between human and cat (no specific researchers mentioned, just the project's name).
* Another study on cat vocalisation by which the authors found significant effects of context on duration and mean fundamental frequency, but not on fundamental frequency range, and proposed that this prosodic variation reflects the cat's mental or emotional state (researchers not mentioned, likely members of the Meowsic project).

It is worth noting that there are other research projects and studies on cats that are not mentioned in the provided context, such as studies on cat behavior, cat cognition, and cat welfare, among others.
------------------------------------------------------------------------------
'''