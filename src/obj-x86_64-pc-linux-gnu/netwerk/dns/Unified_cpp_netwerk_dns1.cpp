#define MOZ_UNIFIED_BUILD
#include "/worker/build/netwerk/dns/TRRServiceBase.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/dns/TRRServiceBase.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/dns/TRRServiceBase.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/netwerk/dns/TRRServiceChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/dns/TRRServiceChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/dns/TRRServiceChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/netwerk/dns/TRRServiceParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/dns/TRRServiceParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/dns/TRRServiceParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/netwerk/dns/nsDNSService2.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/dns/nsDNSService2.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/dns/nsDNSService2.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/netwerk/dns/nsIDNService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/dns/nsIDNService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/dns/nsIDNService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif