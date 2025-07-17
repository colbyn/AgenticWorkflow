# TL;DR

Overall, I'm attempting to generalize some techniques I've learned from this project as presented/demonstrated in this [YouTube video](https://youtu.be/nofJLw51xSk?si=587YwGXe4AB-2u3O): **'How I autogenerate massive (dictionary) datasets with ChatGPT/LLMs and why this matters'**.


# Example

## Execute

```
$ cargo run --bin xml-ai -- run notes/StandaloneExamples.html --name question-1 --key-file secrets/open-ai.key --output .xml-ai/latest.json
```

> The above will execute the following prompt template:
> ```html
> <prompt name="question-1">
>     <msg role="system">
>         <p>You are a helpful assistant.</p>
>     </msg>
>     <msg role="user">
>         <p>What is heavier, a kilo of feathers or a pound of steel?</p>
>     </msg>
>     <breakpoint role="assistant"></breakpoint>
>     <msg role="user">
>         <p>Explain your reasoning.</p>
>     </msg>
>     <breakpoint role="assistant"></breakpoint>
>     <msg role="user">
>         <p>Would you like to revise your answer?</p>
>     </msg>
>     <breakpoint role="assistant"></breakpoint>
>     <msg role="user">
>         <p>Finally, after considering your thoughts, please state just the answer.</p>
>     </msg>
> </prompt>
> ```
> This will result in **4 LLM invocations**.
> - Three 4 LLM invocations from the `<breakpoint role="assistant"></breakpoint>`
> - The final unevaluated message will result in a final LLM invocation. 
> ---
>
> **Output example:**
>
> > A kilogram of feathers is heavier than a pound of steel. This is due to the difference between the metric and imperial systems of measurement. A kilogram is approximately 2.2 pounds, so a kilogram of any material, feathers included, is heavier than a pound of any other material, steel included.
> >
> > ---
> >
> > In terms of mass, a kilogram is a larger unit of measurement compared to a pound. Specifically, 1 kilogram is approximately equal to 2.20462 pounds. Therefore, 1 kilogram of any substance, including feathers, will always be heavier than 1 pound of any other substance, including steel. Thus, a kilogram (kilo) of feathers is heavier than a pound of steel.
> >
> > ---
> >
> > Sorry for any confusion, but there's no need for a revision. The answer is clear in the measurement units: 1 kilogram (kg) of any substance is heavier than 1 pound (lb) of any substance - because 1 kilogram is  approximately equal to 2.20462 pounds. So, 1 kilogram of feathers is indeed heavier than 1 pound of steel.
> >
> > ---
> >
> > A kilogram of feathers is heavier than a pound of steel.

## Overview

### Documents

An XML/HTML document is currently a collection of

- `<prompt>`

#### `<prompt>`

A prompt element will consist of the following

- `<msg>`: A message element.
- `<breakpoint>`: A breakpoint element; will evaluate all prior messages in the conversation history and then append a new message thereto with the provided `role="[ROLE]"`.
- `<set>`: Sets/Updates the prompt settings. Unlike my [previous LLM/AI prompt templating format](https://github.com/colbyn/ai-subsystems) since a prompt can itself result in multiple LLM invocations, this needs to be set in a manner that can then be updated throughout the workflow.

##### `<msg>`

A message element consists of either all text or elements where each element consists of the following:

- `<p>`: A paragraph

The rationale for this design decision is to better control formatting, as demonstrated in my [YouTube video](https://youtu.be/nofJLw51xSk?si=587YwGXe4AB-2u3O) (**'How I autogenerate massive (dictionary) datasets with ChatGPT/LLMs and why this matters'**). My philosophy is that all input tokens as part of the prompt engineering text should be as perfect as possible, including ensuring unnecessary whitespace.

# Future work

## JSON dataset generation/population 

I'm considering something akin to the following,

```html
<schema src="./schema-1.json" id="schema-1"></schema>

<prompt name="fix-json-object" input:schema="of type Schema" input:json-value="of type String">
    <set response-format="json-object"></set>
    <msg role="system">
        <p>You previously returned invalid JSON that does not conform to the expected schema.</p>
        <p>Your task is to fix the syntax and ensure the structure matches the schema exactly.</p>
        <p>⚠️ Only return the **corrected JSON data** — do not include the schema, explanation, or any additional text.</p>
    </msg>
    <msg role="user">
        <p>Here is the malformed JSON:</p>
        <p from="json-value" format="pretty"></p>
    </msg>

    <msg role="user">
        <p>Here is the expected JSON schema:</p>
        <p from="schema" format="pretty"></p>
    </msg>
</prompt>
```

But this is something I'm still working on. In general, verification is a must when designing complex LLM centric workflows as discussed at length in my [YouTube video](https://youtu.be/nofJLw51xSk?si=587YwGXe4AB-2u3O) (**'How I autogenerate massive (dictionary) datasets with ChatGPT/LLMs and why this matters'**).

