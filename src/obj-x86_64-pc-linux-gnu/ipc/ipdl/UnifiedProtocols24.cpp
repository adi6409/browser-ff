#define MOZ_UNIFIED_BUILD
#include "PPluginSurfaceChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginSurfaceChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginSurfaceChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginSurfaceParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginSurfaceParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginSurfaceParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginWidget.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginWidget.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginWidget.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginWidgetChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginWidgetChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginWidgetChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPluginWidgetParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPluginWidgetParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPluginWidgetParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPresentation.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPresentation.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPresentation.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPresentationBuilder.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPresentationBuilder.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPresentationBuilder.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPresentationBuilderChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPresentationBuilderChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPresentationBuilderChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPresentationBuilderParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPresentationBuilderParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPresentationBuilderParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPresentationChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPresentationChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPresentationChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPresentationParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPresentationParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPresentationParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPresentationRequest.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPresentationRequest.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPresentationRequest.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPresentationRequestChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPresentationRequestChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPresentationRequestChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPresentationRequestParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPresentationRequestParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPresentationRequestParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPrintProgressDialog.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPrintProgressDialog.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPrintProgressDialog.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PPrintProgressDialogChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PPrintProgressDialogChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PPrintProgressDialogChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif