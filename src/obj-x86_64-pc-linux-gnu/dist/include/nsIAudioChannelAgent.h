/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/audiochannel/nsIAudioChannelAgent.idl
 */

#ifndef __gen_nsIAudioChannelAgent_h__
#define __gen_nsIAudioChannelAgent_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindow; /* forward declaration */

typedef uint32_t  nsSuspendedTypes;


/* starting interface:    nsISuspendedTypes */
#define NS_ISUSPENDEDTYPES_IID_STR "2822a840-f009-11e5-a837-0800200c9a66"

#define NS_ISUSPENDEDTYPES_IID \
  {0x2822a840, 0xf009, 0x11e5, \
    { 0xa8, 0x37, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 }}

class NS_NO_VTABLE nsISuspendedTypes : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISUSPENDEDTYPES_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISuspendedTypes;

  enum {
    NONE_SUSPENDED = 0U,
    SUSPENDED_BLOCK = 1U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISuspendedTypes, NS_ISUSPENDEDTYPES_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISUSPENDEDTYPES \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISUSPENDEDTYPES \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISUSPENDEDTYPES(_to) \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISUSPENDEDTYPES(_to) \


/* starting interface:    nsIAudioChannelAgentCallback */
#define NS_IAUDIOCHANNELAGENTCALLBACK_IID_STR "15c05894-408e-4798-b527-a8c32d9c5f8c"

#define NS_IAUDIOCHANNELAGENTCALLBACK_IID \
  {0x15c05894, 0x408e, 0x4798, \
    { 0xb5, 0x27, 0xa8, 0xc3, 0x2d, 0x9c, 0x5f, 0x8c }}

class NS_NO_VTABLE nsIAudioChannelAgentCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUDIOCHANNELAGENTCALLBACK_IID)

  /* void windowVolumeChanged (in float aVolume, in bool aMuted); */
  NS_IMETHOD WindowVolumeChanged(float aVolume, bool aMuted) = 0;

  /* void windowSuspendChanged (in uint32_t aSuspend); */
  NS_IMETHOD WindowSuspendChanged(uint32_t aSuspend) = 0;

  /* void windowAudioCaptureChanged (in bool aCapture); */
  NS_IMETHOD WindowAudioCaptureChanged(bool aCapture) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAudioChannelAgentCallback, NS_IAUDIOCHANNELAGENTCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUDIOCHANNELAGENTCALLBACK \
  NS_IMETHOD WindowVolumeChanged(float aVolume, bool aMuted) override; \
  NS_IMETHOD WindowSuspendChanged(uint32_t aSuspend) override; \
  NS_IMETHOD WindowAudioCaptureChanged(bool aCapture) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUDIOCHANNELAGENTCALLBACK \
  nsresult WindowVolumeChanged(float aVolume, bool aMuted); \
  nsresult WindowSuspendChanged(uint32_t aSuspend); \
  nsresult WindowAudioCaptureChanged(bool aCapture); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUDIOCHANNELAGENTCALLBACK(_to) \
  NS_IMETHOD WindowVolumeChanged(float aVolume, bool aMuted) override { return _to WindowVolumeChanged(aVolume, aMuted); } \
  NS_IMETHOD WindowSuspendChanged(uint32_t aSuspend) override { return _to WindowSuspendChanged(aSuspend); } \
  NS_IMETHOD WindowAudioCaptureChanged(bool aCapture) override { return _to WindowAudioCaptureChanged(aCapture); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUDIOCHANNELAGENTCALLBACK(_to) \
  NS_IMETHOD WindowVolumeChanged(float aVolume, bool aMuted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WindowVolumeChanged(aVolume, aMuted); } \
  NS_IMETHOD WindowSuspendChanged(uint32_t aSuspend) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WindowSuspendChanged(aSuspend); } \
  NS_IMETHOD WindowAudioCaptureChanged(bool aCapture) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WindowAudioCaptureChanged(aCapture); } 


/* starting interface:    nsIAudioChannelAgent */
#define NS_IAUDIOCHANNELAGENT_IID_STR "4d212770-5d7b-446f-9394-632e351d96ee"

#define NS_IAUDIOCHANNELAGENT_IID \
  {0x4d212770, 0x5d7b, 0x446f, \
    { 0x93, 0x94, 0x63, 0x2e, 0x35, 0x1d, 0x96, 0xee }}

class NS_NO_VTABLE nsIAudioChannelAgent : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUDIOCHANNELAGENT_IID)

  enum {
    AUDIO_AGENT_STATE_NORMAL = 0,
    AUDIO_AGENT_STATE_MUTED = 1,
    AUDIO_AGENT_STATE_FADED = 2
  };

  /* void init (in mozIDOMWindow window, in nsIAudioChannelAgentCallback callback); */
  NS_IMETHOD Init(mozIDOMWindow *window, nsIAudioChannelAgentCallback *callback) = 0;

  /* void initWithWeakCallback (in mozIDOMWindow window, in nsIAudioChannelAgentCallback callback); */
  NS_IMETHOD InitWithWeakCallback(mozIDOMWindow *window, nsIAudioChannelAgentCallback *callback) = 0;

  /* void notifyStartedPlaying (in uint8_t audible); */
  NS_IMETHOD NotifyStartedPlaying(uint8_t audible) = 0;

  /* void notifyStoppedPlaying (); */
  NS_IMETHOD NotifyStoppedPlaying(void) = 0;

  /* void notifyStartedAudible (in uint8_t audible, in uint32_t reason); */
  NS_IMETHOD NotifyStartedAudible(uint8_t audible, uint32_t reason) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAudioChannelAgent, NS_IAUDIOCHANNELAGENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUDIOCHANNELAGENT \
  NS_IMETHOD Init(mozIDOMWindow *window, nsIAudioChannelAgentCallback *callback) override; \
  NS_IMETHOD InitWithWeakCallback(mozIDOMWindow *window, nsIAudioChannelAgentCallback *callback) override; \
  NS_IMETHOD NotifyStartedPlaying(uint8_t audible) override; \
  NS_IMETHOD NotifyStoppedPlaying(void) override; \
  NS_IMETHOD NotifyStartedAudible(uint8_t audible, uint32_t reason) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUDIOCHANNELAGENT \
  nsresult Init(mozIDOMWindow *window, nsIAudioChannelAgentCallback *callback); \
  nsresult InitWithWeakCallback(mozIDOMWindow *window, nsIAudioChannelAgentCallback *callback); \
  nsresult NotifyStartedPlaying(uint8_t audible); \
  nsresult NotifyStoppedPlaying(void); \
  nsresult NotifyStartedAudible(uint8_t audible, uint32_t reason); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUDIOCHANNELAGENT(_to) \
  NS_IMETHOD Init(mozIDOMWindow *window, nsIAudioChannelAgentCallback *callback) override { return _to Init(window, callback); } \
  NS_IMETHOD InitWithWeakCallback(mozIDOMWindow *window, nsIAudioChannelAgentCallback *callback) override { return _to InitWithWeakCallback(window, callback); } \
  NS_IMETHOD NotifyStartedPlaying(uint8_t audible) override { return _to NotifyStartedPlaying(audible); } \
  NS_IMETHOD NotifyStoppedPlaying(void) override { return _to NotifyStoppedPlaying(); } \
  NS_IMETHOD NotifyStartedAudible(uint8_t audible, uint32_t reason) override { return _to NotifyStartedAudible(audible, reason); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUDIOCHANNELAGENT(_to) \
  NS_IMETHOD Init(mozIDOMWindow *window, nsIAudioChannelAgentCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(window, callback); } \
  NS_IMETHOD InitWithWeakCallback(mozIDOMWindow *window, nsIAudioChannelAgentCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithWeakCallback(window, callback); } \
  NS_IMETHOD NotifyStartedPlaying(uint8_t audible) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyStartedPlaying(audible); } \
  NS_IMETHOD NotifyStoppedPlaying(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyStoppedPlaying(); } \
  NS_IMETHOD NotifyStartedAudible(uint8_t audible, uint32_t reason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyStartedAudible(audible, reason); } 


#endif /* __gen_nsIAudioChannelAgent_h__ */
