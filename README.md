# Langchain

## Que es Langchain

LangChain es un framework de código abierto para el desarrollo de aplicaciones
que utilizan modelos de lenguaje de gran tamaño (LLM). Fue creado inicialmente en Python.
Las herramientas y API de LangChain simplifican el proceso de creación de aplicaciones
impulsadas por LLM, como chatbots y agentes virtuales.

## Que es un LLM

Un Large Language Model (LLM) es un modelo de inteligencia artificial desarrollado para entender y generar texto humano con un alto grado de precisión y coherencia.
Estos modelos son entrenados en grandes cantidades de texto para aprender patrones
del lenguaje y son capaces de responder preguntas, escribir ensayos, resumir textos,
traducir lenguajes, y esencialmente realizar cualquier tarea lingüística que un humano
pueda hacer.

### Funcionamiento del LLM

El LLM utiliza una arquitectura llamada Transformer. Esta arquitectura presta especial
atención a la contextualización de palabras, lo que significa que el modelo comprende
el significado de una palabra basándose en su posición en relación con las demás palabras
de la frase.

## Analogia para entender el funcionamiento de un LLM

Existía un niño con una memoria y tiempo ilimitados, nacido en una biblioteca donde
disponía de acceso a todos los libros y páginas web del mundo. Aunque aislado y sin
enseñanzas recibidas, era el único ser vivo dentro de aquel refugio de saberes.

Tenía un oficio diario, leer. Sin interpretar lo que sus ojos veían, consumía palabra
tras palabra, línea tras línea. No comprendía lo que leía,sin embargo, día tras día,
año tras año, sus horas estaban llenas de infinitas páginas
de información.

Después de una larga cadena de años, una interrupción llegó a su soledad. Alguien
se acercó a la puerta de la biblioteca y deslizó un papel por debajo de la misma.

El papel planteaba una pregunta: "¿Quién es el presidente de Perú?"

El niño, al observar el papel, reconocía las cadenas de caracteres que se plasmaban
en él. Recordaba las multitudes de artículos sobre el presidente de Perú que había
'leído'. Recordaba detalles, fechas de nacimiento, tiempos en el cargo, a pesar de
que no entendía el significado en aquel momento.
Y, entonces, con una capacidad asombrosa, el niño respondió:

"El presidente de Perú es Dina Boluarte..."

## Tipos de "Entrenamiento"

### Fine Tunning

El término Fine-Tuning en el contexto de los modelos de lenguaje de gran tamaño (LLM),
se refiere al proceso de reajustar un modelo pre-entrenado en un nuevo conjunto de
datos para mejorar su rendimiento. En lugar de entrenar un modelo desde cero, el Fine-Tuning
utiliza un modelo que ya ha sido entrenado para una tarea similar y luego ajusta sus
parámetros para adaptarlo a la nueva tarea. Este enfoque ahorra tiempo y recursos
computacionales, y a menudo mejora los resultados del modelo en la nueva tarea.

Por ejemplo, podríamos tomar un modelo como GPT-4 y adaptarlo para que responda de cierto modo.

- El Fine-Tuning es muy útil para:

  - Hacer que el modelo responda de forma más amigable.
  - Hacer que el modelo actúe como un vendedor.
  - Lograr que el modelo adopte ciertas personalidades.
  - Habilitar al modelo para responder usando JSON.
  - Capacitar al modelo para responder usando Markdown.

- Sin embargo, existen limitaciones en el uso del Fine-Tuning:

  - No es eficiente si nuestro objetivo es lograr que el modelo aprenda informacion deterministica.
  - Tendremos que gastar mucho tiempo y no se garantiza que responderá con la información deseada.

### Embeddings

Los "Embeddings", en el contexto de los modelos de lenguaje, son representaciones
vectoriales de palabras o frases en un espacio de alta dimensión. Estas representaciones
capturan el contexto semántico y sintáctico de las palabras, basándose en su co-ocurrencia
con otras palabras en el corpus del texto. Estos vectores son utilizados por modelos
como los LLMs para entender e interpretar el significado de las palabras en relación
con las demás.

Para simplificar, los embeddings convierten las palabras en una forma que las máquinas
puedan entender y procesar. Estos números dentro del vector representan diversas características
de las palabras.

Por lo tanto, si visualizamos este espacio vectorial, palabras con significados o
usos similares estarán más cercanas entre sí. Por ejemplo, en este espacio, la palabra
"hamburguesa" estaría más cerca de "chorizo" que de "carro", ya que "hamburguesa"
y "chorizo" están relacionadas con la comida. De manera similar, "ketchup" y "rojo"
estarían cerca el uno del otro, ya que "ketchup" es comúnmente asociado con el color
rojo, además "ketchup" también estaría cerca de "hamburguesa" y "chorizo" dado su
uso común como condimento para estos alimentos.

Los embeddings nos abren un mundo de posibilidades que exploraremos en breve. Nos
permiten proporcionar una especie de memoria a largo plazo a los LLMs, ya que podemos
guardar información en forma de texto y su correspondiente representación vectorial
en una base de datos. Posteriormente, podemos recuperar esa información y enviarla
como contexto al LLM.

- Los embeddings son útiles para:
  - Hacer que el LLM responda información precisa.
  - Enviar nuestra información al LLM para que la utilice en sus respuestas.
  - Permitir que el LLM tenga memoria a largo plazo.

### Conclusion

En conclusión, tanto el Fine-Tuning como los Embeddings son técnicas valiosas en el
campo de los modelos de lenguaje de gran tamaño.

El Fine-Tuning permite mejorar y personalizar el desempeño del modelo al reajustarlo
en un conjunto de datos nuevo o específico, ahorrando tiempo y recursos en comparación
con entrenar un nuevo modelo desde cero. Sus aplicaciones pueden ir desde lograr una
interacción más amigable hasta la adopción de ciertas personalidades por parte del
modelo.

Por otro lado mediante los `Embeddings`, somos capaces de construir una especie de memoria
a largo plazo para los LLMs, como
también personalizar y mejorar la precisión de las respuestas dadas por el modelo
basándose en estos vectores de palabras y frases.

Juntos, el Fine-Tuning y los Embeddings ofrecen poderosos instrumentos para mejorar,
personalizar y aprovechar al máximo los LLMs, permitiendo adaptar estos modelos a
una amplia gama de usos y aplicaciones. Sin embargo, es esencial tener en cuenta sus
limitaciones y considerar su uso apropiado en función de los objetivos y tareas particulares.

- Embedding example
  [Embedding](embeddings/)

## Conceptos Basicos

En esta sección, es clave entender bien las ideas que están jugando en el mundo de
los Modelos de Lenguaje de Gran Tamaño (LLMs) y LangChain.
Esto es súper importante porque conocer estos conceptos nos ayuda a entender mejor
LangChain. Y hasta podríamos crear nuestro propio LangChain.

### Prompts

Un "prompt" en el contexto de Modelos de Lenguaje de Gran Tamaño (LLMs) se refiere
a la entrada de texto que se le proporciona al modelo para generar una respuesta o
completar la entrada de texto dada. Estos prompts pueden ser desde una única palabra,
hasta frases completas o incluso párrafos.

Usando los Prompts podemos usar los prompts para guiar la generación del texto del
modelo en una dirección
en base a los patrones de lenguaje que ha aprendido relacionados con los cuentos de hadas.

- Esta técnica es útil para:

  - Obtener respuestas específicas del modelo.
  - Generar textos creativos como cuentos, poemas, canciones, etc.
  - Obtener explicaciones de conceptos científicos, definiciones tecnológicas, etc.
  - Generar texto en diferentes estilos y tonos.
    Los prompts son una herramienta crucial para interactuar con los modelos de lenguaje y obtener el tipo de texto que necesitamos. Sin embargo, lo más difícil es construir un prompt que dé una respuesta útil. Como los modelos se entrenan en datos masivos, muestran una distribución amplia de posibles respuestas. A fin de cuentas, los modelos de lenguaje solo pueden generar texto basándose en lo que han visto en sus datos de entrenamiento y, aunque pueden producir información factual y lógicamente coherente, no pueden aplicar la comprensión común del mundo tal como los humanos. Por lo tanto, es esencial aprender a definir prompts precisos y efectivos para obtener los mejores resultados posibles del modelo.

- Tipos de Prompt basado en el modelo:
  Actualmente hay 2 tipos prompt los 2 grandes tipos de "modelo"
  - Completition Models
  - Chat models

#### Completition Models

Los Modelos de Completación, como sugiere su nombre, están diseñados para completar
un texto dado. Reciben una entrada de texto, conocida como "prompt", y generan una
secuencia de texto que parece una continuación coherente y natural de la entrada.

##### Ejemplo de un Modelo de Completación:

