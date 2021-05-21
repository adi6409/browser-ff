#define MOZ_UNIFIED_BUILD
#include "/worker/build/js/xpconnect/tests/components/native/xpctest_attributes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/tests/components/native/xpctest_attributes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/tests/components/native/xpctest_attributes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/tests/components/native/xpctest_cenums.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/tests/components/native/xpctest_cenums.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/tests/components/native/xpctest_cenums.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/tests/components/native/xpctest_module.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/tests/components/native/xpctest_module.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/tests/components/native/xpctest_module.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/tests/components/native/xpctest_params.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/tests/components/native/xpctest_params.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/tests/components/native/xpctest_params.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/tests/components/native/xpctest_returncode.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/tests/components/native/xpctest_returncode.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/tests/components/native/xpctest_returncode.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif