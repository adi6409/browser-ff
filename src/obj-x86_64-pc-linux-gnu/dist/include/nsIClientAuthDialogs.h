/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIClientAuthDialogs.idl
 */

#ifndef __gen_nsIClientAuthDialogs_h__
#define __gen_nsIClientAuthDialogs_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */

class nsIInterfaceRequestor; /* forward declaration */


/* starting interface:    nsIClientAuthDialogs */
#define NS_ICLIENTAUTHDIALOGS_IID_STR "fa4c7520-1433-11d5-ba24-00108303b117"

#define NS_ICLIENTAUTHDIALOGS_IID \
  {0xfa4c7520, 0x1433, 0x11d5, \
    { 0xba, 0x24, 0x00, 0x10, 0x83, 0x03, 0xb1, 0x17 }}

class NS_NO_VTABLE nsIClientAuthDialogs : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICLIENTAUTHDIALOGS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIClientAuthDialogs;

  /* [must_use] boolean chooseCertificate (in AUTF8String hostname, in long port, in AUTF8String organization, in AUTF8String issuerOrg, in nsIArray certList, out unsigned long selectedIndex, out boolean rememberClientAuthCertificate); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ChooseCertificate(const nsACString& hostname, int32_t port, const nsACString& organization, const nsACString& issuerOrg, nsIArray *certList, uint32_t *selectedIndex, bool *rememberClientAuthCertificate, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIClientAuthDialogs, NS_ICLIENTAUTHDIALOGS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICLIENTAUTHDIALOGS \
  [[nodiscard]] NS_IMETHOD ChooseCertificate(const nsACString& hostname, int32_t port, const nsACString& organization, const nsACString& issuerOrg, nsIArray *certList, uint32_t *selectedIndex, bool *rememberClientAuthCertificate, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICLIENTAUTHDIALOGS \
  [[nodiscard]] nsresult ChooseCertificate(const nsACString& hostname, int32_t port, const nsACString& organization, const nsACString& issuerOrg, nsIArray *certList, uint32_t *selectedIndex, bool *rememberClientAuthCertificate, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICLIENTAUTHDIALOGS(_to) \
  [[nodiscard]] NS_IMETHOD ChooseCertificate(const nsACString& hostname, int32_t port, const nsACString& organization, const nsACString& issuerOrg, nsIArray *certList, uint32_t *selectedIndex, bool *rememberClientAuthCertificate, bool *_retval) override { return _to ChooseCertificate(hostname, port, organization, issuerOrg, certList, selectedIndex, rememberClientAuthCertificate, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICLIENTAUTHDIALOGS(_to) \
  [[nodiscard]] NS_IMETHOD ChooseCertificate(const nsACString& hostname, int32_t port, const nsACString& organization, const nsACString& issuerOrg, nsIArray *certList, uint32_t *selectedIndex, bool *rememberClientAuthCertificate, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ChooseCertificate(hostname, port, organization, issuerOrg, certList, selectedIndex, rememberClientAuthCertificate, _retval); } 

#define NS_CLIENTAUTHDIALOGS_CONTRACTID "@mozilla.org/nsClientAuthDialogs;1"

#endif /* __gen_nsIClientAuthDialogs_h__ */
