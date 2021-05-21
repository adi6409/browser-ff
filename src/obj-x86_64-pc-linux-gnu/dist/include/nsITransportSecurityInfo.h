/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/socket/nsITransportSecurityInfo.idl
 */

#ifndef __gen_nsITransportSecurityInfo_h__
#define __gen_nsITransportSecurityInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIX509Cert; /* forward declaration */

namespace IPC {
  class Message;
}
class PickleIterator;

/* starting interface:    nsITransportSecurityInfo */
#define NS_ITRANSPORTSECURITYINFO_IID_STR "216112d3-28bc-4671-b057-f98cc09ba1ea"

#define NS_ITRANSPORTSECURITYINFO_IID \
  {0x216112d3, 0x28bc, 0x4671, \
    { 0xb0, 0x57, 0xf9, 0x8c, 0xc0, 0x9b, 0xa1, 0xea }}

class NS_NO_VTABLE nsITransportSecurityInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITRANSPORTSECURITYINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITransportSecurityInfo;

  /* readonly attribute unsigned long securityState; */
  NS_IMETHOD GetSecurityState(uint32_t *aSecurityState) = 0;

  /* readonly attribute long errorCode; */
  NS_IMETHOD GetErrorCode(int32_t *aErrorCode) = 0;

  /* readonly attribute AString errorCodeString; */
  NS_IMETHOD GetErrorCodeString(nsAString& aErrorCodeString) = 0;

  /* readonly attribute Array<nsIX509Cert> failedCertChain; */
  NS_IMETHOD GetFailedCertChain(nsTArray<RefPtr<nsIX509Cert>>& aFailedCertChain) = 0;

  /* readonly attribute nsIX509Cert serverCert; */
  NS_IMETHOD GetServerCert(nsIX509Cert **aServerCert) = 0;

  /* readonly attribute Array<nsIX509Cert> succeededCertChain; */
  NS_IMETHOD GetSucceededCertChain(nsTArray<RefPtr<nsIX509Cert>>& aSucceededCertChain) = 0;

  /* [must_use] readonly attribute ACString cipherName; */
  [[nodiscard]] NS_IMETHOD GetCipherName(nsACString& aCipherName) = 0;

  /* [must_use] readonly attribute unsigned long keyLength; */
  [[nodiscard]] NS_IMETHOD GetKeyLength(uint32_t *aKeyLength) = 0;

  /* [must_use] readonly attribute unsigned long secretKeyLength; */
  [[nodiscard]] NS_IMETHOD GetSecretKeyLength(uint32_t *aSecretKeyLength) = 0;

  /* [must_use] readonly attribute ACString keaGroupName; */
  [[nodiscard]] NS_IMETHOD GetKeaGroupName(nsACString& aKeaGroupName) = 0;

  /* [must_use] readonly attribute ACString signatureSchemeName; */
  [[nodiscard]] NS_IMETHOD GetSignatureSchemeName(nsACString& aSignatureSchemeName) = 0;

  enum {
    SSL_VERSION_3 = 0,
    TLS_VERSION_1 = 1,
    TLS_VERSION_1_1 = 2,
    TLS_VERSION_1_2 = 3,
    TLS_VERSION_1_3 = 4
  };

  /* [must_use] readonly attribute unsigned short protocolVersion; */
  [[nodiscard]] NS_IMETHOD GetProtocolVersion(uint16_t *aProtocolVersion) = 0;

  enum {
    CERTIFICATE_TRANSPARENCY_NOT_APPLICABLE = 0,
    CERTIFICATE_TRANSPARENCY_POLICY_COMPLIANT = 5,
    CERTIFICATE_TRANSPARENCY_POLICY_NOT_ENOUGH_SCTS = 6,
    CERTIFICATE_TRANSPARENCY_POLICY_NOT_DIVERSE_SCTS = 7
  };

  /* [must_use] readonly attribute unsigned short certificateTransparencyStatus; */
  [[nodiscard]] NS_IMETHOD GetCertificateTransparencyStatus(uint16_t *aCertificateTransparencyStatus) = 0;

  /* [must_use] readonly attribute boolean isAcceptedEch; */
  [[nodiscard]] NS_IMETHOD GetIsAcceptedEch(bool *aIsAcceptedEch) = 0;

  /* [must_use] readonly attribute boolean isDelegatedCredential; */
  [[nodiscard]] NS_IMETHOD GetIsDelegatedCredential(bool *aIsDelegatedCredential) = 0;

  /* [must_use] readonly attribute boolean isDomainMismatch; */
  [[nodiscard]] NS_IMETHOD GetIsDomainMismatch(bool *aIsDomainMismatch) = 0;

  /* [must_use] readonly attribute boolean isNotValidAtThisTime; */
  [[nodiscard]] NS_IMETHOD GetIsNotValidAtThisTime(bool *aIsNotValidAtThisTime) = 0;

  /* [must_use] readonly attribute boolean isUntrusted; */
  [[nodiscard]] NS_IMETHOD GetIsUntrusted(bool *aIsUntrusted) = 0;

  /* [must_use] readonly attribute boolean isExtendedValidation; */
  [[nodiscard]] NS_IMETHOD GetIsExtendedValidation(bool *aIsExtendedValidation) = 0;

  /* [noscript,notxpcom] void SerializeToIPC (in IpcMessagePtr aMsg); */
  NS_IMETHOD_(void) SerializeToIPC(IPC::Message * aMsg) = 0;

  /* [noscript,notxpcom] bool DeserializeFromIPC ([const] in IpcMessagePtr aMsg, in PickleIteratorPtr aIter); */
  NS_IMETHOD_(bool) DeserializeFromIPC(const IPC::Message * aMsg, PickleIterator * aIter) = 0;

  /* readonly attribute ACString negotiatedNPN; */
  NS_IMETHOD GetNegotiatedNPN(nsACString& aNegotiatedNPN) = 0;

  /* readonly attribute boolean resumed; */
  NS_IMETHOD GetResumed(bool *aResumed) = 0;

  /* attribute boolean isBuiltCertChainRootBuiltInRoot; */
  NS_IMETHOD GetIsBuiltCertChainRootBuiltInRoot(bool *aIsBuiltCertChainRootBuiltInRoot) = 0;
  NS_IMETHOD SetIsBuiltCertChainRootBuiltInRoot(bool aIsBuiltCertChainRootBuiltInRoot) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITransportSecurityInfo, NS_ITRANSPORTSECURITYINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITRANSPORTSECURITYINFO \
  NS_IMETHOD GetSecurityState(uint32_t *aSecurityState) override; \
  NS_IMETHOD GetErrorCode(int32_t *aErrorCode) override; \
  NS_IMETHOD GetErrorCodeString(nsAString& aErrorCodeString) override; \
  NS_IMETHOD GetFailedCertChain(nsTArray<RefPtr<nsIX509Cert>>& aFailedCertChain) override; \
  NS_IMETHOD GetServerCert(nsIX509Cert **aServerCert) override; \
  NS_IMETHOD GetSucceededCertChain(nsTArray<RefPtr<nsIX509Cert>>& aSucceededCertChain) override; \
  [[nodiscard]] NS_IMETHOD GetCipherName(nsACString& aCipherName) override; \
  [[nodiscard]] NS_IMETHOD GetKeyLength(uint32_t *aKeyLength) override; \
  [[nodiscard]] NS_IMETHOD GetSecretKeyLength(uint32_t *aSecretKeyLength) override; \
  [[nodiscard]] NS_IMETHOD GetKeaGroupName(nsACString& aKeaGroupName) override; \
  [[nodiscard]] NS_IMETHOD GetSignatureSchemeName(nsACString& aSignatureSchemeName) override; \
  [[nodiscard]] NS_IMETHOD GetProtocolVersion(uint16_t *aProtocolVersion) override; \
  [[nodiscard]] NS_IMETHOD GetCertificateTransparencyStatus(uint16_t *aCertificateTransparencyStatus) override; \
  [[nodiscard]] NS_IMETHOD GetIsAcceptedEch(bool *aIsAcceptedEch) override; \
  [[nodiscard]] NS_IMETHOD GetIsDelegatedCredential(bool *aIsDelegatedCredential) override; \
  [[nodiscard]] NS_IMETHOD GetIsDomainMismatch(bool *aIsDomainMismatch) override; \
  [[nodiscard]] NS_IMETHOD GetIsNotValidAtThisTime(bool *aIsNotValidAtThisTime) override; \
  [[nodiscard]] NS_IMETHOD GetIsUntrusted(bool *aIsUntrusted) override; \
  [[nodiscard]] NS_IMETHOD GetIsExtendedValidation(bool *aIsExtendedValidation) override; \
  NS_IMETHOD_(void) SerializeToIPC(IPC::Message * aMsg) override; \
  NS_IMETHOD_(bool) DeserializeFromIPC(const IPC::Message * aMsg, PickleIterator * aIter) override; \
  NS_IMETHOD GetNegotiatedNPN(nsACString& aNegotiatedNPN) override; \
  NS_IMETHOD GetResumed(bool *aResumed) override; \
  NS_IMETHOD GetIsBuiltCertChainRootBuiltInRoot(bool *aIsBuiltCertChainRootBuiltInRoot) override; \
  NS_IMETHOD SetIsBuiltCertChainRootBuiltInRoot(bool aIsBuiltCertChainRootBuiltInRoot) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITRANSPORTSECURITYINFO \
  nsresult GetSecurityState(uint32_t *aSecurityState); \
  nsresult GetErrorCode(int32_t *aErrorCode); \
  nsresult GetErrorCodeString(nsAString& aErrorCodeString); \
  nsresult GetFailedCertChain(nsTArray<RefPtr<nsIX509Cert>>& aFailedCertChain); \
  nsresult GetServerCert(nsIX509Cert **aServerCert); \
  nsresult GetSucceededCertChain(nsTArray<RefPtr<nsIX509Cert>>& aSucceededCertChain); \
  [[nodiscard]] nsresult GetCipherName(nsACString& aCipherName); \
  [[nodiscard]] nsresult GetKeyLength(uint32_t *aKeyLength); \
  [[nodiscard]] nsresult GetSecretKeyLength(uint32_t *aSecretKeyLength); \
  [[nodiscard]] nsresult GetKeaGroupName(nsACString& aKeaGroupName); \
  [[nodiscard]] nsresult GetSignatureSchemeName(nsACString& aSignatureSchemeName); \
  [[nodiscard]] nsresult GetProtocolVersion(uint16_t *aProtocolVersion); \
  [[nodiscard]] nsresult GetCertificateTransparencyStatus(uint16_t *aCertificateTransparencyStatus); \
  [[nodiscard]] nsresult GetIsAcceptedEch(bool *aIsAcceptedEch); \
  [[nodiscard]] nsresult GetIsDelegatedCredential(bool *aIsDelegatedCredential); \
  [[nodiscard]] nsresult GetIsDomainMismatch(bool *aIsDomainMismatch); \
  [[nodiscard]] nsresult GetIsNotValidAtThisTime(bool *aIsNotValidAtThisTime); \
  [[nodiscard]] nsresult GetIsUntrusted(bool *aIsUntrusted); \
  [[nodiscard]] nsresult GetIsExtendedValidation(bool *aIsExtendedValidation); \
  nsresult_(void) SerializeToIPC(IPC::Message * aMsg); \
  nsresult_(bool) DeserializeFromIPC(const IPC::Message * aMsg, PickleIterator * aIter); \
  nsresult GetNegotiatedNPN(nsACString& aNegotiatedNPN); \
  nsresult GetResumed(bool *aResumed); \
  nsresult GetIsBuiltCertChainRootBuiltInRoot(bool *aIsBuiltCertChainRootBuiltInRoot); \
  nsresult SetIsBuiltCertChainRootBuiltInRoot(bool aIsBuiltCertChainRootBuiltInRoot); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITRANSPORTSECURITYINFO(_to) \
  NS_IMETHOD GetSecurityState(uint32_t *aSecurityState) override { return _to GetSecurityState(aSecurityState); } \
  NS_IMETHOD GetErrorCode(int32_t *aErrorCode) override { return _to GetErrorCode(aErrorCode); } \
  NS_IMETHOD GetErrorCodeString(nsAString& aErrorCodeString) override { return _to GetErrorCodeString(aErrorCodeString); } \
  NS_IMETHOD GetFailedCertChain(nsTArray<RefPtr<nsIX509Cert>>& aFailedCertChain) override { return _to GetFailedCertChain(aFailedCertChain); } \
  NS_IMETHOD GetServerCert(nsIX509Cert **aServerCert) override { return _to GetServerCert(aServerCert); } \
  NS_IMETHOD GetSucceededCertChain(nsTArray<RefPtr<nsIX509Cert>>& aSucceededCertChain) override { return _to GetSucceededCertChain(aSucceededCertChain); } \
  [[nodiscard]] NS_IMETHOD GetCipherName(nsACString& aCipherName) override { return _to GetCipherName(aCipherName); } \
  [[nodiscard]] NS_IMETHOD GetKeyLength(uint32_t *aKeyLength) override { return _to GetKeyLength(aKeyLength); } \
  [[nodiscard]] NS_IMETHOD GetSecretKeyLength(uint32_t *aSecretKeyLength) override { return _to GetSecretKeyLength(aSecretKeyLength); } \
  [[nodiscard]] NS_IMETHOD GetKeaGroupName(nsACString& aKeaGroupName) override { return _to GetKeaGroupName(aKeaGroupName); } \
  [[nodiscard]] NS_IMETHOD GetSignatureSchemeName(nsACString& aSignatureSchemeName) override { return _to GetSignatureSchemeName(aSignatureSchemeName); } \
  [[nodiscard]] NS_IMETHOD GetProtocolVersion(uint16_t *aProtocolVersion) override { return _to GetProtocolVersion(aProtocolVersion); } \
  [[nodiscard]] NS_IMETHOD GetCertificateTransparencyStatus(uint16_t *aCertificateTransparencyStatus) override { return _to GetCertificateTransparencyStatus(aCertificateTransparencyStatus); } \
  [[nodiscard]] NS_IMETHOD GetIsAcceptedEch(bool *aIsAcceptedEch) override { return _to GetIsAcceptedEch(aIsAcceptedEch); } \
  [[nodiscard]] NS_IMETHOD GetIsDelegatedCredential(bool *aIsDelegatedCredential) override { return _to GetIsDelegatedCredential(aIsDelegatedCredential); } \
  [[nodiscard]] NS_IMETHOD GetIsDomainMismatch(bool *aIsDomainMismatch) override { return _to GetIsDomainMismatch(aIsDomainMismatch); } \
  [[nodiscard]] NS_IMETHOD GetIsNotValidAtThisTime(bool *aIsNotValidAtThisTime) override { return _to GetIsNotValidAtThisTime(aIsNotValidAtThisTime); } \
  [[nodiscard]] NS_IMETHOD GetIsUntrusted(bool *aIsUntrusted) override { return _to GetIsUntrusted(aIsUntrusted); } \
  [[nodiscard]] NS_IMETHOD GetIsExtendedValidation(bool *aIsExtendedValidation) override { return _to GetIsExtendedValidation(aIsExtendedValidation); } \
  NS_IMETHOD_(void) SerializeToIPC(IPC::Message * aMsg) override { return _to SerializeToIPC(aMsg); } \
  NS_IMETHOD_(bool) DeserializeFromIPC(const IPC::Message * aMsg, PickleIterator * aIter) override { return _to DeserializeFromIPC(aMsg, aIter); } \
  NS_IMETHOD GetNegotiatedNPN(nsACString& aNegotiatedNPN) override { return _to GetNegotiatedNPN(aNegotiatedNPN); } \
  NS_IMETHOD GetResumed(bool *aResumed) override { return _to GetResumed(aResumed); } \
  NS_IMETHOD GetIsBuiltCertChainRootBuiltInRoot(bool *aIsBuiltCertChainRootBuiltInRoot) override { return _to GetIsBuiltCertChainRootBuiltInRoot(aIsBuiltCertChainRootBuiltInRoot); } \
  NS_IMETHOD SetIsBuiltCertChainRootBuiltInRoot(bool aIsBuiltCertChainRootBuiltInRoot) override { return _to SetIsBuiltCertChainRootBuiltInRoot(aIsBuiltCertChainRootBuiltInRoot); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITRANSPORTSECURITYINFO(_to) \
  NS_IMETHOD GetSecurityState(uint32_t *aSecurityState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecurityState(aSecurityState); } \
  NS_IMETHOD GetErrorCode(int32_t *aErrorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetErrorCode(aErrorCode); } \
  NS_IMETHOD GetErrorCodeString(nsAString& aErrorCodeString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetErrorCodeString(aErrorCodeString); } \
  NS_IMETHOD GetFailedCertChain(nsTArray<RefPtr<nsIX509Cert>>& aFailedCertChain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFailedCertChain(aFailedCertChain); } \
  NS_IMETHOD GetServerCert(nsIX509Cert **aServerCert) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetServerCert(aServerCert); } \
  NS_IMETHOD GetSucceededCertChain(nsTArray<RefPtr<nsIX509Cert>>& aSucceededCertChain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSucceededCertChain(aSucceededCertChain); } \
  [[nodiscard]] NS_IMETHOD GetCipherName(nsACString& aCipherName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCipherName(aCipherName); } \
  [[nodiscard]] NS_IMETHOD GetKeyLength(uint32_t *aKeyLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeyLength(aKeyLength); } \
  [[nodiscard]] NS_IMETHOD GetSecretKeyLength(uint32_t *aSecretKeyLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecretKeyLength(aSecretKeyLength); } \
  [[nodiscard]] NS_IMETHOD GetKeaGroupName(nsACString& aKeaGroupName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeaGroupName(aKeaGroupName); } \
  [[nodiscard]] NS_IMETHOD GetSignatureSchemeName(nsACString& aSignatureSchemeName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSignatureSchemeName(aSignatureSchemeName); } \
  [[nodiscard]] NS_IMETHOD GetProtocolVersion(uint16_t *aProtocolVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProtocolVersion(aProtocolVersion); } \
  [[nodiscard]] NS_IMETHOD GetCertificateTransparencyStatus(uint16_t *aCertificateTransparencyStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCertificateTransparencyStatus(aCertificateTransparencyStatus); } \
  [[nodiscard]] NS_IMETHOD GetIsAcceptedEch(bool *aIsAcceptedEch) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsAcceptedEch(aIsAcceptedEch); } \
  [[nodiscard]] NS_IMETHOD GetIsDelegatedCredential(bool *aIsDelegatedCredential) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsDelegatedCredential(aIsDelegatedCredential); } \
  [[nodiscard]] NS_IMETHOD GetIsDomainMismatch(bool *aIsDomainMismatch) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsDomainMismatch(aIsDomainMismatch); } \
  [[nodiscard]] NS_IMETHOD GetIsNotValidAtThisTime(bool *aIsNotValidAtThisTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsNotValidAtThisTime(aIsNotValidAtThisTime); } \
  [[nodiscard]] NS_IMETHOD GetIsUntrusted(bool *aIsUntrusted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsUntrusted(aIsUntrusted); } \
  [[nodiscard]] NS_IMETHOD GetIsExtendedValidation(bool *aIsExtendedValidation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsExtendedValidation(aIsExtendedValidation); } \
  NS_IMETHOD_(void) SerializeToIPC(IPC::Message * aMsg) override; \
  NS_IMETHOD_(bool) DeserializeFromIPC(const IPC::Message * aMsg, PickleIterator * aIter) override; \
  NS_IMETHOD GetNegotiatedNPN(nsACString& aNegotiatedNPN) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNegotiatedNPN(aNegotiatedNPN); } \
  NS_IMETHOD GetResumed(bool *aResumed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResumed(aResumed); } \
  NS_IMETHOD GetIsBuiltCertChainRootBuiltInRoot(bool *aIsBuiltCertChainRootBuiltInRoot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsBuiltCertChainRootBuiltInRoot(aIsBuiltCertChainRootBuiltInRoot); } \
  NS_IMETHOD SetIsBuiltCertChainRootBuiltInRoot(bool aIsBuiltCertChainRootBuiltInRoot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsBuiltCertChainRootBuiltInRoot(aIsBuiltCertChainRootBuiltInRoot); } 


#endif /* __gen_nsITransportSecurityInfo_h__ */
