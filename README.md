## What is curld?

Curld is a simple wrapper around curl. It allows you to save curl input so that It can be easily run later.

## Why?

I wanted to have some of the features that Postman and Insomnia have, but from the terminal.

## Use

`curld run -- -X POST httpbin.org/post -H "accept: application/json"`

### Template syntax
You can use templates to be prompted for input when you run a command. This is useful when running a saved command.

The syntax is `\${var}` or `'${var}'` (note that the $ must either be escaped or inside single quotes) where `var` is the name of the item you want to be prompted for.

Templates of the same name will only be prompted for once and filled in where appropriate.

`curld run -- -X '${POST}' httpbin.org/post -H "accept: application/json" -d '{"productId": 123456, "quantity": ${quantity}, "on_hand": ${quantity}}'`
