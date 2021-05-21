#define MOZ_UNIFIED_BUILD
#include "/worker/build/storage/mozStorageRow.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStorageRow.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStorageRow.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/mozStorageSQLFunctions.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStorageSQLFunctions.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStorageSQLFunctions.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/mozStorageService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStorageService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStorageService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/mozStorageStatement.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStorageStatement.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStorageStatement.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/mozStorageStatementJSHelper.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStorageStatementJSHelper.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStorageStatementJSHelper.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/mozStorageStatementParams.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStorageStatementParams.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStorageStatementParams.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/mozStorageStatementRow.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStorageStatementRow.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStorageStatementRow.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif