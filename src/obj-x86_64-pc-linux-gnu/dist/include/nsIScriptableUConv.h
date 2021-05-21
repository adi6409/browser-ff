/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/uconv/nsIScriptableUConv.idl
 */

#ifndef __gen_nsIScriptableUConv_h__
#define __gen_nsIScriptableUConv_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */

// {0A698C44-3BFF-11d4-9649-00C0CA135B4E}
#define NS_ISCRIPTABLEUNICODECONVERTER_CID { 0x0A698C44, 0x3BFF, 0x11d4, { 0x96, 0x49, 0x00, 0xC0, 0xCA, 0x13, 0x5B, 0x4E } }
#define NS_ISCRIPTABLEUNICODECONVERTER_CONTRACTID "@mozilla.org/intl/scriptableunicodeconverter"

/* starting interface:    nsIScriptableUnicodeConverter */
#define NS_ISCRIPTABLEUNICODECONVERTER_IID_STR "f36ee324-5c1c-437f-ba10-2b4db7a18031"

#define NS_ISCRIPTABLEUNICODECONVERTER_IID \
  {0xf36ee324, 0x5c1c, 0x437f, \
    { 0xba, 0x10, 0x2b, 0x4d, 0xb7, 0xa1, 0x80, 0x31 }}

class NS_NO_VTABLE nsIScriptableUnicodeConverter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISCRIPTABLEUNICODECONVERTER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIScriptableUnicodeConverter;

  /* ACString ConvertFromUnicode (in AString aSrc); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConvertFromUnicode(const nsAString& aSrc, nsACString& _retval) = 0;

  /* ACString Finish (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Finish(nsACString& _retval) = 0;

  /* AString ConvertToUnicode (in ACString aSrc); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConvertToUnicode(const nsACString& aSrc, nsAString& _retval) = 0;

  /* void convertToByteArray (in AString aString, [optional] out unsigned long aLen, [array, size_is (aLen), retval] out octet aData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConvertToByteArray(const nsAString& aString, uint32_t *aLen, uint8_t **aData) = 0;

  /* nsIInputStream convertToInputStream (in AString aString); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConvertToInputStream(const nsAString& aString, nsIInputStream **_retval) = 0;

  /* attribute ACString charset; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCharset(nsACString& aCharset) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCharset(const nsACString& aCharset) = 0;

  /* attribute boolean isInternal; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsInternal(bool *aIsInternal) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetIsInternal(bool aIsInternal) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIScriptableUnicodeConverter, NS_ISCRIPTABLEUNICODECONVERTER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISCRIPTABLEUNICODECONVERTER \
  NS_IMETHOD ConvertFromUnicode(const nsAString& aSrc, nsACString& _retval) override; \
  NS_IMETHOD Finish(nsACString& _retval) override; \
  NS_IMETHOD ConvertToUnicode(const nsACString& aSrc, nsAString& _retval) override; \
  NS_IMETHOD ConvertToByteArray(const nsAString& aString, uint32_t *aLen, uint8_t **aData) override; \
  NS_IMETHOD ConvertToInputStream(const nsAString& aString, nsIInputStream **_retval) override; \
  NS_IMETHOD GetCharset(nsACString& aCharset) override; \
  NS_IMETHOD SetCharset(const nsACString& aCharset) override; \
  NS_IMETHOD GetIsInternal(bool *aIsInternal) override; \
  NS_IMETHOD SetIsInternal(bool aIsInternal) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISCRIPTABLEUNICODECONVERTER \
  nsresult ConvertFromUnicode(const nsAString& aSrc, nsACString& _retval); \
  nsresult Finish(nsACString& _retval); \
  nsresult ConvertToUnicode(const nsACString& aSrc, nsAString& _retval); \
  nsresult ConvertToByteArray(const nsAString& aString, uint32_t *aLen, uint8_t **aData); \
  nsresult ConvertToInputStream(const nsAString& aString, nsIInputStream **_retval); \
  nsresult GetCharset(nsACString& aCharset); \
  nsresult SetCharset(const nsACString& aCharset); \
  nsresult GetIsInternal(bool *aIsInternal); \
  nsresult SetIsInternal(bool aIsInternal); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISCRIPTABLEUNICODECONVERTER(_to) \
  NS_IMETHOD ConvertFromUnicode(const nsAString& aSrc, nsACString& _retval) override { return _to ConvertFromUnicode(aSrc, _retval); } \
  NS_IMETHOD Finish(nsACString& _retval) override { return _to Finish(_retval); } \
  NS_IMETHOD ConvertToUnicode(const nsACString& aSrc, nsAString& _retval) override { return _to ConvertToUnicode(aSrc, _retval); } \
  NS_IMETHOD ConvertToByteArray(const nsAString& aString, uint32_t *aLen, uint8_t **aData) override { return _to ConvertToByteArray(aString, aLen, aData); } \
  NS_IMETHOD ConvertToInputStream(const nsAString& aString, nsIInputStream **_retval) override { return _to ConvertToInputStream(aString, _retval); } \
  NS_IMETHOD GetCharset(nsACString& aCharset) override { return _to GetCharset(aCharset); } \
  NS_IMETHOD SetCharset(const nsACString& aCharset) override { return _to SetCharset(aCharset); } \
  NS_IMETHOD GetIsInternal(bool *aIsInternal) override { return _to GetIsInternal(aIsInternal); } \
  NS_IMETHOD SetIsInternal(bool aIsInternal) override { return _to SetIsInternal(aIsInternal); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISCRIPTABLEUNICODECONVERTER(_to) \
  NS_IMETHOD ConvertFromUnicode(const nsAString& aSrc, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConvertFromUnicode(aSrc, _retval); } \
  NS_IMETHOD Finish(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Finish(_retval); } \
  NS_IMETHOD ConvertToUnicode(const nsACString& aSrc, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConvertToUnicode(aSrc, _retval); } \
  NS_IMETHOD ConvertToByteArray(const nsAString& aString, uint32_t *aLen, uint8_t **aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConvertToByteArray(aString, aLen, aData); } \
  NS_IMETHOD ConvertToInputStream(const nsAString& aString, nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConvertToInputStream(aString, _retval); } \
  NS_IMETHOD GetCharset(nsACString& aCharset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCharset(aCharset); } \
  NS_IMETHOD SetCharset(const nsACString& aCharset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCharset(aCharset); } \
  NS_IMETHOD GetIsInternal(bool *aIsInternal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsInternal(aIsInternal); } \
  NS_IMETHOD SetIsInternal(bool aIsInternal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsInternal(aIsInternal); } 


#endif /* __gen_nsIScriptableUConv_h__ */
