#define MOZ_UNIFIED_BUILD
#include "/worker/build/ipc/glue/IPCStreamParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/IPCStreamParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/IPCStreamParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/IPCStreamSource.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/IPCStreamSource.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/IPCStreamSource.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/IPCStreamUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/IPCStreamUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/IPCStreamUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/IdleSchedulerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/IdleSchedulerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/IdleSchedulerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/IdleSchedulerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/IdleSchedulerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/IdleSchedulerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/InputStreamUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/InputStreamUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/InputStreamUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/LibrarySandboxPreload.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/LibrarySandboxPreload.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/LibrarySandboxPreload.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/MessageChannel.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/MessageChannel.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/MessageChannel.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/MessageLink.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/MessageLink.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/MessageLink.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/MessagePump.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/MessagePump.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/MessagePump.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/MiniTransceiver.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/MiniTransceiver.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/MiniTransceiver.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/ProcessChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/ProcessChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/ProcessChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/ProcessUtils_common.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/ProcessUtils_common.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/ProcessUtils_common.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/ProcessUtils_linux.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/ProcessUtils_linux.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/ProcessUtils_linux.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/ProtocolUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/ProtocolUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/ProtocolUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/ScopedXREEmbed.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/ScopedXREEmbed.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/ScopedXREEmbed.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif