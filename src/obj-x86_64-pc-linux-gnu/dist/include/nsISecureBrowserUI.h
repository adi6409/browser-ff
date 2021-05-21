/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISecureBrowserUI.idl
 */

#ifndef __gen_nsISecureBrowserUI_h__
#define __gen_nsISecureBrowserUI_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsITransportSecurityInfo; /* forward declaration */


/* starting interface:    nsISecureBrowserUI */
#define NS_ISECUREBROWSERUI_IID_STR "718c662a-f810-4a80-a6c9-0b1810ecade2"

#define NS_ISECUREBROWSERUI_IID \
  {0x718c662a, 0xf810, 0x4a80, \
    { 0xa6, 0xc9, 0x0b, 0x18, 0x10, 0xec, 0xad, 0xe2 }}

class NS_NO_VTABLE nsISecureBrowserUI : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISECUREBROWSERUI_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISecureBrowserUI;

  /* readonly attribute unsigned long state; */
  NS_IMETHOD GetState(uint32_t *aState) = 0;

  /* readonly attribute bool isSecureContext; */
  NS_IMETHOD GetIsSecureContext(bool *aIsSecureContext) = 0;

  /* readonly attribute nsITransportSecurityInfo secInfo; */
  NS_IMETHOD GetSecInfo(nsITransportSecurityInfo **aSecInfo) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISecureBrowserUI, NS_ISECUREBROWSERUI_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISECUREBROWSERUI \
  NS_IMETHOD GetState(uint32_t *aState) override; \
  NS_IMETHOD GetIsSecureContext(bool *aIsSecureContext) override; \
  NS_IMETHOD GetSecInfo(nsITransportSecurityInfo **aSecInfo) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISECUREBROWSERUI \
  nsresult GetState(uint32_t *aState); \
  nsresult GetIsSecureContext(bool *aIsSecureContext); \
  nsresult GetSecInfo(nsITransportSecurityInfo **aSecInfo); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISECUREBROWSERUI(_to) \
  NS_IMETHOD GetState(uint32_t *aState) override { return _to GetState(aState); } \
  NS_IMETHOD GetIsSecureContext(bool *aIsSecureContext) override { return _to GetIsSecureContext(aIsSecureContext); } \
  NS_IMETHOD GetSecInfo(nsITransportSecurityInfo **aSecInfo) override { return _to GetSecInfo(aSecInfo); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISECUREBROWSERUI(_to) \
  NS_IMETHOD GetState(uint32_t *aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetState(aState); } \
  NS_IMETHOD GetIsSecureContext(bool *aIsSecureContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsSecureContext(aIsSecureContext); } \
  NS_IMETHOD GetSecInfo(nsITransportSecurityInfo **aSecInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecInfo(aSecInfo); } 

#define NS_SECURE_BROWSER_UI_CONTRACTID "@mozilla.org/secure_browser_ui;1"

#endif /* __gen_nsISecureBrowserUI_h__ */
