#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/promise/Promise.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/promise/Promise.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/promise/Promise.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/promise/PromiseDebugging.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/promise/PromiseDebugging.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/promise/PromiseDebugging.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/promise/PromiseNativeHandler.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/promise/PromiseNativeHandler.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/promise/PromiseNativeHandler.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif