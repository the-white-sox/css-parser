# Grammar

## Limitations/Restrictions

### Colors

We support the 16 built-in level 1 CSS color keywords, in addition to the following:

- RGB
- HSL
- HEX

We do not support the CSS `color` function

### Pseudo Elements

We do not support using Pseudo Elements on rule selectors (example: `::before`)

### Browser Prefixing

We do not support using browser prefixes on rule selectors (example: `-webkit-appearance`)

### Grid and Flex

we are not supporting grid or flex related properties

### Whitespace

This document intentionally ignores whitespace.

### Operator Persistence and ambiguity

This document intentionally ignores operator persistence. This mean that this document describes an ambiguous grammar

## Utilities

```bnf
<digit> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
<digits> ::= <digit> <digits> | <digit>
<number> ::= <digits> | <digits> "." <digits>
<alphanumeric> ::= <letter> | <digit>
<alphanumerics> ::= <alphanumeric> | <alphanumeric> <alphanumerics>
<special-char> ::= ":" | "/" | "."
```

## Stylesheet

```bnf
<stylesheet> ::= <rule-list> <EOF>
<rule-list> ::= <rule> <rule-list> | <media-query> <rule-list> | ""
```

## Imports

```bnf
<import> ::= "@import" <url>
```

## Media Query

```bnf
<media-query> ::= "@media" <media-query-condition-list> "{" <rule-list> "}"
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

```css
@media print {
}

@media (color) {
}

@media (max-width: 1250px) {
}

@media (pointer: fine), (pointer: coarse) {
}

@media screen and (min-width: 30em) and (orientation: landscape) {
}

@media not print and (monochrome) {
}

@media (not (color)) or (hover) {
}
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

.class p
:not(a.class#id > :hover:not(a.class)) p

## Values

```bnf
<percentage> ::= <number> "%"
<length> ::= "0" | <number> <length-unit>
<length-unit> ::= "px" | "cm" | "in" | "pt" | "em" | "rem" | "vh" | "vw" | "vb" | "vi" | "vmin" | "vmax"
<url-string> ::= <alphanumerics> | <special-char>
<url> ::= "url(" <alphanumerics> ")"
```
