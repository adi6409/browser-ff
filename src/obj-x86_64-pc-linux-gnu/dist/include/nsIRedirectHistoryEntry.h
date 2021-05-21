/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRedirectHistoryEntry.idl
 */

#ifndef __gen_nsIRedirectHistoryEntry_h__
#define __gen_nsIRedirectHistoryEntry_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsIRedirectHistoryEntry */
#define NS_IREDIRECTHISTORYENTRY_IID_STR "133b2905-0eba-411c-a8bb-f59787142aa2"

#define NS_IREDIRECTHISTORYENTRY_IID \
  {0x133b2905, 0x0eba, 0x411c, \
    { 0xa8, 0xbb, 0xf5, 0x97, 0x87, 0x14, 0x2a, 0xa2 }}

class NS_NO_VTABLE nsIRedirectHistoryEntry : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREDIRECTHISTORYENTRY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIRedirectHistoryEntry;

  /* readonly attribute nsIPrincipal principal; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) = 0;

  /* readonly attribute nsIURI referrerURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetReferrerURI(nsIURI **aReferrerURI) = 0;

  /* readonly attribute ACString remoteAddress; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRemoteAddress(nsACString& aRemoteAddress) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRedirectHistoryEntry, NS_IREDIRECTHISTORYENTRY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREDIRECTHISTORYENTRY \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override; \
  NS_IMETHOD GetReferrerURI(nsIURI **aReferrerURI) override; \
  NS_IMETHOD GetRemoteAddress(nsACString& aRemoteAddress) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREDIRECTHISTORYENTRY \
  nsresult GetPrincipal(nsIPrincipal **aPrincipal); \
  nsresult GetReferrerURI(nsIURI **aReferrerURI); \
  nsresult GetRemoteAddress(nsACString& aRemoteAddress); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREDIRECTHISTORYENTRY(_to) \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return _to GetPrincipal(aPrincipal); } \
  NS_IMETHOD GetReferrerURI(nsIURI **aReferrerURI) override { return _to GetReferrerURI(aReferrerURI); } \
  NS_IMETHOD GetRemoteAddress(nsACString& aRemoteAddress) override { return _to GetRemoteAddress(aRemoteAddress); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREDIRECTHISTORYENTRY(_to) \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrincipal(aPrincipal); } \
  NS_IMETHOD GetReferrerURI(nsIURI **aReferrerURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReferrerURI(aReferrerURI); } \
  NS_IMETHOD GetRemoteAddress(nsACString& aRemoteAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRemoteAddress(aRemoteAddress); } 


#endif /* __gen_nsIRedirectHistoryEntry_h__ */
