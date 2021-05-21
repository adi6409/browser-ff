/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIConverterInputStream.idl
 */

#ifndef __gen_nsIConverterInputStream_h__
#define __gen_nsIConverterInputStream_h__


#ifndef __gen_nsIUnicharInputStream_h__
#include "nsIUnicharInputStream.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */


/* starting interface:    nsIConverterInputStream */
#define NS_ICONVERTERINPUTSTREAM_IID_STR "fc66ffb6-5404-4908-a4a3-27f92fa0579d"

#define NS_ICONVERTERINPUTSTREAM_IID \
  {0xfc66ffb6, 0x5404, 0x4908, \
    { 0xa4, 0xa3, 0x27, 0xf9, 0x2f, 0xa0, 0x57, 0x9d }}

class NS_NO_VTABLE nsIConverterInputStream : public nsIUnicharInputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONVERTERINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIConverterInputStream;

  enum {
    DEFAULT_REPLACEMENT_CHARACTER = 65533U,
    ERRORS_ARE_FATAL = 0U
  };

  /* void init (in nsIInputStream aStream, in string aCharset, in long aBufferSize, in char16_t aReplacementChar); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIInputStream *aStream, const char * aCharset, int32_t aBufferSize, char16_t aReplacementChar) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIConverterInputStream, NS_ICONVERTERINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONVERTERINPUTSTREAM \
  NS_IMETHOD Init(nsIInputStream *aStream, const char * aCharset, int32_t aBufferSize, char16_t aReplacementChar) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONVERTERINPUTSTREAM \
  nsresult Init(nsIInputStream *aStream, const char * aCharset, int32_t aBufferSize, char16_t aReplacementChar); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONVERTERINPUTSTREAM(_to) \
  NS_IMETHOD Init(nsIInputStream *aStream, const char * aCharset, int32_t aBufferSize, char16_t aReplacementChar) override { return _to Init(aStream, aCharset, aBufferSize, aReplacementChar); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONVERTERINPUTSTREAM(_to) \
  NS_IMETHOD Init(nsIInputStream *aStream, const char * aCharset, int32_t aBufferSize, char16_t aReplacementChar) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aStream, aCharset, aBufferSize, aReplacementChar); } 


#endif /* __gen_nsIConverterInputStream_h__ */
