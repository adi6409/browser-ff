/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsILineInputStream.idl
 */

#ifndef __gen_nsILineInputStream_h__
#define __gen_nsILineInputStream_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsILineInputStream */
#define NS_ILINEINPUTSTREAM_IID_STR "c97b466c-1e6e-4773-a4ab-2b2b3190a7a6"

#define NS_ILINEINPUTSTREAM_IID \
  {0xc97b466c, 0x1e6e, 0x4773, \
    { 0xa4, 0xab, 0x2b, 0x2b, 0x31, 0x90, 0xa7, 0xa6 }}

class NS_NO_VTABLE nsILineInputStream : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILINEINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILineInputStream;

  /* boolean readLine (out ACString aLine); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadLine(nsACString& aLine, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILineInputStream, NS_ILINEINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILINEINPUTSTREAM \
  NS_IMETHOD ReadLine(nsACString& aLine, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILINEINPUTSTREAM \
  nsresult ReadLine(nsACString& aLine, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILINEINPUTSTREAM(_to) \
  NS_IMETHOD ReadLine(nsACString& aLine, bool *_retval) override { return _to ReadLine(aLine, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILINEINPUTSTREAM(_to) \
  NS_IMETHOD ReadLine(nsACString& aLine, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadLine(aLine, _retval); } 


#endif /* __gen_nsILineInputStream_h__ */
