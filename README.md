# bigdecimal-rs-pow

Code snippets for power and root operations on bigdecimal types.

## Disclaimer

### Only use this code if you need something that works **NOT** something highly performant or "bulletproof"

I do not understand/have not reviewed most of the math done with this algorithm and cannot vouch for its efficiency in any way. Portions of the algorithm I came up with myself I also cannot guarantee will function efficiently or performantly.

There may be unforseen ways this code could break.

## Usage

Take a look at the [source](./src/main.rs). It should be relatively easy to understand how to use.

If you are handling negative powers or roots you will need to add your own handling for them. My code only supports positive numbers (including zero).

## License

This code is dual-licensed under the permissive [MIT](https://opensource.org/licenses/MIT) & [Apache 2.0](https://opensource.org/licenses/Apache-2.0) licenses.

This is the same as bigdecimal-rs.

Any project using bigdecimal-rs may also use this code.