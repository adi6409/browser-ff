/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/chrome/nsIToolkitChromeRegistry.idl
 */

#ifndef __gen_nsIToolkitChromeRegistry_h__
#define __gen_nsIToolkitChromeRegistry_h__


#ifndef __gen_nsIChromeRegistry_h__
#include "nsIChromeRegistry.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIUTF8StringEnumerator; /* forward declaration */


/* starting interface:    nsIToolkitChromeRegistry */
#define NS_ITOOLKITCHROMEREGISTRY_IID_STR "8727651c-9530-45a0-b81e-0e0690c30c50"

#define NS_ITOOLKITCHROMEREGISTRY_IID \
  {0x8727651c, 0x9530, 0x45a0, \
    { 0xb8, 0x1e, 0x0e, 0x06, 0x90, 0xc3, 0x0c, 0x50 }}

class NS_NO_VTABLE nsIToolkitChromeRegistry : public nsIXULChromeRegistry {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITOOLKITCHROMEREGISTRY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIToolkitChromeRegistry;

  /* nsIUTF8StringEnumerator getLocalesForPackage (in AUTF8String aPackage); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLocalesForPackage(const nsACString& aPackage, nsIUTF8StringEnumerator **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIToolkitChromeRegistry, NS_ITOOLKITCHROMEREGISTRY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITOOLKITCHROMEREGISTRY \
  NS_IMETHOD GetLocalesForPackage(const nsACString& aPackage, nsIUTF8StringEnumerator **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITOOLKITCHROMEREGISTRY \
  nsresult GetLocalesForPackage(const nsACString& aPackage, nsIUTF8StringEnumerator **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITOOLKITCHROMEREGISTRY(_to) \
  NS_IMETHOD GetLocalesForPackage(const nsACString& aPackage, nsIUTF8StringEnumerator **_retval) override { return _to GetLocalesForPackage(aPackage, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITOOLKITCHROMEREGISTRY(_to) \
  NS_IMETHOD GetLocalesForPackage(const nsACString& aPackage, nsIUTF8StringEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLocalesForPackage(aPackage, _retval); } 


#endif /* __gen_nsIToolkitChromeRegistry_h__ */
