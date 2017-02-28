
  var m = Module();
  var status = m._malloc(4);
  var buf = m._malloc(2048);

  return function(func) {
    if (func.length >= 2048) return null;
    m.stringToUTF8(func.substr(1), buf, 2048);
    var ret = m['_demangle'](buf, 0, 0, status);
    var result = null;
    if (m.HEAP32[status >> 2] === 0 && ret) {
      result = m.Pointer_stringify(ret);
      m._free(ret);
    }
    return result;
  };
})();

// The emscripten compiler exports the Module object; we just want
// the demangle function
if (typeof module === "object" && typeof module.exports === "object") {
  module.exports = demangle;
}
