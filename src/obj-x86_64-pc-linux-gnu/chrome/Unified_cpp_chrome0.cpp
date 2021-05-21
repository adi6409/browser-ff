#define MOZ_UNIFIED_BUILD
#include "/worker/build/chrome/nsChromeProtocolHandler.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/chrome/nsChromeProtocolHandler.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/chrome/nsChromeProtocolHandler.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/chrome/nsChromeRegistry.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/chrome/nsChromeRegistry.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/chrome/nsChromeRegistry.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/chrome/nsChromeRegistryChrome.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/chrome/nsChromeRegistryChrome.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/chrome/nsChromeRegistryChrome.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/chrome/nsChromeRegistryContent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/chrome/nsChromeRegistryContent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/chrome/nsChromeRegistryContent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif