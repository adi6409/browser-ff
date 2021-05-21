/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsISeekableStream.idl
 */

#ifndef __gen_nsISeekableStream_h__
#define __gen_nsISeekableStream_h__


#ifndef __gen_nsITellableStream_h__
#include "nsITellableStream.h"
#endif

#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsISeekableStream */
#define NS_ISEEKABLESTREAM_IID_STR "8429d350-1040-4661-8b71-f2a6ba455980"

#define NS_ISEEKABLESTREAM_IID \
  {0x8429d350, 0x1040, 0x4661, \
    { 0x8b, 0x71, 0xf2, 0xa6, 0xba, 0x45, 0x59, 0x80 }}

class NS_NO_VTABLE nsISeekableStream : public nsITellableStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISEEKABLESTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISeekableStream;

  enum {
    NS_SEEK_SET = 0,
    NS_SEEK_CUR = 1,
    NS_SEEK_END = 2
  };

  /* void seek (in long whence, in long long offset); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Seek(int32_t whence, int64_t offset) = 0;

  /* void setEOF (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetEOF(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISeekableStream, NS_ISEEKABLESTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISEEKABLESTREAM \
  NS_IMETHOD Seek(int32_t whence, int64_t offset) override; \
  NS_IMETHOD SetEOF(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISEEKABLESTREAM \
  nsresult Seek(int32_t whence, int64_t offset); \
  nsresult SetEOF(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISEEKABLESTREAM(_to) \
  NS_IMETHOD Seek(int32_t whence, int64_t offset) override { return _to Seek(whence, offset); } \
  NS_IMETHOD SetEOF(void) override { return _to SetEOF(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISEEKABLESTREAM(_to) \
  NS_IMETHOD Seek(int32_t whence, int64_t offset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Seek(whence, offset); } \
  NS_IMETHOD SetEOF(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEOF(); } 


#endif /* __gen_nsISeekableStream_h__ */
