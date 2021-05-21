/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIOSKeyStore.idl
 */

#ifndef __gen_nsIOSKeyStore_h__
#define __gen_nsIOSKeyStore_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIOSKeyStore */
#define NS_IOSKEYSTORE_IID_STR "57972956-5718-42d2-8070-b3fc72212eaf"

#define NS_IOSKEYSTORE_IID \
  {0x57972956, 0x5718, 0x42d2, \
    { 0x80, 0x70, 0xb3, 0xfc, 0x72, 0x21, 0x2e, 0xaf }}

class NS_NO_VTABLE nsIOSKeyStore : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IOSKEYSTORE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIOSKeyStore;

  /* [implicit_jscontext,must_use] Promise asyncGenerateSecret (in ACString label); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AsyncGenerateSecret(const nsACString& label, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext,must_use] Promise asyncSecretAvailable (in ACString label); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AsyncSecretAvailable(const nsACString& label, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext,must_use] Promise asyncRecoverSecret (in ACString label, in ACString recoveryPhrase); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AsyncRecoverSecret(const nsACString& label, const nsACString& recoveryPhrase, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext,must_use] Promise asyncDeleteSecret (in ACString label); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AsyncDeleteSecret(const nsACString& label, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext,must_use] Promise asyncEncryptBytes (in ACString label, in Array<uint8_t> inBytes); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AsyncEncryptBytes(const nsACString& label, const nsTArray<uint8_t >& inBytes, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext,must_use] Promise asyncDecryptBytes (in ACString label, in ACString encryptedBase64Text); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AsyncDecryptBytes(const nsACString& label, const nsACString& encryptedBase64Text, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext,must_use] Promise asyncLock (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AsyncLock(JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext,must_use] Promise asyncUnlock (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AsyncUnlock(JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* readonly attribute bool isNSSKeyStore; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsNSSKeyStore(bool *aIsNSSKeyStore) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIOSKeyStore, NS_IOSKEYSTORE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIOSKEYSTORE \
  [[nodiscard]] NS_IMETHOD AsyncGenerateSecret(const nsACString& label, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  [[nodiscard]] NS_IMETHOD AsyncSecretAvailable(const nsACString& label, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  [[nodiscard]] NS_IMETHOD AsyncRecoverSecret(const nsACString& label, const nsACString& recoveryPhrase, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  [[nodiscard]] NS_IMETHOD AsyncDeleteSecret(const nsACString& label, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  [[nodiscard]] NS_IMETHOD AsyncEncryptBytes(const nsACString& label, const nsTArray<uint8_t >& inBytes, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  [[nodiscard]] NS_IMETHOD AsyncDecryptBytes(const nsACString& label, const nsACString& encryptedBase64Text, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  [[nodiscard]] NS_IMETHOD AsyncLock(JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  [[nodiscard]] NS_IMETHOD AsyncUnlock(JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetIsNSSKeyStore(bool *aIsNSSKeyStore) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIOSKEYSTORE \
  [[nodiscard]] nsresult AsyncGenerateSecret(const nsACString& label, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  [[nodiscard]] nsresult AsyncSecretAvailable(const nsACString& label, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  [[nodiscard]] nsresult AsyncRecoverSecret(const nsACString& label, const nsACString& recoveryPhrase, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  [[nodiscard]] nsresult AsyncDeleteSecret(const nsACString& label, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  [[nodiscard]] nsresult AsyncEncryptBytes(const nsACString& label, const nsTArray<uint8_t >& inBytes, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  [[nodiscard]] nsresult AsyncDecryptBytes(const nsACString& label, const nsACString& encryptedBase64Text, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  [[nodiscard]] nsresult AsyncLock(JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  [[nodiscard]] nsresult AsyncUnlock(JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult GetIsNSSKeyStore(bool *aIsNSSKeyStore); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIOSKEYSTORE(_to) \
  [[nodiscard]] NS_IMETHOD AsyncGenerateSecret(const nsACString& label, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to AsyncGenerateSecret(label, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncSecretAvailable(const nsACString& label, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to AsyncSecretAvailable(label, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncRecoverSecret(const nsACString& label, const nsACString& recoveryPhrase, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to AsyncRecoverSecret(label, recoveryPhrase, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncDeleteSecret(const nsACString& label, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to AsyncDeleteSecret(label, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncEncryptBytes(const nsACString& label, const nsTArray<uint8_t >& inBytes, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to AsyncEncryptBytes(label, inBytes, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncDecryptBytes(const nsACString& label, const nsACString& encryptedBase64Text, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to AsyncDecryptBytes(label, encryptedBase64Text, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncLock(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to AsyncLock(cx, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncUnlock(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to AsyncUnlock(cx, _retval); } \
  NS_IMETHOD GetIsNSSKeyStore(bool *aIsNSSKeyStore) override { return _to GetIsNSSKeyStore(aIsNSSKeyStore); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIOSKEYSTORE(_to) \
  [[nodiscard]] NS_IMETHOD AsyncGenerateSecret(const nsACString& label, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncGenerateSecret(label, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncSecretAvailable(const nsACString& label, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncSecretAvailable(label, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncRecoverSecret(const nsACString& label, const nsACString& recoveryPhrase, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncRecoverSecret(label, recoveryPhrase, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncDeleteSecret(const nsACString& label, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncDeleteSecret(label, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncEncryptBytes(const nsACString& label, const nsTArray<uint8_t >& inBytes, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncEncryptBytes(label, inBytes, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncDecryptBytes(const nsACString& label, const nsACString& encryptedBase64Text, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncDecryptBytes(label, encryptedBase64Text, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncLock(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncLock(cx, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncUnlock(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncUnlock(cx, _retval); } \
  NS_IMETHOD GetIsNSSKeyStore(bool *aIsNSSKeyStore) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsNSSKeyStore(aIsNSSKeyStore); } 


#endif /* __gen_nsIOSKeyStore_h__ */
