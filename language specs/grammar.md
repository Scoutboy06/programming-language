# Grammar

## Numerical evaluation

```php
<factor>      ::= <number> | "(" <expression> ")"
<expression>  ::= <term> | <term> "+" <expression> | <term> "-" <expression>
<term>        ::= <factor> | <factor> "*" <term> | <factor> "/" <term>
```

## Programming language

### Specs

<!-- ```php
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
``` -->

### Variables

```ts
var global_variable = 123;
let scope_variable = 456;
const constant = 789;
```

### Comments

```ts
// Single-line comment

/*
Multi-
line
comment
*/
```

### Types

```ts
let num: number = 123;
let str: string = 'abc';
let bool: boolean = true;
let nothing: null = null;

let arr1: Array = [];
let arr2: string[] = [];

let obj1: Object = { a: 'b' };
let obj2: Record<string, string> = { c: 'd' };
```

### Conditionals

```ts
if (condition) {
  // Do something
} else if (other_condition) {
  // Do something else
} else {
  // Do something different
}
```

### Functions

```ts
function sum(a: number, b: number): number {
  return a + b;
}

const sum = (a: number, b: number): number => a + b;
```

### Loops

```ts
while(conditional) { ... }
```

```ts
for(let i = 0; i < 5; i++) { ... }
```

```ts
for(let i in arr) { ... }
```

```ts
for(let item of arr) { ... }
```
