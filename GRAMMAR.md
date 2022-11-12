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

### Operator Persistence and ambiguity

This document intentionally ignores operator persistence. This mean that this document describes an ambiguous grammar

### Shorthands

we are not supporting multi type shorthand properties like `font` and `border` but we are supporting shorthands for multiple sides like `border-color` and `margin`

## Utilities

```bnf
<digit> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
<digits> ::= <digit> <digits> | <digit>
<hex-digit> ::= <digit> | "a" | "b" | "c" | "d" | "e" | "f" | "A" | "B" | "C" | "D" | "E" | "F"
<hex-byte> ::= <hex-digit> <hex-digit>
<number> ::= <digits> | <digits> "." <digits>
<alphanumeric> ::= <letter> | <digit>
<alphanumerics> ::= <alphanumeric> | <alphanumeric> <alphanumerics>
<special-char> ::= ":" | "/" | "."
```

## Stylesheet

```bnf
<stylesheet> ::= <imports> <rules> <EOF>
<rules> ::= <rule> <rules> | <media-query> <rule> | ""
```

## Imports

```bnf
<imports> ::= <import> <imports> | ""
<import> ::= "@import" <url> ";" | "@import" <url> <media-query-condition-list> ";"
```

## Media Query

```bnf
<media-query> ::= "@media" <media-query-condition-list> "{" <rule> "}"
<media-query-condition-list> ::= <media-query-condition> "," <media-query-condition-list> | <media-query-condition>
<media-query-condition> ::= <media-type> | "(" <media-feature> ")" | "not" <media-query-condition> | <media-query-condition> "and" <media-query-condition> | <media-query-condition> "or" <media-query-condition> | "(" <media-query-condition> ")"
<media-type> ::= "all" | "screen" | "print"
<media-feature> ::= "color" | "monochrome"
<media-feature> ::= "width:" <length> | "min-width:" <length> | "max-width:" <length> | "height:" <length> | "min-height:" <length> | "max-height:" <length>
<media-feature> ::= "orientation:" <orientation-value> | "orientation:" <orientation-value>
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
<rule> ::= <selector-list> "{" <declaration-list> "}"
```

## Selectors

```bnf
<selector-list> ::= <selector> "," <selector-list> | <selector>
<selector> ::= <ciap-list> | <element-list>
<element-list> ::= <element> | <element-list> <ciap-list> | <ciap-list> <element-list> | <element-list> <element-list> | <element-list> ">" <element-list> | <element-list> "+" <element-list> | <element-list> "~" <element-list>
<ciap-list> ::= <class-list> | <id-list> | <attribute-list> | <pseudo-class-list>
<class-list> ::= "." <class> | <class-list> "." <class> | <class-list> " ." <class> | <ciap-list> "." <class>
<id-list> ::= "#" <id> | <ciap-list> "#" <id>
<attribute-list> ::= "[" <attribute> "]" | "[" <attribute> "=" <value> "]" | <ciap-list> "[" <attribute> "]" | <ciap-list>  "[" <attribute> "=" <value> "]"
<pseudo-class-list> ::= ":" <pseudo-class> |<ciap-list> ":" <pseudo-class>  | ":" <pseudo-class-other> "(" selector ")"
<element> ::= "div" | "body" | "p"
<attribute> ::= "target" | "title" | "lang" | "href"
<pseudo-class> ::= "focus" | "focus-within" | "focus-visible" | "hover" | "visited" | "default" | "active" | "target" | "root"
<pseudo-class-other> ::= "not" | "has"
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
<declaration> ::= <color-property> ":" <color> | <sides-color-property> ":"
<declaration> ::= <length-property> ":" <length-or-percentage> | <side-lengths-property> ":" <side-lengths>
<declaration> ::= "font-family" ":" <string>
<declaration> ::= "opacity" ":" <alpha>
<declaration> ::= "text-align" ":" <text-align-value>
<color-property> ::= "color" | "background-color"
<sides-color-property> ::= "border-color"
<length-property> ::= "font-size" | "height" | "width"
<side-lengths-property> ::= "margin" | "padding" | "border-width" | "border-radius"
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
<length-or-percentage> ::= <length> | <percentage>|
<percentage> ::= <number> "%"
<length> ::= "0" | <number> <length-unit>
<length-unit> ::= "px" | "cm" | "in" | "pt" | "em" | "rem" | "vh" | "vw" | "vb" | "vi" | "vmin" | "vmax"
<url-string> ::= <alphanumerics> | <special-char>
<url> ::= "url(" <alphanumerics> ")"
<sides-color> :== <color> | <color> <color> | <color> <color> <color> <color>
<color> ::= "black" | "silver" | "gray" | "grey" | "white" | "maroon" | "red" | "purple" | "fuchsia" | "green" | "lime" | "olive" | "yellow" | "navy" | "blue" | "teal" | "aqua" | <rgb> | <hsl> | <hex>
<rgb> ::= "rgb(" <0-255> "," <0-255> "," <0-255> ")" | "rgba(" <0-255> "," <0-255> "," <0-255> "," <alpha> ")"
<hex> ::= "#" <hex-byte> <hex-byte> <hex-byte> | "#" <hex-byte> <hex-byte> <hex-byte> <hex-byte> | "#" <hex-byte> <hex-digit> | "#" <hex-byte> <hex-byte>
<hsl> ::= "hsl(" <0-360> "," <0-100> "%," <0-100> "%)" | "hsla(" <0-360> "," <0-100> "%," <0-100> "%," <alpha> ")"
<alpha> ::= "0." <digits> | "." <digits> | 1 | 0
<text-align-value> ::= "left" | "right" | "center" | "justify"
```
