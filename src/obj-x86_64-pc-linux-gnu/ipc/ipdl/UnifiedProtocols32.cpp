#define MOZ_UNIFIED_BUILD
#include "PTemporaryIPCBlob.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PTemporaryIPCBlob.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PTemporaryIPCBlob.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PTemporaryIPCBlobChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PTemporaryIPCBlobChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PTemporaryIPCBlobChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PTemporaryIPCBlobParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PTemporaryIPCBlobParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PTemporaryIPCBlobParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PTestShell.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PTestShell.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PTestShell.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PTestShellChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PTestShellChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PTestShellChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PTestShellCommand.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PTestShellCommand.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PTestShellCommand.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PTestShellCommandChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PTestShellCommandChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PTestShellCommandChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PTestShellCommandParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PTestShellCommandParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PTestShellCommandParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PTestShellParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PTestShellParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PTestShellParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PTexture.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PTexture.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PTexture.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PTextureChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PTextureChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PTextureChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PTextureParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PTextureParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PTextureParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PTransportProvider.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PTransportProvider.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PTransportProvider.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PTransportProviderChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PTransportProviderChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PTransportProviderChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PTransportProviderParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PTransportProviderParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PTransportProviderParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PUDPSocket.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PUDPSocket.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PUDPSocket.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif