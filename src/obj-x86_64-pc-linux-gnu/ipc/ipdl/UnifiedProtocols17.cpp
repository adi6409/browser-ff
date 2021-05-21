#define MOZ_UNIFIED_BUILD
#include "PGMPContentParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPContentParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPContentParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPServiceChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPServiceChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPServiceChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPServiceParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPServiceParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPServiceParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPStorage.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPStorage.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPStorage.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPStorageChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPStorageChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPStorageChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPStorageParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPStorageParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPStorageParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPTimer.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPTimer.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPTimer.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPTimerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPTimerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPTimerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPTimerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPTimerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPTimerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPVideoDecoder.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPVideoDecoder.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPVideoDecoder.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPVideoDecoderChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPVideoDecoderChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPVideoDecoderChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPVideoDecoderParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPVideoDecoderParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPVideoDecoderParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPVideoEncoder.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPVideoEncoder.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPVideoEncoder.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPVideoEncoderChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPVideoEncoderChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPVideoEncoderChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif