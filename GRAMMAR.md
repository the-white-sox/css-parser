# Grammar

## Limitations/Restrictions

### Colors

We support the 16 built-in level 1 CSS color keywords, in addition to the following:

- RGB
- HSL
- HEX

We do not support the CSS `color` function

### Pseudo Elements

We do not support using Pseudo Elements (example: `::before`)

### Browser Prefixing

We do not support using browser prefixes (example: `-webkit-appearance`)

### Grid and Flex

we are not supporting grid or flex related properties

### Whitespace

This document intentionally ignores whitespace.

### Case sensitivity

Although CSS is often case insensitive. This document is intentionally case sensitive.

### Operator Persistence and ambiguity

This document intentionally ignores operator persistence. This mean that this document describes an ambiguous grammar

### Shorthands

we are not supporting multi type shorthand properties like `font` and `border` but we are supporting shorthands for multiple sides like `border-color` and `margin`

### Things we might add later

these are features we don't currently have included but might include in the future.

- value functions `calc(max(100vh, 100%) - 64px)`
- more properties
- key frames
- font face declarations

## Tokens

```bnf
<digit> ::= "0" - "9"
<identifier-start-character> ::= "a" - "z" | "A" - "Z" | "_" | "-"
<identifier-character> ::= <identifier-start-character> | <digit>
<identifier-characters> ::= <identifier-character> <identifier-characters> | <identifier-character>
<identifier> ::= <identifier-start-character> <identifier-characters>
<function> ::= <identifier> "("
<at-keyword> ::= "@" <identifier>
<hash> ::= "#" <identifier-characters>
<string> ::= """ <string-characters> """ | "'" <string-characters> "'"
<url> ::= "url(" <url-characters> ")"
<digits> ::= <digit> <digits> | <digit>
<number> ::= <digits> | <digits> "." <digits> | "+" <digits> | "+" <digits> "." <digits> | "-" <digits> | "-" <digits> "." <digits>
<percentage> ::= <number> "%"
<dimension> ::= <number> <identifier>
<whitespace-character> ::= " " | "\t" | "\n" | "\r"
<whitespace> ::= <whitespace-character> <whitespace> | <whitespace-character>
```

## Stylesheet

```bnf
<stylesheet> ::= <imports> <rules> <EOF>
<rules> ::= <ruleset> <rules> | <media-query> <rules> | ""
```

## Imports

```bnf
<imports> ::= <import> <imports> | ""
<import> ::= "@import" <url> ";" | "@import" <url> <media-query-condition-list> ";"
```

## Media Query

```bnf
<media-query> ::= "@media" <media-query-condition-list> "{" <rules> "}"
<media-query-condition-list> ::= <media-query-condition> "," <media-query-condition-list> | <media-query-condition>
<media-query-condition> ::= <media-type> | "(" <media-feature> ")" | "not" <media-query-condition> | <media-query-condition> "and" <media-query-condition> | <media-query-condition> "or" <media-query-condition> | "(" <media-query-condition> ")"
<media-type> ::= "all" | "screen" | "print"
<media-feature> ::= "color" | "monochrome"
<media-feature> ::= "width:" <length> | "min-width:" <length> | "max-width:" <length> | "height:" <length> | "min-height:" <length> | "max-height:" <length>
<media-feature> ::= "orientation:" <orientation-value>
<media-feature> ::= "hover:" <hover-value> | "any-hover:" <hover-value> | "pointer:" <pointer-value> | "any-pointer:" <pointer-value>
<media-feature> ::= "prefers-color-scheme:" <prefers-color-scheme-value>
<orientation-value> ::= "portrait" | "landscape"
<hover-value> ::= "hover" | "none"
<pointer-value> ::= "fine" | "coarse" | "none"
<prefers-color-scheme-value> ::= "dark" | "light"
```

### Examples

<!-- prettier-ignore -->
```css
@media print {}
@media (color) {}
@media (max-width: 1250px) {}
@media (pointer: fine), (pointer: coarse) {}
@media screen and (min-width: 30em) and (orientation: landscape) {}
@media not print and (monochrome) {}
@media (not (color)) or (hover) {}
```

## Rules

```bnf
<ruleset> ::= <selector-list> "{" <declaration-list> "}"
```

## Selectors

