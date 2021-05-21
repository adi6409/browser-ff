/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsITellableStream.idl
 */

#ifndef __gen_nsITellableStream_h__
#define __gen_nsITellableStream_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsITellableStream */
#define NS_ITELLABLESTREAM_IID_STR "ee942946-4538-45d2-bf05-ffdbf5932621"

#define NS_ITELLABLESTREAM_IID \
  {0xee942946, 0x4538, 0x45d2, \
    { 0xbf, 0x05, 0xff, 0xdb, 0xf5, 0x93, 0x26, 0x21 }}

class NS_NO_VTABLE nsITellableStream : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITELLABLESTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITellableStream;

  /* long long tell (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Tell(int64_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITellableStream, NS_ITELLABLESTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITELLABLESTREAM \
  NS_IMETHOD Tell(int64_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITELLABLESTREAM \
  nsresult Tell(int64_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITELLABLESTREAM(_to) \
  NS_IMETHOD Tell(int64_t *_retval) override { return _to Tell(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITELLABLESTREAM(_to) \
  NS_IMETHOD Tell(int64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Tell(_retval); } 


#endif /* __gen_nsITellableStream_h__ */
