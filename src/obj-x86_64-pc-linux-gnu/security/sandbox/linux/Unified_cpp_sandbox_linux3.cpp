#define MOZ_UNIFIED_BUILD
#include "/worker/build/security/sandbox/linux/Sandbox.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/sandbox/linux/Sandbox.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/sandbox/linux/Sandbox.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/security/sandbox/linux/SandboxBrokerClient.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/sandbox/linux/SandboxBrokerClient.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/sandbox/linux/SandboxBrokerClient.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/security/sandbox/linux/SandboxFilter.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/sandbox/linux/SandboxFilter.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/sandbox/linux/SandboxFilter.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/security/sandbox/linux/SandboxFilterUtil.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/sandbox/linux/SandboxFilterUtil.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/sandbox/linux/SandboxFilterUtil.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/security/sandbox/linux/SandboxHooks.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/sandbox/linux/SandboxHooks.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/sandbox/linux/SandboxHooks.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/security/sandbox/linux/SandboxInfo.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/sandbox/linux/SandboxInfo.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/sandbox/linux/SandboxInfo.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/security/sandbox/linux/SandboxLogging.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/sandbox/linux/SandboxLogging.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/sandbox/linux/SandboxLogging.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/security/sandbox/linux/SandboxOpenedFiles.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/sandbox/linux/SandboxOpenedFiles.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/sandbox/linux/SandboxOpenedFiles.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/security/sandbox/linux/SandboxReporterClient.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/sandbox/linux/SandboxReporterClient.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/sandbox/linux/SandboxReporterClient.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/security/sandbox/linux/broker/SandboxBrokerCommon.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/sandbox/linux/broker/SandboxBrokerCommon.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/sandbox/linux/broker/SandboxBrokerCommon.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif