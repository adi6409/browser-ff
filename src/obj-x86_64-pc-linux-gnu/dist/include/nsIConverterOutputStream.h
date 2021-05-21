/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIConverterOutputStream.idl
 */

#ifndef __gen_nsIConverterOutputStream_h__
#define __gen_nsIConverterOutputStream_h__


#ifndef __gen_nsIUnicharOutputStream_h__
#include "nsIUnicharOutputStream.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIOutputStream; /* forward declaration */


/* starting interface:    nsIConverterOutputStream */
#define NS_ICONVERTEROUTPUTSTREAM_IID_STR "4b71113a-cb0d-479f-8ed5-01daeba2e8d4"

#define NS_ICONVERTEROUTPUTSTREAM_IID \
  {0x4b71113a, 0xcb0d, 0x479f, \
    { 0x8e, 0xd5, 0x01, 0xda, 0xeb, 0xa2, 0xe8, 0xd4 }}

class NS_NO_VTABLE nsIConverterOutputStream : public nsIUnicharOutputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONVERTEROUTPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIConverterOutputStream;

  /* void init (in nsIOutputStream aOutStream, in string aCharset); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIOutputStream *aOutStream, const char * aCharset) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIConverterOutputStream, NS_ICONVERTEROUTPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONVERTEROUTPUTSTREAM \
  NS_IMETHOD Init(nsIOutputStream *aOutStream, const char * aCharset) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONVERTEROUTPUTSTREAM \
  nsresult Init(nsIOutputStream *aOutStream, const char * aCharset); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONVERTEROUTPUTSTREAM(_to) \
  NS_IMETHOD Init(nsIOutputStream *aOutStream, const char * aCharset) override { return _to Init(aOutStream, aCharset); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONVERTEROUTPUTSTREAM(_to) \
  NS_IMETHOD Init(nsIOutputStream *aOutStream, const char * aCharset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aOutStream, aCharset); } 


#endif /* __gen_nsIConverterOutputStream_h__ */
