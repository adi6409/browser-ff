#define MOZ_UNIFIED_BUILD
#include "/worker/build/js/src/frontend/SwitchEmitter.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/frontend/SwitchEmitter.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/frontend/SwitchEmitter.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/frontend/TDZCheckCache.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/frontend/TDZCheckCache.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/frontend/TDZCheckCache.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/frontend/TokenStream.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/frontend/TokenStream.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/frontend/TokenStream.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/frontend/TryEmitter.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/frontend/TryEmitter.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/frontend/TryEmitter.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/frontend/WhileEmitter.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/frontend/WhileEmitter.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/frontend/WhileEmitter.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif