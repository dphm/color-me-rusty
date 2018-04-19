(async function() {
  // Number of steps from red to green,
  // green to blue, blue to red
  const NUM_STEPS = 200

  // Fetch the WebAssembly file
  const wasm = await fetch("color-me-rusty.gc.wasm")

  // Read the raw binary data from the ArrayBuffer
  const bytes = await wasm.arrayBuffer()

  // Instantiate the WebAssembly module
  const module = await WebAssembly.instantiate(bytes, {
    // Import jsSetBackgroundColor into the WebAssembly env
    // so it can be used in Rust
    env: {
      jsSetBackgroundColor
    }
  })
  const mod = module.instance

  // Sets the document body background color.
  //
  // The color is specified by a pointer to an Array of
  // unsigned 8-bit integers of length 3, stored in the
  // linear memory of the WebAssembly module instance.
  function jsSetBackgroundColor(valsPtr) {
    let vals = new Uint8ClampedArray(mod.exports.memory.buffer, valsPtr, 3)
    let paddedVals = Array.from(vals).map(padValue)
    let rgb = `rgb(${paddedVals[0]}, ${paddedVals[1]}, ${paddedVals[2]})`
    document.body.style.backgroundColor = rgb
    document.getElementById('color').innerText = rgb
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

  // Calls set_background_color in a loop.
  function render(step) {
    if (!step) step = 0
    mod.exports.set_background_color(step, NUM_STEPS)
    window.requestAnimationFrame(() => render(step + 1))
  }

  // Begins the rendering loop.
  window.requestAnimationFrame(render)
})()
