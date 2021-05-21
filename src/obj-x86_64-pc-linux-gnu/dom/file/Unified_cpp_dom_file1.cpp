#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/file/StringBlobImpl.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/file/StringBlobImpl.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/file/StringBlobImpl.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/file/TemporaryFileBlobImpl.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/file/TemporaryFileBlobImpl.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/file/TemporaryFileBlobImpl.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif