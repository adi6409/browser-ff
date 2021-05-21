#define MOZ_UNIFIED_BUILD
#include "/worker/build/netwerk/wifi/nsWifiAccessPoint.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/wifi/nsWifiAccessPoint.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/wifi/nsWifiAccessPoint.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/netwerk/wifi/nsWifiMonitor.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/wifi/nsWifiMonitor.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/wifi/nsWifiMonitor.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/netwerk/wifi/nsWifiScannerDBus.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/wifi/nsWifiScannerDBus.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/wifi/nsWifiScannerDBus.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif