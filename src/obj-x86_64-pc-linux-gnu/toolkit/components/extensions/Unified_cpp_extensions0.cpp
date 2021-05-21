#define MOZ_UNIFIED_BUILD
#include "/worker/build/toolkit/components/extensions/ExtensionPolicyService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/components/extensions/ExtensionPolicyService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/components/extensions/ExtensionPolicyService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/components/extensions/MatchPattern.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/components/extensions/MatchPattern.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/components/extensions/MatchPattern.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/components/extensions/WebExtensionPolicy.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/components/extensions/WebExtensionPolicy.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/components/extensions/WebExtensionPolicy.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif