import sys
import os
import gradio as gr
import logging
from openai import OpenAI, OpenAIError
from datetime import datetime
from hybrid import embedding_model, reranker_model, create_hybrid_retriever, retrieve, InMemoryDocumentStore, PromptBuilder, OllamaGenerator
import argparse
import ollama

'''
If openai key is set, a OAIMODEL also has to be set, or unset
for default mini-4o.

export OPENAI_API_KEY='...'
unset OAIMODEL

For ollama:
unset OPENAI_API_KEY
export OAIMODEL=llama3.2:latest

Debug output goes to lucrisbot.log. For printed output:
export DEBUG=1
'''

# openAI API credits:
# https://platform.openai.com/settings/organization/billing/overview

logger = logging.getLogger("LUCRIS")
logger.setLevel(logging.DEBUG)

formatter = logging.Formatter("%(asctime)s - %(levelname)s - %(message)s")
file_handler = logging.FileHandler("lucrisbot.log")
file_handler.setLevel(logging.DEBUG)
file_handler.setFormatter(formatter)

console_handler = logging.StreamHandler()
console_handler.setLevel(logging.INFO)
formatter = logging.Formatter("%(asctime)s - %(message)s", "%Y-%m-%d %H:%M:%S")
console_handler.setFormatter(formatter)

logger.addHandler(file_handler)
logger.addHandler(console_handler)

# --------

parser = argparse.ArgumentParser()
parser.add_argument("-d", "--datastore", help="Datastore filename.", default="research_docs_ns.store")
parser.add_argument("-s", "--share", action='store_true', help="Creates a public link.", default=False)
args = parser.parse_args()

# We can't save a logfile in a HF space, printing it allows us to
# save the output (copy/paste) later from the web console output.
# The trick is to define the DEBUG variable in the HF settings.
def DBG(a_str):
    if os.getenv('DEBUG'):
        print(a_str) # kraai
    else:
        logger.debug(a_str) # local
    
DBG("Starting the chatbot")

'''
model='llama3.1:latest'
 modified_at=datetime.datetime(2024, 8, 13, 16, 10, 49, 883243, tzinfo=TzInfo(+02:00))
 digest='91ab477bec9d27086a119e33c471ae7afbd786cc4fbd8f38d8af0a0b949d53aa'
 size=4661230977
 details=ModelDetails(parent_model='', format='gguf', family='llama', families=['llama'], parameter_size='8.0B', quantization_level='Q4_0')
'''

def get_ollama_models():
    try:
        result = ollama.list()
    except:
        DBG("Error, cannot load ollama models.")
        sys.exit(2)
    model_names = []
    for model in result["models"]:
        DBG(f"Ollama: {model.model} / {model.details.parameter_size} / {model.details.quantization_level}")
        model_names.append(model.model)
    return model_names

# ----

# These are defined in hybrid.py
DBG(f"Embedding model: {embedding_model}")
DBG(f"Reranker model: {reranker_model}")

gen_model = os.getenv('OAIMODEL')
model_provider = "ollama"

# OpenAI
try:
    openai_client = OpenAI(
        api_key=os.environ.get("OAIKEY"),
    )
    DBG("Using OpenAI")
    model_provider = "openai"
    if not gen_model:
        gen_model = "gpt-4.1-mini"
except OpenAIError:
    try:
        openai_client = OpenAI(
            base_url="http://localhost:11434/v1",
            api_key="ollama",  # required, but unused
        )
        DBG("Using local Ollama.")
        if not gen_model:
            model_names = get_ollama_models()
            if len(model_names) > 0:
                gen_model = model_names[0]
            else:
                DBG("No ollama models found?")
                sys.exit(4)
    except Exception as e:
        print(e)
        DBG("No AI provider available. Exit.")
        sys.exit(1)
DBG(f"Model provider: {model_provider}")
DBG(f"Generation model: {gen_model}")

# Contact OpenAI "moderator".
def moderator(message):
    return False
    DBG("CALLING MODERATOR")
    response = openai_client.moderations.create(
        model="omni-moderation-latest",
        input=message,
    )
    response_dict = response.model_dump()
    is_flagged = response_dict["results"][0]["flagged"]
    DBG("MODERATOR")
    DBG(response_dict)
    return is_flagged


# Retrieve context from the doc store.
def get_context(message, retriever, cutoff, top_k=8):
    docs = retrieve(retriever, message, top_k=top_k)
    result = []
    width = os.get_terminal_size().columns
    for doc in docs:
        txt = " ".join(doc.content.split())
        txt_width = width - 34 - 8 - 3 - 1 # float and ... and LF
        txt = txt[0:txt_width]+"..."
        DBG("{:.5f} {}".format(doc.score, txt))
        if doc.score >= cutoff:
            result.append(doc) # was doc.content
    return result

# ----

def format_history(history):
    for h in history:
        try:
            role = h['role']
            cont = h['content']
        except TypeError:
            role = h.role
            cont = h.content
        print(role)
        print("        ", cont)

# https://www.gradio.app/guides/theming-guide
theme = gr.themes.Monochrome(
    font=[
        #gr.themes.GoogleFont('TagesSchrift'),
        #gr.themes.GoogleFont('ui-sans-serif'),
        #'system-ui',
        #'sans-serif'
    ],
).set(
    #background_fill_secondary_dark='*neutral_400',
    #background_fill_primary_dark='*neutral_800'
)

with gr.Blocks(theme=theme) as demo_blocks:
    if model_provider == "ollama":
        model_names = get_ollama_models()
    else:
        model_names = [ gen_model ]

    # gr.Markdown("# Chat with Lucris data")
    chatbot = gr.Chatbot(
        type="messages",
        label="",
        resizable=True,
        #avatar_images=(None, "./pufendorf1.jpg"),
        placeholder="<strong>Lucris</strong>",
    )
    with gr.Row():
        with gr.Column(scale=9):
            msg = gr.Textbox(
                placeholder="Your question",
                submit_btn="Ask",
                label="",
                lines=1,
                container=False,
            )
        with gr.Column(scale=1):
            clear = gr.Button(
                "Clear",
                elem_classes="self-center",
            )
    with gr.Row():
        other = gr.Textbox(
            "Answer in Swedish",
            #info="Question to Samuel",
            label="Extra",
            container=True,
            visible=False,
        )
        lang = gr.Radio(
            ["English", "Swedish"],
            label="Language",
            info="The bot speaks ...",
            value="English",
            interactive=True,
            visible=False,
        )
        val = gr.Slider(0, 28,
            value = 8,
            label="Context size (number of retrieved docs)",
            step=1.0
        )
        cutoff = gr.Slider(0, 1,
            value = 0.0,
            label="Context match cut-off (higher is better match)",
            step=0.01
        )
    ignore_extras = gr.Checkbox(label="Ignore extras", value=False, visible=False)

    with gr.Row():
        model_selector = gr.Dropdown(
            choices=model_names,
            value=gen_model,
            label="Choose model"
        )
        tmp = gr.Slider(0.05, 2,
            value = 0.1,
            label="Temperature",
            step=0.05
        )
        npredict = gr.Slider(10, 10000,
            value = 8000,
            label="Num predict",
            step=10
        )
    selected_lang = "Answer in British English"
    def get_selected_lang(foo):
        selected_lang = "Answer in "+foo
    lang.change(fn=get_selected_lang, inputs=lang)
    
    def user(user_message, history: list):
        # gr.ChatMessage(role="user", content=user_message)
        # history.append(gr.ChatMessage(role="assistant", content="Hello, how can I help you?"))
        history.append(gr.ChatMessage(role="user", content=user_message))
        return "", history #String ends up in textbox, thus empty.

       
    # New bot using the HayStack prompt builder.
    def newbot_pipeline(history: list, slider_val, tmp_val, cutoff, ignore_extras, npredict, chosen_model):
        last = history[-1]
        now = datetime.now() # current date and time
        date_time = now.strftime("%Y%m%dT%H%M%S")
        DBG(date_time)
        user_message = last['content']
        DBG(last['role'].upper() + ": " + user_message)
        
        ctxkeep = int(slider_val)
        DBG(f"CONTEXT SIZE: {slider_val}")
        DBG(f"CUT-OFF:{cutoff}")
        context = get_context(user_message, hybrid_retrieval, cutoff, ctxkeep)
        DBG("SELECTED CONTEXT")
        for x in context:
            DBG(x)
        # ctxkeep zero means no extra knowledge at all.
        DBG(f"IGNORE EXTRAS:{ignore_extras}")
        DBG("MODEL: "+str(chosen_model))

        messages=[]
        #messages += history[:-1] # because the prompt has the context.
        messages.append({"role": "user", "content": user_message})

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
        prompt = prompt_builder.run(question=user_message, documents=context)
        prompt = prompt["prompt"]
        DBG(prompt)
        his = gr.ChatMessage(role="assistant", content="")
        history.append(his)

        DBG(f"TEMP: {tmp_val}")
        DBG(f"NUM PREDICT: {npredict}")
    
        partial = ""
        def _cb(chunk):
            nonlocal partial
            partial += chunk.content
            return partial

        generator = OllamaGenerator(
            model=chosen_model, #gen_model,
            url="http://localhost:11434",
            generation_kwargs={
                "num_predict": npredict,
                "temperature": tmp_val,
                "num_ctx": 12028,
                "repeat_last_n": -1,
            },
            streaming_callback=_cb
        )
        partial_message = ""
        try:
            for x in generator.run(prompt):
                partial_message = partial
                his = gr.ChatMessage(role="assistant", content=partial_message)
                history[-1] = his
                yield history #partial_message
        except Exception as e:
            DBG(e)
            partial_message = "There is something wrong with the model."
            partial_message += "\n" + str(e)
            his = gr.ChatMessage(role="assistant", content=partial_message)
            history[-1] = his
            yield history #partial_message

        DBG(partial_message)
        
        
    # msg.submit(user, [msg, chatbot], [msg, chatbot], queue=False).then(
    #     newbot, chatbot, chatbot
    # )
    msg.submit(user, [msg, chatbot], [msg, chatbot], queue=False).then(
        newbot_pipeline, [chatbot, val, tmp, cutoff, ignore_extras, npredict, model_selector], chatbot
    )
    # clear.click(lambda: None, None, chatbot, queue=False)
    clear.click(lambda: ([], ""), None, [chatbot, msg], queue=False)

if __name__ == "__main__":
    print("Starting")
    
    terminal_width = os.get_terminal_size().columns
    
    DBG(f"Loading document store {args.datastore}...")
    doc_store = InMemoryDocumentStore().load_from_disk(args.datastore)
    #doc_store = InMemoryDocumentStore().load_from_disk("testdata.store")
    #doc_store = InMemoryDocumentStore().load_from_disk("research_docs_sp.store")
    DBG(f"Number of documents: {doc_store.count_documents()}.")

    # Docs are already indexed/embedded in the sotre.
    hybrid_retrieval = create_hybrid_retriever(doc_store)
    '''
    pipeline.add_component(instance=InMemoryBM25Retriever(document_store=document_store), name="retriever")
    pipeline.run(data={"retriever":
            {"query": query,
             "filters": {"field": "meta.version", "operator": ">", "value": 1.21}}})


    documents = retrieve(hybrid_retrieval, query, top_k=args.top_k)
    for doc in documents:
        #print(doc.id, doc.meta["names"], ":", doc.meta["title"])
        print_res(doc, terminal_width)
    '''

    DBG("Creating UI.")
    if args.shared:
        demo_blocks.launch(share=True)
    else:
        demo_blocks.launch()
