/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICertStorage.idl
 */

#ifndef __gen_nsICertStorage_h__
#define __gen_nsICertStorage_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIVariant_h__
#include "nsIVariant.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#define NS_CERTSTORAGE_CONTRACTID "@mozilla.org/security/certstorage;1"

/* starting interface:    nsICertStorageCallback */
#define NS_ICERTSTORAGECALLBACK_IID_STR "3f8fe26a-a436-4ad4-9c1c-a53c60973c31"

#define NS_ICERTSTORAGECALLBACK_IID \
  {0x3f8fe26a, 0xa436, 0x4ad4, \
    { 0x9c, 0x1c, 0xa5, 0x3c, 0x60, 0x97, 0x3c, 0x31 }}

class NS_NO_VTABLE nsICertStorageCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICERTSTORAGECALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICertStorageCallback;

  /* [must_use] void done (in nsresult rv, in nsIVariant result); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD Done(nsresult rv, nsIVariant *result) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICertStorageCallback, NS_ICERTSTORAGECALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICERTSTORAGECALLBACK \
  [[nodiscard]] NS_IMETHOD Done(nsresult rv, nsIVariant *result) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICERTSTORAGECALLBACK \
  [[nodiscard]] nsresult Done(nsresult rv, nsIVariant *result); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICERTSTORAGECALLBACK(_to) \
  [[nodiscard]] NS_IMETHOD Done(nsresult rv, nsIVariant *result) override { return _to Done(rv, result); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICERTSTORAGECALLBACK(_to) \
  [[nodiscard]] NS_IMETHOD Done(nsresult rv, nsIVariant *result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Done(rv, result); } 


/* starting interface:    nsIRevocationState */
#define NS_IREVOCATIONSTATE_IID_STR "96db6fd7-6b64-4a5a-955d-310bd9ca4234"

#define NS_IREVOCATIONSTATE_IID \
  {0x96db6fd7, 0x6b64, 0x4a5a, \
    { 0x95, 0x5d, 0x31, 0x0b, 0xd9, 0xca, 0x42, 0x34 }}

class NS_NO_VTABLE nsIRevocationState : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREVOCATIONSTATE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIRevocationState;

  /* readonly attribute short state; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetState(int16_t *aState) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRevocationState, NS_IREVOCATIONSTATE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREVOCATIONSTATE \
  NS_IMETHOD GetState(int16_t *aState) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREVOCATIONSTATE \
  nsresult GetState(int16_t *aState); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREVOCATIONSTATE(_to) \
  NS_IMETHOD GetState(int16_t *aState) override { return _to GetState(aState); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREVOCATIONSTATE(_to) \
  NS_IMETHOD GetState(int16_t *aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetState(aState); } 


/* starting interface:    nsIIssuerAndSerialRevocationState */
#define NS_IISSUERANDSERIALREVOCATIONSTATE_IID_STR "23ce3546-f1b9-46f6-8de3-77704da5702f"

#define NS_IISSUERANDSERIALREVOCATIONSTATE_IID \
  {0x23ce3546, 0xf1b9, 0x46f6, \
    { 0x8d, 0xe3, 0x77, 0x70, 0x4d, 0xa5, 0x70, 0x2f }}

class NS_NO_VTABLE nsIIssuerAndSerialRevocationState : public nsIRevocationState {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IISSUERANDSERIALREVOCATIONSTATE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIIssuerAndSerialRevocationState;

  /* readonly attribute ACString issuer; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIssuer(nsACString& aIssuer) = 0;

  /* readonly attribute ACString serial; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSerial(nsACString& aSerial) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIIssuerAndSerialRevocationState, NS_IISSUERANDSERIALREVOCATIONSTATE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIISSUERANDSERIALREVOCATIONSTATE \
  NS_IMETHOD GetIssuer(nsACString& aIssuer) override; \
  NS_IMETHOD GetSerial(nsACString& aSerial) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIISSUERANDSERIALREVOCATIONSTATE \
  nsresult GetIssuer(nsACString& aIssuer); \
  nsresult GetSerial(nsACString& aSerial); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIISSUERANDSERIALREVOCATIONSTATE(_to) \
  NS_IMETHOD GetIssuer(nsACString& aIssuer) override { return _to GetIssuer(aIssuer); } \
  NS_IMETHOD GetSerial(nsACString& aSerial) override { return _to GetSerial(aSerial); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIISSUERANDSERIALREVOCATIONSTATE(_to) \
  NS_IMETHOD GetIssuer(nsACString& aIssuer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIssuer(aIssuer); } \
  NS_IMETHOD GetSerial(nsACString& aSerial) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSerial(aSerial); } 


/* starting interface:    nsISubjectAndPubKeyRevocationState */
#define NS_ISUBJECTANDPUBKEYREVOCATIONSTATE_IID_STR "e78b51b4-6fa4-41e2-92ce-e9404f541e96"

#define NS_ISUBJECTANDPUBKEYREVOCATIONSTATE_IID \
  {0xe78b51b4, 0x6fa4, 0x41e2, \
    { 0x92, 0xce, 0xe9, 0x40, 0x4f, 0x54, 0x1e, 0x96 }}

class NS_NO_VTABLE nsISubjectAndPubKeyRevocationState : public nsIRevocationState {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISUBJECTANDPUBKEYREVOCATIONSTATE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISubjectAndPubKeyRevocationState;

  /* readonly attribute ACString subject; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSubject(nsACString& aSubject) = 0;

  /* readonly attribute ACString pubKey; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPubKey(nsACString& aPubKey) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISubjectAndPubKeyRevocationState, NS_ISUBJECTANDPUBKEYREVOCATIONSTATE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISUBJECTANDPUBKEYREVOCATIONSTATE \
  NS_IMETHOD GetSubject(nsACString& aSubject) override; \
  NS_IMETHOD GetPubKey(nsACString& aPubKey) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISUBJECTANDPUBKEYREVOCATIONSTATE \
  nsresult GetSubject(nsACString& aSubject); \
  nsresult GetPubKey(nsACString& aPubKey); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISUBJECTANDPUBKEYREVOCATIONSTATE(_to) \
  NS_IMETHOD GetSubject(nsACString& aSubject) override { return _to GetSubject(aSubject); } \
  NS_IMETHOD GetPubKey(nsACString& aPubKey) override { return _to GetPubKey(aPubKey); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISUBJECTANDPUBKEYREVOCATIONSTATE(_to) \
  NS_IMETHOD GetSubject(nsACString& aSubject) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSubject(aSubject); } \
  NS_IMETHOD GetPubKey(nsACString& aPubKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPubKey(aPubKey); } 


/* starting interface:    nsICRLiteState */
#define NS_ICRLITESTATE_IID_STR "5d0d22be-185f-4cf0-b73b-c5a911273e77"

#define NS_ICRLITESTATE_IID \
  {0x5d0d22be, 0x185f, 0x4cf0, \
    { 0xb7, 0x3b, 0xc5, 0xa9, 0x11, 0x27, 0x3e, 0x77 }}

class NS_NO_VTABLE nsICRLiteState : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICRLITESTATE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICRLiteState;

  /* readonly attribute ACString subject; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSubject(nsACString& aSubject) = 0;

  /* readonly attribute ACString spkiHash; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSpkiHash(nsACString& aSpkiHash) = 0;

  /* readonly attribute short state; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetState(int16_t *aState) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICRLiteState, NS_ICRLITESTATE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICRLITESTATE \
  NS_IMETHOD GetSubject(nsACString& aSubject) override; \
  NS_IMETHOD GetSpkiHash(nsACString& aSpkiHash) override; \
  NS_IMETHOD GetState(int16_t *aState) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICRLITESTATE \
  nsresult GetSubject(nsACString& aSubject); \
  nsresult GetSpkiHash(nsACString& aSpkiHash); \
  nsresult GetState(int16_t *aState); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICRLITESTATE(_to) \
  NS_IMETHOD GetSubject(nsACString& aSubject) override { return _to GetSubject(aSubject); } \
  NS_IMETHOD GetSpkiHash(nsACString& aSpkiHash) override { return _to GetSpkiHash(aSpkiHash); } \
  NS_IMETHOD GetState(int16_t *aState) override { return _to GetState(aState); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICRLITESTATE(_to) \
  NS_IMETHOD GetSubject(nsACString& aSubject) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSubject(aSubject); } \
  NS_IMETHOD GetSpkiHash(nsACString& aSpkiHash) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSpkiHash(aSpkiHash); } \
  NS_IMETHOD GetState(int16_t *aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetState(aState); } 


/* starting interface:    nsICertInfo */
#define NS_ICERTINFO_IID_STR "27b66f5e-0faf-403b-95b4-bc11691ac50d"

#define NS_ICERTINFO_IID \
  {0x27b66f5e, 0x0faf, 0x403b, \
    { 0x95, 0xb4, 0xbc, 0x11, 0x69, 0x1a, 0xc5, 0x0d }}

class NS_NO_VTABLE nsICertInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICERTINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICertInfo;

  /* readonly attribute ACString cert; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCert(nsACString& aCert) = 0;

  /* readonly attribute ACString subject; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSubject(nsACString& aSubject) = 0;

  /* readonly attribute short trust; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTrust(int16_t *aTrust) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICertInfo, NS_ICERTINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICERTINFO \
  NS_IMETHOD GetCert(nsACString& aCert) override; \
  NS_IMETHOD GetSubject(nsACString& aSubject) override; \
  NS_IMETHOD GetTrust(int16_t *aTrust) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICERTINFO \
  nsresult GetCert(nsACString& aCert); \
  nsresult GetSubject(nsACString& aSubject); \
  nsresult GetTrust(int16_t *aTrust); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICERTINFO(_to) \
  NS_IMETHOD GetCert(nsACString& aCert) override { return _to GetCert(aCert); } \
  NS_IMETHOD GetSubject(nsACString& aSubject) override { return _to GetSubject(aSubject); } \
  NS_IMETHOD GetTrust(int16_t *aTrust) override { return _to GetTrust(aTrust); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICERTINFO(_to) \
  NS_IMETHOD GetCert(nsACString& aCert) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCert(aCert); } \
  NS_IMETHOD GetSubject(nsACString& aSubject) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSubject(aSubject); } \
  NS_IMETHOD GetTrust(int16_t *aTrust) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTrust(aTrust); } 


/* starting interface:    nsICertStorage */
#define NS_ICERTSTORAGE_IID_STR "327100a7-3401-45ef-b160-bf880f1016fd"

#define NS_ICERTSTORAGE_IID \
  {0x327100a7, 0x3401, 0x45ef, \
    { 0xb1, 0x60, 0xbf, 0x88, 0x0f, 0x10, 0x16, 0xfd }}

class NS_NO_VTABLE nsICertStorage : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICERTSTORAGE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICertStorage;

  enum {
    DATA_TYPE_REVOCATION = 1U,
    DATA_TYPE_CERTIFICATE = 2U,
    DATA_TYPE_CRLITE = 3U,
    DATA_TYPE_CRLITE_FILTER_FULL = 4U,
    DATA_TYPE_CRLITE_FILTER_INCREMENTAL = 5U
  };

  /* [must_use] void hasPriorData (in octet type, in nsICertStorageCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD HasPriorData(uint8_t type, nsICertStorageCallback *callback) = 0;

  enum {
    STATE_UNSET = 0,
    STATE_ENFORCE = 1,
    STATE_NOT_ENROLLED = 2
  };

  /* [must_use] void setRevocations (in Array<nsIRevocationState> revocations, in nsICertStorageCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD SetRevocations(const nsTArray<RefPtr<nsIRevocationState>>& revocations, nsICertStorageCallback *callback) = 0;

  /* [must_use] short getRevocationState (in Array<octet> issuer, in Array<octet> serial, in Array<octet> subject, in Array<octet> pubkey); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetRevocationState(const nsTArray<uint8_t >& issuer, const nsTArray<uint8_t >& serial, const nsTArray<uint8_t >& subject, const nsTArray<uint8_t >& pubkey, int16_t *_retval) = 0;

  /* [must_use] boolean isBlocklistFresh (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD IsBlocklistFresh(bool *_retval) = 0;

  /* [must_use] void setCRLiteState (in Array<nsICRLiteState> crliteState, in nsICertStorageCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD SetCRLiteState(const nsTArray<RefPtr<nsICRLiteState>>& crliteState, nsICertStorageCallback *callback) = 0;

  /* [must_use] short getCRLiteState (in Array<octet> subject, in Array<octet> spki); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetCRLiteState(const nsTArray<uint8_t >& subject, const nsTArray<uint8_t >& spki, int16_t *_retval) = 0;

  /* [must_use] void setFullCRLiteFilter (in Array<octet> filter, in uint64_t timestamp, in nsICertStorageCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD SetFullCRLiteFilter(const nsTArray<uint8_t >& filter, uint64_t timestamp, nsICertStorageCallback *callback) = 0;

  /* [must_use] short getCRLiteRevocationState (in Array<octet> issuer, in Array<octet> issuerSPKI, in Array<octet> serialNumber, out uint64_t validBefore); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetCRLiteRevocationState(const nsTArray<uint8_t >& issuer, const nsTArray<uint8_t >& issuerSPKI, const nsTArray<uint8_t >& serialNumber, uint64_t *validBefore, int16_t *_retval) = 0;

  /* [must_use] void addCRLiteStash (in Array<octet> stash, in nsICertStorageCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AddCRLiteStash(const nsTArray<uint8_t >& stash, nsICertStorageCallback *callback) = 0;

  /* [must_use] bool isCertRevokedByStash (in Array<octet> issuerSPKI, in Array<octet> serialNumber); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD IsCertRevokedByStash(const nsTArray<uint8_t >& issuerSPKI, const nsTArray<uint8_t >& serialNumber, bool *_retval) = 0;

  enum {
    TRUST_INHERIT = 0,
    TRUST_ANCHOR = 1
  };

  /* [must_use] void addCerts (in Array<nsICertInfo> certs, in nsICertStorageCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AddCerts(const nsTArray<RefPtr<nsICertInfo>>& certs, nsICertStorageCallback *callback) = 0;

  /* [must_use] void removeCertsByHashes (in Array<ACString> hashes, in nsICertStorageCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD RemoveCertsByHashes(const nsTArray<nsCString >& hashes, nsICertStorageCallback *callback) = 0;

  /* [must_use] Array<Array<octet>> findCertsBySubject (in Array<octet> subject); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD FindCertsBySubject(const nsTArray<uint8_t >& subject, nsTArray<nsTArray<uint8_t >>& _retval) = 0;

  /* [must_use] int32_t GetRemainingOperationCount (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetRemainingOperationCount(int32_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICertStorage, NS_ICERTSTORAGE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICERTSTORAGE \
  [[nodiscard]] NS_IMETHOD HasPriorData(uint8_t type, nsICertStorageCallback *callback) override; \
  [[nodiscard]] NS_IMETHOD SetRevocations(const nsTArray<RefPtr<nsIRevocationState>>& revocations, nsICertStorageCallback *callback) override; \
  [[nodiscard]] NS_IMETHOD GetRevocationState(const nsTArray<uint8_t >& issuer, const nsTArray<uint8_t >& serial, const nsTArray<uint8_t >& subject, const nsTArray<uint8_t >& pubkey, int16_t *_retval) override; \
  [[nodiscard]] NS_IMETHOD IsBlocklistFresh(bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD SetCRLiteState(const nsTArray<RefPtr<nsICRLiteState>>& crliteState, nsICertStorageCallback *callback) override; \
  [[nodiscard]] NS_IMETHOD GetCRLiteState(const nsTArray<uint8_t >& subject, const nsTArray<uint8_t >& spki, int16_t *_retval) override; \
  [[nodiscard]] NS_IMETHOD SetFullCRLiteFilter(const nsTArray<uint8_t >& filter, uint64_t timestamp, nsICertStorageCallback *callback) override; \
  [[nodiscard]] NS_IMETHOD GetCRLiteRevocationState(const nsTArray<uint8_t >& issuer, const nsTArray<uint8_t >& issuerSPKI, const nsTArray<uint8_t >& serialNumber, uint64_t *validBefore, int16_t *_retval) override; \
  [[nodiscard]] NS_IMETHOD AddCRLiteStash(const nsTArray<uint8_t >& stash, nsICertStorageCallback *callback) override; \
  [[nodiscard]] NS_IMETHOD IsCertRevokedByStash(const nsTArray<uint8_t >& issuerSPKI, const nsTArray<uint8_t >& serialNumber, bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD AddCerts(const nsTArray<RefPtr<nsICertInfo>>& certs, nsICertStorageCallback *callback) override; \
  [[nodiscard]] NS_IMETHOD RemoveCertsByHashes(const nsTArray<nsCString >& hashes, nsICertStorageCallback *callback) override; \
  [[nodiscard]] NS_IMETHOD FindCertsBySubject(const nsTArray<uint8_t >& subject, nsTArray<nsTArray<uint8_t >>& _retval) override; \
  [[nodiscard]] NS_IMETHOD GetRemainingOperationCount(int32_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICERTSTORAGE \
  [[nodiscard]] nsresult HasPriorData(uint8_t type, nsICertStorageCallback *callback); \
  [[nodiscard]] nsresult SetRevocations(const nsTArray<RefPtr<nsIRevocationState>>& revocations, nsICertStorageCallback *callback); \
  [[nodiscard]] nsresult GetRevocationState(const nsTArray<uint8_t >& issuer, const nsTArray<uint8_t >& serial, const nsTArray<uint8_t >& subject, const nsTArray<uint8_t >& pubkey, int16_t *_retval); \
  [[nodiscard]] nsresult IsBlocklistFresh(bool *_retval); \
  [[nodiscard]] nsresult SetCRLiteState(const nsTArray<RefPtr<nsICRLiteState>>& crliteState, nsICertStorageCallback *callback); \
  [[nodiscard]] nsresult GetCRLiteState(const nsTArray<uint8_t >& subject, const nsTArray<uint8_t >& spki, int16_t *_retval); \
  [[nodiscard]] nsresult SetFullCRLiteFilter(const nsTArray<uint8_t >& filter, uint64_t timestamp, nsICertStorageCallback *callback); \
  [[nodiscard]] nsresult GetCRLiteRevocationState(const nsTArray<uint8_t >& issuer, const nsTArray<uint8_t >& issuerSPKI, const nsTArray<uint8_t >& serialNumber, uint64_t *validBefore, int16_t *_retval); \
  [[nodiscard]] nsresult AddCRLiteStash(const nsTArray<uint8_t >& stash, nsICertStorageCallback *callback); \
  [[nodiscard]] nsresult IsCertRevokedByStash(const nsTArray<uint8_t >& issuerSPKI, const nsTArray<uint8_t >& serialNumber, bool *_retval); \
  [[nodiscard]] nsresult AddCerts(const nsTArray<RefPtr<nsICertInfo>>& certs, nsICertStorageCallback *callback); \
  [[nodiscard]] nsresult RemoveCertsByHashes(const nsTArray<nsCString >& hashes, nsICertStorageCallback *callback); \
  [[nodiscard]] nsresult FindCertsBySubject(const nsTArray<uint8_t >& subject, nsTArray<nsTArray<uint8_t >>& _retval); \
  [[nodiscard]] nsresult GetRemainingOperationCount(int32_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICERTSTORAGE(_to) \
  [[nodiscard]] NS_IMETHOD HasPriorData(uint8_t type, nsICertStorageCallback *callback) override { return _to HasPriorData(type, callback); } \
  [[nodiscard]] NS_IMETHOD SetRevocations(const nsTArray<RefPtr<nsIRevocationState>>& revocations, nsICertStorageCallback *callback) override { return _to SetRevocations(revocations, callback); } \
  [[nodiscard]] NS_IMETHOD GetRevocationState(const nsTArray<uint8_t >& issuer, const nsTArray<uint8_t >& serial, const nsTArray<uint8_t >& subject, const nsTArray<uint8_t >& pubkey, int16_t *_retval) override { return _to GetRevocationState(issuer, serial, subject, pubkey, _retval); } \
  [[nodiscard]] NS_IMETHOD IsBlocklistFresh(bool *_retval) override { return _to IsBlocklistFresh(_retval); } \
  [[nodiscard]] NS_IMETHOD SetCRLiteState(const nsTArray<RefPtr<nsICRLiteState>>& crliteState, nsICertStorageCallback *callback) override { return _to SetCRLiteState(crliteState, callback); } \
  [[nodiscard]] NS_IMETHOD GetCRLiteState(const nsTArray<uint8_t >& subject, const nsTArray<uint8_t >& spki, int16_t *_retval) override { return _to GetCRLiteState(subject, spki, _retval); } \
  [[nodiscard]] NS_IMETHOD SetFullCRLiteFilter(const nsTArray<uint8_t >& filter, uint64_t timestamp, nsICertStorageCallback *callback) override { return _to SetFullCRLiteFilter(filter, timestamp, callback); } \
  [[nodiscard]] NS_IMETHOD GetCRLiteRevocationState(const nsTArray<uint8_t >& issuer, const nsTArray<uint8_t >& issuerSPKI, const nsTArray<uint8_t >& serialNumber, uint64_t *validBefore, int16_t *_retval) override { return _to GetCRLiteRevocationState(issuer, issuerSPKI, serialNumber, validBefore, _retval); } \
  [[nodiscard]] NS_IMETHOD AddCRLiteStash(const nsTArray<uint8_t >& stash, nsICertStorageCallback *callback) override { return _to AddCRLiteStash(stash, callback); } \
  [[nodiscard]] NS_IMETHOD IsCertRevokedByStash(const nsTArray<uint8_t >& issuerSPKI, const nsTArray<uint8_t >& serialNumber, bool *_retval) override { return _to IsCertRevokedByStash(issuerSPKI, serialNumber, _retval); } \
  [[nodiscard]] NS_IMETHOD AddCerts(const nsTArray<RefPtr<nsICertInfo>>& certs, nsICertStorageCallback *callback) override { return _to AddCerts(certs, callback); } \
  [[nodiscard]] NS_IMETHOD RemoveCertsByHashes(const nsTArray<nsCString >& hashes, nsICertStorageCallback *callback) override { return _to RemoveCertsByHashes(hashes, callback); } \
  [[nodiscard]] NS_IMETHOD FindCertsBySubject(const nsTArray<uint8_t >& subject, nsTArray<nsTArray<uint8_t >>& _retval) override { return _to FindCertsBySubject(subject, _retval); } \
  [[nodiscard]] NS_IMETHOD GetRemainingOperationCount(int32_t *_retval) override { return _to GetRemainingOperationCount(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICERTSTORAGE(_to) \
  [[nodiscard]] NS_IMETHOD HasPriorData(uint8_t type, nsICertStorageCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasPriorData(type, callback); } \
  [[nodiscard]] NS_IMETHOD SetRevocations(const nsTArray<RefPtr<nsIRevocationState>>& revocations, nsICertStorageCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRevocations(revocations, callback); } \
  [[nodiscard]] NS_IMETHOD GetRevocationState(const nsTArray<uint8_t >& issuer, const nsTArray<uint8_t >& serial, const nsTArray<uint8_t >& subject, const nsTArray<uint8_t >& pubkey, int16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRevocationState(issuer, serial, subject, pubkey, _retval); } \
  [[nodiscard]] NS_IMETHOD IsBlocklistFresh(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsBlocklistFresh(_retval); } \
  [[nodiscard]] NS_IMETHOD SetCRLiteState(const nsTArray<RefPtr<nsICRLiteState>>& crliteState, nsICertStorageCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCRLiteState(crliteState, callback); } \
  [[nodiscard]] NS_IMETHOD GetCRLiteState(const nsTArray<uint8_t >& subject, const nsTArray<uint8_t >& spki, int16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCRLiteState(subject, spki, _retval); } \
  [[nodiscard]] NS_IMETHOD SetFullCRLiteFilter(const nsTArray<uint8_t >& filter, uint64_t timestamp, nsICertStorageCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFullCRLiteFilter(filter, timestamp, callback); } \
  [[nodiscard]] NS_IMETHOD GetCRLiteRevocationState(const nsTArray<uint8_t >& issuer, const nsTArray<uint8_t >& issuerSPKI, const nsTArray<uint8_t >& serialNumber, uint64_t *validBefore, int16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCRLiteRevocationState(issuer, issuerSPKI, serialNumber, validBefore, _retval); } \
  [[nodiscard]] NS_IMETHOD AddCRLiteStash(const nsTArray<uint8_t >& stash, nsICertStorageCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddCRLiteStash(stash, callback); } \
  [[nodiscard]] NS_IMETHOD IsCertRevokedByStash(const nsTArray<uint8_t >& issuerSPKI, const nsTArray<uint8_t >& serialNumber, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCertRevokedByStash(issuerSPKI, serialNumber, _retval); } \
  [[nodiscard]] NS_IMETHOD AddCerts(const nsTArray<RefPtr<nsICertInfo>>& certs, nsICertStorageCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddCerts(certs, callback); } \
  [[nodiscard]] NS_IMETHOD RemoveCertsByHashes(const nsTArray<nsCString >& hashes, nsICertStorageCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveCertsByHashes(hashes, callback); } \
  [[nodiscard]] NS_IMETHOD FindCertsBySubject(const nsTArray<uint8_t >& subject, nsTArray<nsTArray<uint8_t >>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FindCertsBySubject(subject, _retval); } \
  [[nodiscard]] NS_IMETHOD GetRemainingOperationCount(int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRemainingOperationCount(_retval); } 


#endif /* __gen_nsICertStorage_h__ */
