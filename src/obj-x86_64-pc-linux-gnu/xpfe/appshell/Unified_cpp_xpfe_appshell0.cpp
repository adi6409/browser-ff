#define MOZ_UNIFIED_BUILD
#include "/worker/build/xpfe/appshell/AppWindow.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpfe/appshell/AppWindow.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpfe/appshell/AppWindow.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpfe/appshell/nsAppShellService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpfe/appshell/nsAppShellService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpfe/appshell/nsAppShellService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpfe/appshell/nsAppShellWindowEnumerator.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpfe/appshell/nsAppShellWindowEnumerator.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpfe/appshell/nsAppShellWindowEnumerator.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpfe/appshell/nsChromeTreeOwner.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpfe/appshell/nsChromeTreeOwner.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpfe/appshell/nsChromeTreeOwner.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpfe/appshell/nsContentTreeOwner.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpfe/appshell/nsContentTreeOwner.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpfe/appshell/nsContentTreeOwner.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpfe/appshell/nsWindowMediator.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpfe/appshell/nsWindowMediator.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpfe/appshell/nsWindowMediator.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif