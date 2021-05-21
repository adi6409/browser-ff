/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRedirectResultListener.idl
 */

#ifndef __gen_nsIRedirectResultListener_h__
#define __gen_nsIRedirectResultListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIRedirectResultListener */
#define NS_IREDIRECTRESULTLISTENER_IID_STR "85cd2640-e91e-41ac-bdca-1dbf10dc131e"

#define NS_IREDIRECTRESULTLISTENER_IID \
  {0x85cd2640, 0xe91e, 0x41ac, \
    { 0xbd, 0xca, 0x1d, 0xbf, 0x10, 0xdc, 0x13, 0x1e }}

class NS_NO_VTABLE nsIRedirectResultListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREDIRECTRESULTLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIRedirectResultListener;

  /* void onRedirectResult (in boolean proceeding); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnRedirectResult(bool proceeding) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRedirectResultListener, NS_IREDIRECTRESULTLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREDIRECTRESULTLISTENER \
  NS_IMETHOD OnRedirectResult(bool proceeding) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREDIRECTRESULTLISTENER \
  nsresult OnRedirectResult(bool proceeding); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREDIRECTRESULTLISTENER(_to) \
  NS_IMETHOD OnRedirectResult(bool proceeding) override { return _to OnRedirectResult(proceeding); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREDIRECTRESULTLISTENER(_to) \
  NS_IMETHOD OnRedirectResult(bool proceeding) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnRedirectResult(proceeding); } 


#endif /* __gen_nsIRedirectResultListener_h__ */
