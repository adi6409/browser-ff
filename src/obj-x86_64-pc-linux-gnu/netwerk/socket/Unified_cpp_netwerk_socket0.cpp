#define MOZ_UNIFIED_BUILD
#include "/worker/build/netwerk/socket/nsSOCKSIOLayer.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/socket/nsSOCKSIOLayer.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/socket/nsSOCKSIOLayer.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/netwerk/socket/nsSOCKSSocketProvider.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/socket/nsSOCKSSocketProvider.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/socket/nsSOCKSSocketProvider.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/netwerk/socket/nsSocketProviderService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/socket/nsSocketProviderService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/socket/nsSocketProviderService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/netwerk/socket/nsUDPSocketProvider.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/socket/nsUDPSocketProvider.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/socket/nsUDPSocketProvider.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif