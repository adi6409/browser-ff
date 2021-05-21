#define MOZ_UNIFIED_BUILD
#include "/worker/build/layout/base/nsLayoutUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/base/nsLayoutUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/base/nsLayoutUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/layout/base/nsPresArena.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/base/nsPresArena.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/base/nsPresArena.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/layout/base/nsPresContext.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/base/nsPresContext.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/base/nsPresContext.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/layout/base/nsQuoteList.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/base/nsQuoteList.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/base/nsQuoteList.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/layout/base/nsRefreshObservers.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/base/nsRefreshObservers.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/base/nsRefreshObservers.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/layout/base/nsStyleChangeList.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/base/nsStyleChangeList.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/base/nsStyleChangeList.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/layout/base/nsStyleSheetService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/base/nsStyleSheetService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/base/nsStyleSheetService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif