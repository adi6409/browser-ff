#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/media/gmp/GMPVideoHost.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/gmp/GMPVideoHost.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/gmp/GMPVideoHost.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/gmp/GMPVideoPlaneImpl.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/gmp/GMPVideoPlaneImpl.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/gmp/GMPVideoPlaneImpl.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/gmp/GMPVideoi420FrameImpl.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/gmp/GMPVideoi420FrameImpl.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/gmp/GMPVideoi420FrameImpl.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif