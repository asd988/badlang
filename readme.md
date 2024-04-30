# badlang

why? idk

## it's simple

```badlang
# this is a comment

# there are only integers
# define a variable
# first must be the name of the new variable, second must be either a number or another variable
# there are no expressions, only one operation at a time
a = 1

# print a variable
< a
# you can also put text next to it
< a, the value of a

# operations
a += 1
a -= 1
a *= 1
a /= 1
a %= 1
a max= 1 # sets a to the max of a and 1
a min= 1 # sets a to the min of a and 1
a invert # sets a to 1 if a is 0, and 0 if a is not 0
a delete # deletes the variable

# tags
@here
# stacked tags
@@func
# if you use a stacked tag, you can use the following to return to the calling point
# you can also use it to return to the end of the program
return

# goto
jmp here

# it can be conditional
jmp here if a
# it only jumps if the variable is not 0
```

## Examples
examples can be found in the examples folder

## How to run
```sh
cargo run --bin badlang path/to/file.badlang
```
> Note that the file doesn't neccesarily have to have the extension `.badlang`

## language server
The language server can be compiled with the following command
```sh
cargo build --bin lsp
```

The vscode extension can be ran by pressing `F5` inside vscode, or inside the `Run and Debug` tab. 
You might have to call `npm install` inside the `extension` folder to install the dependencies.

## what's next?
For now I'm done with this project. I justed wanted to try out making an LSP server, and I might expand on it in the future.
However, I'm definitely not going to work on the language itself anymore. 

LSP
- [x] go to definition for tags
- [x] hover documentation for tags
- [x] errors for jumps to undefined tags
- [x] errors for variables that are never defined