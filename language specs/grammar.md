# Grammar

## Numerical evaluation

```html
<factor>      ::= <number> | "(" <expression> ")"
<expression>  ::= <term> | <term> "+" <expression> | <term> "-" <expression>
<term>        ::= <factor> | <factor> "*" <term> | <factor> "/" <term>
```

## Programming language

```html
<program>           ::= <statement_list>
<statement_list>    ::= <statement> | <statement> <statement_list>
<statement>         ::= <assignment> | <if_statement> | <while_statement> | <return_statement>
<assignment>        ::= <identifier> "=" <expression> ";"
<if_statement>      ::= "if" "(" <expression> ")" <block> [ "else" <block> ]
<while_statement>   ::= "while" "(" <expression> ")" <block>
<return_statement>  ::= "return" <expression> ";"
<block>             ::= "{" <statement_list> "}"
<expression>        ::= <term> | <term> "+" <expression> | <term> "-" <expression>
<term>              ::= <factor> | <factor> "*" <term> | <factor> "/" <term>
<factor>            ::= <identifier> | <number> | "(" <expression> ")"
```
