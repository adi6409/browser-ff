#define MOZ_UNIFIED_BUILD
#include "PWebGL.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebGL.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebGL.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebGLChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebGLChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebGLChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebGLParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebGLParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebGLParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebGPU.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebGPU.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebGPU.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebGPUChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebGPUChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebGPUChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebGPUParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebGPUParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebGPUParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebRenderBridge.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebRenderBridge.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebRenderBridge.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebRenderBridgeChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebRenderBridgeChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebRenderBridgeChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebRenderBridgeParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebRenderBridgeParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebRenderBridgeParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebSocket.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebSocket.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebSocket.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebSocketChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebSocketChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebSocketChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebSocketEventListener.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebSocketEventListener.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebSocketEventListener.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebSocketEventListenerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebSocketEventListenerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebSocketEventListenerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebSocketEventListenerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebSocketEventListenerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebSocketEventListenerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebSocketParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebSocketParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebSocketParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebrtcGlobal.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebrtcGlobal.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebrtcGlobal.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif