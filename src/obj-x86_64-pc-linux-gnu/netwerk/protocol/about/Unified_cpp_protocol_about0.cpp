#define MOZ_UNIFIED_BUILD
#include "/worker/build/netwerk/protocol/about/nsAboutBlank.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/protocol/about/nsAboutBlank.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/protocol/about/nsAboutBlank.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/netwerk/protocol/about/nsAboutCache.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/protocol/about/nsAboutCache.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/protocol/about/nsAboutCache.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/netwerk/protocol/about/nsAboutCacheEntry.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/protocol/about/nsAboutCacheEntry.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/protocol/about/nsAboutCacheEntry.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/netwerk/protocol/about/nsAboutProtocolHandler.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/protocol/about/nsAboutProtocolHandler.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/protocol/about/nsAboutProtocolHandler.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif