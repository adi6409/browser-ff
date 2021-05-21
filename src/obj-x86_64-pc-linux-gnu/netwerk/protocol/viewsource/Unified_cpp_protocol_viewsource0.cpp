#define MOZ_UNIFIED_BUILD
#include "/worker/build/netwerk/protocol/viewsource/nsViewSourceChannel.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/protocol/viewsource/nsViewSourceChannel.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/protocol/viewsource/nsViewSourceChannel.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/netwerk/protocol/viewsource/nsViewSourceHandler.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/protocol/viewsource/nsViewSourceHandler.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/protocol/viewsource/nsViewSourceHandler.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif