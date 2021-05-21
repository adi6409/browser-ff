#define MOZ_UNIFIED_BUILD
#include "/worker/build/intl/uconv/nsConverterInputStream.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/intl/uconv/nsConverterInputStream.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/intl/uconv/nsConverterInputStream.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/intl/uconv/nsConverterOutputStream.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/intl/uconv/nsConverterOutputStream.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/intl/uconv/nsConverterOutputStream.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/intl/uconv/nsScriptableUConv.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/intl/uconv/nsScriptableUConv.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/intl/uconv/nsScriptableUConv.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/intl/uconv/nsTextToSubURI.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/intl/uconv/nsTextToSubURI.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/intl/uconv/nsTextToSubURI.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif