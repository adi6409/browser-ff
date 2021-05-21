#define MOZ_UNIFIED_BUILD
#include "/worker/build/security/manager/pki/nsNSSDialogHelper.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/manager/pki/nsNSSDialogHelper.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/manager/pki/nsNSSDialogHelper.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/security/manager/pki/nsNSSDialogs.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/manager/pki/nsNSSDialogs.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/manager/pki/nsNSSDialogs.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif