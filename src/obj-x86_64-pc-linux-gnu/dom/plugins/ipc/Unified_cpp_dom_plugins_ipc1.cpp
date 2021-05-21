#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/plugins/ipc/PluginScriptableObjectChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/plugins/ipc/PluginScriptableObjectChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/plugins/ipc/PluginScriptableObjectChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/plugins/ipc/PluginScriptableObjectParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/plugins/ipc/PluginScriptableObjectParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/plugins/ipc/PluginScriptableObjectParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif