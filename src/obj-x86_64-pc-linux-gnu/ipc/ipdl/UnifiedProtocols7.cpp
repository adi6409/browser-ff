#define MOZ_UNIFIED_BUILD
#include "PBackgroundSDBRequestParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundSDBRequestParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundSDBRequestParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundSessionStorageCache.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundSessionStorageCache.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundSessionStorageCache.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundSessionStorageCacheChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundSessionStorageCacheChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundSessionStorageCacheChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundSessionStorageCacheParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundSessionStorageCacheParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundSessionStorageCacheParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundSessionStorageManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundSessionStorageManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundSessionStorageManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundSessionStorageManagerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundSessionStorageManagerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundSessionStorageManagerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundSessionStorageManagerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundSessionStorageManagerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundSessionStorageManagerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundSharedTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundSharedTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundSharedTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundStorage.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundStorage.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundStorage.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundStorageChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundStorageChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundStorageChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundStorageParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundStorageParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundStorageParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundTest.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundTest.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundTest.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundTestChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundTestChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundTestChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundTestParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundTestParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundTestParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBenchmarkStorage.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBenchmarkStorage.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBenchmarkStorage.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBenchmarkStorageChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBenchmarkStorageChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBenchmarkStorageChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif