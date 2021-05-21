/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsILocalCertService.idl
 */

#ifndef __gen_nsILocalCertService_h__
#define __gen_nsILocalCertService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIX509Cert; /* forward declaration */

class nsILocalCertGetCallback; /* forward declaration */

class nsILocalCertCallback; /* forward declaration */


/* starting interface:    nsILocalCertService */
#define NS_ILOCALCERTSERVICE_IID_STR "9702fdd4-4c2c-439c-ba2e-19cda018eb99"

#define NS_ILOCALCERTSERVICE_IID \
  {0x9702fdd4, 0x4c2c, 0x439c, \
    { 0xba, 0x2e, 0x19, 0xcd, 0xa0, 0x18, 0xeb, 0x99 }}

class NS_NO_VTABLE nsILocalCertService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOCALCERTSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILocalCertService;

  /* [must_use] void getOrCreateCert (in ACString nickname, in nsILocalCertGetCallback cb); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetOrCreateCert(const nsACString& nickname, nsILocalCertGetCallback *cb) = 0;

  /* [must_use] void removeCert (in ACString nickname, in nsILocalCertCallback cb); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD RemoveCert(const nsACString& nickname, nsILocalCertCallback *cb) = 0;

  /* [must_use] readonly attribute boolean loginPromptRequired; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetLoginPromptRequired(bool *aLoginPromptRequired) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILocalCertService, NS_ILOCALCERTSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOCALCERTSERVICE \
  [[nodiscard]] NS_IMETHOD GetOrCreateCert(const nsACString& nickname, nsILocalCertGetCallback *cb) override; \
  [[nodiscard]] NS_IMETHOD RemoveCert(const nsACString& nickname, nsILocalCertCallback *cb) override; \
  [[nodiscard]] NS_IMETHOD GetLoginPromptRequired(bool *aLoginPromptRequired) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOCALCERTSERVICE \
  [[nodiscard]] nsresult GetOrCreateCert(const nsACString& nickname, nsILocalCertGetCallback *cb); \
  [[nodiscard]] nsresult RemoveCert(const nsACString& nickname, nsILocalCertCallback *cb); \
  [[nodiscard]] nsresult GetLoginPromptRequired(bool *aLoginPromptRequired); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOCALCERTSERVICE(_to) \
  [[nodiscard]] NS_IMETHOD GetOrCreateCert(const nsACString& nickname, nsILocalCertGetCallback *cb) override { return _to GetOrCreateCert(nickname, cb); } \
  [[nodiscard]] NS_IMETHOD RemoveCert(const nsACString& nickname, nsILocalCertCallback *cb) override { return _to RemoveCert(nickname, cb); } \
  [[nodiscard]] NS_IMETHOD GetLoginPromptRequired(bool *aLoginPromptRequired) override { return _to GetLoginPromptRequired(aLoginPromptRequired); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOCALCERTSERVICE(_to) \
  [[nodiscard]] NS_IMETHOD GetOrCreateCert(const nsACString& nickname, nsILocalCertGetCallback *cb) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOrCreateCert(nickname, cb); } \
  [[nodiscard]] NS_IMETHOD RemoveCert(const nsACString& nickname, nsILocalCertCallback *cb) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveCert(nickname, cb); } \
  [[nodiscard]] NS_IMETHOD GetLoginPromptRequired(bool *aLoginPromptRequired) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoginPromptRequired(aLoginPromptRequired); } 


/* starting interface:    nsILocalCertGetCallback */
#define NS_ILOCALCERTGETCALLBACK_IID_STR "cc09633e-7c70-4093-a9cf-79ab676ca8a9"

#define NS_ILOCALCERTGETCALLBACK_IID \
  {0xcc09633e, 0x7c70, 0x4093, \
    { 0xa9, 0xcf, 0x79, 0xab, 0x67, 0x6c, 0xa8, 0xa9 }}

class NS_NO_VTABLE nsILocalCertGetCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOCALCERTGETCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILocalCertGetCallback;

  /* void handleCert (in nsIX509Cert cert, in nsresult result); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleCert(nsIX509Cert *cert, nsresult result) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILocalCertGetCallback, NS_ILOCALCERTGETCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOCALCERTGETCALLBACK \
  NS_IMETHOD HandleCert(nsIX509Cert *cert, nsresult result) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOCALCERTGETCALLBACK \
  nsresult HandleCert(nsIX509Cert *cert, nsresult result); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOCALCERTGETCALLBACK(_to) \
  NS_IMETHOD HandleCert(nsIX509Cert *cert, nsresult result) override { return _to HandleCert(cert, result); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOCALCERTGETCALLBACK(_to) \
  NS_IMETHOD HandleCert(nsIX509Cert *cert, nsresult result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleCert(cert, result); } 


/* starting interface:    nsILocalCertCallback */
#define NS_ILOCALCERTCALLBACK_IID_STR "518124e9-55e6-4e23-97c0-4995b3a1bec6"

#define NS_ILOCALCERTCALLBACK_IID \
  {0x518124e9, 0x55e6, 0x4e23, \
    { 0x97, 0xc0, 0x49, 0x95, 0xb3, 0xa1, 0xbe, 0xc6 }}

class NS_NO_VTABLE nsILocalCertCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOCALCERTCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILocalCertCallback;

  /* void handleResult (in nsresult result); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleResult(nsresult result) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILocalCertCallback, NS_ILOCALCERTCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOCALCERTCALLBACK \
  NS_IMETHOD HandleResult(nsresult result) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOCALCERTCALLBACK \
  nsresult HandleResult(nsresult result); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOCALCERTCALLBACK(_to) \
  NS_IMETHOD HandleResult(nsresult result) override { return _to HandleResult(result); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOCALCERTCALLBACK(_to) \
  NS_IMETHOD HandleResult(nsresult result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleResult(result); } 

#define LOCALCERTSERVICE_CONTRACTID \
  "@mozilla.org/security/local-cert-service;1"

#endif /* __gen_nsILocalCertService_h__ */
