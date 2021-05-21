#define MOZ_UNIFIED_BUILD
#include "XULElementBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "XULElementBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "XULElementBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "XULElementBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "XULFrameElementBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "XULFrameElementBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "XULFrameElementBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "XULFrameElementBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "XULMenuElementBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "XULMenuElementBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "XULMenuElementBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "XULMenuElementBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "XULPopupElementBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "XULPopupElementBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "XULPopupElementBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "XULPopupElementBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "XULTextElementBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "XULTextElementBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "XULTextElementBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "XULTextElementBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "XULTreeElementBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "XULTreeElementBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "XULTreeElementBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "XULTreeElementBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif