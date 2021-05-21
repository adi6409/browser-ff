#define MOZ_UNIFIED_BUILD
#include "/worker/build/js/src/jit/ValueNumbering.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/jit/ValueNumbering.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/jit/ValueNumbering.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/jit/WarpBuilder.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/jit/WarpBuilder.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/jit/WarpBuilder.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/jit/WarpBuilderShared.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/jit/WarpBuilderShared.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/jit/WarpBuilderShared.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/jit/WarpCacheIRTranspiler.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/jit/WarpCacheIRTranspiler.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/jit/WarpCacheIRTranspiler.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/jit/WarpOracle.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/jit/WarpOracle.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/jit/WarpOracle.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/jit/WarpSnapshot.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/jit/WarpSnapshot.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/jit/WarpSnapshot.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif