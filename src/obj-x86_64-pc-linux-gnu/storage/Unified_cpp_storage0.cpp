#define MOZ_UNIFIED_BUILD
#include "/worker/build/storage/FileSystemModule.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/FileSystemModule.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/FileSystemModule.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/ObfuscatingVFS.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/ObfuscatingVFS.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/ObfuscatingVFS.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/SQLCollations.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/SQLCollations.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/SQLCollations.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/StorageBaseStatementInternal.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/StorageBaseStatementInternal.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/StorageBaseStatementInternal.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/TelemetryVFS.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/TelemetryVFS.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/TelemetryVFS.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/VacuumManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/VacuumManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/VacuumManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/Variant.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/Variant.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/Variant.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/mozStorageArgValueArray.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStorageArgValueArray.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStorageArgValueArray.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/mozStorageAsyncStatement.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStorageAsyncStatement.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStorageAsyncStatement.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/mozStorageAsyncStatementExecution.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStorageAsyncStatementExecution.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStorageAsyncStatementExecution.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/mozStorageAsyncStatementJSHelper.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStorageAsyncStatementJSHelper.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStorageAsyncStatementJSHelper.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/mozStorageAsyncStatementParams.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStorageAsyncStatementParams.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStorageAsyncStatementParams.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/mozStorageBindingParamsArray.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStorageBindingParamsArray.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStorageBindingParamsArray.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/mozStorageError.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStorageError.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStorageError.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/mozStoragePrivateHelpers.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStoragePrivateHelpers.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStoragePrivateHelpers.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/storage/mozStorageResultSet.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/storage/mozStorageResultSet.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/storage/mozStorageResultSet.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif