#define MOZ_UNIFIED_BUILD
#include "/worker/build/modules/libmar/sign/mar_sign.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libmar/sign/mar_sign.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libmar/sign/mar_sign.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/libmar/sign/nss_secutil.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libmar/sign/nss_secutil.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libmar/sign/nss_secutil.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif