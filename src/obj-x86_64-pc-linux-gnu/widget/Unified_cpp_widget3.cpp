#define MOZ_UNIFIED_BUILD
#include "/worker/build/widget/nsUserIdleService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/widget/nsUserIdleService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/widget/nsUserIdleService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/widget/nsXPLookAndFeel.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/widget/nsXPLookAndFeel.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/widget/nsXPLookAndFeel.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif