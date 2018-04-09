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

  window.requestAnimationFrame(step)
})()
