/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsISound.idl
 */

#ifndef __gen_nsISound_h__
#define __gen_nsISound_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURL; /* forward declaration */


/* starting interface:    nsISound */
#define NS_ISOUND_IID_STR "c3c28d92-a17f-43df-976d-4eeae6f995fc"

#define NS_ISOUND_IID \
  {0xc3c28d92, 0xa17f, 0x43df, \
    { 0x97, 0x6d, 0x4e, 0xea, 0xe6, 0xf9, 0x95, 0xfc }}

class NS_NO_VTABLE nsISound : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISOUND_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISound;

  /* void play (in nsIURL aURL); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Play(nsIURL *aURL) = 0;

  /* void beep (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Beep(void) = 0;

  /* void init (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(void) = 0;

  enum {
    EVENT_NEW_MAIL_RECEIVED = 0U,
    EVENT_ALERT_DIALOG_OPEN = 1U,
    EVENT_CONFIRM_DIALOG_OPEN = 2U,
    EVENT_PROMPT_DIALOG_OPEN = 3U,
    EVENT_SELECT_DIALOG_OPEN = 4U,
    EVENT_MENU_EXECUTE = 5U,
    EVENT_MENU_POPUP = 6U,
    EVENT_EDITOR_MAX_LEN = 7U
  };

  /* void playEventSound (in unsigned long aEventId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PlayEventSound(uint32_t aEventId) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISound, NS_ISOUND_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISOUND \
  NS_IMETHOD Play(nsIURL *aURL) override; \
  NS_IMETHOD Beep(void) override; \
  NS_IMETHOD Init(void) override; \
  NS_IMETHOD PlayEventSound(uint32_t aEventId) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISOUND \
  nsresult Play(nsIURL *aURL); \
  nsresult Beep(void); \
  nsresult Init(void); \
  nsresult PlayEventSound(uint32_t aEventId); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISOUND(_to) \
  NS_IMETHOD Play(nsIURL *aURL) override { return _to Play(aURL); } \
  NS_IMETHOD Beep(void) override { return _to Beep(); } \
  NS_IMETHOD Init(void) override { return _to Init(); } \
  NS_IMETHOD PlayEventSound(uint32_t aEventId) override { return _to PlayEventSound(aEventId); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISOUND(_to) \
  NS_IMETHOD Play(nsIURL *aURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Play(aURL); } \
  NS_IMETHOD Beep(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Beep(); } \
  NS_IMETHOD Init(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(); } \
  NS_IMETHOD PlayEventSound(uint32_t aEventId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PlayEventSound(aEventId); } 


#endif /* __gen_nsISound_h__ */
