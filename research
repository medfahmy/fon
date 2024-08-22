# https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system

```
    VARIABLE:   [A-Za-z]([A-Za-z0-9_])*

    binding:
        | VARIABLE

    binding_list:
        | binding
        | binding ',' binding_list

    bindings:
        | binding
        | '(' binding_list ')'

    expr_primary:
        | LITERAL
        | VARIABLE
        | "()"          -- Unit
        | '(' expr ')'

    expr_unary:
        | expr_primary
        | '-' expr_primary

    expr_application:
        | expr_unary
        | expr_application '(' expr_list ')'

    expr_multiplicative:
        | expr_application
        | expr_multiplicative '%' expr_application

    expr_additive:
        | expr_multiplicative
        | expr_additive '+' expr_multiplicative

    expr_comparative:
        | expr_additive
        | expr_additive '==' expr_additive

    expr_lambda:
        | expr_comparative
        | bindings '->' expr

    expr_list:
        | expr
        | expr ',' expr

    expr:
        | expr_lambda
        | '(' expr_list ')'

    expr_binding:
        | expr
        | bindings '=' expr
```

This language supports simple expressions, lambdas, tuples, and bindings (assignments). These are all valid expressions:

```
123
a + b
(x, y)
f (x, y)
x -> x
id = x -> x
swap = (x, y) -> (y, x)
(odd, even) = (x -> x % 2 == 0, x -> x % 2 == 1)
```

To add optional type annotations to this language requires minimal effort. We make the annotation optional in bindings:

```
   +type:
   +    | VARIABLE

    binding:
        | VARIABLE
   +    | VARIABLE ':' type
```

And then we create an optionally typed expression, changing the expr_primary rule to support these on any expression, and also changing the expr_binding rule to allow them at the top level.

```
   +expr_optionally_typed:
   +    | expr
   +    | expr ':' type

    expr_primary:
        | LITERAL
        | VARIABLE
        | "()"
   -    | '(' expr ')'
   +    | '(' expr_optionally_typed ')'

    expr_binding:
   -    | expr
   -    | bindings '=' expr
   +    | expr_optionally_typed
   +    | bindings '=' expr_optionally_typed
```

That's basically it. Although in this simple example we've not made any interesting types because they're just variables. But because we picked an unused operator for type annotations, we can make this type rule essentially a whole new language itself, where the RHS of : can be anything which doesn't contain a : or = outside of unbalanced parens. A more interesting type grammar might look something like this:

```
    TYPE_NAME:      [A-Z]([A-Za-z0-9_])*

    TYPE_VARIABLE:  [a-z]([A-Za-z0-9_])*

    type_primary:
        | TYPE_NAME
        | TYPE_VARIABLE
        | "()"            -- Unit
        | '(' type ')'

    type_application:
        | type_primary
        | type_primary '[' type_list ']'

    type_function:
        | type_application
        | type_application "->" type_function

    type_list:
        | type_function
        | type_function ',' type_list

    type:
        | type_function
        | '(' type_list ')'
```

The above examples annotated with types:

```
123 : Int 
a + b : Int
(x, y) : (Int, Int)
f (x, y) : Foo
x -> x : a -> a
id = x -> x : a -> a
swap = (x, y) -> (y : b, x : a)
(odd : Int -> Bool, even : Int -> Bool) = (x -> x % 2 == 0, x -> x % 2 == 1)
```

The lambdas and bindings are more flexible in how we define types. We can use any of the following really:

```
// type signature can stand by itself
id : a -> a   
id = x -> x

// Can combine type signature and binding into one-liner.
id = x -> x : a -> a

// Type of a function can go on its name.
swap : (a, b) -> (b, a) = (x, y) -> (y, x)

// Or on the individual arguments
swap = (x : a, y : b) -> (y, x)

// Or on the result
swap = (x, y) -> ((y, x) : (b, a))

// Or on the parts of the result
swap = (x, y) -> (y : b, x : a)

// Or any combination of the above.
```

The types can basically go anywhere, and the same syntax is used consistently. We'll get the expressions annotated in the AST. The assumption is we'll then use type inference to figure out the types of all of the non-annotated expressions.

It's difficult to get much simpler than this without going for S-expressions or similar, but S-expressions are not nice to work with when using type annotations. There's several typed lisps which have tried, and one of the common approaches is to just use (: value Type), where this sub-expression gets eliminated to value after type checking. Basically the same thing, but with prefix syntax and extra parens.

Going further, one might use the :: operator within the type grammar to denote kinds, where we could optionally annotate a type with its kind. The grammar for kinds would not interfere with the grammar for types or expressions, as long as they don't contain =, : or :: and they have balanced parens.

```
id : a -> a :: * -> *
id = x -> x
```
