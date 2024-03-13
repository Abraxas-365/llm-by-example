# Langchain

## Que Vamos a Ver Hoy

...

## Que no vamos a ver

...

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

## langchain

- Prompting
- Que es un chain
  - Crear ejemplo de chains
- Que es un Agente
  - Crear un agente
- Que son tools

  - Crear un tool
  - Tool de internet
  - tool de memoria con embeddings

- Memoria

- Crear una applicacion

## Conlucion

- Cuando usar y cuando no usare lagchain
