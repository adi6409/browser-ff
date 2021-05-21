#define MOZ_UNIFIED_BUILD
#include "/worker/build/startupcache/StartupCache.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/startupcache/StartupCache.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/startupcache/StartupCache.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/startupcache/StartupCacheInfo.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/startupcache/StartupCacheInfo.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/startupcache/StartupCacheInfo.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/startupcache/StartupCacheUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/startupcache/StartupCacheUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/startupcache/StartupCacheUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif