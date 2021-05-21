/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheEntryDoomCallback.idl
 */

#ifndef __gen_nsICacheEntryDoomCallback_h__
#define __gen_nsICacheEntryDoomCallback_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsICacheEntryDoomCallback */
#define NS_ICACHEENTRYDOOMCALLBACK_IID_STR "2f8896be-232f-4140-afb3-1faffb56f3c6"

#define NS_ICACHEENTRYDOOMCALLBACK_IID \
  {0x2f8896be, 0x232f, 0x4140, \
    { 0xaf, 0xb3, 0x1f, 0xaf, 0xfb, 0x56, 0xf3, 0xc6 }}

class NS_NO_VTABLE nsICacheEntryDoomCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHEENTRYDOOMCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheEntryDoomCallback;

  /* void onCacheEntryDoomed (in nsresult aResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnCacheEntryDoomed(nsresult aResult) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheEntryDoomCallback, NS_ICACHEENTRYDOOMCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHEENTRYDOOMCALLBACK \
  NS_IMETHOD OnCacheEntryDoomed(nsresult aResult) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHEENTRYDOOMCALLBACK \
  nsresult OnCacheEntryDoomed(nsresult aResult); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHEENTRYDOOMCALLBACK(_to) \
  NS_IMETHOD OnCacheEntryDoomed(nsresult aResult) override { return _to OnCacheEntryDoomed(aResult); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHEENTRYDOOMCALLBACK(_to) \
  NS_IMETHOD OnCacheEntryDoomed(nsresult aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnCacheEntryDoomed(aResult); } 


#endif /* __gen_nsICacheEntryDoomCallback_h__ */
