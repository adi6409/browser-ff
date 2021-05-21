#define MOZ_UNIFIED_BUILD
#include "/worker/build/modules/libjar/nsJAR.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libjar/nsJAR.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libjar/nsJAR.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/libjar/nsJARChannel.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libjar/nsJARChannel.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libjar/nsJARChannel.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/libjar/nsJARInputStream.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libjar/nsJARInputStream.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libjar/nsJARInputStream.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/libjar/nsJARProtocolHandler.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libjar/nsJARProtocolHandler.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libjar/nsJARProtocolHandler.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/libjar/nsJARURI.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libjar/nsJARURI.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libjar/nsJARURI.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/libjar/nsZipArchive.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libjar/nsZipArchive.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libjar/nsZipArchive.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif