/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/security/nsIContentSecurityManager.idl
 */

#ifndef __gen_nsIContentSecurityManager_h__
#define __gen_nsIContentSecurityManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIStreamListener; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsIContentSecurityManager */
#define NS_ICONTENTSECURITYMANAGER_IID_STR "3a9a1818-2ae8-4ec5-a340-8b29d31fca3b"

#define NS_ICONTENTSECURITYMANAGER_IID \
  {0x3a9a1818, 0x2ae8, 0x4ec5, \
    { 0xa3, 0x40, 0x8b, 0x29, 0xd3, 0x1f, 0xca, 0x3b }}

class NS_NO_VTABLE nsIContentSecurityManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTSECURITYMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentSecurityManager;

  /* nsIStreamListener performSecurityCheck (in nsIChannel aChannel, in nsIStreamListener aStreamListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PerformSecurityCheck(nsIChannel *aChannel, nsIStreamListener *aStreamListener, nsIStreamListener **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentSecurityManager, NS_ICONTENTSECURITYMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTSECURITYMANAGER \
  NS_IMETHOD PerformSecurityCheck(nsIChannel *aChannel, nsIStreamListener *aStreamListener, nsIStreamListener **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTSECURITYMANAGER \
  nsresult PerformSecurityCheck(nsIChannel *aChannel, nsIStreamListener *aStreamListener, nsIStreamListener **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTSECURITYMANAGER(_to) \
  NS_IMETHOD PerformSecurityCheck(nsIChannel *aChannel, nsIStreamListener *aStreamListener, nsIStreamListener **_retval) override { return _to PerformSecurityCheck(aChannel, aStreamListener, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTSECURITYMANAGER(_to) \
  NS_IMETHOD PerformSecurityCheck(nsIChannel *aChannel, nsIStreamListener *aStreamListener, nsIStreamListener **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PerformSecurityCheck(aChannel, aStreamListener, _retval); } 


#endif /* __gen_nsIContentSecurityManager_h__ */
