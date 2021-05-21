#define MOZ_UNIFIED_BUILD
#include "/worker/build/security/apps/AppSignatureVerification.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/apps/AppSignatureVerification.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/apps/AppSignatureVerification.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/security/apps/AppTrustDomain.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/apps/AppTrustDomain.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/apps/AppTrustDomain.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif