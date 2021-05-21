#define MOZ_UNIFIED_BUILD
#include "PRemoteDecoderManagerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteDecoderManagerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteDecoderManagerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteDecoderManagerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteDecoderManagerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteDecoderManagerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteDecoderParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteDecoderParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteDecoderParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteLazyInputStream.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteLazyInputStream.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteLazyInputStream.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteLazyInputStreamChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteLazyInputStreamChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteLazyInputStreamChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteLazyInputStreamParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteLazyInputStreamParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteLazyInputStreamParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemotePrintJob.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemotePrintJob.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemotePrintJob.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemotePrintJobChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemotePrintJobChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemotePrintJobChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemotePrintJobParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemotePrintJobParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemotePrintJobParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteSpellcheckEngine.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteSpellcheckEngine.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteSpellcheckEngine.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteSpellcheckEngineChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteSpellcheckEngineChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteSpellcheckEngineChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteSpellcheckEngineParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteSpellcheckEngineParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteSpellcheckEngineParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteWorker.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteWorker.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteWorker.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteWorkerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteWorkerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteWorkerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteWorkerController.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteWorkerController.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteWorkerController.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteWorkerControllerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteWorkerControllerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteWorkerControllerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif