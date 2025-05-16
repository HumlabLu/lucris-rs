#import "@preview/showybox:2.0.4": showybox

#set text(font: "Optima", size: 11pt)

#showybox(
frame: (
    border-color: red.darken(50%),
    title-color: white, //red.lighten(60%),
    body-color: white, //red.lighten(80%)
  ),  
  title-style: (
    color: black,
    weight: "regular",
    align: center
  ),
  title: [Question 9], 
  [Summarise the research on animals at Lund University.],
)
#showybox(
frame: (
    border-color: red.darken(50%),
    title-color: white, //red.lighten(60%),
    body-color: white, //red.lighten(80%)
  ),  
  title-style: (
    color: black,
    weight: "regular",
    align: center
  ),
  title: [Answer],
  [Based on the provided context, the research on animals at Lund University is extensive and diverse. Several researchers have contributed to various studies and initiatives.

Marie Leth-Espensen, Tobias Linné, Gina Song Lopez, and Naja Yndal-Olsen have conducted research on Critical Animal Studies, including the development of the course "Critical Animal Studies: Animals in Society, Culture and the Media" and the formation of the Lund University Critical Animal Studies Network (LUCASN). Their research (Index 1) aims to equip students with analytical tools to critically assess norms and structures that organize human-animal relations.

Marianne Gullberg has published several annual reports on the Lund University Humanities Lab, which provide insights into the research conducted within the lab. Her reports (Index 3, 5, 7, 9, 11, 13, 15) cover various topics, including research on human-animal relations, but specific details on animal research are not provided.

Stina Oredsson, Sandra Coecke, Jan van der Valk, and Mathieu Vinken have conducted research on "animal-free research" (Index 8), which explores the concept of animal-free research.

Mathias Osvath and Tomas Persson have conducted research on the Lund University Primate Research Station (Index 10).

Erika Andersson Cederholm and Kristina Jennbert have published research on "Exploring the Animal Turn: Human-animal relations in Science, Society and Culture" (Index 14), which examines human-animal relations in various contexts.

Frida Splendido and Marianne Gullberg have conducted research on the Lund University Humanities Lab Annual Report 2023 (Index 16), which provides an overview of the lab's research activities.

L-A Hansson and Susanne Åkesson have published research on "An introduction to animal movement" (Index 18), which provides an overview of animal movement and related topics.

Su Mi Dahlgaard-Park has published research on Lund University's Journey Toward Excellence (Index 12), which discusses the university's efforts to improve its research and education.

Carl Fehrman has published research on Lund and Learning (Index 17), which provides an overview of the city of Lund's history and development.

In summary, the research on animals at Lund University is diverse and includes studies on Critical Animal Studies, human-animal relations, animal-free research, primate research, and animal movement.]
)
#showybox(
frame: (
    border-color: red.darken(50%),
    title-color: white, //red.lighten(60%),
    body-color: white, //red.lighten(80%)
  ),  
  title-style: (
    color: black,
    weight: "regular",
    align: center
  ),
  title: [Note], 
  [model='llama3.1', extractionmodel='mistral', embeddings=True, query=None, research=None, showprompt=False, temp=0.5, top_k=19, rank_k=0],
)

