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

  var n = 0
  function step() {
    mod.exports.draw_frame(n++)
    window.requestAnimationFrame(step)
  }

  window.requestAnimationFrame(step)
})()
