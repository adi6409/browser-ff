/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIContentSignatureVerifier.idl
 */

#ifndef __gen_nsIContentSignatureVerifier_h__
#define __gen_nsIContentSignatureVerifier_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIContentSignatureReceiverCallback; /* forward declaration */


/* starting interface:    nsIContentSignatureVerifier */
#define NS_ICONTENTSIGNATUREVERIFIER_IID_STR "45a5fe2f-c350-4b86-962d-02d5aaaa955a"

#define NS_ICONTENTSIGNATUREVERIFIER_IID \
  {0x45a5fe2f, 0xc350, 0x4b86, \
    { 0x96, 0x2d, 0x02, 0xd5, 0xaa, 0xaa, 0x95, 0x5a }}

class NS_NO_VTABLE nsIContentSignatureVerifier : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTSIGNATUREVERIFIER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentSignatureVerifier;

  /* [implicit_jscontext,must_use] Promise asyncVerifyContentSignature (in ACString aData, in ACString aContentSignatureHeader, in ACString aCertificateChain, in ACString aHostname); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AsyncVerifyContentSignature(const nsACString& aData, const nsACString& aContentSignatureHeader, const nsACString& aCertificateChain, const nsACString& aHostname, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentSignatureVerifier, NS_ICONTENTSIGNATUREVERIFIER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTSIGNATUREVERIFIER \
  [[nodiscard]] NS_IMETHOD AsyncVerifyContentSignature(const nsACString& aData, const nsACString& aContentSignatureHeader, const nsACString& aCertificateChain, const nsACString& aHostname, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTSIGNATUREVERIFIER \
  [[nodiscard]] nsresult AsyncVerifyContentSignature(const nsACString& aData, const nsACString& aContentSignatureHeader, const nsACString& aCertificateChain, const nsACString& aHostname, JSContext* cx, ::mozilla::dom::Promise * * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTSIGNATUREVERIFIER(_to) \
  [[nodiscard]] NS_IMETHOD AsyncVerifyContentSignature(const nsACString& aData, const nsACString& aContentSignatureHeader, const nsACString& aCertificateChain, const nsACString& aHostname, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to AsyncVerifyContentSignature(aData, aContentSignatureHeader, aCertificateChain, aHostname, cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTSIGNATUREVERIFIER(_to) \
  [[nodiscard]] NS_IMETHOD AsyncVerifyContentSignature(const nsACString& aData, const nsACString& aContentSignatureHeader, const nsACString& aCertificateChain, const nsACString& aHostname, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncVerifyContentSignature(aData, aContentSignatureHeader, aCertificateChain, aHostname, cx, _retval); } 


#endif /* __gen_nsIContentSignatureVerifier_h__ */
