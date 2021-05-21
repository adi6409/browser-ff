#define MOZ_UNIFIED_BUILD
#include "/worker/build/security/sandbox/linux/SandboxBrokerClient.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/sandbox/linux/SandboxBrokerClient.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/sandbox/linux/SandboxBrokerClient.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/security/sandbox/linux/gtest/TestBroker.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/sandbox/linux/gtest/TestBroker.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/sandbox/linux/gtest/TestBroker.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/security/sandbox/linux/gtest/TestBrokerPolicy.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/sandbox/linux/gtest/TestBrokerPolicy.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/sandbox/linux/gtest/TestBrokerPolicy.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif