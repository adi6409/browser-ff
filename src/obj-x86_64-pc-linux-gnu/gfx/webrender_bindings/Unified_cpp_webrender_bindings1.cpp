#define MOZ_UNIFIED_BUILD
#include "/worker/build/gfx/webrender_bindings/RendererScreenshotGrabber.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/gfx/webrender_bindings/RendererScreenshotGrabber.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/gfx/webrender_bindings/RendererScreenshotGrabber.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/gfx/webrender_bindings/WebRenderAPI.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/gfx/webrender_bindings/WebRenderAPI.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/gfx/webrender_bindings/WebRenderAPI.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/gfx/webrender_bindings/WebRenderTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/gfx/webrender_bindings/WebRenderTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/gfx/webrender_bindings/WebRenderTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif