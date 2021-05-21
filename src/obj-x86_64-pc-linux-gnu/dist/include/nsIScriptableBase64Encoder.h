/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIScriptableBase64Encoder.idl
 */

#ifndef __gen_nsIScriptableBase64Encoder_h__
#define __gen_nsIScriptableBase64Encoder_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */


/* starting interface:    nsIScriptableBase64Encoder */
#define NS_ISCRIPTABLEBASE64ENCODER_IID_STR "9479c864-d1f9-45ab-b7b9-28b907bd2ba9"

#define NS_ISCRIPTABLEBASE64ENCODER_IID \
  {0x9479c864, 0xd1f9, 0x45ab, \
    { 0xb7, 0xb9, 0x28, 0xb9, 0x07, 0xbd, 0x2b, 0xa9 }}

class NS_NO_VTABLE nsIScriptableBase64Encoder : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISCRIPTABLEBASE64ENCODER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIScriptableBase64Encoder;

  /* ACString encodeToCString (in nsIInputStream stream, in unsigned long length); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EncodeToCString(nsIInputStream *stream, uint32_t length, nsACString& _retval) = 0;

  /* AString encodeToString (in nsIInputStream stream, in unsigned long length); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EncodeToString(nsIInputStream *stream, uint32_t length, nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIScriptableBase64Encoder, NS_ISCRIPTABLEBASE64ENCODER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISCRIPTABLEBASE64ENCODER \
  NS_IMETHOD EncodeToCString(nsIInputStream *stream, uint32_t length, nsACString& _retval) override; \
  NS_IMETHOD EncodeToString(nsIInputStream *stream, uint32_t length, nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISCRIPTABLEBASE64ENCODER \
  nsresult EncodeToCString(nsIInputStream *stream, uint32_t length, nsACString& _retval); \
  nsresult EncodeToString(nsIInputStream *stream, uint32_t length, nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISCRIPTABLEBASE64ENCODER(_to) \
  NS_IMETHOD EncodeToCString(nsIInputStream *stream, uint32_t length, nsACString& _retval) override { return _to EncodeToCString(stream, length, _retval); } \
  NS_IMETHOD EncodeToString(nsIInputStream *stream, uint32_t length, nsAString& _retval) override { return _to EncodeToString(stream, length, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISCRIPTABLEBASE64ENCODER(_to) \
  NS_IMETHOD EncodeToCString(nsIInputStream *stream, uint32_t length, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EncodeToCString(stream, length, _retval); } \
  NS_IMETHOD EncodeToString(nsIInputStream *stream, uint32_t length, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EncodeToString(stream, length, _retval); } 


#endif /* __gen_nsIScriptableBase64Encoder_h__ */
