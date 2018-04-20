(async function() {
  const FRAMES_PER_COLOR = 100

  // Fetch the WebAssembly binary data.
  const wasm = await fetch('color-me-rusty.gc.wasm')
  const bytes = await wasm.arrayBuffer()

  // Instantiate the WebAssembly module from the binary data,
  // and import setColor into WebAssembly.
  const module = await WebAssembly.instantiate(bytes, { env: { setColor }})
  const mod = module.instance

  // Sets the background color and text to the rgb color values
  // in WebAssembly linear memory.
  function setColor(valsPtr) {
    let vals = new Uint8ClampedArray(mod.exports.memory.buffer, valsPtr, 3)
    setBackgroundColor(vals)
    setText(vals)
  }

  // Sets the document body background color.
  function setBackgroundColor(vals) {
    document.body.style.backgroundColor = rgbString(vals)
  }

  // Sets the color heading text with padded values for consistent size.
  function setText(vals) {
    let paddedVals = Array.from(vals).map(padValue)
    document.getElementById('color').innerText = rgbString(paddedVals)
  }

  // Interpolates rgb color values into a CSS-compatible string.
  function rgbString(vals) {
    return `rgb(${vals[0]}, ${vals[1]}, ${vals[2]})`
  }

  // Pads a number with leading zeroes (maximum 3 digits).
  function padValue(value) {
    if (typeof(value) !== 'number') {
      throw TypeError('Value must be a number!')
    }

    var chars = value.toString().split('')
    for (var i = chars.length; i < 3; i++) {
      chars.unshift('0')
    }
    return chars.join('')
  }

  // Sets a new color on every frame
  function render(frame) {
    if (!frame) frame = 0
    mod.exports.set_frame_color(frame, FRAMES_PER_COLOR)
    window.requestAnimationFrame(() => render(frame + 1))
  }

  // Begins the rendering loop.
  window.requestAnimationFrame(render)
})()
