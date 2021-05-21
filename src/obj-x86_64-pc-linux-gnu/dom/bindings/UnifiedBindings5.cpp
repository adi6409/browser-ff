#define MOZ_UNIFIED_BUILD
#include "EventHandlerBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "EventHandlerBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "EventHandlerBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "EventHandlerBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "EventListenerBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "EventListenerBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "EventListenerBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "EventListenerBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "EventSourceBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "EventSourceBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "EventSourceBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "EventSourceBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "EventTargetBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "EventTargetBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "EventTargetBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "EventTargetBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "ExtendableEventBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "ExtendableEventBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "ExtendableEventBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "ExtendableEventBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "ExtendableMessageEventBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "ExtendableMessageEventBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "ExtendableMessageEventBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "ExtendableMessageEventBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "ExternalBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "ExternalBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "ExternalBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "ExternalBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FailedCertSecurityInfoBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FailedCertSecurityInfoBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FailedCertSecurityInfoBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FailedCertSecurityInfoBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FakePluginTagInitBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FakePluginTagInitBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FakePluginTagInitBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FakePluginTagInitBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FeaturePolicyBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FeaturePolicyBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FeaturePolicyBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FeaturePolicyBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FetchBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FetchBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FetchBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FetchBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FetchEventBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FetchEventBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FetchEventBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FetchEventBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FetchObserverBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FetchObserverBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FetchObserverBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FetchObserverBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FileBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FileBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FileBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FileBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FileListBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FileListBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FileListBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FileListBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FileModeBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FileModeBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FileModeBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FileModeBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FileReaderBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FileReaderBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FileReaderBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FileReaderBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FileReaderSyncBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FileReaderSyncBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FileReaderSyncBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FileReaderSyncBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FileSystemBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FileSystemBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FileSystemBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FileSystemBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FileSystemDirectoryEntryBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FileSystemDirectoryEntryBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FileSystemDirectoryEntryBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FileSystemDirectoryEntryBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FileSystemDirectoryReaderBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FileSystemDirectoryReaderBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FileSystemDirectoryReaderBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FileSystemDirectoryReaderBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FileSystemEntryBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FileSystemEntryBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FileSystemEntryBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FileSystemEntryBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FileSystemFileEntryBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FileSystemFileEntryBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FileSystemFileEntryBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FileSystemFileEntryBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FinalizationRegistryBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FinalizationRegistryBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FinalizationRegistryBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FinalizationRegistryBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FlexBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FlexBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FlexBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FlexBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FluentBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FluentBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FluentBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FluentBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FocusEventBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FocusEventBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FocusEventBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FocusEventBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FontFaceBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FontFaceBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FontFaceBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FontFaceBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FontFaceSetBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FontFaceSetBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FontFaceSetBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FontFaceSetBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FontFaceSetLoadEvent.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FontFaceSetLoadEvent.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FontFaceSetLoadEvent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FontFaceSetLoadEvent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FontFaceSetLoadEventBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FontFaceSetLoadEventBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FontFaceSetLoadEventBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FontFaceSetLoadEventBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FontFaceSourceBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "FontFaceSourceBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FontFaceSourceBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FontFaceSourceBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif