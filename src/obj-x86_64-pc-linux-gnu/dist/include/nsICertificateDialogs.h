/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICertificateDialogs.idl
 */

#ifndef __gen_nsICertificateDialogs_h__
#define __gen_nsICertificateDialogs_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInterfaceRequestor; /* forward declaration */

class nsIX509Cert; /* forward declaration */


/* starting interface:    nsICertificateDialogs */
#define NS_ICERTIFICATEDIALOGS_IID_STR "da871dab-f69e-4173-ab26-99fcd47b0e85"

#define NS_ICERTIFICATEDIALOGS_IID \
  {0xda871dab, 0xf69e, 0x4173, \
    { 0xab, 0x26, 0x99, 0xfc, 0xd4, 0x7b, 0x0e, 0x85 }}

class NS_NO_VTABLE nsICertificateDialogs : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICERTIFICATEDIALOGS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICertificateDialogs;

  /* [must_use] boolean confirmDownloadCACert (in nsIInterfaceRequestor ctx, in nsIX509Cert cert, out unsigned long trust); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ConfirmDownloadCACert(nsIInterfaceRequestor *ctx, nsIX509Cert *cert, uint32_t *trust, bool *_retval) = 0;

  /* [must_use] boolean setPKCS12FilePassword (in nsIInterfaceRequestor ctx, out AString password); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD SetPKCS12FilePassword(nsIInterfaceRequestor *ctx, nsAString& password, bool *_retval) = 0;

  /* [must_use] boolean getPKCS12FilePassword (in nsIInterfaceRequestor ctx, out AString password); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetPKCS12FilePassword(nsIInterfaceRequestor *ctx, nsAString& password, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICertificateDialogs, NS_ICERTIFICATEDIALOGS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICERTIFICATEDIALOGS \
  [[nodiscard]] NS_IMETHOD ConfirmDownloadCACert(nsIInterfaceRequestor *ctx, nsIX509Cert *cert, uint32_t *trust, bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD SetPKCS12FilePassword(nsIInterfaceRequestor *ctx, nsAString& password, bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD GetPKCS12FilePassword(nsIInterfaceRequestor *ctx, nsAString& password, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICERTIFICATEDIALOGS \
  [[nodiscard]] nsresult ConfirmDownloadCACert(nsIInterfaceRequestor *ctx, nsIX509Cert *cert, uint32_t *trust, bool *_retval); \
  [[nodiscard]] nsresult SetPKCS12FilePassword(nsIInterfaceRequestor *ctx, nsAString& password, bool *_retval); \
  [[nodiscard]] nsresult GetPKCS12FilePassword(nsIInterfaceRequestor *ctx, nsAString& password, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICERTIFICATEDIALOGS(_to) \
  [[nodiscard]] NS_IMETHOD ConfirmDownloadCACert(nsIInterfaceRequestor *ctx, nsIX509Cert *cert, uint32_t *trust, bool *_retval) override { return _to ConfirmDownloadCACert(ctx, cert, trust, _retval); } \
  [[nodiscard]] NS_IMETHOD SetPKCS12FilePassword(nsIInterfaceRequestor *ctx, nsAString& password, bool *_retval) override { return _to SetPKCS12FilePassword(ctx, password, _retval); } \
  [[nodiscard]] NS_IMETHOD GetPKCS12FilePassword(nsIInterfaceRequestor *ctx, nsAString& password, bool *_retval) override { return _to GetPKCS12FilePassword(ctx, password, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICERTIFICATEDIALOGS(_to) \
  [[nodiscard]] NS_IMETHOD ConfirmDownloadCACert(nsIInterfaceRequestor *ctx, nsIX509Cert *cert, uint32_t *trust, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConfirmDownloadCACert(ctx, cert, trust, _retval); } \
  [[nodiscard]] NS_IMETHOD SetPKCS12FilePassword(nsIInterfaceRequestor *ctx, nsAString& password, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPKCS12FilePassword(ctx, password, _retval); } \
  [[nodiscard]] NS_IMETHOD GetPKCS12FilePassword(nsIInterfaceRequestor *ctx, nsAString& password, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPKCS12FilePassword(ctx, password, _retval); } 

#define NS_CERTIFICATEDIALOGS_CONTRACTID "@mozilla.org/nsCertificateDialogs;1"

#endif /* __gen_nsICertificateDialogs_h__ */
