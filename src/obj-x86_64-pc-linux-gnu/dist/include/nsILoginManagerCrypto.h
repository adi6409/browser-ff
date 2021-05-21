/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginManagerCrypto.idl
 */

#ifndef __gen_nsILoginManagerCrypto_h__
#define __gen_nsILoginManagerCrypto_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsILoginManagerCrypto */
#define NS_ILOGINMANAGERCRYPTO_IID_STR "2030770e-542e-40cd-8061-cd9d4ad4227f"

#define NS_ILOGINMANAGERCRYPTO_IID \
  {0x2030770e, 0x542e, 0x40cd, \
    { 0x80, 0x61, 0xcd, 0x9d, 0x4a, 0xd4, 0x22, 0x7f }}

class NS_NO_VTABLE nsILoginManagerCrypto : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOGINMANAGERCRYPTO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoginManagerCrypto;

  enum {
    ENCTYPE_BASE64 = 0U,
    ENCTYPE_SDR = 1U
  };

  /* AString encrypt (in AString plainText); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Encrypt(const nsAString& plainText, nsAString& _retval) = 0;

  /* Promise encryptMany (in jsval plainTexts); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EncryptMany(JS::HandleValue plainTexts, ::mozilla::dom::Promise * * _retval) = 0;

  /* AString decrypt (in AString cipherText); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Decrypt(const nsAString& cipherText, nsAString& _retval) = 0;

  /* Promise decryptMany (in jsval cipherTexts); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DecryptMany(JS::HandleValue cipherTexts, ::mozilla::dom::Promise * * _retval) = 0;

  /* readonly attribute boolean uiBusy; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUiBusy(bool *aUiBusy) = 0;

  /* readonly attribute boolean isLoggedIn; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsLoggedIn(bool *aIsLoggedIn) = 0;

  /* readonly attribute unsigned long defaultEncType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultEncType(uint32_t *aDefaultEncType) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoginManagerCrypto, NS_ILOGINMANAGERCRYPTO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOGINMANAGERCRYPTO \
  NS_IMETHOD Encrypt(const nsAString& plainText, nsAString& _retval) override; \
  NS_IMETHOD EncryptMany(JS::HandleValue plainTexts, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD Decrypt(const nsAString& cipherText, nsAString& _retval) override; \
  NS_IMETHOD DecryptMany(JS::HandleValue cipherTexts, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetUiBusy(bool *aUiBusy) override; \
  NS_IMETHOD GetIsLoggedIn(bool *aIsLoggedIn) override; \
  NS_IMETHOD GetDefaultEncType(uint32_t *aDefaultEncType) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOGINMANAGERCRYPTO \
  nsresult Encrypt(const nsAString& plainText, nsAString& _retval); \
  nsresult EncryptMany(JS::HandleValue plainTexts, ::mozilla::dom::Promise * * _retval); \
  nsresult Decrypt(const nsAString& cipherText, nsAString& _retval); \
  nsresult DecryptMany(JS::HandleValue cipherTexts, ::mozilla::dom::Promise * * _retval); \
  nsresult GetUiBusy(bool *aUiBusy); \
  nsresult GetIsLoggedIn(bool *aIsLoggedIn); \
  nsresult GetDefaultEncType(uint32_t *aDefaultEncType); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOGINMANAGERCRYPTO(_to) \
  NS_IMETHOD Encrypt(const nsAString& plainText, nsAString& _retval) override { return _to Encrypt(plainText, _retval); } \
  NS_IMETHOD EncryptMany(JS::HandleValue plainTexts, ::mozilla::dom::Promise * * _retval) override { return _to EncryptMany(plainTexts, _retval); } \
  NS_IMETHOD Decrypt(const nsAString& cipherText, nsAString& _retval) override { return _to Decrypt(cipherText, _retval); } \
  NS_IMETHOD DecryptMany(JS::HandleValue cipherTexts, ::mozilla::dom::Promise * * _retval) override { return _to DecryptMany(cipherTexts, _retval); } \
  NS_IMETHOD GetUiBusy(bool *aUiBusy) override { return _to GetUiBusy(aUiBusy); } \
  NS_IMETHOD GetIsLoggedIn(bool *aIsLoggedIn) override { return _to GetIsLoggedIn(aIsLoggedIn); } \
  NS_IMETHOD GetDefaultEncType(uint32_t *aDefaultEncType) override { return _to GetDefaultEncType(aDefaultEncType); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOGINMANAGERCRYPTO(_to) \
  NS_IMETHOD Encrypt(const nsAString& plainText, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Encrypt(plainText, _retval); } \
  NS_IMETHOD EncryptMany(JS::HandleValue plainTexts, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EncryptMany(plainTexts, _retval); } \
  NS_IMETHOD Decrypt(const nsAString& cipherText, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Decrypt(cipherText, _retval); } \
  NS_IMETHOD DecryptMany(JS::HandleValue cipherTexts, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DecryptMany(cipherTexts, _retval); } \
  NS_IMETHOD GetUiBusy(bool *aUiBusy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUiBusy(aUiBusy); } \
  NS_IMETHOD GetIsLoggedIn(bool *aIsLoggedIn) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsLoggedIn(aIsLoggedIn); } \
  NS_IMETHOD GetDefaultEncType(uint32_t *aDefaultEncType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultEncType(aDefaultEncType); } 


#endif /* __gen_nsILoginManagerCrypto_h__ */