```bnf
<selector-list> ::= <complex-selector> "," <selector-list> | <complex-selector>
<complex-selector> ::= <compound-selector> <combinator> <complex-selector> | <compound-selector>
<combinator> ::= <whitespace> | "+" | ">" | "~"
<compound-selector> ::= <element-selector> | <element-selector> <non-element-compound-selector> | <non-element-compound-selector>
<element-selector> ::= "*" | <identifier>
<non-element-compound-selector> ::= <non-element-simple-selector> <non-element-compound-selector> | <non-element-simple-selector>
<non-element-simple-selector> ::= <basic-selector> | <pseudo-class>
<basic-selector> ::= "." <identifier> | "#" <identifier> | <attribute-selector>
<attribute-selector> ::= "[" <identifier> "]" | "[" <identifier> "=" <string> "]" | "[" <identifier> "~=" <string> "]" | "[" <identifier> "|=" <string> "]" | "[" <identifier> "^=" <string> "]" | "[" <identifier> "$=" <string> "]" | "[" <identifier> "*=" <string> "]"
<pseudo-class> ::= ":focus" | ":focus-within" | ":focus-visible" | ":hover" | ":visited" | ":default" | ":active" | ":target" | ":root" | ":checked"
<pseudo-class> ::= ":not(" <complex-selector> ")" | ":has(" <combinator> <complex-selector> ")"
```

### Examples

<!-- prettier-ignore -->
```css
* {}
p {}
#id {}
.class {}
[title="title"] {}
:focus {}
.class p {}
:not(a.class#id > :hover:not(a.class)) p {}
p.class.class2#id[target].class:has(p.class) {}
```

## Declarations

```bnf
<declaration-list> ::= <declaration> ";" <declaration-list> | <declaration> | <declaration> ";"
<declaration> ::= <color-property> ":" <color> | <sides-color-property> ":" <sides-color>
<declaration> ::= <length-property> ":" <length-or-percentage> | <side-lengths-property> ":" <side-lengths>
<declaration> ::= "font-family" ":" <string> | <identifier> | <identifiers>
<declaration> ::= "opacity" ":" <alpha>
<declaration> ::= "text-align" ":" <text-align-value>
<declaration> ::= "display" ":" <display>
<color-property> ::= "color" | "background-color"
<sides-color-property> ::= "border-color"
<length-property> ::= "font-size" | "height" | "width"
<side-lengths-property> ::= "margin" | "padding" | "border-width" | "border-radius"
<identifiers> ::= <identifier> <identifiers> | <identifier>
<display> ::= "block" | "inline" | "inline-block"
```

### Examples

<!-- prettier-ignore -->
```css
* {
    text-align: center;
    color: red;
    background-color: blue;
    height: 100px;
    width: 100px;
    font-size: 18px;
    font-family: Arial;
    font-family: "Arial";
    opacity: 0.5;
    padding: 10px;
}
```

## Values

```bnf
<side-lengths> ::= <length-or-percentage> | <length-or-percentage> <length-or-percentage> | <length-or-percentage> <length-or-percentage> <length-or-percentage> <length-or-percentage>
<length-or-percentage> ::= <length> | <percentage>
<percentage> ::= <number> "%"
<length> ::= "0" | <number> <length-unit>
<length-unit> ::= "px" | "cm" | "in" | "pt" | "em" | "rem" | "vh" | "vw" | "vb" | "vi" | "vmin" | "vmax"
<sides-color> ::= <color> | <color> <color> | <color> <color> <color> <color>
<color> ::= "black" | "silver" | "gray" | "grey" | "white" | "maroon" | "red" | "purple" | "fuchsia" | "green" | "lime" | "olive" | "yellow" | "navy" | "blue" | "teal" | "aqua" | <rgb> | <hsl> | <hex>
<rgb> ::= "rgb(" <0-255> "," <0-255> "," <0-255> ")" | "rgba(" <0-255> "," <0-255> "," <0-255> "," <alpha> ")"
<hex> ::= "#" <hex-byte> <hex-byte> <hex-byte> | "#" <hex-byte> <hex-byte> <hex-byte> <hex-byte> | "#" <hex-digit> <hex-digit> <hex-digit> | "#" <hex-digit> <hex-digit> <hex-digit> <hex-digit>
<hex-digit> ::= <digit> | "a" - "f" | "A" - "F"
<hex-byte> ::= <hex-digit> <hex-digit>
<hsl> ::= "hsl(" <0-360> "," <0-100> "%," <0-100> "%)" | "hsla(" <0-360> "," <0-100> "%," <0-100> "%," <alpha> ")"
<alpha> ::= "0." <digits> | "." <digits> | 1 | 0
<text-align-value> ::= "left" | "right" | "center" | "justify"
```
