/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/webspeech/synth/nsISpeechService.idl
 */

#ifndef __gen_nsISpeechService_h__
#define __gen_nsISpeechService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsISpeechTaskCallback */
#define NS_ISPEECHTASKCALLBACK_IID_STR "c576de0c-8a3d-4570-be7e-9876d3e5bed2"

#define NS_ISPEECHTASKCALLBACK_IID \
  {0xc576de0c, 0x8a3d, 0x4570, \
    { 0xbe, 0x7e, 0x98, 0x76, 0xd3, 0xe5, 0xbe, 0xd2 }}

class NS_NO_VTABLE nsISpeechTaskCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISPEECHTASKCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISpeechTaskCallback;

  /* void onPause (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnPause(void) = 0;

  /* void onResume (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnResume(void) = 0;

  /* void onCancel (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnCancel(void) = 0;

  /* void onVolumeChanged (in float aVolume); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnVolumeChanged(float aVolume) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISpeechTaskCallback, NS_ISPEECHTASKCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISPEECHTASKCALLBACK \
  NS_IMETHOD OnPause(void) override; \
  NS_IMETHOD OnResume(void) override; \
  NS_IMETHOD OnCancel(void) override; \
  NS_IMETHOD OnVolumeChanged(float aVolume) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISPEECHTASKCALLBACK \
  nsresult OnPause(void); \
  nsresult OnResume(void); \
  nsresult OnCancel(void); \
  nsresult OnVolumeChanged(float aVolume); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISPEECHTASKCALLBACK(_to) \
  NS_IMETHOD OnPause(void) override { return _to OnPause(); } \
  NS_IMETHOD OnResume(void) override { return _to OnResume(); } \
  NS_IMETHOD OnCancel(void) override { return _to OnCancel(); } \
  NS_IMETHOD OnVolumeChanged(float aVolume) override { return _to OnVolumeChanged(aVolume); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISPEECHTASKCALLBACK(_to) \
  NS_IMETHOD OnPause(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnPause(); } \
  NS_IMETHOD OnResume(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnResume(); } \
  NS_IMETHOD OnCancel(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnCancel(); } \
  NS_IMETHOD OnVolumeChanged(float aVolume) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnVolumeChanged(aVolume); } 


/* starting interface:    nsISpeechTask */
#define NS_ISPEECHTASK_IID_STR "ad59949c-2437-4b35-8eeb-d760caab75c5"

#define NS_ISPEECHTASK_IID \
  {0xad59949c, 0x2437, 0x4b35, \
    { 0x8e, 0xeb, 0xd7, 0x60, 0xca, 0xab, 0x75, 0xc5 }}

class NS_NO_VTABLE nsISpeechTask : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISPEECHTASK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISpeechTask;

  /* void setup (in nsISpeechTaskCallback aCallback); */
  NS_IMETHOD Setup(nsISpeechTaskCallback *aCallback) = 0;

  /* void dispatchStart (); */
  NS_IMETHOD DispatchStart(void) = 0;

  /* void dispatchEnd (in float aElapsedTime, in unsigned long aCharIndex); */
  NS_IMETHOD DispatchEnd(float aElapsedTime, uint32_t aCharIndex) = 0;

  /* void dispatchPause (in float aElapsedTime, in unsigned long aCharIndex); */
  NS_IMETHOD DispatchPause(float aElapsedTime, uint32_t aCharIndex) = 0;

  /* void dispatchResume (in float aElapsedTime, in unsigned long aCharIndex); */
  NS_IMETHOD DispatchResume(float aElapsedTime, uint32_t aCharIndex) = 0;

  /* void dispatchError (in float aElapsedTime, in unsigned long aCharIndex); */
  NS_IMETHOD DispatchError(float aElapsedTime, uint32_t aCharIndex) = 0;

  /* [optional_argc] void dispatchBoundary (in AString aName, in float aElapsedTime, in unsigned long aCharIndex, [optional] in unsigned long aCharLength); */
  NS_IMETHOD DispatchBoundary(const nsAString& aName, float aElapsedTime, uint32_t aCharIndex, uint32_t aCharLength, uint8_t _argc) = 0;

  /* void dispatchMark (in AString aName, in float aElapsedTime, in unsigned long aCharIndex); */
  NS_IMETHOD DispatchMark(const nsAString& aName, float aElapsedTime, uint32_t aCharIndex) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISpeechTask, NS_ISPEECHTASK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISPEECHTASK \
  NS_IMETHOD Setup(nsISpeechTaskCallback *aCallback) override; \
  NS_IMETHOD DispatchStart(void) override; \
  NS_IMETHOD DispatchEnd(float aElapsedTime, uint32_t aCharIndex) override; \
  NS_IMETHOD DispatchPause(float aElapsedTime, uint32_t aCharIndex) override; \
  NS_IMETHOD DispatchResume(float aElapsedTime, uint32_t aCharIndex) override; \
  NS_IMETHOD DispatchError(float aElapsedTime, uint32_t aCharIndex) override; \
  NS_IMETHOD DispatchBoundary(const nsAString& aName, float aElapsedTime, uint32_t aCharIndex, uint32_t aCharLength, uint8_t _argc) override; \
  NS_IMETHOD DispatchMark(const nsAString& aName, float aElapsedTime, uint32_t aCharIndex) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISPEECHTASK \
  nsresult Setup(nsISpeechTaskCallback *aCallback); \
  nsresult DispatchStart(void); \
  nsresult DispatchEnd(float aElapsedTime, uint32_t aCharIndex); \
  nsresult DispatchPause(float aElapsedTime, uint32_t aCharIndex); \
  nsresult DispatchResume(float aElapsedTime, uint32_t aCharIndex); \
  nsresult DispatchError(float aElapsedTime, uint32_t aCharIndex); \
  nsresult DispatchBoundary(const nsAString& aName, float aElapsedTime, uint32_t aCharIndex, uint32_t aCharLength, uint8_t _argc); \
  nsresult DispatchMark(const nsAString& aName, float aElapsedTime, uint32_t aCharIndex); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISPEECHTASK(_to) \
  NS_IMETHOD Setup(nsISpeechTaskCallback *aCallback) override { return _to Setup(aCallback); } \
  NS_IMETHOD DispatchStart(void) override { return _to DispatchStart(); } \
  NS_IMETHOD DispatchEnd(float aElapsedTime, uint32_t aCharIndex) override { return _to DispatchEnd(aElapsedTime, aCharIndex); } \
  NS_IMETHOD DispatchPause(float aElapsedTime, uint32_t aCharIndex) override { return _to DispatchPause(aElapsedTime, aCharIndex); } \
  NS_IMETHOD DispatchResume(float aElapsedTime, uint32_t aCharIndex) override { return _to DispatchResume(aElapsedTime, aCharIndex); } \
  NS_IMETHOD DispatchError(float aElapsedTime, uint32_t aCharIndex) override { return _to DispatchError(aElapsedTime, aCharIndex); } \
  NS_IMETHOD DispatchBoundary(const nsAString& aName, float aElapsedTime, uint32_t aCharIndex, uint32_t aCharLength, uint8_t _argc) override { return _to DispatchBoundary(aName, aElapsedTime, aCharIndex, aCharLength, _argc); } \
  NS_IMETHOD DispatchMark(const nsAString& aName, float aElapsedTime, uint32_t aCharIndex) override { return _to DispatchMark(aName, aElapsedTime, aCharIndex); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISPEECHTASK(_to) \
  NS_IMETHOD Setup(nsISpeechTaskCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Setup(aCallback); } \
  NS_IMETHOD DispatchStart(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DispatchStart(); } \
  NS_IMETHOD DispatchEnd(float aElapsedTime, uint32_t aCharIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DispatchEnd(aElapsedTime, aCharIndex); } \
  NS_IMETHOD DispatchPause(float aElapsedTime, uint32_t aCharIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DispatchPause(aElapsedTime, aCharIndex); } \
  NS_IMETHOD DispatchResume(float aElapsedTime, uint32_t aCharIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DispatchResume(aElapsedTime, aCharIndex); } \
  NS_IMETHOD DispatchError(float aElapsedTime, uint32_t aCharIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DispatchError(aElapsedTime, aCharIndex); } \
  NS_IMETHOD DispatchBoundary(const nsAString& aName, float aElapsedTime, uint32_t aCharIndex, uint32_t aCharLength, uint8_t _argc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DispatchBoundary(aName, aElapsedTime, aCharIndex, aCharLength, _argc); } \
  NS_IMETHOD DispatchMark(const nsAString& aName, float aElapsedTime, uint32_t aCharIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DispatchMark(aName, aElapsedTime, aCharIndex); } 


/* starting interface:    nsISpeechService */
#define NS_ISPEECHSERVICE_IID_STR "9b7d59db-88ff-43d0-b6ee-9f63d042d08f"

#define NS_ISPEECHSERVICE_IID \
  {0x9b7d59db, 0x88ff, 0x43d0, \
    { 0xb6, 0xee, 0x9f, 0x63, 0xd0, 0x42, 0xd0, 0x8f }}

class NS_NO_VTABLE nsISpeechService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISPEECHSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISpeechService;

  /* void speak (in AString aText, in AString aUri, in float aVolume, in float aRate, in float aPitch, in nsISpeechTask aTask); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Speak(const nsAString& aText, const nsAString& aUri, float aVolume, float aRate, float aPitch, nsISpeechTask *aTask) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISpeechService, NS_ISPEECHSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISPEECHSERVICE \
  NS_IMETHOD Speak(const nsAString& aText, const nsAString& aUri, float aVolume, float aRate, float aPitch, nsISpeechTask *aTask) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISPEECHSERVICE \
  nsresult Speak(const nsAString& aText, const nsAString& aUri, float aVolume, float aRate, float aPitch, nsISpeechTask *aTask); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISPEECHSERVICE(_to) \
  NS_IMETHOD Speak(const nsAString& aText, const nsAString& aUri, float aVolume, float aRate, float aPitch, nsISpeechTask *aTask) override { return _to Speak(aText, aUri, aVolume, aRate, aPitch, aTask); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISPEECHSERVICE(_to) \
  NS_IMETHOD Speak(const nsAString& aText, const nsAString& aUri, float aVolume, float aRate, float aPitch, nsISpeechTask *aTask) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Speak(aText, aUri, aVolume, aRate, aPitch, aTask); } 

// This is the service category speech services could use to start up as
// a component.
#define NS_SPEECH_SYNTH_STARTED "speech-synth-started"

#endif /* __gen_nsISpeechService_h__ */
