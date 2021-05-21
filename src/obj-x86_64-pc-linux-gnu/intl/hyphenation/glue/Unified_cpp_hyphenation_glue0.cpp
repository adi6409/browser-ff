#define MOZ_UNIFIED_BUILD
#include "/worker/build/intl/hyphenation/glue/nsHyphenationManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/intl/hyphenation/glue/nsHyphenationManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/intl/hyphenation/glue/nsHyphenationManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/intl/hyphenation/glue/nsHyphenator.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/intl/hyphenation/glue/nsHyphenator.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/intl/hyphenation/glue/nsHyphenator.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif