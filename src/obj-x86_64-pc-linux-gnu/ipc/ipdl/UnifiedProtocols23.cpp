#define MOZ_UNIFIED_BUILD
#include "PPaymentRequest.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPaymentRequest.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPaymentRequest.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPaymentRequestChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPaymentRequestChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPaymentRequestChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPaymentRequestParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPaymentRequestParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPaymentRequestParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginBackgroundDestroyer.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginBackgroundDestroyer.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginBackgroundDestroyer.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginBackgroundDestroyerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginBackgroundDestroyerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginBackgroundDestroyerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginBackgroundDestroyerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginBackgroundDestroyerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginBackgroundDestroyerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginInstance.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginInstance.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginInstance.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginInstanceChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginInstanceChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginInstanceChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginInstanceParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginInstanceParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginInstanceParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginModule.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginModule.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginModule.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginModuleChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginModuleChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginModuleChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginModuleParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginModuleParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginModuleParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginScriptableObject.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginScriptableObject.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginScriptableObject.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginScriptableObjectChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginScriptableObjectChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginScriptableObjectChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginScriptableObjectParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginScriptableObjectParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginScriptableObjectParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginSurface.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginSurface.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginSurface.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif