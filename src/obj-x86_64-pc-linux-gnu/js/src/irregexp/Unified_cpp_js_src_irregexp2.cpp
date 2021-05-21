#define MOZ_UNIFIED_BUILD
#include "/worker/build/js/src/irregexp/imported/regexp-parser.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/irregexp/imported/regexp-parser.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/irregexp/imported/regexp-parser.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/irregexp/imported/regexp-stack.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/irregexp/imported/regexp-stack.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/irregexp/imported/regexp-stack.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/irregexp/imported/special-case.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/irregexp/imported/special-case.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/irregexp/imported/special-case.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/irregexp/util/UnicodeShim.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/irregexp/util/UnicodeShim.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/irregexp/util/UnicodeShim.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif