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

## fibbonaci

```badlang

fib_n = 10
jmp fib_get_nth
< fib_result, is the result
return

# get the nth fibbonaci number
@fib_get_nth
    a = 1
    b = 1
    c = 1

    @fib_loop
        a += b
        b = c
        c = a
        < a

        fib_n -= 1

        jmp fib_loop if fib_n

    fib_result = a
    return