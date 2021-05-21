#define MOZ_UNIFIED_BUILD
#include "/worker/build/js/src/irregexp/RegExpAPI.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/irregexp/RegExpAPI.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/irregexp/RegExpAPI.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/irregexp/RegExpShim.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/irregexp/RegExpShim.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/irregexp/RegExpShim.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/irregexp/imported/property-sequences.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/irregexp/imported/property-sequences.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/irregexp/imported/property-sequences.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/irregexp/imported/regexp-ast.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/irregexp/imported/regexp-ast.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/irregexp/imported/regexp-ast.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/irregexp/imported/regexp-bytecode-generator.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/irregexp/imported/regexp-bytecode-generator.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/irregexp/imported/regexp-bytecode-generator.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/irregexp/imported/regexp-bytecode-peephole.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/irregexp/imported/regexp-bytecode-peephole.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/irregexp/imported/regexp-bytecode-peephole.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif