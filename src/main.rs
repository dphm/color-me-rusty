mod color;

use color::Color;

extern {
    /// Sets the background color of an HTML document body.
    ///
    /// The color is specified by a pointer to a Vec<u8>
    /// containing [red, green, blue] values.
    fn jsSetBackgroundColor(valsPtr: *const u8);
}

/// Sets the background color of an HTML document based on the current step.
///
/// The color cycles linearly through red, green, and blue.
/// It takes `num_steps` steps to interpolate from red to green,
/// and `num_steps` steps to interpolate from green to blue.
///
/// When `step as f32 / num_steps as f32` is:
/// * `0.0`, then the color is red.
/// * `0.5`, then the color is between red and green.
/// * `1.0`, then the color is green.
/// * `1.5`, then the color is between green and blue.
/// * `2.0`, then the color is blue.
/// * `2.5`, then the color is between blue and red.
/// * `3.0`, then the color is red.
/// * ...
///
/// This function is available in JavaScript via the WebAssembly module instance exports.
///
/// # Examples
///
///```javascript
/// (async function() {
///   const wasm = await fetch('path/to/color-me-rusty.wasm')
///   const bytes = await wasm.arrayBuffer()
///   const module = await WebAssembly.instantiate(bytes, {
///     env: {
///       jsSetBackgroundColor
///     }
///   })
///
///   function jsSetBackgroundColor(valsPtr) {
///     let vals = new Uint8ClampedArray(module.instance.exports.memory.buffer, valsPtr, 3)
///     let rgb = `rgb(${vals[0]}, ${vals[1]}, ${vals[2]})`
///     document.body.style.backgroundColor = rgb
///   }
///
///   // Sets the document body background color to blue.
///   module.instance.exports.set_background_color(200, 100)
/// })()
/// ```
#[no_mangle]
pub extern fn set_background_color(step: u32, num_steps: u32) {
    let colors = vec![&Color::RED, &Color::GREEN, &Color::BLUE];
    let color  = Color::interpolate_linear(&colors, step, num_steps);
    unsafe { jsSetBackgroundColor(color.values().as_ptr()); }
}

fn main() {}
