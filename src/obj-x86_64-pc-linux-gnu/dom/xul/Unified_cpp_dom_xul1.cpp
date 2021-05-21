#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/xul/nsXULPrototypeDocument.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/xul/nsXULPrototypeDocument.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/xul/nsXULPrototypeDocument.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/xul/nsXULSortService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/xul/nsXULSortService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/xul/nsXULSortService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif