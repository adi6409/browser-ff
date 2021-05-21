#define MOZ_UNIFIED_BUILD
#include "/worker/build/modules/libjar/zipwriter/StreamFunctions.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libjar/zipwriter/StreamFunctions.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libjar/zipwriter/StreamFunctions.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/libjar/zipwriter/nsDeflateConverter.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libjar/zipwriter/nsDeflateConverter.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libjar/zipwriter/nsDeflateConverter.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/libjar/zipwriter/nsZipDataStream.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libjar/zipwriter/nsZipDataStream.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libjar/zipwriter/nsZipDataStream.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/libjar/zipwriter/nsZipHeader.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libjar/zipwriter/nsZipHeader.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libjar/zipwriter/nsZipHeader.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/libjar/zipwriter/nsZipWriter.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libjar/zipwriter/nsZipWriter.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libjar/zipwriter/nsZipWriter.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif