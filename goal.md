Word Play is a Steam game where you are given a 4x4 grid of letters which you use to make words.

I would like to experiment with AI visual question answering.

I have Ollama running locally and I would like to use the Gemma3:12b model to identify the letters in the grid and subsequently identify the words that can be spelled using those letters.

I have included cain.txt which we can filter to find words.


We will need the user to designate the rectangle that describes the letter grid.

We will then programmatically screen capture this rectangle.

This image will then be fed into the LLM.

We will use various prompts to interrogate the image to identify the letters in the grid.

===
~
❯ ollama ls
NAME                                                  ID              SIZE      MODIFIED
devstral:latest                                       c4b2fa0c33d7    14 GB     2 weeks ago
gemma3:1b                                             8648f39daa8f    815 MB    5 weeks ago
phi4-mini-reasoning:latest                            3ca8c2865ce9    3.2 GB    5 weeks ago
phi4-reasoning:latest                                 47e2630ccbcd    11 GB     5 weeks ago
qwen3:14b                                             7d7da67570e2    9.3 GB    5 weeks ago
qwen3:8b                                              e4b5fd7f8af0    5.2 GB    5 weeks ago
qwen3:4b                                              a383baf4993b    2.6 GB    5 weeks ago
qwen3:1.7b                                            458ce03a2187    1.4 GB    5 weeks ago
qwen3:0.6b                                            3bae9c93586b    522 MB    5 weeks ago
gemma3:12b                                            6fd036cefda5    8.1 GB    2 months ago
gemma3:27b                                            30ddded7fba6    17 GB     2 months ago
unclemusclez/smollm-135m-instruct-devinator:latest    d24a131eaedd    271 MB    2 months ago
qwq:32b                                               cc1091b0e276    19 GB     3 months ago
deepseek-r1:14b                                       ea35dfe18182    9.0 GB    4 months ago
deepseek-r1:8b                                        28f8fd6cdc67    4.9 GB    4 months ago
deepseek-r1:32b                                       38056bbcbb2d    19 GB     4 months ago
gemma2:2b                                             8ccf136fdd52    1.6 GB    4 months ago
x/llama3.2-vision:latest                              25e973636a29    7.9 GB    4 months ago
phi4:14b                                              ac896e5b8b34    9.1 GB    4 months ago
vanilj/Phi-4:latest                                   f00b090d5270    9.1 GB    5 months ago
bge-m3:latest                                         790764642607    1.2 GB    6 months ago
qwen2.5:iq4                                           c1e4b06ac94d    8.1 GB    6 months ago
snowflake-arctic-embed:22m                            bf75350e1752    45 MB     6 months ago
qwq:latest                                            1211a3265dc8    19 GB     6 months ago
~
❯ ollama run gemma3:12b
>>> What are the letters in this 4x4 grid? C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06\Word_
... Play_0jjFmdqvBt.png
Added image 'C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06\Word_Play_0jjFmdqvBt.png'
Here's the list of letters in the 4x4 grid, along with their point values:

*   **E** - 1, 2
*   **N** - 1
*   **O** - 1
*   **T** - 1
*   **A** - 1
*   **U** - 1, 2
*   **X** - 8
*   **J** - 8
*   **V** - 4
*   **P** - 3

>>> /clear
Cleared session context
>>> What are these letters? C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06\Word_Play_0jjFmdqvBt
... .png
Added image 'C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06\Word_Play_0jjFmdqvBt.png'
Here's the list of letters from the image:

*   E
*   N
*   O
*   T
*   A
*   U
*   X
*   D
*   J
*   V
*   O
*   P

>>> What is the last row of letters? C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06\Word_Play_0
... jjFmdqvBt.png
Added image 'C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06\Word_Play_0jjFmdqvBt.png'
The last row of letters is:

**U O V P**

>>> What is the first row of letters? C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06\Word_Play_
... 0jjFmdqvBt.png
Added image 'C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06\Word_Play_0jjFmdqvBt.png'
The first row of letters is:

**E N O T**

>>> What is the second row of letters? C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06\Word_Play
... _0jjFmdqvBt.png
Added image 'C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06\Word_Play_0jjFmdqvBt.png'
The second row of letters is:

**A U X**

>>> What is the
Use Ctrl + d or /bye to exit.
>>> /clear
Cleared session context
>>> What is the second row of four letters? C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06\Word
... _Play_0jjFmdqvBt.png
Added image 'C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06\Word_Play_0jjFmdqvBt.png'
The second row of four letters is **AUX**.

>>> /clear
Cleared session context
>>> What are the four letters in the second row? C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06
... \Word_Play_0jjFmdqvBt.png
Added image 'C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06\Word_Play_0jjFmdqvBt.png'
The four letters in the second row are **A U X E**.

>>> What are the letters in the third row? There should be four of them C:\Users\TeamD\OneDrive\Pictures\Screenshots\Sha
... reX Screenshots\2025-06\Word_Play_0jjFmdqvBt.png
Added image 'C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06\Word_Play_0jjFmdqvBt.png'
The letters in the third row are **D A J E**.

>>> What words can be spelled uis
Use Ctrl + d or /bye to exit.
>>> /clear
Cleared session context
>>> What words can be spelled with these letters? C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-0
... 6\Word_Play_0jjFmdqvBt.png
Added image 'C:\Users\TeamD\OneDrive\Pictures\Screenshots\ShareX Screenshots\2025-06\Word_Play_0jjFmdqvBt.png'
Here's a list of words that can be spelled using the letters provided:

**3-Letter Words:**

*   eat
*   tea
*   toe
*   net
*   one
*   pot
*   due
*   out
*   vet

**4-Letter Words:**

*   done
*   tone
*   vote
*   dune
*   pout

**5-Letter Words:**

*   donut

**6-Letter Words:**

*   jovote

**7-Letter Words:**

*   outdone

**8-Letter Words:**

*   adjourn
*   outvoted



Let me know if you'd like me to try and find more!

>>> /clear
Cleared session context
>>> What words can be spelled using the following characters (scrabble style): AGAEIZUXDAJEUOSA
Okay, let's see what words we can make from the letters: **A G A E I Z U X D A J E U O S A**

Here's a breakdown, categorized by length and with some longer/less common words included.  I'm aiming for
generally accepted English words.

**2-Letter Words:**

*   AE
*   AG
*   AS
*   AD
*   DU
*   EU
*   US

**3-Letter Words:**

*   AGE
*   ADS
*   AUE
*   DAE
*   DUA
*   EAU
*   GAS
*   GAD
*   GAU
*   SEA
*   SAE
*   USE
*   USA

**4-Letter Words:**

*   AGES
*   DAZE
*   DUES
*   GAES
*   GAZE
*   JADE
*   JAZZ
*   JUGS
*   SAE
*   SEAD
*   SUED
*   USED
*   USED
*   ZEDS

**5-Letter Words:**

*   ADZES
*   DAUES
*   GAUZE
*   JADES
*   JAZED
*   SAUDE
*   SUADE
*   USAGE

**6-Letter Words:**

*   JAZZED
*   SAUZED

**7-Letter Words:**

*   GAUDED

**Notes:**

*   I'm prioritizing common words. There might be some obscure or regional words possible.
*   I'm assuming standard Scrabble rules (no proper nouns, abbreviations, etc.).
*   I'm not including plurals unless the singular form is also a valid word.



Let me know if you'd like me to focus on words of a specific length or type!

>>> Send a message (/? for help)
===

From this experiment with the CLI, I have determined that it will probably take multiple prompts to get an accurate representation of the letters.

The longer we let the model "think" (the more times we ask for "letters in this image"/nth row/nth column) the more confidence we will have in the answer

We can create multiple independent programs to isolate the individual tasks.

The first program is the User Region Identification and Naming Exosystem.
When the program starts, it will darken the monitor.
When the user begins a left click, it begins drawing a rectangle where the region is scissored/excluded from the darkening.

The URINE program should function as follows

urine.exe --prompt "Identify the region with the letters" --debug

The prompt and debug parameters should be optional.
The debug parameter should set the tracing level to debug when present.

The prompt text, if provided, will be drawn in large text on the center monitor.


The wincs crate has a monitor enumeration and screencapture functionality.

Use clap derive to parse the arguments.

Create a helper fn to init tracing with a boolean debug parameter.

Once we have the region of the screen, we can make a separate program that acceptsregion XY with height as parameters forcreating a screen capture programmatically where when we call the program it willoutput APNG to the specified output path.

So the user will usethe urine program to identify the region.and then the screen capture program willbe called with the region that's been identified and that will produce an image and then that image will be passed to the language models will have a third program that is responsible for identifying the letters that are in the grid.and then a fourth program will be responsible for accepting the list of letters and producing the ranked suggestions for what words to try.

Each of the programs should have the --debug argument.

Each of the programs should be an independent binary.

Each of the programs should use clap derive to parse the arguments.

For the urine program, thefunctionality where we darken the screen and accept the user input can be accomplished by using bevy because bevy supports transparent windows so when the bevy program starts it willcreate a window for each of the monitors and then we'llhandleattending to the clicks and creating the rectangle and drawing the sprites that darken the window and the region that excludes that.

I have a few crates or directories open in VS Code workspace already.

ollama-rs - third-party crate for interacting with Ollama
aura-exporter - a rust project I made that demonstrates some of the argument parsing and automata style programming in rust
youre-muted-btw - a rust project I made that launches a program that starts a system tray icon and handles spawning a bevy program as a separate program to perform window operations
ratatui - third-party crate for terminal user interfaces
winc - a rust project I made that handles monitor enumeration and screen capture on windows
cloud_terrastodon_user_input - a rust project I made that makes it easy to use fzf to interact with the user

This entire will be windows-only.

Please begin planning and implementation towards our end goal as described.

===

Please begin planning and implementation towards our end goal as described.

This is currently an empty directory with goal.md, cain.txt, and LICENSE.

You will need to create the workspace Cargo.toml and the crates within.

You are responsible for interrogating the existing included Rust projects to adhere to my stated and implied desired coding styles.