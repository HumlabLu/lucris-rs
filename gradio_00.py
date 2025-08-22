# Uses VENVLLAMA
#  pip install gradio ollama chromadb llama_index                                              │
#  pip install llama-index-vector-stores-chroma                                                │
#  pip install llama-index-embeddings-ollama                                                   │
#  pip install llama-index-llms-ollama
import gradio as gr
import ollama
import subprocess
import chromadb
import shutil
import os
from llama_index.core import VectorStoreIndex, SimpleDirectoryReader
from llama_index.core import StorageContext
from llama_index.vector_stores.chroma import ChromaVectorStore
from llama_index.embeddings.ollama import OllamaEmbedding
from llama_index.llms.ollama import Ollama

# Model configurations
RAG_MODEL = "nomic-embed-text"  # "mxbai-embed-large"  # Model used for embedding in the RAG interface


# Fetch available models by parsing `ollama list` output
def get_models():
    result = subprocess.run(["ollama", "list"], capture_output=True, text=True)
    lines = result.stdout.strip().splitlines()[1:]  # Skip header line
    models = [
        line.split()[0] for line in lines
    ]  # Get model names from the first column
    return models


# Main conversational response function with model selection
def ollama_response(
    system_prompt, user_prompt, model_name, temperature, top_p, output_length
):
    options = {"temperature": temperature, "top_p": top_p, "max_tokens": output_length}
    combined_prompt = f"{system_prompt}\n\nUser: {user_prompt}\nAssistant:"
    response = ollama.generate(
        model=model_name, prompt=combined_prompt, options=options
    )
    return response["response"]


# RAG setup function returning a dictionary with necessary components
def setup_rag():
    # Load documents and initialize vector store and embedding
    documents = SimpleDirectoryReader("./pdfs").load_data()
    for doc in documents:
        if doc.metadata["page_label"] == "1":
            print(doc.metadata)
    db = chromadb.PersistentClient(path="./chroma_db")
    chroma_collection = db.get_or_create_collection("quickstart")
    vector_store = ChromaVectorStore(chroma_collection=chroma_collection)
    storage_context = StorageContext.from_defaults(vector_store=vector_store)

    # Use the RAG model for embedding
    embed_model = OllamaEmbedding(
        model_name=RAG_MODEL, base_url="http://localhost:11434"
    )
    index = VectorStoreIndex.from_vector_store(
        vector_store, storage_context=storage_context, embed_model=embed_model
    )

    return {
        "chroma_collection": chroma_collection,
        "storage_context": storage_context,
        "embed_model": embed_model,
        "index": index,
    }


# Function to handle RAG query with rag_components as a parameter
def rag_query(documents, user_prompt, rag_components):
    # Ensure the directory exists
    target_directory = "./pdfs"
    os.makedirs(target_directory, exist_ok=True)

    # Copy uploaded documents to the target directory
    if documents:
        for doc_path in documents:
            shutil.copy(doc_path, target_directory)

    # Load the documents from the directory
    documents = SimpleDirectoryReader(target_directory).load_data()

    # Recreate the index with the loaded documents
    vector_store = ChromaVectorStore(
        chroma_collection=rag_components["chroma_collection"]
    )
    storage_context = StorageContext.from_defaults(vector_store=vector_store)

    index = VectorStoreIndex.from_documents(
        documents,
        storage_context=storage_context,
        embed_model=rag_components["embed_model"],
    )

    # Create query engine and query
    query_engine = index.as_query_engine(
        llm=Ollama(model="llama3.2:latest", request_timeout=60.0, streaming=True)
    )
    response = query_engine.query(user_prompt)

    # Return the response content directly
    return response.response  # Use response.response or response['content'] depending on the API's response format


# Interface setup with primary and secondary sections
with gr.Blocks() as interface:
    # Main interface section, LLM question and answer
    gr.Markdown("# Local LLM Interface with System and User Prompts")
    with gr.Row():
        # Left column with prompts and settings
        with gr.Column(scale=1):
            system_prompt = gr.Textbox(
                lines=3,
                placeholder="Enter system prompt here...",
                label="System Prompt",
            )
            user_prompt = gr.Textbox(
                lines=5, placeholder="Enter user prompt here...", label="User Prompt"
            )
            model_dropdown = gr.Dropdown(
                choices=get_models(),
                label="Select Model",
                type="value",
                value="llama3.2:latest",
            )
            temperature_slider = gr.Slider(
                0.1, 1.0, step=0.1, value=0.7, label="Temperature"
            )
            top_p_slider = gr.Slider(0.1, 1.0, step=0.1, value=0.9, label="Top P")
            output_length_slider = gr.Slider(
                50, 500, step=10, value=150, label="Output Length"
            )
            main_button = gr.Button("Generate Response")

        # Right column for output
        with gr.Column(scale=1):
            main_output = gr.Textbox(
                lines=15, label="Model Response", interactive=False
            )

    # Button click action for main response
    main_button.click(
        fn=ollama_response,
        inputs=[
            system_prompt,
            user_prompt,
            model_dropdown,
            temperature_slider,
            top_p_slider,
            output_length_slider,
        ],
        outputs=main_output,
    )

    # Secondary RAG interface section
    gr.Markdown("## RAG Document Interface")
    with gr.Row():
        # Left column with document upload and user prompt for RAG
        with gr.Column(scale=1):
            doc_input = gr.File(
                type="filepath", label="Upload Document", file_count="multiple"
            )
            rag_user_prompt = gr.Textbox(
                lines=3, placeholder="Enter query here...", label="User Prompt for RAG"
            )
            rag_button = gr.Button("Query RAG")

        # Right column for RAG output
        with gr.Column(scale=1):
            rag_output = gr.Textbox(lines=15, label="RAG Response", interactive=False)

    # RAG query action with rag_components passed in
    rag_button.click(
        fn=lambda docs, query: rag_query(docs, query, rag_components),
        inputs=[doc_input, rag_user_prompt],
        outputs=rag_output,
    )

rag_components = setup_rag()

# Launch interface
if __name__ == "__main__":
    interface.launch(share=False, debug=True)
