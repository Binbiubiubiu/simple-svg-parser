const { loadBinding } = require('@node-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'simple-svg-parser' is the name of native addon
 * the second arguments was decided by `napi.name` field in `package.json`
 * the third arguments was decided by `name` field in `package.json`
 * `loadBinding` helper will load `simple-svg-parser.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `@napi-rs/simple-svg-parser-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'simple-svg-parser', '@napi-rs/simple-svg-parser')
