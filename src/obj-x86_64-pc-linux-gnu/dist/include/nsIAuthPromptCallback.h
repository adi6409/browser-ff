/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthPromptCallback.idl
 */

#ifndef __gen_nsIAuthPromptCallback_h__
#define __gen_nsIAuthPromptCallback_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAuthInformation; /* forward declaration */


/* starting interface:    nsIAuthPromptCallback */
#define NS_IAUTHPROMPTCALLBACK_IID_STR "bdc387d7-2d29-4cac-92f1-dd75d786631d"

#define NS_IAUTHPROMPTCALLBACK_IID \
  {0xbdc387d7, 0x2d29, 0x4cac, \
    { 0x92, 0xf1, 0xdd, 0x75, 0xd7, 0x86, 0x63, 0x1d }}

class NS_NO_VTABLE nsIAuthPromptCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUTHPROMPTCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAuthPromptCallback;

  /* void onAuthAvailable (in nsISupports aContext, in nsIAuthInformation aAuthInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnAuthAvailable(nsISupports *aContext, nsIAuthInformation *aAuthInfo) = 0;

  /* void onAuthCancelled (in nsISupports aContext, in boolean userCancel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnAuthCancelled(nsISupports *aContext, bool userCancel) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAuthPromptCallback, NS_IAUTHPROMPTCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUTHPROMPTCALLBACK \
  NS_IMETHOD OnAuthAvailable(nsISupports *aContext, nsIAuthInformation *aAuthInfo) override; \
  NS_IMETHOD OnAuthCancelled(nsISupports *aContext, bool userCancel) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUTHPROMPTCALLBACK \
  nsresult OnAuthAvailable(nsISupports *aContext, nsIAuthInformation *aAuthInfo); \
  nsresult OnAuthCancelled(nsISupports *aContext, bool userCancel); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUTHPROMPTCALLBACK(_to) \
  NS_IMETHOD OnAuthAvailable(nsISupports *aContext, nsIAuthInformation *aAuthInfo) override { return _to OnAuthAvailable(aContext, aAuthInfo); } \
  NS_IMETHOD OnAuthCancelled(nsISupports *aContext, bool userCancel) override { return _to OnAuthCancelled(aContext, userCancel); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUTHPROMPTCALLBACK(_to) \
  NS_IMETHOD OnAuthAvailable(nsISupports *aContext, nsIAuthInformation *aAuthInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnAuthAvailable(aContext, aAuthInfo); } \
  NS_IMETHOD OnAuthCancelled(nsISupports *aContext, bool userCancel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnAuthCancelled(aContext, userCancel); } 


#endif /* __gen_nsIAuthPromptCallback_h__ */