Si proporcionamos al modelo el prompt `"En un mundo dominado por dragones..."`,
una posible continuación generada por el modelo podría ser:
`"...las humanidad ha tenido que adaptar su forma de vida para sobrevivir. Las ciudades están construidas en lo alto de montañas inaccesibles, lejos del alcance de los dragones, y los viajes entre ellas se realizan a través de una red de túneles subterráneos. La gente ha aprendido a utilizar la magia para camuflarse y proteger sus hogares de los dragones. A pesar de la amenaza constante, la humanidad ha encontrado maneras de coexistir con estas criaturas míticas, estudiándolas y aprendiendo de ellas. La existencia de los dragones ha dado lugar a una edad de valentía y descubrimiento, donde cada día presenta nuevos desafíos y oportunidades."
Este apunte es aleatorio y solo sirve como ejemplo, la continuación de texto que generará el modelo puede variar cada vez que se realice la tarea con el mismo prompt.`

Si desamos generar una conversacion con estos modelos el pormpt se veria asi:

```
user: Mi nombre es luis
assistant: mi nombre es chatgpt
user: Te gustan los dragones?
assistant:
```

Tendriamos que decirle al modelo que se detenga al querer escribir `user`

En OPEN AI los modelos que son de este tipo son los siguientes

- babbage-002
- davinci-002

#### Chat Models

Los Modelos de Chat están diseñados para generar una respuesta coherente basada en una serie de entradas de texto que representan un diálogo entre dos o más participantes. A diferencia de los Modelos de Completación, que simplemente generan una continuación del texto dado, los Modelos de Chat tienen en cuenta toda la conversación anterior para generar respuestas adecuadas.
Estos modelos son particularmente útiles para aplicaciones de inteligencia artificial conversacional, como asistentes virtuales, chatbots de servicio al cliente, tutorías de aprendizaje automático, y más.

##### Ejemplo de un Modelo de Chat:

En los modelos de chat, la entrada ya sabe que es un usuario. Esto se alimenta en forma de una cadena de objetos JSON.
Por ejemplo, si el usuario dice "Mi nombre es Luis", esto se representaría de la siguiente manera en formato JSON:

```json
[
  {
    "role": "user",
    "content": "Mi nombre es Luis"
  }
]
```

En este caso, cada objeto JSON en la matriz representa un turno en la conversación.
El campo "role" determina si el turno corresponde al usuario o al modelo de IA (generalmente
etiquetado como "assistant", y el campo "content" contiene el texto de ese turno en la conversación.

En OPEN AI los modelos que son de este tipo son los siguientes

- gpt4
- gpt3.5

#### Ejemples Practicos

Los "prompts" son la base de toda aplicación que hace uso de Modelos de Lenguaje de Gran Tamaño (LLMs). Aquí te proporciono un par de ejemplos sobre cómo se utilizan en contextos prácticos:
Un ejemplo sencillo de un prompt puede ser para la generación de ideas creativas. Supongamos que necesitas nombres para una nueva marca de medias. Podrías utilizar un prompt como:
`Genera nombres creativos para una marca de medias que se llama {{nombre}}"`
Donde {{nombre}} puede ser sustituido por el nombre que estés considerando para la marca. El modelo entonces generará opciones creativas basadas en ese nombre.
Por otro lado, un prompt más complejo se podría utilizar en configuraciones más específicas, como es el caso de los "Agentes". Aunque profundizaremos más sobre los "Agentes" más adelante, un ejemplo de un prompt avanzado podría ser solicitar al modelo que genere una respuesta de un Asistente Virtual en un contexto específico de atención al cliente.

```rust
pub const PREFIX: &str = r#"

Assistant is designed to be able to assist with a wide range of tasks, from answering simple questions to providing in-depth explanations and discussions on a wide range of topics. As a language model, Assistant is able to generate human-like text based on the input it receives, allowing it to engage in natural-sounding conversations and provide responses that are coherent and relevant to the topic at hand.

Assistant is constantly learning and improving, and its capabilities are constantly evolving. It is able to process and understand large amounts of text, and can use this knowledge to provide accurate and informative responses to a wide range of questions. Additionally, Assistant is able to generate its own text based on the input it receives, allowing it to engage in discussions and provide explanations and descriptions on a wide range of topics.

Overall, Assistant is a powerful system that can help with a wide range of tasks and provide valuable insights and information on a wide range of topics. Whether you need help with a specific question or just want to have a conversation about a particular topic, Assistant is here to assist."#;

pub const FORMAT_INSTRUCTIONS: &str = r#"RESPONSE FORMAT INSTRUCTIONS
----------------------------

When responding to me, please output a response in one of two formats:

**Option 1:**
Use this if you want the human to use a tool.
Markdown code snippet formatted in the following schema:

{
    "action": string, \\ The action to take. Must be one of {{tool_names}}
    "action_input": string \\ The input to the action
}

**Option #2:**
Use this if you want to respond directly to the human. Markdown code snippet formatted in the following schema:

{
    "action": "Final Answer",
    "action_input": string \\ You should put what you want to return to use here
}


pub const SUFFIX: &str = r#"TOOLS
------
Assistant can ask the user to use tools to look up information that may be helpful in answering the users original question. The tools the human can use are:

{{tools}}

{{format_instructions}}

USER'S INPUT
Here is the user's input (remember to respond with a markdown code snippet of a json blob with a single action, and NOTHING else):

{{input}}"#;

pub const TEMPLATE_TOOL_RESPONSE: &str = r#"TOOL RESPONSE:
---------------------
{{observation}}

USER'S INPUT
--------------------

Okay, so what is the response to my last comment? If using information obtained from the tools you must mention it explicitly without mentioning the tool names - I have forgotten all TOOL RESPONSES! Remember to respond with a markdown code snippet of a json blob with a single action, and NOTHING else."#;
```

### Chains

Un `Chain` es un concepto fundamental sobre el que se basa LangChain.
Básicamente, un `Chain` es la combinación de un `LLM` y un `prompt`.

Para proporcionar un insumo para una cadena, reemplazamos una parte variable en el prompt.
En esencia, tenemos algo similar a esto:
Chain = LLM(GPT3.5) + Prompt

El input de la cadena reemplaza una sección variable del prompt, generando así el
prompt final que se alimentará al modelo de lenguaje.
Por ejemplo, considera el siguiente prompt:
Prompt: `Dame un nombre creativo para una tienda que se venda {{producto}}`

La "cadena" se formaría con la combinación del LLM elegido, digamos GPT-3.5, y el prompt dado.
Si el insumo del `Chain` es `medias`, el prompt final que se alimenta al LLM sería:

Prompt final: `Dame un nombre creativo para una tienda que se venda medias`
Y la salida del Chain sería la respuesta generada por el modelo de lenguaje a este
prompt, que podría ser un nombre creativo para una tienda de medias.

Ahora podríamos crear otra `Chain` que genere un eslogan:
Indicador: `Dame un eslogan para una tienda con este nombre: {{nombre}}`
Podemos encadenarlos de tal manera que la salida de la primera cadena sea la entrada de la siguiente, concatenando así las cadenas.

Ejemplos:

- [Simple Chain](chains/simple_chain)
- [Sequential Chain](chains/sequential_chain)

### Agentes

Los agentes representan la unión de un Prompt, Chain(LLM) y Tools.

Para continuar hablando de Agentes, debemos abordar el último concepto crucial que
nos falta: las Tools.

#### Tools

Los Modelos de Lenguaje (LLMs) no tienen acceso al mundo exterior; están confinados
a su propia biblioteca de conocimientos.

Esto significa que, por ejemplo, si la biblioteca de un LLM sólo tiene acceso a datos
hasta el año 2023, el modelo no podrá buscar por sí mismo información en internet
más allá de esa fecha.

El LLM no obtendrá conocimientos adicionales a menos que nosotros se los proporcionemos
de forma explícita. No podrá conocer el clima de hoy, etc.

En pocas palabras, una herramienta (Tool) es un envoltorio (wrapper) sobre alguna
herramienta del exterior. Algunos de los tools más comunes que podemos encontrar son:

- Acceso a internet
- SQL
- Similarity search en una base de datos de embeddings
- Todo lo que puedas imaginar que tenga una API

Ésta es una forma de extender las capacidades de los modelos de lenguaje, permitiéndoles
interactuar con el mundo exterior y acceder a información y funcionalidades adicionales.

¿Cómo logramos que el LLM sepa qué herramienta usar? Bueno, hace falta entender cierta
información adicional acerca de las herramientas.

Cada Tool debe tener los siguientes valores:

- Nombre: el nombre de la herramienta
- Descripción: una descripción de qué es la herramienta, y si es posible, debe incluir cómo es el input
- Call: la función que se ejecutará cuando se llame a la herramienta

Un ejemplo de pseudo codigo seria este:

```
struct WheatherTool{
name: WhetherTool
description: This tool give you the wheter of a country.
func: fn get_wheater(country:string)->string{
Una funcion que llame a un api que de el clima
}
}
```

#### Prompt

Ahora que entendemos como son los tools, vamos a entender un poco como se ve el prompt
de un agente.

Un ejemplo simple que sea facil de visulisar seria algo asi

---

Assistant can ask the user to use tools to look up information that may be helpful
in answering the users original question. The tools the human can use are:
{{toolsNames}}

RESPONSE FORMAT INSTRUCTIONS
When responding to me, please output a response in one of two formats:

**Option 1:**
Use this if you want the human to use a tool.
Markdown code snippet formatted in the following schema:

```
{
    "action": string, \\ The action to take. Must be one of {{toolNames}}
    "action_input": string \\ The input to the action
}
```

**Option #2:**
Use this if you want to respond directly to the human. Markdown code snippet formatted in the following schema:

```
{
    "action": "Final Answer",
    "action_input": string \\ You should put what you want to return to use here
}

```

USER'S INPUT
Here is the user's input (remember to respond with a markdown code snippet of a json blob with a single action, and NOTHING else):

{{input}}"#;

---

Si prestamos atencion nos daremos cuenta que hemos hecho que el LLM nos devulva una respuesta estructurada,
en forma de json, lo que nos va a permitir extraer el json y saber que accion tomar , o cual es la reaspuesta final.

#### Pasos que sigue un agente

1. LLega un input

2. Se le envia al LLM, el prompt completo, incluyendo los tools , el input.

3. El LLM piensa: "Tengo algun tool que me sirva para responde la pregunta"

4. Si si tiene un tool que le sirva, devuelve el json con el nombre del tool y el input de este.

5. Con codigo extraemos ese json y ejecutamos el tool correspondiente, con el input que nos da el agente.

6. Se envia al agente la respuesta del tool, el agente se hace la siguiente preguta:
   Con esta info y estos tools , puedo responder la pregunta.

7. Si si, se genera la respunta con el Json de Final Answer, si no el ciclo se repite N veces.

Ejemplo:

- [Agente Con tool de Walfram](agents/agent_example)

#### Ventajas y Desventajas

- Ventajas

  - Puede comunicarse con el exterior
  - No es estatico como los chains, puede escojer que herrmienta es mejor para la ocacion

- Desventajas
  - Es lento

### Semantic Routing

Semantic routing es una técnica que surge a partir del uso de embeddings para dirigir el flujo de conversaciones o solicitudes según su contenido semántico.
Consiste en la creación de "Routers", donde cada router contiene frases que se convierten en embeddings. Estos routers se utilizan posteriormente para redirigir el input del usuario hacia una acción específica basándose en el contenido semántico de su entrada.
Ejemplo:
Si tienes un chatbot y deseas evitar que entable conversaciones sobre política, podrías crear un router de política con las siguientes frases:

```rust
let politica = [
  "isn't politics the best thing ever",
  "why don't you tell me about your political opinions",
  "don't you just love the president",
  "they're going to destroy this country!",
  "they will save the country!",
];
```

Este sería el pseudocódigo para redirigir el input del usuario evitando conversaciones sobre política:

```rust
let router_layer = create_router_for(politica);

if router_layer.call(user_input) == "politica" {
  return "Sorry, we can't talk about politics here.";
}
```

Si el input del usuario se evalúa y coincide con el tema de política, el chatbot proporciona
una respuesta estándar indicando que no puede hablar sobre ese tema.

#### Dynamic Semantic Routing

Dynamic Semantic Routing es una evolución del Semantic Routing que incorpora una mayor
flexibilidad y adaptabilidad en el enrutamiento de las solicitudes del usuario según
su contenido semántico.
Esta técnica combina las capacidades de Semantic Routing con la generación dinámica
de inputs para herramientas específicas, emulando de cierta manera la funcionalidad
de los agentes. Así, el Dynamic Routing no solo selecciona el router adecuado en función
del input del usuario, sino que también emplea un Large Language Model (LLM) para
generar el input apropiado de una herramienta, basándose en la descripción de la misma
y el input inicial del usuario.

Funcionamiento:

1. Detección Semántica: Al igual que en Semantic Routing, el sistema primero identifica
   el tema o contexto del input del usuario empleando un conjunto predefinido de routers semánticos.

2. Selección del Router: Una vez identificado el tema, el sistema selecciona el router
   correspondiente, que determina las posibles herramientas o acciones a ejecutar.

3. Generación Dinámica del Input: En vez de redirigir directamente a una acción estándar,
   Dynamic Semantic Routing aprovecha un LLM para analizar el contexto del user input y la descripción de la herramienta seleccionada, generando dinámicamente un input personalizado para dicha herramienta.

- Ventajas
  - Es veloz
