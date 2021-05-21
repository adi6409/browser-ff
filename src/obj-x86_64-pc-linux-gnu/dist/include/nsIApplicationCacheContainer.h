/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIApplicationCacheContainer.idl
 */

#ifndef __gen_nsIApplicationCacheContainer_h__
#define __gen_nsIApplicationCacheContainer_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIApplicationCache; /* forward declaration */


/* starting interface:    nsIApplicationCacheContainer */
#define NS_IAPPLICATIONCACHECONTAINER_IID_STR "bbb80700-1f7f-4258-aff4-1743cc5a7d23"

#define NS_IAPPLICATIONCACHECONTAINER_IID \
  {0xbbb80700, 0x1f7f, 0x4258, \
    { 0xaf, 0xf4, 0x17, 0x43, 0xcc, 0x5a, 0x7d, 0x23 }}

class NS_NO_VTABLE nsIApplicationCacheContainer : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAPPLICATIONCACHECONTAINER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIApplicationCacheContainer;

  /* attribute nsIApplicationCache applicationCache; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetApplicationCache(nsIApplicationCache **aApplicationCache) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetApplicationCache(nsIApplicationCache *aApplicationCache) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIApplicationCacheContainer, NS_IAPPLICATIONCACHECONTAINER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAPPLICATIONCACHECONTAINER \
  NS_IMETHOD GetApplicationCache(nsIApplicationCache **aApplicationCache) override; \
  NS_IMETHOD SetApplicationCache(nsIApplicationCache *aApplicationCache) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAPPLICATIONCACHECONTAINER \
  nsresult GetApplicationCache(nsIApplicationCache **aApplicationCache); \
  nsresult SetApplicationCache(nsIApplicationCache *aApplicationCache); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAPPLICATIONCACHECONTAINER(_to) \
  NS_IMETHOD GetApplicationCache(nsIApplicationCache **aApplicationCache) override { return _to GetApplicationCache(aApplicationCache); } \
  NS_IMETHOD SetApplicationCache(nsIApplicationCache *aApplicationCache) override { return _to SetApplicationCache(aApplicationCache); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAPPLICATIONCACHECONTAINER(_to) \
  NS_IMETHOD GetApplicationCache(nsIApplicationCache **aApplicationCache) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetApplicationCache(aApplicationCache); } \
  NS_IMETHOD SetApplicationCache(nsIApplicationCache *aApplicationCache) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetApplicationCache(aApplicationCache); } 


#endif /* __gen_nsIApplicationCacheContainer_h__ */
