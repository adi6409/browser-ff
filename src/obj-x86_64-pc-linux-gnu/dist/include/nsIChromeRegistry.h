/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/chrome/nsIChromeRegistry.idl
 */

#ifndef __gen_nsIChromeRegistry_h__
#define __gen_nsIChromeRegistry_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */


/* starting interface:    nsIChromeRegistry */
#define NS_ICHROMEREGISTRY_IID_STR "249fb5ad-ae29-4e2c-a728-ba5cf464d188"

#define NS_ICHROMEREGISTRY_IID \
  {0x249fb5ad, 0xae29, 0x4e2c, \
    { 0xa7, 0x28, 0xba, 0x5c, 0xf4, 0x64, 0xd1, 0x88 }}

class NS_NO_VTABLE nsIChromeRegistry : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICHROMEREGISTRY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIChromeRegistry;

  enum {
    NONE = 0,
    PARTIAL = 1,
    FULL = 2
  };

  /* nsIURI convertChromeURL (in nsIURI aChromeURL); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConvertChromeURL(nsIURI *aChromeURL, nsIURI **_retval) = 0;

  /* void checkForNewChrome (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CheckForNewChrome(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIChromeRegistry, NS_ICHROMEREGISTRY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICHROMEREGISTRY \
  NS_IMETHOD ConvertChromeURL(nsIURI *aChromeURL, nsIURI **_retval) override; \
  NS_IMETHOD CheckForNewChrome(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICHROMEREGISTRY \
  nsresult ConvertChromeURL(nsIURI *aChromeURL, nsIURI **_retval); \
  nsresult CheckForNewChrome(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICHROMEREGISTRY(_to) \
  NS_IMETHOD ConvertChromeURL(nsIURI *aChromeURL, nsIURI **_retval) override { return _to ConvertChromeURL(aChromeURL, _retval); } \
  NS_IMETHOD CheckForNewChrome(void) override { return _to CheckForNewChrome(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICHROMEREGISTRY(_to) \
  NS_IMETHOD ConvertChromeURL(nsIURI *aChromeURL, nsIURI **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConvertChromeURL(aChromeURL, _retval); } \
  NS_IMETHOD CheckForNewChrome(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckForNewChrome(); } 


/* starting interface:    nsIXULChromeRegistry */
#define NS_IXULCHROMEREGISTRY_IID_STR "93251ddf-5e85-4172-ac2a-31780562974f"

#define NS_IXULCHROMEREGISTRY_IID \
  {0x93251ddf, 0x5e85, 0x4172, \
    { 0xac, 0x2a, 0x31, 0x78, 0x05, 0x62, 0x97, 0x4f }}

class NS_NO_VTABLE nsIXULChromeRegistry : public nsIChromeRegistry {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXULCHROMEREGISTRY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIXULChromeRegistry;

  /* boolean isLocaleRTL (in ACString package); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsLocaleRTL(const nsACString& package, bool *_retval) = 0;

  /* boolean allowScriptsForPackage (in nsIURI url); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AllowScriptsForPackage(nsIURI *url, bool *_retval) = 0;

  /* boolean allowContentToAccess (in nsIURI url); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AllowContentToAccess(nsIURI *url, bool *_retval) = 0;

  /* boolean canLoadURLRemotely (in nsIURI url); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CanLoadURLRemotely(nsIURI *url, bool *_retval) = 0;

  /* boolean mustLoadURLRemotely (in nsIURI url); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MustLoadURLRemotely(nsIURI *url, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXULChromeRegistry, NS_IXULCHROMEREGISTRY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXULCHROMEREGISTRY \
  NS_IMETHOD IsLocaleRTL(const nsACString& package, bool *_retval) override; \
  NS_IMETHOD AllowScriptsForPackage(nsIURI *url, bool *_retval) override; \
  NS_IMETHOD AllowContentToAccess(nsIURI *url, bool *_retval) override; \
  NS_IMETHOD CanLoadURLRemotely(nsIURI *url, bool *_retval) override; \
  NS_IMETHOD MustLoadURLRemotely(nsIURI *url, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXULCHROMEREGISTRY \
  nsresult IsLocaleRTL(const nsACString& package, bool *_retval); \
  nsresult AllowScriptsForPackage(nsIURI *url, bool *_retval); \
  nsresult AllowContentToAccess(nsIURI *url, bool *_retval); \
  nsresult CanLoadURLRemotely(nsIURI *url, bool *_retval); \
  nsresult MustLoadURLRemotely(nsIURI *url, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXULCHROMEREGISTRY(_to) \
  NS_IMETHOD IsLocaleRTL(const nsACString& package, bool *_retval) override { return _to IsLocaleRTL(package, _retval); } \
  NS_IMETHOD AllowScriptsForPackage(nsIURI *url, bool *_retval) override { return _to AllowScriptsForPackage(url, _retval); } \
  NS_IMETHOD AllowContentToAccess(nsIURI *url, bool *_retval) override { return _to AllowContentToAccess(url, _retval); } \
  NS_IMETHOD CanLoadURLRemotely(nsIURI *url, bool *_retval) override { return _to CanLoadURLRemotely(url, _retval); } \
  NS_IMETHOD MustLoadURLRemotely(nsIURI *url, bool *_retval) override { return _to MustLoadURLRemotely(url, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXULCHROMEREGISTRY(_to) \
  NS_IMETHOD IsLocaleRTL(const nsACString& package, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsLocaleRTL(package, _retval); } \
  NS_IMETHOD AllowScriptsForPackage(nsIURI *url, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AllowScriptsForPackage(url, _retval); } \
  NS_IMETHOD AllowContentToAccess(nsIURI *url, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AllowContentToAccess(url, _retval); } \
  NS_IMETHOD CanLoadURLRemotely(nsIURI *url, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanLoadURLRemotely(url, _retval); } \
  NS_IMETHOD MustLoadURLRemotely(nsIURI *url, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MustLoadURLRemotely(url, _retval); } 


#define NS_CHROMEREGISTRY_CONTRACTID \
  "@mozilla.org/chrome/chrome-registry;1"
/**
 * Chrome registry will notify various caches that all chrome files need
 * flushing.
 */
#define NS_CHROME_FLUSH_TOPIC \
  "chrome-flush-caches"

#endif /* __gen_nsIChromeRegistry_h__ */
