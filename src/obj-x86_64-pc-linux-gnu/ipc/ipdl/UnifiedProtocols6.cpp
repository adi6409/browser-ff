#define MOZ_UNIFIED_BUILD
#include "PBackgroundLSSimpleRequestParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundLSSimpleRequestParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundLSSimpleRequestParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundLSSnapshot.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundLSSnapshot.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundLSSnapshot.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundLSSnapshotChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundLSSnapshotChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundLSSnapshotChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundLSSnapshotParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundLSSnapshotParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundLSSnapshotParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundLocalStorageCache.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundLocalStorageCache.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundLocalStorageCache.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundLocalStorageCacheChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundLocalStorageCacheChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundLocalStorageCacheChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundLocalStorageCacheParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundLocalStorageCacheParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundLocalStorageCacheParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundMutableFile.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundMutableFile.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundMutableFile.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundMutableFileChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundMutableFileChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundMutableFileChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundMutableFileParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundMutableFileParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundMutableFileParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundSDBConnection.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundSDBConnection.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundSDBConnection.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundSDBConnectionChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundSDBConnectionChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundSDBConnectionChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundSDBConnectionParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundSDBConnectionParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundSDBConnectionParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundSDBRequest.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundSDBRequest.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundSDBRequest.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundSDBRequestChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundSDBRequestChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundSDBRequestChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif