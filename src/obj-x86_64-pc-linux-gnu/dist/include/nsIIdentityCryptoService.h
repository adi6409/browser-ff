/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/services/crypto/component/nsIIdentityCryptoService.idl
 */

#ifndef __gen_nsIIdentityCryptoService_h__
#define __gen_nsIIdentityCryptoService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIIdentityKeyGenCallback; /* forward declaration */

class nsIIdentitySignCallback; /* forward declaration */


/* starting interface:    nsIIdentityCryptoService */
#define NS_IIDENTITYCRYPTOSERVICE_IID_STR "f087e6bc-dd33-4f6c-a106-dd786e052ee9"

#define NS_IIDENTITYCRYPTOSERVICE_IID \
  {0xf087e6bc, 0xdd33, 0x4f6c, \
    { 0xa1, 0x06, 0xdd, 0x78, 0x6e, 0x05, 0x2e, 0xe9 }}

class NS_NO_VTABLE nsIIdentityCryptoService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IIDENTITYCRYPTOSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIIdentityCryptoService;

  /* void generateKeyPair (in AUTF8String algorithm, in nsIIdentityKeyGenCallback callback); */
  NS_IMETHOD GenerateKeyPair(const nsACString& algorithm, nsIIdentityKeyGenCallback *callback) = 0;

  /* ACString base64UrlEncode (in AUTF8String toEncode); */
  NS_IMETHOD Base64UrlEncode(const nsACString& toEncode, nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIIdentityCryptoService, NS_IIDENTITYCRYPTOSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIIDENTITYCRYPTOSERVICE \
  NS_IMETHOD GenerateKeyPair(const nsACString& algorithm, nsIIdentityKeyGenCallback *callback) override; \
  NS_IMETHOD Base64UrlEncode(const nsACString& toEncode, nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIIDENTITYCRYPTOSERVICE \
  nsresult GenerateKeyPair(const nsACString& algorithm, nsIIdentityKeyGenCallback *callback); \
  nsresult Base64UrlEncode(const nsACString& toEncode, nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIIDENTITYCRYPTOSERVICE(_to) \
  NS_IMETHOD GenerateKeyPair(const nsACString& algorithm, nsIIdentityKeyGenCallback *callback) override { return _to GenerateKeyPair(algorithm, callback); } \
  NS_IMETHOD Base64UrlEncode(const nsACString& toEncode, nsACString& _retval) override { return _to Base64UrlEncode(toEncode, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIIDENTITYCRYPTOSERVICE(_to) \
  NS_IMETHOD GenerateKeyPair(const nsACString& algorithm, nsIIdentityKeyGenCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GenerateKeyPair(algorithm, callback); } \
  NS_IMETHOD Base64UrlEncode(const nsACString& toEncode, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Base64UrlEncode(toEncode, _retval); } 


/* starting interface:    nsIIdentityKeyPair */
#define NS_IIDENTITYKEYPAIR_IID_STR "73962dc7-8ee7-4346-a12b-b039e1d9b54d"

#define NS_IIDENTITYKEYPAIR_IID \
  {0x73962dc7, 0x8ee7, 0x4346, \
    { 0xa1, 0x2b, 0xb0, 0x39, 0xe1, 0xd9, 0xb5, 0x4d }}

class NS_NO_VTABLE nsIIdentityKeyPair : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IIDENTITYKEYPAIR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIIdentityKeyPair;

  /* readonly attribute AUTF8String keyType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetKeyType(nsACString& aKeyType) = 0;

  /* readonly attribute AUTF8String hexRSAPublicKeyExponent; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHexRSAPublicKeyExponent(nsACString& aHexRSAPublicKeyExponent) = 0;

  /* readonly attribute AUTF8String hexRSAPublicKeyModulus; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHexRSAPublicKeyModulus(nsACString& aHexRSAPublicKeyModulus) = 0;

  /* readonly attribute AUTF8String hexDSAPrime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHexDSAPrime(nsACString& aHexDSAPrime) = 0;

  /* readonly attribute AUTF8String hexDSASubPrime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHexDSASubPrime(nsACString& aHexDSASubPrime) = 0;

  /* readonly attribute AUTF8String hexDSAGenerator; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHexDSAGenerator(nsACString& aHexDSAGenerator) = 0;

  /* readonly attribute AUTF8String hexDSAPublicValue; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHexDSAPublicValue(nsACString& aHexDSAPublicValue) = 0;

  /* void sign (in AUTF8String aText, in nsIIdentitySignCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Sign(const nsACString& aText, nsIIdentitySignCallback *callback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIIdentityKeyPair, NS_IIDENTITYKEYPAIR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIIDENTITYKEYPAIR \
  NS_IMETHOD GetKeyType(nsACString& aKeyType) override; \
  NS_IMETHOD GetHexRSAPublicKeyExponent(nsACString& aHexRSAPublicKeyExponent) override; \
  NS_IMETHOD GetHexRSAPublicKeyModulus(nsACString& aHexRSAPublicKeyModulus) override; \
  NS_IMETHOD GetHexDSAPrime(nsACString& aHexDSAPrime) override; \
  NS_IMETHOD GetHexDSASubPrime(nsACString& aHexDSASubPrime) override; \
  NS_IMETHOD GetHexDSAGenerator(nsACString& aHexDSAGenerator) override; \
  NS_IMETHOD GetHexDSAPublicValue(nsACString& aHexDSAPublicValue) override; \
  NS_IMETHOD Sign(const nsACString& aText, nsIIdentitySignCallback *callback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIIDENTITYKEYPAIR \
  nsresult GetKeyType(nsACString& aKeyType); \
  nsresult GetHexRSAPublicKeyExponent(nsACString& aHexRSAPublicKeyExponent); \
  nsresult GetHexRSAPublicKeyModulus(nsACString& aHexRSAPublicKeyModulus); \
  nsresult GetHexDSAPrime(nsACString& aHexDSAPrime); \
  nsresult GetHexDSASubPrime(nsACString& aHexDSASubPrime); \
  nsresult GetHexDSAGenerator(nsACString& aHexDSAGenerator); \
  nsresult GetHexDSAPublicValue(nsACString& aHexDSAPublicValue); \
  nsresult Sign(const nsACString& aText, nsIIdentitySignCallback *callback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIIDENTITYKEYPAIR(_to) \
  NS_IMETHOD GetKeyType(nsACString& aKeyType) override { return _to GetKeyType(aKeyType); } \
  NS_IMETHOD GetHexRSAPublicKeyExponent(nsACString& aHexRSAPublicKeyExponent) override { return _to GetHexRSAPublicKeyExponent(aHexRSAPublicKeyExponent); } \
  NS_IMETHOD GetHexRSAPublicKeyModulus(nsACString& aHexRSAPublicKeyModulus) override { return _to GetHexRSAPublicKeyModulus(aHexRSAPublicKeyModulus); } \
  NS_IMETHOD GetHexDSAPrime(nsACString& aHexDSAPrime) override { return _to GetHexDSAPrime(aHexDSAPrime); } \
  NS_IMETHOD GetHexDSASubPrime(nsACString& aHexDSASubPrime) override { return _to GetHexDSASubPrime(aHexDSASubPrime); } \
  NS_IMETHOD GetHexDSAGenerator(nsACString& aHexDSAGenerator) override { return _to GetHexDSAGenerator(aHexDSAGenerator); } \
  NS_IMETHOD GetHexDSAPublicValue(nsACString& aHexDSAPublicValue) override { return _to GetHexDSAPublicValue(aHexDSAPublicValue); } \
  NS_IMETHOD Sign(const nsACString& aText, nsIIdentitySignCallback *callback) override { return _to Sign(aText, callback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIIDENTITYKEYPAIR(_to) \
  NS_IMETHOD GetKeyType(nsACString& aKeyType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeyType(aKeyType); } \
  NS_IMETHOD GetHexRSAPublicKeyExponent(nsACString& aHexRSAPublicKeyExponent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHexRSAPublicKeyExponent(aHexRSAPublicKeyExponent); } \
  NS_IMETHOD GetHexRSAPublicKeyModulus(nsACString& aHexRSAPublicKeyModulus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHexRSAPublicKeyModulus(aHexRSAPublicKeyModulus); } \
  NS_IMETHOD GetHexDSAPrime(nsACString& aHexDSAPrime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHexDSAPrime(aHexDSAPrime); } \
  NS_IMETHOD GetHexDSASubPrime(nsACString& aHexDSASubPrime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHexDSASubPrime(aHexDSASubPrime); } \
  NS_IMETHOD GetHexDSAGenerator(nsACString& aHexDSAGenerator) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHexDSAGenerator(aHexDSAGenerator); } \
  NS_IMETHOD GetHexDSAPublicValue(nsACString& aHexDSAPublicValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHexDSAPublicValue(aHexDSAPublicValue); } \
  NS_IMETHOD Sign(const nsACString& aText, nsIIdentitySignCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Sign(aText, callback); } 


/* starting interface:    nsIIdentityKeyGenCallback */
#define NS_IIDENTITYKEYGENCALLBACK_IID_STR "90f24ca2-2b05-4ca9-8aec-89d38e2f905a"

#define NS_IIDENTITYKEYGENCALLBACK_IID \
  {0x90f24ca2, 0x2b05, 0x4ca9, \
    { 0x8a, 0xec, 0x89, 0xd3, 0x8e, 0x2f, 0x90, 0x5a }}

class NS_NO_VTABLE nsIIdentityKeyGenCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IIDENTITYKEYGENCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIIdentityKeyGenCallback;

  /* void generateKeyPairFinished (in nsresult rv, in nsIIdentityKeyPair keyPair); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GenerateKeyPairFinished(nsresult rv, nsIIdentityKeyPair *keyPair) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIIdentityKeyGenCallback, NS_IIDENTITYKEYGENCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIIDENTITYKEYGENCALLBACK \
  NS_IMETHOD GenerateKeyPairFinished(nsresult rv, nsIIdentityKeyPair *keyPair) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIIDENTITYKEYGENCALLBACK \
  nsresult GenerateKeyPairFinished(nsresult rv, nsIIdentityKeyPair *keyPair); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIIDENTITYKEYGENCALLBACK(_to) \
  NS_IMETHOD GenerateKeyPairFinished(nsresult rv, nsIIdentityKeyPair *keyPair) override { return _to GenerateKeyPairFinished(rv, keyPair); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIIDENTITYKEYGENCALLBACK(_to) \
  NS_IMETHOD GenerateKeyPairFinished(nsresult rv, nsIIdentityKeyPair *keyPair) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GenerateKeyPairFinished(rv, keyPair); } 


/* starting interface:    nsIIdentitySignCallback */
#define NS_IIDENTITYSIGNCALLBACK_IID_STR "2d3e5036-374b-4b47-a430-1196b67b890f"

#define NS_IIDENTITYSIGNCALLBACK_IID \
  {0x2d3e5036, 0x374b, 0x4b47, \
    { 0xa4, 0x30, 0x11, 0x96, 0xb6, 0x7b, 0x89, 0x0f }}

class NS_NO_VTABLE nsIIdentitySignCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IIDENTITYSIGNCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIIdentitySignCallback;

  /* void signFinished (in nsresult rv, in ACString base64urlSignature); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SignFinished(nsresult rv, const nsACString& base64urlSignature) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIIdentitySignCallback, NS_IIDENTITYSIGNCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIIDENTITYSIGNCALLBACK \
  NS_IMETHOD SignFinished(nsresult rv, const nsACString& base64urlSignature) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIIDENTITYSIGNCALLBACK \
  nsresult SignFinished(nsresult rv, const nsACString& base64urlSignature); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIIDENTITYSIGNCALLBACK(_to) \
  NS_IMETHOD SignFinished(nsresult rv, const nsACString& base64urlSignature) override { return _to SignFinished(rv, base64urlSignature); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIIDENTITYSIGNCALLBACK(_to) \
  NS_IMETHOD SignFinished(nsresult rv, const nsACString& base64urlSignature) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SignFinished(rv, base64urlSignature); } 


#endif /* __gen_nsIIdentityCryptoService_h__ */
