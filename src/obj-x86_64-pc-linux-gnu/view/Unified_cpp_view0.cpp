#define MOZ_UNIFIED_BUILD
#include "/worker/build/view/nsView.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/view/nsView.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/view/nsView.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/view/nsViewManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/view/nsViewManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/view/nsViewManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif