#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/presentation/provider/DeviceProviderHelpers.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/presentation/provider/DeviceProviderHelpers.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/presentation/provider/DeviceProviderHelpers.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/presentation/provider/MulticastDNSDeviceProvider.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/presentation/provider/MulticastDNSDeviceProvider.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/presentation/provider/MulticastDNSDeviceProvider.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif