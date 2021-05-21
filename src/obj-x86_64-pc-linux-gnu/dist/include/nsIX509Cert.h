/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIX509Cert.idl
 */

#ifndef __gen_nsIX509Cert_h__
#define __gen_nsIX509Cert_h__


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
class nsIArray; /* forward declaration */

class nsIX509CertValidity; /* forward declaration */

class nsICertVerificationListener; /* forward declaration */

namespace IPC {
  class Message;
}
class PickleIterator;
 /* forward declaration */
 typedef struct CERTCertificateStr CERTCertificate;

/* starting interface:    nsIX509Cert */
#define NS_IX509CERT_IID_STR "bdc3979a-5422-4cd5-8589-696b6e96ea83"

#define NS_IX509CERT_IID \
  {0xbdc3979a, 0x5422, 0x4cd5, \
    { 0x85, 0x89, 0x69, 0x6b, 0x6e, 0x96, 0xea, 0x83 }}

class NS_NO_VTABLE nsIX509Cert : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IX509CERT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIX509Cert;

  /* readonly attribute AString emailAddress; */
  NS_IMETHOD GetEmailAddress(nsAString& aEmailAddress) = 0;

  /* [must_use] readonly attribute bool isBuiltInRoot; */
  [[nodiscard]] NS_IMETHOD GetIsBuiltInRoot(bool *aIsBuiltInRoot) = 0;

  /* [must_use] Array<AString> getEmailAddresses (); */
  [[nodiscard]] NS_IMETHOD GetEmailAddresses(nsTArray<nsString >& _retval) = 0;

  /* [must_use] boolean containsEmailAddress (in AString aEmailAddress); */
  [[nodiscard]] NS_IMETHOD ContainsEmailAddress(const nsAString& aEmailAddress, bool *_retval) = 0;

  /* readonly attribute AString subjectName; */
  NS_IMETHOD GetSubjectName(nsAString& aSubjectName) = 0;

  /* readonly attribute AString commonName; */
  NS_IMETHOD GetCommonName(nsAString& aCommonName) = 0;

  /* readonly attribute AString organization; */
  NS_IMETHOD GetOrganization(nsAString& aOrganization) = 0;

  /* [must_use] readonly attribute AString organizationalUnit; */
  [[nodiscard]] NS_IMETHOD GetOrganizationalUnit(nsAString& aOrganizationalUnit) = 0;

  /* readonly attribute AString sha256Fingerprint; */
  NS_IMETHOD GetSha256Fingerprint(nsAString& aSha256Fingerprint) = 0;

  /* [must_use] readonly attribute AString sha1Fingerprint; */
  [[nodiscard]] NS_IMETHOD GetSha1Fingerprint(nsAString& aSha1Fingerprint) = 0;

  /* readonly attribute AString tokenName; */
  NS_IMETHOD GetTokenName(nsAString& aTokenName) = 0;

  /* readonly attribute AString issuerName; */
  NS_IMETHOD GetIssuerName(nsAString& aIssuerName) = 0;

  /* [must_use] readonly attribute AString serialNumber; */
  [[nodiscard]] NS_IMETHOD GetSerialNumber(nsAString& aSerialNumber) = 0;

  /* [must_use] readonly attribute AString issuerCommonName; */
  [[nodiscard]] NS_IMETHOD GetIssuerCommonName(nsAString& aIssuerCommonName) = 0;

  /* readonly attribute AString issuerOrganization; */
  NS_IMETHOD GetIssuerOrganization(nsAString& aIssuerOrganization) = 0;

  /* [must_use] readonly attribute AString issuerOrganizationUnit; */
  [[nodiscard]] NS_IMETHOD GetIssuerOrganizationUnit(nsAString& aIssuerOrganizationUnit) = 0;

  /* readonly attribute nsIX509CertValidity validity; */
  NS_IMETHOD GetValidity(nsIX509CertValidity **aValidity) = 0;

  /* [must_use] readonly attribute ACString dbKey; */
  [[nodiscard]] NS_IMETHOD GetDbKey(nsACString& aDbKey) = 0;

  /* [must_use] readonly attribute AString displayName; */
  [[nodiscard]] NS_IMETHOD GetDisplayName(nsAString& aDisplayName) = 0;

  enum {
    UNKNOWN_CERT = 0U,
    CA_CERT = 1U,
    USER_CERT = 2U,
    EMAIL_CERT = 4U,
    SERVER_CERT = 8U,
    ANY_CERT = 65535U
  };

  /* readonly attribute unsigned long certType; */
  NS_IMETHOD GetCertType(uint32_t *aCertType) = 0;

  /* [must_use] readonly attribute AString keyUsages; */
  [[nodiscard]] NS_IMETHOD GetKeyUsages(nsAString& aKeyUsages) = 0;

  /* [must_use] Array<octet> getRawDER (); */
  [[nodiscard]] NS_IMETHOD GetRawDER(nsTArray<uint8_t >& _retval) = 0;

  /* [must_use] ACString getBase64DERString (); */
  [[nodiscard]] NS_IMETHOD GetBase64DERString(nsACString& _retval) = 0;

  /* [must_use] boolean equals (in nsIX509Cert other); */
  [[nodiscard]] NS_IMETHOD Equals(nsIX509Cert *other, bool *_retval) = 0;

  /* [must_use] readonly attribute ACString sha256SubjectPublicKeyInfoDigest; */
  [[nodiscard]] NS_IMETHOD GetSha256SubjectPublicKeyInfoDigest(nsACString& aSha256SubjectPublicKeyInfoDigest) = 0;

  /* [must_use,noscript,notxpcom] CERTCertificatePtr getCert (); */
  [[nodiscard]] NS_IMETHOD_(CERTCertificate *) GetCert(void) = 0;

  /* [noscript,notxpcom] void SerializeToIPC (in IpcMessagePtr aMsg); */
  NS_IMETHOD_(void) SerializeToIPC(IPC::Message * aMsg) = 0;

  /* [noscript,notxpcom] bool DeserializeFromIPC ([const] in IpcMessagePtr aMsg, in PickleIteratorPtr aIter); */
  NS_IMETHOD_(bool) DeserializeFromIPC(const IPC::Message * aMsg, PickleIterator * aIter) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIX509Cert, NS_IX509CERT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIX509CERT \
  NS_IMETHOD GetEmailAddress(nsAString& aEmailAddress) override; \
  [[nodiscard]] NS_IMETHOD GetIsBuiltInRoot(bool *aIsBuiltInRoot) override; \
  [[nodiscard]] NS_IMETHOD GetEmailAddresses(nsTArray<nsString >& _retval) override; \
  [[nodiscard]] NS_IMETHOD ContainsEmailAddress(const nsAString& aEmailAddress, bool *_retval) override; \
  NS_IMETHOD GetSubjectName(nsAString& aSubjectName) override; \
  NS_IMETHOD GetCommonName(nsAString& aCommonName) override; \
  NS_IMETHOD GetOrganization(nsAString& aOrganization) override; \
  [[nodiscard]] NS_IMETHOD GetOrganizationalUnit(nsAString& aOrganizationalUnit) override; \
  NS_IMETHOD GetSha256Fingerprint(nsAString& aSha256Fingerprint) override; \
  [[nodiscard]] NS_IMETHOD GetSha1Fingerprint(nsAString& aSha1Fingerprint) override; \
  NS_IMETHOD GetTokenName(nsAString& aTokenName) override; \
  NS_IMETHOD GetIssuerName(nsAString& aIssuerName) override; \
  [[nodiscard]] NS_IMETHOD GetSerialNumber(nsAString& aSerialNumber) override; \
  [[nodiscard]] NS_IMETHOD GetIssuerCommonName(nsAString& aIssuerCommonName) override; \
  NS_IMETHOD GetIssuerOrganization(nsAString& aIssuerOrganization) override; \
  [[nodiscard]] NS_IMETHOD GetIssuerOrganizationUnit(nsAString& aIssuerOrganizationUnit) override; \
  NS_IMETHOD GetValidity(nsIX509CertValidity **aValidity) override; \
  [[nodiscard]] NS_IMETHOD GetDbKey(nsACString& aDbKey) override; \
  [[nodiscard]] NS_IMETHOD GetDisplayName(nsAString& aDisplayName) override; \
  NS_IMETHOD GetCertType(uint32_t *aCertType) override; \
  [[nodiscard]] NS_IMETHOD GetKeyUsages(nsAString& aKeyUsages) override; \
  [[nodiscard]] NS_IMETHOD GetRawDER(nsTArray<uint8_t >& _retval) override; \
  [[nodiscard]] NS_IMETHOD GetBase64DERString(nsACString& _retval) override; \
  [[nodiscard]] NS_IMETHOD Equals(nsIX509Cert *other, bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD GetSha256SubjectPublicKeyInfoDigest(nsACString& aSha256SubjectPublicKeyInfoDigest) override; \
  [[nodiscard]] NS_IMETHOD_(CERTCertificate *) GetCert(void) override; \
  NS_IMETHOD_(void) SerializeToIPC(IPC::Message * aMsg) override; \
  NS_IMETHOD_(bool) DeserializeFromIPC(const IPC::Message * aMsg, PickleIterator * aIter) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIX509CERT \
  nsresult GetEmailAddress(nsAString& aEmailAddress); \
  [[nodiscard]] nsresult GetIsBuiltInRoot(bool *aIsBuiltInRoot); \
  [[nodiscard]] nsresult GetEmailAddresses(nsTArray<nsString >& _retval); \
  [[nodiscard]] nsresult ContainsEmailAddress(const nsAString& aEmailAddress, bool *_retval); \
  nsresult GetSubjectName(nsAString& aSubjectName); \
  nsresult GetCommonName(nsAString& aCommonName); \
  nsresult GetOrganization(nsAString& aOrganization); \
  [[nodiscard]] nsresult GetOrganizationalUnit(nsAString& aOrganizationalUnit); \
  nsresult GetSha256Fingerprint(nsAString& aSha256Fingerprint); \
  [[nodiscard]] nsresult GetSha1Fingerprint(nsAString& aSha1Fingerprint); \
  nsresult GetTokenName(nsAString& aTokenName); \
  nsresult GetIssuerName(nsAString& aIssuerName); \
  [[nodiscard]] nsresult GetSerialNumber(nsAString& aSerialNumber); \
  [[nodiscard]] nsresult GetIssuerCommonName(nsAString& aIssuerCommonName); \
  nsresult GetIssuerOrganization(nsAString& aIssuerOrganization); \
  [[nodiscard]] nsresult GetIssuerOrganizationUnit(nsAString& aIssuerOrganizationUnit); \
  nsresult GetValidity(nsIX509CertValidity **aValidity); \
  [[nodiscard]] nsresult GetDbKey(nsACString& aDbKey); \
  [[nodiscard]] nsresult GetDisplayName(nsAString& aDisplayName); \
  nsresult GetCertType(uint32_t *aCertType); \
  [[nodiscard]] nsresult GetKeyUsages(nsAString& aKeyUsages); \
  [[nodiscard]] nsresult GetRawDER(nsTArray<uint8_t >& _retval); \
  [[nodiscard]] nsresult GetBase64DERString(nsACString& _retval); \
  [[nodiscard]] nsresult Equals(nsIX509Cert *other, bool *_retval); \
  [[nodiscard]] nsresult GetSha256SubjectPublicKeyInfoDigest(nsACString& aSha256SubjectPublicKeyInfoDigest); \
  [[nodiscard]] nsresult_(CERTCertificate *) GetCert(void); \
  nsresult_(void) SerializeToIPC(IPC::Message * aMsg); \
  nsresult_(bool) DeserializeFromIPC(const IPC::Message * aMsg, PickleIterator * aIter); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIX509CERT(_to) \
  NS_IMETHOD GetEmailAddress(nsAString& aEmailAddress) override { return _to GetEmailAddress(aEmailAddress); } \
  [[nodiscard]] NS_IMETHOD GetIsBuiltInRoot(bool *aIsBuiltInRoot) override { return _to GetIsBuiltInRoot(aIsBuiltInRoot); } \
  [[nodiscard]] NS_IMETHOD GetEmailAddresses(nsTArray<nsString >& _retval) override { return _to GetEmailAddresses(_retval); } \
  [[nodiscard]] NS_IMETHOD ContainsEmailAddress(const nsAString& aEmailAddress, bool *_retval) override { return _to ContainsEmailAddress(aEmailAddress, _retval); } \
  NS_IMETHOD GetSubjectName(nsAString& aSubjectName) override { return _to GetSubjectName(aSubjectName); } \
  NS_IMETHOD GetCommonName(nsAString& aCommonName) override { return _to GetCommonName(aCommonName); } \
  NS_IMETHOD GetOrganization(nsAString& aOrganization) override { return _to GetOrganization(aOrganization); } \
  [[nodiscard]] NS_IMETHOD GetOrganizationalUnit(nsAString& aOrganizationalUnit) override { return _to GetOrganizationalUnit(aOrganizationalUnit); } \
  NS_IMETHOD GetSha256Fingerprint(nsAString& aSha256Fingerprint) override { return _to GetSha256Fingerprint(aSha256Fingerprint); } \
  [[nodiscard]] NS_IMETHOD GetSha1Fingerprint(nsAString& aSha1Fingerprint) override { return _to GetSha1Fingerprint(aSha1Fingerprint); } \
  NS_IMETHOD GetTokenName(nsAString& aTokenName) override { return _to GetTokenName(aTokenName); } \
  NS_IMETHOD GetIssuerName(nsAString& aIssuerName) override { return _to GetIssuerName(aIssuerName); } \
  [[nodiscard]] NS_IMETHOD GetSerialNumber(nsAString& aSerialNumber) override { return _to GetSerialNumber(aSerialNumber); } \
  [[nodiscard]] NS_IMETHOD GetIssuerCommonName(nsAString& aIssuerCommonName) override { return _to GetIssuerCommonName(aIssuerCommonName); } \
  NS_IMETHOD GetIssuerOrganization(nsAString& aIssuerOrganization) override { return _to GetIssuerOrganization(aIssuerOrganization); } \
  [[nodiscard]] NS_IMETHOD GetIssuerOrganizationUnit(nsAString& aIssuerOrganizationUnit) override { return _to GetIssuerOrganizationUnit(aIssuerOrganizationUnit); } \
  NS_IMETHOD GetValidity(nsIX509CertValidity **aValidity) override { return _to GetValidity(aValidity); } \
  [[nodiscard]] NS_IMETHOD GetDbKey(nsACString& aDbKey) override { return _to GetDbKey(aDbKey); } \
  [[nodiscard]] NS_IMETHOD GetDisplayName(nsAString& aDisplayName) override { return _to GetDisplayName(aDisplayName); } \
  NS_IMETHOD GetCertType(uint32_t *aCertType) override { return _to GetCertType(aCertType); } \
  [[nodiscard]] NS_IMETHOD GetKeyUsages(nsAString& aKeyUsages) override { return _to GetKeyUsages(aKeyUsages); } \
  [[nodiscard]] NS_IMETHOD GetRawDER(nsTArray<uint8_t >& _retval) override { return _to GetRawDER(_retval); } \
  [[nodiscard]] NS_IMETHOD GetBase64DERString(nsACString& _retval) override { return _to GetBase64DERString(_retval); } \
  [[nodiscard]] NS_IMETHOD Equals(nsIX509Cert *other, bool *_retval) override { return _to Equals(other, _retval); } \
  [[nodiscard]] NS_IMETHOD GetSha256SubjectPublicKeyInfoDigest(nsACString& aSha256SubjectPublicKeyInfoDigest) override { return _to GetSha256SubjectPublicKeyInfoDigest(aSha256SubjectPublicKeyInfoDigest); } \
  [[nodiscard]] NS_IMETHOD_(CERTCertificate *) GetCert(void) override { return _to GetCert(); } \
  NS_IMETHOD_(void) SerializeToIPC(IPC::Message * aMsg) override { return _to SerializeToIPC(aMsg); } \
  NS_IMETHOD_(bool) DeserializeFromIPC(const IPC::Message * aMsg, PickleIterator * aIter) override { return _to DeserializeFromIPC(aMsg, aIter); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIX509CERT(_to) \
  NS_IMETHOD GetEmailAddress(nsAString& aEmailAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEmailAddress(aEmailAddress); } \
  [[nodiscard]] NS_IMETHOD GetIsBuiltInRoot(bool *aIsBuiltInRoot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsBuiltInRoot(aIsBuiltInRoot); } \
  [[nodiscard]] NS_IMETHOD GetEmailAddresses(nsTArray<nsString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEmailAddresses(_retval); } \
  [[nodiscard]] NS_IMETHOD ContainsEmailAddress(const nsAString& aEmailAddress, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ContainsEmailAddress(aEmailAddress, _retval); } \
  NS_IMETHOD GetSubjectName(nsAString& aSubjectName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSubjectName(aSubjectName); } \
  NS_IMETHOD GetCommonName(nsAString& aCommonName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCommonName(aCommonName); } \
  NS_IMETHOD GetOrganization(nsAString& aOrganization) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOrganization(aOrganization); } \
  [[nodiscard]] NS_IMETHOD GetOrganizationalUnit(nsAString& aOrganizationalUnit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOrganizationalUnit(aOrganizationalUnit); } \
  NS_IMETHOD GetSha256Fingerprint(nsAString& aSha256Fingerprint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSha256Fingerprint(aSha256Fingerprint); } \
  [[nodiscard]] NS_IMETHOD GetSha1Fingerprint(nsAString& aSha1Fingerprint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSha1Fingerprint(aSha1Fingerprint); } \
  NS_IMETHOD GetTokenName(nsAString& aTokenName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTokenName(aTokenName); } \
  NS_IMETHOD GetIssuerName(nsAString& aIssuerName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIssuerName(aIssuerName); } \
  [[nodiscard]] NS_IMETHOD GetSerialNumber(nsAString& aSerialNumber) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSerialNumber(aSerialNumber); } \
  [[nodiscard]] NS_IMETHOD GetIssuerCommonName(nsAString& aIssuerCommonName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIssuerCommonName(aIssuerCommonName); } \
  NS_IMETHOD GetIssuerOrganization(nsAString& aIssuerOrganization) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIssuerOrganization(aIssuerOrganization); } \
  [[nodiscard]] NS_IMETHOD GetIssuerOrganizationUnit(nsAString& aIssuerOrganizationUnit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIssuerOrganizationUnit(aIssuerOrganizationUnit); } \
  NS_IMETHOD GetValidity(nsIX509CertValidity **aValidity) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValidity(aValidity); } \
  [[nodiscard]] NS_IMETHOD GetDbKey(nsACString& aDbKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDbKey(aDbKey); } \
  [[nodiscard]] NS_IMETHOD GetDisplayName(nsAString& aDisplayName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplayName(aDisplayName); } \
  NS_IMETHOD GetCertType(uint32_t *aCertType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCertType(aCertType); } \
  [[nodiscard]] NS_IMETHOD GetKeyUsages(nsAString& aKeyUsages) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeyUsages(aKeyUsages); } \
  [[nodiscard]] NS_IMETHOD GetRawDER(nsTArray<uint8_t >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRawDER(_retval); } \
  [[nodiscard]] NS_IMETHOD GetBase64DERString(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBase64DERString(_retval); } \
  [[nodiscard]] NS_IMETHOD Equals(nsIX509Cert *other, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Equals(other, _retval); } \
  [[nodiscard]] NS_IMETHOD GetSha256SubjectPublicKeyInfoDigest(nsACString& aSha256SubjectPublicKeyInfoDigest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSha256SubjectPublicKeyInfoDigest(aSha256SubjectPublicKeyInfoDigest); } \
  [[nodiscard]] NS_IMETHOD_(CERTCertificate *) GetCert(void) override; \
  NS_IMETHOD_(void) SerializeToIPC(IPC::Message * aMsg) override; \
  NS_IMETHOD_(bool) DeserializeFromIPC(const IPC::Message * aMsg, PickleIterator * aIter) override; 


#endif /* __gen_nsIX509Cert_h__ */
