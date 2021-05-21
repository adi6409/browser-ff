#define MOZ_UNIFIED_BUILD
#include "/worker/build/uriloader/preload/FetchPreloader.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/uriloader/preload/FetchPreloader.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/uriloader/preload/FetchPreloader.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/uriloader/preload/PreloadHashKey.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/uriloader/preload/PreloadHashKey.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/uriloader/preload/PreloadHashKey.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/uriloader/preload/PreloadService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/uriloader/preload/PreloadService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/uriloader/preload/PreloadService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/uriloader/preload/PreloaderBase.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/uriloader/preload/PreloaderBase.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/uriloader/preload/PreloaderBase.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif