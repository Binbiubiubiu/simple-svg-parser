# @napi-rs/simple-svg-parser

a simple parser for svg

## Usage

``` sh
npm install @napi-rs/simple-svg-parser
```

code
``` js
const {parse,stringify} = require('@napi-rs/simple-svg-parser');

const svg_str = '<svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" version="1.1"></svg>';

let obj = parse(svg_str); // ouput ast object
stringify(obj); // output string 

```