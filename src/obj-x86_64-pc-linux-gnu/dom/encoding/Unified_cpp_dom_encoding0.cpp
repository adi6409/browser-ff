#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/encoding/TextDecoder.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/encoding/TextDecoder.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/encoding/TextDecoder.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/encoding/TextEncoder.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/encoding/TextEncoder.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/encoding/TextEncoder.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif