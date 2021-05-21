/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsITokenDialogs.idl
 */

#ifndef __gen_nsITokenDialogs_h__
#define __gen_nsITokenDialogs_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInterfaceRequestor; /* forward declaration */

class nsIProtectedAuthThread; /* forward declaration */


/* starting interface:    nsITokenDialogs */
#define NS_ITOKENDIALOGS_IID_STR "a1cbc159-468c-495d-8068-61dd538cbcca"

#define NS_ITOKENDIALOGS_IID \
  {0xa1cbc159, 0x468c, 0x495d, \
    { 0x80, 0x68, 0x61, 0xdd, 0x53, 0x8c, 0xbc, 0xca }}

class NS_NO_VTABLE nsITokenDialogs : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITOKENDIALOGS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITokenDialogs;

  /* [must_use] void displayProtectedAuth (in nsIInterfaceRequestor ctx, in nsIProtectedAuthThread runnable); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD DisplayProtectedAuth(nsIInterfaceRequestor *ctx, nsIProtectedAuthThread *runnable) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITokenDialogs, NS_ITOKENDIALOGS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITOKENDIALOGS \
  [[nodiscard]] NS_IMETHOD DisplayProtectedAuth(nsIInterfaceRequestor *ctx, nsIProtectedAuthThread *runnable) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITOKENDIALOGS \
  [[nodiscard]] nsresult DisplayProtectedAuth(nsIInterfaceRequestor *ctx, nsIProtectedAuthThread *runnable); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITOKENDIALOGS(_to) \
  [[nodiscard]] NS_IMETHOD DisplayProtectedAuth(nsIInterfaceRequestor *ctx, nsIProtectedAuthThread *runnable) override { return _to DisplayProtectedAuth(ctx, runnable); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITOKENDIALOGS(_to) \
  [[nodiscard]] NS_IMETHOD DisplayProtectedAuth(nsIInterfaceRequestor *ctx, nsIProtectedAuthThread *runnable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DisplayProtectedAuth(ctx, runnable); } 

#define NS_TOKENDIALOGS_CONTRACTID "@mozilla.org/nsTokenDialogs;1"

#endif /* __gen_nsITokenDialogs_h__ */
