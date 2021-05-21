#define MOZ_UNIFIED_BUILD
#include "/worker/build/third_party/libsrtp/src/crypto/replay/ut_sim.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/third_party/libsrtp/src/crypto/replay/ut_sim.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/third_party/libsrtp/src/crypto/replay/ut_sim.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/third_party/libsrtp/src/srtp/ekt.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/third_party/libsrtp/src/srtp/ekt.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/third_party/libsrtp/src/srtp/ekt.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/third_party/libsrtp/src/srtp/srtp.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/third_party/libsrtp/src/srtp/srtp.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/third_party/libsrtp/src/srtp/srtp.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif