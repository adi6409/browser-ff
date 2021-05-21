/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIX509CertDB.idl
 */

#ifndef __gen_nsIX509CertDB_h__
#define __gen_nsIX509CertDB_h__


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
class nsIArray; /* forward declaration */

class nsIX509Cert; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIInterfaceRequestor; /* forward declaration */

class nsIZipReader; /* forward declaration */

class nsIInputStream; /* forward declaration */

#define NS_X509CERTDB_CONTRACTID "@mozilla.org/security/x509certdb;1"
typedef uint32_t  AppTrustedRoot;


/* starting interface:    nsIOpenSignedAppFileCallback */
#define NS_IOPENSIGNEDAPPFILECALLBACK_IID_STR "fc2b60e5-9a07-47c2-a2cd-b83b68a660ac"

#define NS_IOPENSIGNEDAPPFILECALLBACK_IID \
  {0xfc2b60e5, 0x9a07, 0x47c2, \
    { 0xa2, 0xcd, 0xb8, 0x3b, 0x68, 0xa6, 0x60, 0xac }}

class NS_NO_VTABLE nsIOpenSignedAppFileCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IOPENSIGNEDAPPFILECALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIOpenSignedAppFileCallback;

  /* void openSignedAppFileFinished (in nsresult rv, in nsIZipReader aZipReader, in nsIX509Cert aSignerCert); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenSignedAppFileFinished(nsresult rv, nsIZipReader *aZipReader, nsIX509Cert *aSignerCert) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIOpenSignedAppFileCallback, NS_IOPENSIGNEDAPPFILECALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIOPENSIGNEDAPPFILECALLBACK \
  NS_IMETHOD OpenSignedAppFileFinished(nsresult rv, nsIZipReader *aZipReader, nsIX509Cert *aSignerCert) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIOPENSIGNEDAPPFILECALLBACK \
  nsresult OpenSignedAppFileFinished(nsresult rv, nsIZipReader *aZipReader, nsIX509Cert *aSignerCert); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIOPENSIGNEDAPPFILECALLBACK(_to) \
  NS_IMETHOD OpenSignedAppFileFinished(nsresult rv, nsIZipReader *aZipReader, nsIX509Cert *aSignerCert) override { return _to OpenSignedAppFileFinished(rv, aZipReader, aSignerCert); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIOPENSIGNEDAPPFILECALLBACK(_to) \
  NS_IMETHOD OpenSignedAppFileFinished(nsresult rv, nsIZipReader *aZipReader, nsIX509Cert *aSignerCert) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenSignedAppFileFinished(rv, aZipReader, aSignerCert); } 


/* starting interface:    nsIAsyncBoolCallback */
#define NS_IASYNCBOOLCALLBACK_IID_STR "07c08655-8b11-4650-b6c4-0c145595ceb5"

#define NS_IASYNCBOOLCALLBACK_IID \
  {0x07c08655, 0x8b11, 0x4650, \
    { 0xb6, 0xc4, 0x0c, 0x14, 0x55, 0x95, 0xce, 0xb5 }}

class NS_NO_VTABLE nsIAsyncBoolCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IASYNCBOOLCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAsyncBoolCallback;

  /* void onResult (in bool result); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnResult(bool result) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAsyncBoolCallback, NS_IASYNCBOOLCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIASYNCBOOLCALLBACK \
  NS_IMETHOD OnResult(bool result) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIASYNCBOOLCALLBACK \
  nsresult OnResult(bool result); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIASYNCBOOLCALLBACK(_to) \
  NS_IMETHOD OnResult(bool result) override { return _to OnResult(result); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIASYNCBOOLCALLBACK(_to) \
  NS_IMETHOD OnResult(bool result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnResult(result); } 


/* starting interface:    nsICertVerificationCallback */
#define NS_ICERTVERIFICATIONCALLBACK_IID_STR "49e16fc8-efac-4f57-8361-956ef6b960a4"

#define NS_ICERTVERIFICATIONCALLBACK_IID \
  {0x49e16fc8, 0xefac, 0x4f57, \
    { 0x83, 0x61, 0x95, 0x6e, 0xf6, 0xb9, 0x60, 0xa4 }}

class NS_NO_VTABLE nsICertVerificationCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICERTVERIFICATIONCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICertVerificationCallback;

  /* void verifyCertFinished (in int32_t aPRErrorCode, in Array<nsIX509Cert> aVerifiedChain, in bool aHasEVPolicy); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD VerifyCertFinished(int32_t aPRErrorCode, const nsTArray<RefPtr<nsIX509Cert>>& aVerifiedChain, bool aHasEVPolicy) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICertVerificationCallback, NS_ICERTVERIFICATIONCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICERTVERIFICATIONCALLBACK \
  NS_IMETHOD VerifyCertFinished(int32_t aPRErrorCode, const nsTArray<RefPtr<nsIX509Cert>>& aVerifiedChain, bool aHasEVPolicy) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICERTVERIFICATIONCALLBACK \
  nsresult VerifyCertFinished(int32_t aPRErrorCode, const nsTArray<RefPtr<nsIX509Cert>>& aVerifiedChain, bool aHasEVPolicy); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICERTVERIFICATIONCALLBACK(_to) \
  NS_IMETHOD VerifyCertFinished(int32_t aPRErrorCode, const nsTArray<RefPtr<nsIX509Cert>>& aVerifiedChain, bool aHasEVPolicy) override { return _to VerifyCertFinished(aPRErrorCode, aVerifiedChain, aHasEVPolicy); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICERTVERIFICATIONCALLBACK(_to) \
  NS_IMETHOD VerifyCertFinished(int32_t aPRErrorCode, const nsTArray<RefPtr<nsIX509Cert>>& aVerifiedChain, bool aHasEVPolicy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VerifyCertFinished(aPRErrorCode, aVerifiedChain, aHasEVPolicy); } 


/* starting interface:    nsIX509CertDB */
#define NS_IX509CERTDB_IID_STR "5c16cd9b-5a73-47f1-ab0f-11ede7495cce"

#define NS_IX509CERTDB_IID \
  {0x5c16cd9b, 0x5a73, 0x47f1, \
    { 0xab, 0x0f, 0x11, 0xed, 0xe7, 0x49, 0x5c, 0xce }}

class NS_NO_VTABLE nsIX509CertDB : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IX509CERTDB_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIX509CertDB;

  enum {
    UNTRUSTED = 0U,
    TRUSTED_SSL = 1U,
    TRUSTED_EMAIL = 2U
  };

  /* [must_use] nsIX509Cert findCertByDBKey (in ACString aDBkey); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD FindCertByDBKey(const nsACString& aDBkey, nsIX509Cert **_retval) = 0;

  /* void importCertificates ([array, size_is (length)] in octet data, in unsigned long length, in unsigned long type, in nsIInterfaceRequestor ctx); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ImportCertificates(uint8_t *data, uint32_t length, uint32_t type, nsIInterfaceRequestor *ctx) = 0;

  /* void importEmailCertificate ([array, size_is (length)] in octet data, in unsigned long length, in nsIInterfaceRequestor ctx); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ImportEmailCertificate(uint8_t *data, uint32_t length, nsIInterfaceRequestor *ctx) = 0;

  /* void importUserCertificate ([array, size_is (length)] in octet data, in unsigned long length, in nsIInterfaceRequestor ctx); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ImportUserCertificate(uint8_t *data, uint32_t length, nsIInterfaceRequestor *ctx) = 0;

  /* void deleteCertificate (in nsIX509Cert aCert); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteCertificate(nsIX509Cert *aCert) = 0;

  /* [must_use] void setCertTrust (in nsIX509Cert cert, in unsigned long type, in unsigned long trust); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD SetCertTrust(nsIX509Cert *cert, uint32_t type, uint32_t trust) = 0;

  /* [must_use] void setCertTrustFromString (in nsIX509Cert cert, in ACString trustString); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD SetCertTrustFromString(nsIX509Cert *cert, const nsACString& trustString) = 0;

  /* [must_use] boolean isCertTrusted (in nsIX509Cert cert, in unsigned long certType, in unsigned long trustType); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD IsCertTrusted(nsIX509Cert *cert, uint32_t certType, uint32_t trustType, bool *_retval) = 0;

  /* [must_use] void importCertsFromFile (in nsIFile aFile, in unsigned long aType); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ImportCertsFromFile(nsIFile *aFile, uint32_t aType) = 0;

  enum {
    Success = 0U,
    ERROR_UNKNOWN = 1U,
    ERROR_PKCS12_NOSMARTCARD_EXPORT = 2U,
    ERROR_PKCS12_RESTORE_FAILED = 3U,
    ERROR_PKCS12_BACKUP_FAILED = 4U,
    ERROR_PKCS12_CERT_COLLISION = 5U,
    ERROR_BAD_PASSWORD = 6U,
    ERROR_DECODE_ERROR = 7U,
    ERROR_PKCS12_DUPLICATE_DATA = 8U
  };

  /* [must_use] uint32_t importPKCS12File (in nsIFile aFile, in AString aPassword); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ImportPKCS12File(nsIFile *aFile, const nsAString& aPassword, uint32_t *_retval) = 0;

  /* [must_use] uint32_t exportPKCS12File (in nsIFile aFile, in Array<nsIX509Cert> aCerts, in AString aPassword); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ExportPKCS12File(nsIFile *aFile, const nsTArray<RefPtr<nsIX509Cert>>& aCerts, const nsAString& aPassword, uint32_t *_retval) = 0;

  /* [must_use] nsIX509Cert constructX509FromBase64 (in ACString base64); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ConstructX509FromBase64(const nsACString& base64, nsIX509Cert **_retval) = 0;

  /* [must_use] nsIX509Cert constructX509 (in Array<uint8_t> certDER); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ConstructX509(const nsTArray<uint8_t >& certDER, nsIX509Cert **_retval) = 0;

  enum {
    AppXPCShellRoot = 6U,
    AddonsPublicRoot = 7U,
    AddonsStageRoot = 8U
  };

  /* [must_use] void openSignedAppFileAsync (in AppTrustedRoot trustedRoot, in nsIFile aJarFile, in nsIOpenSignedAppFileCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD OpenSignedAppFileAsync(AppTrustedRoot trustedRoot, nsIFile *aJarFile, nsIOpenSignedAppFileCallback *callback) = 0;

  /* [must_use] nsIX509Cert addCert (in ACString certDER, in ACString trust); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AddCert(const nsACString& certDER, const nsACString& trust, nsIX509Cert **_retval) = 0;

  enum {
    FLAG_LOCAL_ONLY = 1U,
    FLAG_MUST_BE_EV = 2U
  };

  /* [must_use] void asyncVerifyCertAtTime (in nsIX509Cert aCert, in int64_t aUsage, in uint32_t aFlags, in ACString aHostname, in uint64_t aTime, in nsICertVerificationCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AsyncVerifyCertAtTime(nsIX509Cert *aCert, int64_t aUsage, uint32_t aFlags, const nsACString& aHostname, uint64_t aTime, nsICertVerificationCallback *aCallback) = 0;

  /* [must_use] void clearOCSPCache (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ClearOCSPCache(void) = 0;

  /* [must_use] nsIX509Cert addCertFromBase64 (in ACString base64, in ACString trust); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AddCertFromBase64(const nsACString& base64, const nsACString& trust, nsIX509Cert **_retval) = 0;

  /* [must_use] Array<nsIX509Cert> getCerts (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetCerts(nsTArray<RefPtr<nsIX509Cert>>& _retval) = 0;

  /* [must_use] ACString asPKCS7Blob (in Array<nsIX509Cert> certList); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AsPKCS7Blob(const nsTArray<RefPtr<nsIX509Cert>>& certList, nsACString& _retval) = 0;

  /* [must_use] void asyncHasThirdPartyRoots (in nsIAsyncBoolCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AsyncHasThirdPartyRoots(nsIAsyncBoolCallback *callback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIX509CertDB, NS_IX509CERTDB_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIX509CERTDB \
  [[nodiscard]] NS_IMETHOD FindCertByDBKey(const nsACString& aDBkey, nsIX509Cert **_retval) override; \
  NS_IMETHOD ImportCertificates(uint8_t *data, uint32_t length, uint32_t type, nsIInterfaceRequestor *ctx) override; \
  NS_IMETHOD ImportEmailCertificate(uint8_t *data, uint32_t length, nsIInterfaceRequestor *ctx) override; \
  NS_IMETHOD ImportUserCertificate(uint8_t *data, uint32_t length, nsIInterfaceRequestor *ctx) override; \
  NS_IMETHOD DeleteCertificate(nsIX509Cert *aCert) override; \
  [[nodiscard]] NS_IMETHOD SetCertTrust(nsIX509Cert *cert, uint32_t type, uint32_t trust) override; \
  [[nodiscard]] NS_IMETHOD SetCertTrustFromString(nsIX509Cert *cert, const nsACString& trustString) override; \
  [[nodiscard]] NS_IMETHOD IsCertTrusted(nsIX509Cert *cert, uint32_t certType, uint32_t trustType, bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD ImportCertsFromFile(nsIFile *aFile, uint32_t aType) override; \
  [[nodiscard]] NS_IMETHOD ImportPKCS12File(nsIFile *aFile, const nsAString& aPassword, uint32_t *_retval) override; \
  [[nodiscard]] NS_IMETHOD ExportPKCS12File(nsIFile *aFile, const nsTArray<RefPtr<nsIX509Cert>>& aCerts, const nsAString& aPassword, uint32_t *_retval) override; \
  [[nodiscard]] NS_IMETHOD ConstructX509FromBase64(const nsACString& base64, nsIX509Cert **_retval) override; \
  [[nodiscard]] NS_IMETHOD ConstructX509(const nsTArray<uint8_t >& certDER, nsIX509Cert **_retval) override; \
  [[nodiscard]] NS_IMETHOD OpenSignedAppFileAsync(AppTrustedRoot trustedRoot, nsIFile *aJarFile, nsIOpenSignedAppFileCallback *callback) override; \
  [[nodiscard]] NS_IMETHOD AddCert(const nsACString& certDER, const nsACString& trust, nsIX509Cert **_retval) override; \
  [[nodiscard]] NS_IMETHOD AsyncVerifyCertAtTime(nsIX509Cert *aCert, int64_t aUsage, uint32_t aFlags, const nsACString& aHostname, uint64_t aTime, nsICertVerificationCallback *aCallback) override; \
  [[nodiscard]] NS_IMETHOD ClearOCSPCache(void) override; \
  [[nodiscard]] NS_IMETHOD AddCertFromBase64(const nsACString& base64, const nsACString& trust, nsIX509Cert **_retval) override; \
  [[nodiscard]] NS_IMETHOD GetCerts(nsTArray<RefPtr<nsIX509Cert>>& _retval) override; \
  [[nodiscard]] NS_IMETHOD AsPKCS7Blob(const nsTArray<RefPtr<nsIX509Cert>>& certList, nsACString& _retval) override; \
  [[nodiscard]] NS_IMETHOD AsyncHasThirdPartyRoots(nsIAsyncBoolCallback *callback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIX509CERTDB \
  [[nodiscard]] nsresult FindCertByDBKey(const nsACString& aDBkey, nsIX509Cert **_retval); \
  nsresult ImportCertificates(uint8_t *data, uint32_t length, uint32_t type, nsIInterfaceRequestor *ctx); \
  nsresult ImportEmailCertificate(uint8_t *data, uint32_t length, nsIInterfaceRequestor *ctx); \
  nsresult ImportUserCertificate(uint8_t *data, uint32_t length, nsIInterfaceRequestor *ctx); \
  nsresult DeleteCertificate(nsIX509Cert *aCert); \
  [[nodiscard]] nsresult SetCertTrust(nsIX509Cert *cert, uint32_t type, uint32_t trust); \
  [[nodiscard]] nsresult SetCertTrustFromString(nsIX509Cert *cert, const nsACString& trustString); \
  [[nodiscard]] nsresult IsCertTrusted(nsIX509Cert *cert, uint32_t certType, uint32_t trustType, bool *_retval); \
  [[nodiscard]] nsresult ImportCertsFromFile(nsIFile *aFile, uint32_t aType); \
  [[nodiscard]] nsresult ImportPKCS12File(nsIFile *aFile, const nsAString& aPassword, uint32_t *_retval); \
  [[nodiscard]] nsresult ExportPKCS12File(nsIFile *aFile, const nsTArray<RefPtr<nsIX509Cert>>& aCerts, const nsAString& aPassword, uint32_t *_retval); \
  [[nodiscard]] nsresult ConstructX509FromBase64(const nsACString& base64, nsIX509Cert **_retval); \
  [[nodiscard]] nsresult ConstructX509(const nsTArray<uint8_t >& certDER, nsIX509Cert **_retval); \
  [[nodiscard]] nsresult OpenSignedAppFileAsync(AppTrustedRoot trustedRoot, nsIFile *aJarFile, nsIOpenSignedAppFileCallback *callback); \
  [[nodiscard]] nsresult AddCert(const nsACString& certDER, const nsACString& trust, nsIX509Cert **_retval); \
  [[nodiscard]] nsresult AsyncVerifyCertAtTime(nsIX509Cert *aCert, int64_t aUsage, uint32_t aFlags, const nsACString& aHostname, uint64_t aTime, nsICertVerificationCallback *aCallback); \
  [[nodiscard]] nsresult ClearOCSPCache(void); \
  [[nodiscard]] nsresult AddCertFromBase64(const nsACString& base64, const nsACString& trust, nsIX509Cert **_retval); \
  [[nodiscard]] nsresult GetCerts(nsTArray<RefPtr<nsIX509Cert>>& _retval); \
  [[nodiscard]] nsresult AsPKCS7Blob(const nsTArray<RefPtr<nsIX509Cert>>& certList, nsACString& _retval); \
  [[nodiscard]] nsresult AsyncHasThirdPartyRoots(nsIAsyncBoolCallback *callback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIX509CERTDB(_to) \
  [[nodiscard]] NS_IMETHOD FindCertByDBKey(const nsACString& aDBkey, nsIX509Cert **_retval) override { return _to FindCertByDBKey(aDBkey, _retval); } \
  NS_IMETHOD ImportCertificates(uint8_t *data, uint32_t length, uint32_t type, nsIInterfaceRequestor *ctx) override { return _to ImportCertificates(data, length, type, ctx); } \
  NS_IMETHOD ImportEmailCertificate(uint8_t *data, uint32_t length, nsIInterfaceRequestor *ctx) override { return _to ImportEmailCertificate(data, length, ctx); } \
  NS_IMETHOD ImportUserCertificate(uint8_t *data, uint32_t length, nsIInterfaceRequestor *ctx) override { return _to ImportUserCertificate(data, length, ctx); } \
  NS_IMETHOD DeleteCertificate(nsIX509Cert *aCert) override { return _to DeleteCertificate(aCert); } \
  [[nodiscard]] NS_IMETHOD SetCertTrust(nsIX509Cert *cert, uint32_t type, uint32_t trust) override { return _to SetCertTrust(cert, type, trust); } \
  [[nodiscard]] NS_IMETHOD SetCertTrustFromString(nsIX509Cert *cert, const nsACString& trustString) override { return _to SetCertTrustFromString(cert, trustString); } \
  [[nodiscard]] NS_IMETHOD IsCertTrusted(nsIX509Cert *cert, uint32_t certType, uint32_t trustType, bool *_retval) override { return _to IsCertTrusted(cert, certType, trustType, _retval); } \
  [[nodiscard]] NS_IMETHOD ImportCertsFromFile(nsIFile *aFile, uint32_t aType) override { return _to ImportCertsFromFile(aFile, aType); } \
  [[nodiscard]] NS_IMETHOD ImportPKCS12File(nsIFile *aFile, const nsAString& aPassword, uint32_t *_retval) override { return _to ImportPKCS12File(aFile, aPassword, _retval); } \
  [[nodiscard]] NS_IMETHOD ExportPKCS12File(nsIFile *aFile, const nsTArray<RefPtr<nsIX509Cert>>& aCerts, const nsAString& aPassword, uint32_t *_retval) override { return _to ExportPKCS12File(aFile, aCerts, aPassword, _retval); } \
  [[nodiscard]] NS_IMETHOD ConstructX509FromBase64(const nsACString& base64, nsIX509Cert **_retval) override { return _to ConstructX509FromBase64(base64, _retval); } \
  [[nodiscard]] NS_IMETHOD ConstructX509(const nsTArray<uint8_t >& certDER, nsIX509Cert **_retval) override { return _to ConstructX509(certDER, _retval); } \
  [[nodiscard]] NS_IMETHOD OpenSignedAppFileAsync(AppTrustedRoot trustedRoot, nsIFile *aJarFile, nsIOpenSignedAppFileCallback *callback) override { return _to OpenSignedAppFileAsync(trustedRoot, aJarFile, callback); } \
  [[nodiscard]] NS_IMETHOD AddCert(const nsACString& certDER, const nsACString& trust, nsIX509Cert **_retval) override { return _to AddCert(certDER, trust, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncVerifyCertAtTime(nsIX509Cert *aCert, int64_t aUsage, uint32_t aFlags, const nsACString& aHostname, uint64_t aTime, nsICertVerificationCallback *aCallback) override { return _to AsyncVerifyCertAtTime(aCert, aUsage, aFlags, aHostname, aTime, aCallback); } \
  [[nodiscard]] NS_IMETHOD ClearOCSPCache(void) override { return _to ClearOCSPCache(); } \
  [[nodiscard]] NS_IMETHOD AddCertFromBase64(const nsACString& base64, const nsACString& trust, nsIX509Cert **_retval) override { return _to AddCertFromBase64(base64, trust, _retval); } \
  [[nodiscard]] NS_IMETHOD GetCerts(nsTArray<RefPtr<nsIX509Cert>>& _retval) override { return _to GetCerts(_retval); } \
  [[nodiscard]] NS_IMETHOD AsPKCS7Blob(const nsTArray<RefPtr<nsIX509Cert>>& certList, nsACString& _retval) override { return _to AsPKCS7Blob(certList, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncHasThirdPartyRoots(nsIAsyncBoolCallback *callback) override { return _to AsyncHasThirdPartyRoots(callback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIX509CERTDB(_to) \
  [[nodiscard]] NS_IMETHOD FindCertByDBKey(const nsACString& aDBkey, nsIX509Cert **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FindCertByDBKey(aDBkey, _retval); } \
  NS_IMETHOD ImportCertificates(uint8_t *data, uint32_t length, uint32_t type, nsIInterfaceRequestor *ctx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ImportCertificates(data, length, type, ctx); } \
  NS_IMETHOD ImportEmailCertificate(uint8_t *data, uint32_t length, nsIInterfaceRequestor *ctx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ImportEmailCertificate(data, length, ctx); } \
  NS_IMETHOD ImportUserCertificate(uint8_t *data, uint32_t length, nsIInterfaceRequestor *ctx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ImportUserCertificate(data, length, ctx); } \
  NS_IMETHOD DeleteCertificate(nsIX509Cert *aCert) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteCertificate(aCert); } \
  [[nodiscard]] NS_IMETHOD SetCertTrust(nsIX509Cert *cert, uint32_t type, uint32_t trust) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCertTrust(cert, type, trust); } \
  [[nodiscard]] NS_IMETHOD SetCertTrustFromString(nsIX509Cert *cert, const nsACString& trustString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCertTrustFromString(cert, trustString); } \
  [[nodiscard]] NS_IMETHOD IsCertTrusted(nsIX509Cert *cert, uint32_t certType, uint32_t trustType, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCertTrusted(cert, certType, trustType, _retval); } \
  [[nodiscard]] NS_IMETHOD ImportCertsFromFile(nsIFile *aFile, uint32_t aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ImportCertsFromFile(aFile, aType); } \
  [[nodiscard]] NS_IMETHOD ImportPKCS12File(nsIFile *aFile, const nsAString& aPassword, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ImportPKCS12File(aFile, aPassword, _retval); } \
  [[nodiscard]] NS_IMETHOD ExportPKCS12File(nsIFile *aFile, const nsTArray<RefPtr<nsIX509Cert>>& aCerts, const nsAString& aPassword, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExportPKCS12File(aFile, aCerts, aPassword, _retval); } \
  [[nodiscard]] NS_IMETHOD ConstructX509FromBase64(const nsACString& base64, nsIX509Cert **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConstructX509FromBase64(base64, _retval); } \
  [[nodiscard]] NS_IMETHOD ConstructX509(const nsTArray<uint8_t >& certDER, nsIX509Cert **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConstructX509(certDER, _retval); } \
  [[nodiscard]] NS_IMETHOD OpenSignedAppFileAsync(AppTrustedRoot trustedRoot, nsIFile *aJarFile, nsIOpenSignedAppFileCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenSignedAppFileAsync(trustedRoot, aJarFile, callback); } \
  [[nodiscard]] NS_IMETHOD AddCert(const nsACString& certDER, const nsACString& trust, nsIX509Cert **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddCert(certDER, trust, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncVerifyCertAtTime(nsIX509Cert *aCert, int64_t aUsage, uint32_t aFlags, const nsACString& aHostname, uint64_t aTime, nsICertVerificationCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncVerifyCertAtTime(aCert, aUsage, aFlags, aHostname, aTime, aCallback); } \
  [[nodiscard]] NS_IMETHOD ClearOCSPCache(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearOCSPCache(); } \
  [[nodiscard]] NS_IMETHOD AddCertFromBase64(const nsACString& base64, const nsACString& trust, nsIX509Cert **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddCertFromBase64(base64, trust, _retval); } \
  [[nodiscard]] NS_IMETHOD GetCerts(nsTArray<RefPtr<nsIX509Cert>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCerts(_retval); } \
  [[nodiscard]] NS_IMETHOD AsPKCS7Blob(const nsTArray<RefPtr<nsIX509Cert>>& certList, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsPKCS7Blob(certList, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncHasThirdPartyRoots(nsIAsyncBoolCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncHasThirdPartyRoots(callback); } 


#endif /* __gen_nsIX509CertDB_h__ */
