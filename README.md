# Redhowler

- In honour of the South American species *howler monkey*...
- It will be our monkey interpreter written in Rust... 

<img align="left" width="100" height="100" src="https://rustacean.net/assets/rustacean-orig-noshadow.svg">
In this moment of ourselvesdaring venture, we stand at the threshold of 
exploration's infancy. 

Merely acquainting ourselves with the rudimentary foundations, we have
laid the groundwork, establishing the bare essentials required for the 
construction of our code, the famous **"Hello, Monkey Islang!"**. 

With bated breath and vigilant eyes, we observe in real-time the unfolding 
spectacle, seeking insight into the enigmatic tapestry of events that transpire
before us...

---

- Run using `cargo run`
- Currently only the lexer is available and you can use the REPL to tokenize
```sh
Welcome to Monkey Islang!!!
This is the REPL for Monkey programming language.
Feel free to type some code. Ctrl+D to quit
>> let a = 10;
Token { token_type: Let, literal: "let" }
Token { token_type: Ident, literal: "a" }
Token { token_type: Assign, literal: "=" }
Token { token_type: Int, literal: "10" }
Token { token_type: Semicolon, literal: ";" }
>> detected Ctrl+D, exited.
May your trip be as enjoyable as finding extra bananas at the bottom of the bag!
```
- Parsing is in progress...

# Todo

- [x] Lexical Analysis
- [x] Start the REPL
- [ ] Parsing
- [ ] Evaluation
