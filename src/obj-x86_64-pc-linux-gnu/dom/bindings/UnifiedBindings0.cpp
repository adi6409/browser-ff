#define MOZ_UNIFIED_BUILD
#include "APZTestDataBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "APZTestDataBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "APZTestDataBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "APZTestDataBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AbortControllerBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AbortControllerBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AbortControllerBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AbortControllerBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AbortSignalBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AbortSignalBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AbortSignalBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AbortSignalBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AbstractRangeBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AbstractRangeBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AbstractRangeBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AbstractRangeBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AbstractWorkerBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AbstractWorkerBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AbstractWorkerBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AbstractWorkerBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AccessibilityRoleBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AccessibilityRoleBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AccessibilityRoleBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AccessibilityRoleBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AccessibleNodeBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AccessibleNodeBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AccessibleNodeBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AccessibleNodeBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AddonEvent.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AddonEvent.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AddonEvent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AddonEvent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AddonEventBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AddonEventBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AddonEventBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AddonEventBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AddonManagerBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AddonManagerBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AddonManagerBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AddonManagerBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AnalyserNodeBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AnalyserNodeBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AnalyserNodeBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AnalyserNodeBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AnimatableBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AnimatableBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AnimatableBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AnimatableBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AnimationBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AnimationBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AnimationBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AnimationBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AnimationEffectBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AnimationEffectBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AnimationEffectBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AnimationEffectBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AnimationEventBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AnimationEventBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AnimationEventBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AnimationEventBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AnimationPlaybackEvent.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AnimationPlaybackEvent.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AnimationPlaybackEvent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AnimationPlaybackEvent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AnimationPlaybackEventBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AnimationPlaybackEventBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AnimationPlaybackEventBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AnimationPlaybackEventBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AnimationTimelineBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AnimationTimelineBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AnimationTimelineBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AnimationTimelineBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AnonymousContentBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AnonymousContentBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AnonymousContentBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AnonymousContentBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AppInfoBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AppInfoBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AppInfoBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AppInfoBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AppNotificationServiceOptionsBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AppNotificationServiceOptionsBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AppNotificationServiceOptionsBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AppNotificationServiceOptionsBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AriaAttributesBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AriaAttributesBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AriaAttributesBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AriaAttributesBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AttrBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AttrBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AttrBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AttrBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AudioBufferBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AudioBufferBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AudioBufferBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AudioBufferBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AudioBufferSourceNodeBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AudioBufferSourceNodeBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AudioBufferSourceNodeBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AudioBufferSourceNodeBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AudioContextBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AudioContextBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AudioContextBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AudioContextBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AudioDestinationNodeBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AudioDestinationNodeBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AudioDestinationNodeBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AudioDestinationNodeBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AudioListenerBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AudioListenerBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AudioListenerBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AudioListenerBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AudioNodeBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AudioNodeBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AudioNodeBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AudioNodeBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AudioParamBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AudioParamBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AudioParamBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AudioParamBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AudioParamDescriptorBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AudioParamDescriptorBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AudioParamDescriptorBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AudioParamDescriptorBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "AudioParamMapBinding.cpp"
#if defined(_WINDOWS_) && !defined(MOZ_WRAPPED_WINDOWS_H)
#pragma message("wrapper failure reason: " MOZ_WINDOWS_WRAPPER_DISABLED_REASON)
#error "AudioParamMapBinding.cpp included unwrapped windows.h"
#endif
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "AudioParamMapBinding.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "AudioParamMapBinding.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif