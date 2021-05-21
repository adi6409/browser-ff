#define MOZ_UNIFIED_BUILD
#include "/worker/build/extensions/auth/nsAuthGSSAPI.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/auth/nsAuthGSSAPI.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/auth/nsAuthGSSAPI.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/extensions/auth/nsAuthSambaNTLM.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/auth/nsAuthSambaNTLM.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/auth/nsAuthSambaNTLM.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif