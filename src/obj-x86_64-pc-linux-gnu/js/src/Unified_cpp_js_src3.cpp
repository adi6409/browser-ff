#define MOZ_UNIFIED_BUILD
#include "/worker/build/js/src/builtin/TestingFunctions.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/builtin/TestingFunctions.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/builtin/TestingFunctions.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/builtin/WeakMapObject.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/builtin/WeakMapObject.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/builtin/WeakMapObject.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/builtin/WeakRefObject.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/builtin/WeakRefObject.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/builtin/WeakRefObject.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/builtin/WeakSetObject.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/builtin/WeakSetObject.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/builtin/WeakSetObject.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/builtin/intl/Collator.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/builtin/intl/Collator.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/builtin/intl/Collator.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/builtin/intl/CommonFunctions.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/builtin/intl/CommonFunctions.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/builtin/intl/CommonFunctions.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif