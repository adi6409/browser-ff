/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIDocumentStateListener.idl
 */

#ifndef __gen_nsIDocumentStateListener_h__
#define __gen_nsIDocumentStateListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIDocumentStateListener */
#define NS_IDOCUMENTSTATELISTENER_IID_STR "050cdc00-3b8e-11d3-9ce4-a458f454fcbc"

#define NS_IDOCUMENTSTATELISTENER_IID \
  {0x050cdc00, 0x3b8e, 0x11d3, \
    { 0x9c, 0xe4, 0xa4, 0x58, 0xf4, 0x54, 0xfc, 0xbc }}

class NS_NO_VTABLE nsIDocumentStateListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOCUMENTSTATELISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDocumentStateListener;

  /* [can_run_script] void NotifyDocumentWillBeDestroyed (); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyDocumentWillBeDestroyed(void) = 0;

  /* [can_run_script] void NotifyDocumentStateChanged (in boolean aNowDirty); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyDocumentStateChanged(bool aNowDirty) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDocumentStateListener, NS_IDOCUMENTSTATELISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOCUMENTSTATELISTENER \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyDocumentWillBeDestroyed(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyDocumentStateChanged(bool aNowDirty) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOCUMENTSTATELISTENER \
  MOZ_CAN_RUN_SCRIPT nsresult NotifyDocumentWillBeDestroyed(void); \
  MOZ_CAN_RUN_SCRIPT nsresult NotifyDocumentStateChanged(bool aNowDirty); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOCUMENTSTATELISTENER(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyDocumentWillBeDestroyed(void) override { return _to NotifyDocumentWillBeDestroyed(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyDocumentStateChanged(bool aNowDirty) override { return _to NotifyDocumentStateChanged(aNowDirty); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOCUMENTSTATELISTENER(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyDocumentWillBeDestroyed(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyDocumentWillBeDestroyed(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyDocumentStateChanged(bool aNowDirty) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyDocumentStateChanged(aNowDirty); } 


#endif /* __gen_nsIDocumentStateListener_h__ */
