(async function() {
  const imports = {
    env: {
      jsSetBackgroundColor: function(valsPtr) {
        let vals = new Uint8ClampedArray(mod.exports.memory.buffer, valsPtr, 3)
        let rgb = `rgb(${vals[0]}, ${vals[1]}, ${vals[2]})`
        document.body.style.backgroundColor = rgb
        document.getElementById('color').innerText = rgb
      }
    }
  }

  const wasm = await fetch("color-me-rusty.gc.wasm")
  const bytes = await wasm.arrayBuffer()
  const module = await WebAssembly.instantiate(bytes, imports)
  const mod = module.instance

  function step(frame) {
    if (!frame) frame = 0
    mod.exports.draw_frame(frame)
    window.requestAnimationFrame(() => step(frame + 1))
  }

  function padValue(value) {
    if (typeof(value) !== 'number') {
      throw TypeError('Value must be a number!')
    }
    if (value < 0 || value > 255) {
      throw RangeError('Value must be between 0 and 255!')
    }
    var chars = value.toString().split('')
    for (var i = chars.length; i < 3; i++) {
      chars.unshift('0')
    }
    return chars.join('')
  }

  window.requestAnimationFrame(step)
})()
