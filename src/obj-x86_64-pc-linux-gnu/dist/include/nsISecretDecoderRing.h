/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsISecretDecoderRing.idl
 */

#ifndef __gen_nsISecretDecoderRing_h__
#define __gen_nsISecretDecoderRing_h__


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

/* starting interface:    nsISecretDecoderRing */
#define NS_ISECRETDECODERRING_IID_STR "0ec80360-075c-11d4-9fd4-00c04f1b83d8"

#define NS_ISECRETDECODERRING_IID \
  {0x0ec80360, 0x075c, 0x11d4, \
    { 0x9f, 0xd4, 0x00, 0xc0, 0x4f, 0x1b, 0x83, 0xd8 }}

class NS_NO_VTABLE nsISecretDecoderRing : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISECRETDECODERRING_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISecretDecoderRing;

  /* [must_use] ACString encryptString (in ACString text); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD EncryptString(const nsACString& text, nsACString& _retval) = 0;

  /* [implicit_jscontext,must_use] Promise asyncEncryptStrings (in Array<AUTF8String> plaintexts); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AsyncEncryptStrings(const nsTArray<nsCString >& plaintexts, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [must_use] ACString decryptString (in ACString encryptedBase64Text); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD DecryptString(const nsACString& encryptedBase64Text, nsACString& _retval) = 0;

  /* [implicit_jscontext,must_use] Promise asyncDecryptStrings (in Array<ACString> encryptedStrings); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD AsyncDecryptStrings(const nsTArray<nsCString >& encryptedStrings, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [must_use] void changePassword (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ChangePassword(void) = 0;

  /* [must_use] void logout (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD Logout(void) = 0;

  /* [must_use] void logoutAndTeardown (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD LogoutAndTeardown(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISecretDecoderRing, NS_ISECRETDECODERRING_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISECRETDECODERRING \
  [[nodiscard]] NS_IMETHOD EncryptString(const nsACString& text, nsACString& _retval) override; \
  [[nodiscard]] NS_IMETHOD AsyncEncryptStrings(const nsTArray<nsCString >& plaintexts, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  [[nodiscard]] NS_IMETHOD DecryptString(const nsACString& encryptedBase64Text, nsACString& _retval) override; \
  [[nodiscard]] NS_IMETHOD AsyncDecryptStrings(const nsTArray<nsCString >& encryptedStrings, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  [[nodiscard]] NS_IMETHOD ChangePassword(void) override; \
  [[nodiscard]] NS_IMETHOD Logout(void) override; \
  [[nodiscard]] NS_IMETHOD LogoutAndTeardown(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISECRETDECODERRING \
  [[nodiscard]] nsresult EncryptString(const nsACString& text, nsACString& _retval); \
  [[nodiscard]] nsresult AsyncEncryptStrings(const nsTArray<nsCString >& plaintexts, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  [[nodiscard]] nsresult DecryptString(const nsACString& encryptedBase64Text, nsACString& _retval); \
  [[nodiscard]] nsresult AsyncDecryptStrings(const nsTArray<nsCString >& encryptedStrings, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  [[nodiscard]] nsresult ChangePassword(void); \
  [[nodiscard]] nsresult Logout(void); \
  [[nodiscard]] nsresult LogoutAndTeardown(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISECRETDECODERRING(_to) \
  [[nodiscard]] NS_IMETHOD EncryptString(const nsACString& text, nsACString& _retval) override { return _to EncryptString(text, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncEncryptStrings(const nsTArray<nsCString >& plaintexts, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to AsyncEncryptStrings(plaintexts, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD DecryptString(const nsACString& encryptedBase64Text, nsACString& _retval) override { return _to DecryptString(encryptedBase64Text, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncDecryptStrings(const nsTArray<nsCString >& encryptedStrings, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to AsyncDecryptStrings(encryptedStrings, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD ChangePassword(void) override { return _to ChangePassword(); } \
  [[nodiscard]] NS_IMETHOD Logout(void) override { return _to Logout(); } \
  [[nodiscard]] NS_IMETHOD LogoutAndTeardown(void) override { return _to LogoutAndTeardown(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISECRETDECODERRING(_to) \
  [[nodiscard]] NS_IMETHOD EncryptString(const nsACString& text, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EncryptString(text, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncEncryptStrings(const nsTArray<nsCString >& plaintexts, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncEncryptStrings(plaintexts, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD DecryptString(const nsACString& encryptedBase64Text, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DecryptString(encryptedBase64Text, _retval); } \
  [[nodiscard]] NS_IMETHOD AsyncDecryptStrings(const nsTArray<nsCString >& encryptedStrings, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncDecryptStrings(encryptedStrings, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD ChangePassword(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ChangePassword(); } \
  [[nodiscard]] NS_IMETHOD Logout(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Logout(); } \
  [[nodiscard]] NS_IMETHOD LogoutAndTeardown(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LogoutAndTeardown(); } 


#endif /* __gen_nsISecretDecoderRing_h__ */
