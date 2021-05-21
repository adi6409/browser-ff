#define MOZ_UNIFIED_BUILD
#include "/worker/build/layout/svg/SVGUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/svg/SVGUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/svg/SVGUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/layout/svg/SVGViewFrame.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/svg/SVGViewFrame.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/svg/SVGViewFrame.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/layout/svg/SVGViewportFrame.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/svg/SVGViewportFrame.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/svg/SVGViewportFrame.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif