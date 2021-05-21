/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsINSSComponent.idl
 */

#ifndef __gen_nsINSSComponent_h__
#define __gen_nsINSSComponent_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "cert.h"
#include "SharedCertVerifier.h"
#define PSM_COMPONENT_CONTRACTID "@mozilla.org/psm;1"

/* starting interface:    nsINSSComponent */
#define NS_INSSCOMPONENT_IID_STR "a0a8f52b-ea18-4abc-a3ca-eccf704ffe63"

#define NS_INSSCOMPONENT_IID \
  {0xa0a8f52b, 0xea18, 0x4abc, \
    { 0xa3, 0xca, 0xec, 0xcf, 0x70, 0x4f, 0xfe, 0x63 }}

class NS_NO_VTABLE nsINSSComponent : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INSSCOMPONENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINSSComponent;

  /* [noscript] void logoutAuthenticatedPK11 (); */
  NS_IMETHOD LogoutAuthenticatedPK11(void) = 0;

  /* [noscript] bool isCertTestBuiltInRoot (in CERTCertificatePtr cert); */
  NS_IMETHOD IsCertTestBuiltInRoot(CERTCertificate * cert, bool *_retval) = 0;

  /* [noscript] bool isCertContentSigningRoot (in Array<octet> cert); */
  NS_IMETHOD IsCertContentSigningRoot(const nsTArray<uint8_t >& cert, bool *_retval) = 0;

  /* Array<Array<octet>> getEnterpriseRoots (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEnterpriseRoots(nsTArray<nsTArray<uint8_t >>& _retval) = 0;

  /* Array<Array<octet>> getEnterpriseIntermediates (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEnterpriseIntermediates(nsTArray<nsTArray<uint8_t >>& _retval) = 0;

  /* void addEnterpriseIntermediate (in Array<octet> intermediateBytes); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddEnterpriseIntermediate(const nsTArray<uint8_t >& intermediateBytes) = 0;

  /* [noscript] void blockUntilLoadableCertsLoaded (); */
  NS_IMETHOD BlockUntilLoadableCertsLoaded(void) = 0;

  /* [noscript] void checkForSmartCardChanges (); */
  NS_IMETHOD CheckForSmartCardChanges(void) = 0;

  /* [noscript] void issuerMatchesMitmCanary (in string certIssuer); */
  NS_IMETHOD IssuerMatchesMitmCanary(const char * certIssuer) = 0;

  /* [noscript] bool hasActiveSmartCards (); */
  NS_IMETHOD HasActiveSmartCards(bool *_retval) = 0;

  /* [noscript] bool hasUserCertsInstalled (); */
  NS_IMETHOD HasUserCertsInstalled(bool *_retval) = 0;

  /* [noscript] SharedCertVerifierPtr getDefaultCertVerifier (); */
  NS_IMETHOD GetDefaultCertVerifier(mozilla::psm::SharedCertVerifier * * _retval) = 0;

  /* void clearSSLExternalAndInternalSessionCache (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearSSLExternalAndInternalSessionCache(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINSSComponent, NS_INSSCOMPONENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINSSCOMPONENT \
  NS_IMETHOD LogoutAuthenticatedPK11(void) override; \
  NS_IMETHOD IsCertTestBuiltInRoot(CERTCertificate * cert, bool *_retval) override; \
  NS_IMETHOD IsCertContentSigningRoot(const nsTArray<uint8_t >& cert, bool *_retval) override; \
  NS_IMETHOD GetEnterpriseRoots(nsTArray<nsTArray<uint8_t >>& _retval) override; \
  NS_IMETHOD GetEnterpriseIntermediates(nsTArray<nsTArray<uint8_t >>& _retval) override; \
  NS_IMETHOD AddEnterpriseIntermediate(const nsTArray<uint8_t >& intermediateBytes) override; \
  NS_IMETHOD BlockUntilLoadableCertsLoaded(void) override; \
  NS_IMETHOD CheckForSmartCardChanges(void) override; \
  NS_IMETHOD IssuerMatchesMitmCanary(const char * certIssuer) override; \
  NS_IMETHOD HasActiveSmartCards(bool *_retval) override; \
  NS_IMETHOD HasUserCertsInstalled(bool *_retval) override; \
  NS_IMETHOD GetDefaultCertVerifier(mozilla::psm::SharedCertVerifier * * _retval) override; \
  NS_IMETHOD ClearSSLExternalAndInternalSessionCache(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINSSCOMPONENT \
  nsresult LogoutAuthenticatedPK11(void); \
  nsresult IsCertTestBuiltInRoot(CERTCertificate * cert, bool *_retval); \
  nsresult IsCertContentSigningRoot(const nsTArray<uint8_t >& cert, bool *_retval); \
  nsresult GetEnterpriseRoots(nsTArray<nsTArray<uint8_t >>& _retval); \
  nsresult GetEnterpriseIntermediates(nsTArray<nsTArray<uint8_t >>& _retval); \
  nsresult AddEnterpriseIntermediate(const nsTArray<uint8_t >& intermediateBytes); \
  nsresult BlockUntilLoadableCertsLoaded(void); \
  nsresult CheckForSmartCardChanges(void); \
  nsresult IssuerMatchesMitmCanary(const char * certIssuer); \
  nsresult HasActiveSmartCards(bool *_retval); \
  nsresult HasUserCertsInstalled(bool *_retval); \
  nsresult GetDefaultCertVerifier(mozilla::psm::SharedCertVerifier * * _retval); \
  nsresult ClearSSLExternalAndInternalSessionCache(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINSSCOMPONENT(_to) \
  NS_IMETHOD LogoutAuthenticatedPK11(void) override { return _to LogoutAuthenticatedPK11(); } \
  NS_IMETHOD IsCertTestBuiltInRoot(CERTCertificate * cert, bool *_retval) override { return _to IsCertTestBuiltInRoot(cert, _retval); } \
  NS_IMETHOD IsCertContentSigningRoot(const nsTArray<uint8_t >& cert, bool *_retval) override { return _to IsCertContentSigningRoot(cert, _retval); } \
  NS_IMETHOD GetEnterpriseRoots(nsTArray<nsTArray<uint8_t >>& _retval) override { return _to GetEnterpriseRoots(_retval); } \
  NS_IMETHOD GetEnterpriseIntermediates(nsTArray<nsTArray<uint8_t >>& _retval) override { return _to GetEnterpriseIntermediates(_retval); } \
  NS_IMETHOD AddEnterpriseIntermediate(const nsTArray<uint8_t >& intermediateBytes) override { return _to AddEnterpriseIntermediate(intermediateBytes); } \
  NS_IMETHOD BlockUntilLoadableCertsLoaded(void) override { return _to BlockUntilLoadableCertsLoaded(); } \
  NS_IMETHOD CheckForSmartCardChanges(void) override { return _to CheckForSmartCardChanges(); } \
  NS_IMETHOD IssuerMatchesMitmCanary(const char * certIssuer) override { return _to IssuerMatchesMitmCanary(certIssuer); } \
  NS_IMETHOD HasActiveSmartCards(bool *_retval) override { return _to HasActiveSmartCards(_retval); } \
  NS_IMETHOD HasUserCertsInstalled(bool *_retval) override { return _to HasUserCertsInstalled(_retval); } \
  NS_IMETHOD GetDefaultCertVerifier(mozilla::psm::SharedCertVerifier * * _retval) override { return _to GetDefaultCertVerifier(_retval); } \
  NS_IMETHOD ClearSSLExternalAndInternalSessionCache(void) override { return _to ClearSSLExternalAndInternalSessionCache(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINSSCOMPONENT(_to) \
  NS_IMETHOD LogoutAuthenticatedPK11(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LogoutAuthenticatedPK11(); } \
  NS_IMETHOD IsCertTestBuiltInRoot(CERTCertificate * cert, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCertTestBuiltInRoot(cert, _retval); } \
  NS_IMETHOD IsCertContentSigningRoot(const nsTArray<uint8_t >& cert, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCertContentSigningRoot(cert, _retval); } \
  NS_IMETHOD GetEnterpriseRoots(nsTArray<nsTArray<uint8_t >>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEnterpriseRoots(_retval); } \
  NS_IMETHOD GetEnterpriseIntermediates(nsTArray<nsTArray<uint8_t >>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEnterpriseIntermediates(_retval); } \
  NS_IMETHOD AddEnterpriseIntermediate(const nsTArray<uint8_t >& intermediateBytes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddEnterpriseIntermediate(intermediateBytes); } \
  NS_IMETHOD BlockUntilLoadableCertsLoaded(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BlockUntilLoadableCertsLoaded(); } \
  NS_IMETHOD CheckForSmartCardChanges(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckForSmartCardChanges(); } \
  NS_IMETHOD IssuerMatchesMitmCanary(const char * certIssuer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IssuerMatchesMitmCanary(certIssuer); } \
  NS_IMETHOD HasActiveSmartCards(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasActiveSmartCards(_retval); } \
  NS_IMETHOD HasUserCertsInstalled(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasUserCertsInstalled(_retval); } \
  NS_IMETHOD GetDefaultCertVerifier(mozilla::psm::SharedCertVerifier * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultCertVerifier(_retval); } \
  NS_IMETHOD ClearSSLExternalAndInternalSessionCache(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearSSLExternalAndInternalSessionCache(); } 


#endif /* __gen_nsINSSComponent_h__ */
