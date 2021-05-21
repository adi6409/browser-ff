#define MOZ_UNIFIED_BUILD
#include "/worker/build/image/imgLoader.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/image/imgLoader.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/image/imgLoader.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/image/imgRequest.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/image/imgRequest.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/image/imgRequest.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/image/imgRequestProxy.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/image/imgRequestProxy.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/image/imgRequestProxy.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/image/imgTools.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/image/imgTools.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/image/imgTools.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif