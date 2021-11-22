const test = require('ava')

const { stringify, parse } = require('../index')

test('parse function from native code', (t) => {
  const svg_str = '<svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" version="1.1"></svg>';
  t.deepEqual(parse(svg_str), {
      type:'svg',
      attributes: {
          "xmlns":"http://www.w3.org/2000/svg",
          "width":"100%",
          "height":"100%",
          "version":"1.1"
      },
      children:[]
  })
})

test('stringify function from native code', async (t) => {
  t.is(stringify({
        type:'svg',
        attributes: {
            "xmlns":"http://www.w3.org/2000/svg",
            "width":"100%",
            "height":"100%",
            "version":"1.1"
        },
        children:[]
    }), '<svg height="100%" version="1.1" width="100%" xmlns="http://www.w3.org/2000/svg"/>')
})