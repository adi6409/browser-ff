#define MOZ_UNIFIED_BUILD
#include "/worker/build/toolkit/components/ctypes/tests/jsctypes-test-errno.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/components/ctypes/tests/jsctypes-test-errno.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/components/ctypes/tests/jsctypes-test-errno.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/components/ctypes/tests/jsctypes-test-finalizer.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/components/ctypes/tests/jsctypes-test-finalizer.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/components/ctypes/tests/jsctypes-test-finalizer.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/components/ctypes/tests/jsctypes-test.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/components/ctypes/tests/jsctypes-test.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/components/ctypes/tests/jsctypes-test.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif