/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISystemProxySettings.idl
 */

#ifndef __gen_nsISystemProxySettings_h__
#define __gen_nsISystemProxySettings_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsISystemProxySettings */
#define NS_ISYSTEMPROXYSETTINGS_IID_STR "971591cd-277e-409a-bbf6-0a79879cd307"

#define NS_ISYSTEMPROXYSETTINGS_IID \
  {0x971591cd, 0x277e, 0x409a, \
    { 0xbb, 0xf6, 0x0a, 0x79, 0x87, 0x9c, 0xd3, 0x07 }}

class NS_NO_VTABLE nsISystemProxySettings : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISYSTEMPROXYSETTINGS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISystemProxySettings;

  /* readonly attribute bool mainThreadOnly; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMainThreadOnly(bool *aMainThreadOnly) = 0;

  /* readonly attribute AUTF8String PACURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPACURI(nsACString& aPACURI) = 0;

  /* AUTF8String getProxyForURI (in AUTF8String testSpec, in AUTF8String testScheme, in AUTF8String testHost, in int32_t testPort); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProxyForURI(const nsACString& testSpec, const nsACString& testScheme, const nsACString& testHost, int32_t testPort, nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISystemProxySettings, NS_ISYSTEMPROXYSETTINGS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISYSTEMPROXYSETTINGS \
  NS_IMETHOD GetMainThreadOnly(bool *aMainThreadOnly) override; \
  NS_IMETHOD GetPACURI(nsACString& aPACURI) override; \
  NS_IMETHOD GetProxyForURI(const nsACString& testSpec, const nsACString& testScheme, const nsACString& testHost, int32_t testPort, nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISYSTEMPROXYSETTINGS \
  nsresult GetMainThreadOnly(bool *aMainThreadOnly); \
  nsresult GetPACURI(nsACString& aPACURI); \
  nsresult GetProxyForURI(const nsACString& testSpec, const nsACString& testScheme, const nsACString& testHost, int32_t testPort, nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISYSTEMPROXYSETTINGS(_to) \
  NS_IMETHOD GetMainThreadOnly(bool *aMainThreadOnly) override { return _to GetMainThreadOnly(aMainThreadOnly); } \
  NS_IMETHOD GetPACURI(nsACString& aPACURI) override { return _to GetPACURI(aPACURI); } \
  NS_IMETHOD GetProxyForURI(const nsACString& testSpec, const nsACString& testScheme, const nsACString& testHost, int32_t testPort, nsACString& _retval) override { return _to GetProxyForURI(testSpec, testScheme, testHost, testPort, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISYSTEMPROXYSETTINGS(_to) \
  NS_IMETHOD GetMainThreadOnly(bool *aMainThreadOnly) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMainThreadOnly(aMainThreadOnly); } \
  NS_IMETHOD GetPACURI(nsACString& aPACURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPACURI(aPACURI); } \
  NS_IMETHOD GetProxyForURI(const nsACString& testSpec, const nsACString& testScheme, const nsACString& testHost, int32_t testPort, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProxyForURI(testSpec, testScheme, testHost, testPort, _retval); } 


#endif /* __gen_nsISystemProxySettings_h__ */
