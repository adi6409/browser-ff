#define MOZ_UNIFIED_BUILD
#include "/worker/build/js/src/wasm/AsmJS.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/wasm/AsmJS.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/wasm/AsmJS.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/wasm/TypedObject.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/wasm/TypedObject.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/wasm/TypedObject.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/wasm/WasmBaselineCompile.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/wasm/WasmBaselineCompile.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/wasm/WasmBaselineCompile.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/wasm/WasmBuiltins.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/wasm/WasmBuiltins.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/wasm/WasmBuiltins.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/wasm/WasmCode.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/wasm/WasmCode.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/wasm/WasmCode.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/wasm/WasmCompile.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/wasm/WasmCompile.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/wasm/WasmCompile.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif