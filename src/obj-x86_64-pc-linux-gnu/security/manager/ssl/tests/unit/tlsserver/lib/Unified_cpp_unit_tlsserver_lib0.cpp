#define MOZ_UNIFIED_BUILD
#include "/worker/build/security/manager/ssl/tests/unit/tlsserver/lib/OCSPCommon.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/manager/ssl/tests/unit/tlsserver/lib/OCSPCommon.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/manager/ssl/tests/unit/tlsserver/lib/OCSPCommon.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/security/manager/ssl/tests/unit/tlsserver/lib/TLSServer.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/manager/ssl/tests/unit/tlsserver/lib/TLSServer.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/manager/ssl/tests/unit/tlsserver/lib/TLSServer.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif