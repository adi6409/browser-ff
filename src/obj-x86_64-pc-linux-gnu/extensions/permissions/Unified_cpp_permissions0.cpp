#define MOZ_UNIFIED_BUILD
#include "/worker/build/extensions/permissions/Permission.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/permissions/Permission.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/permissions/Permission.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/extensions/permissions/PermissionDelegateHandler.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/permissions/PermissionDelegateHandler.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/permissions/PermissionDelegateHandler.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/extensions/permissions/PermissionManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/permissions/PermissionManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/permissions/PermissionManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif