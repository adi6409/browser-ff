#define MOZ_UNIFIED_BUILD
#include "/worker/build/netwerk/protocol/gio/GIOChannelChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/protocol/gio/GIOChannelChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/protocol/gio/GIOChannelChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/netwerk/protocol/gio/GIOChannelParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/protocol/gio/GIOChannelParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/protocol/gio/GIOChannelParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/netwerk/protocol/gio/nsGIOProtocolHandler.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/protocol/gio/nsGIOProtocolHandler.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/protocol/gio/nsGIOProtocolHandler.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif