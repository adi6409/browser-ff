/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIClipboardOwner.idl
 */

#ifndef __gen_nsIClipboardOwner_h__
#define __gen_nsIClipboardOwner_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsITransferable_h__
#include "nsITransferable.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIClipboardOwner */
#define NS_ICLIPBOARDOWNER_IID_STR "5a31c7a1-e122-11d2-9a57-000064657374"

#define NS_ICLIPBOARDOWNER_IID \
  {0x5a31c7a1, 0xe122, 0x11d2, \
    { 0x9a, 0x57, 0x00, 0x00, 0x64, 0x65, 0x73, 0x74 }}

class NS_NO_VTABLE nsIClipboardOwner : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICLIPBOARDOWNER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIClipboardOwner;

  /* void LosingOwnership (in nsITransferable aTransferable); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LosingOwnership(nsITransferable *aTransferable) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIClipboardOwner, NS_ICLIPBOARDOWNER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICLIPBOARDOWNER \
  NS_IMETHOD LosingOwnership(nsITransferable *aTransferable) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICLIPBOARDOWNER \
  nsresult LosingOwnership(nsITransferable *aTransferable); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICLIPBOARDOWNER(_to) \
  NS_IMETHOD LosingOwnership(nsITransferable *aTransferable) override { return _to LosingOwnership(aTransferable); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICLIPBOARDOWNER(_to) \
  NS_IMETHOD LosingOwnership(nsITransferable *aTransferable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LosingOwnership(aTransferable); } 



#endif /* __gen_nsIClipboardOwner_h__ */
