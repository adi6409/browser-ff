/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsICloneableInputStream.idl
 */

#ifndef __gen_nsICloneableInputStream_h__
#define __gen_nsICloneableInputStream_h__


#ifndef __gen_nsIInputStream_h__
#include "nsIInputStream.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsICloneableInputStream */
#define NS_ICLONEABLEINPUTSTREAM_IID_STR "8149be1f-44d3-4f14-8b65-a57a5fbbeb97"

#define NS_ICLONEABLEINPUTSTREAM_IID \
  {0x8149be1f, 0x44d3, 0x4f14, \
    { 0x8b, 0x65, 0xa5, 0x7a, 0x5f, 0xbb, 0xeb, 0x97 }}

class NS_NO_VTABLE nsICloneableInputStream : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICLONEABLEINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICloneableInputStream;

  /* [infallible] readonly attribute boolean cloneable; */
  NS_IMETHOD GetCloneable(bool *aCloneable) = 0;
  inline bool  GetCloneable()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetCloneable(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* nsIInputStream clone (); */
  NS_IMETHOD Clone(nsIInputStream **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICloneableInputStream, NS_ICLONEABLEINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICLONEABLEINPUTSTREAM \
  using nsICloneableInputStream::GetCloneable; \
  NS_IMETHOD GetCloneable(bool *aCloneable) override; \
  NS_IMETHOD Clone(nsIInputStream **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICLONEABLEINPUTSTREAM \
  using nsICloneableInputStream::GetCloneable; \
  nsresult GetCloneable(bool *aCloneable); \
  nsresult Clone(nsIInputStream **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICLONEABLEINPUTSTREAM(_to) \
  using nsICloneableInputStream::GetCloneable; \
  NS_IMETHOD GetCloneable(bool *aCloneable) override { return _to GetCloneable(aCloneable); } \
  NS_IMETHOD Clone(nsIInputStream **_retval) override { return _to Clone(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICLONEABLEINPUTSTREAM(_to) \
  NS_IMETHOD GetCloneable(bool *aCloneable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCloneable(aCloneable); } \
  NS_IMETHOD Clone(nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clone(_retval); } 


/* starting interface:    nsICloneableInputStreamWithRange */
#define NS_ICLONEABLEINPUTSTREAMWITHRANGE_IID_STR "ece853c3-aded-4cef-8f51-0d1493d60bd5"

#define NS_ICLONEABLEINPUTSTREAMWITHRANGE_IID \
  {0xece853c3, 0xaded, 0x4cef, \
    { 0x8f, 0x51, 0x0d, 0x14, 0x93, 0xd6, 0x0b, 0xd5 }}

class NS_NO_VTABLE nsICloneableInputStreamWithRange : public nsICloneableInputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICLONEABLEINPUTSTREAMWITHRANGE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICloneableInputStreamWithRange;

  /* nsIInputStream cloneWithRange (in uint64_t start, in uint64_t length); */
  NS_IMETHOD CloneWithRange(uint64_t start, uint64_t length, nsIInputStream **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICloneableInputStreamWithRange, NS_ICLONEABLEINPUTSTREAMWITHRANGE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICLONEABLEINPUTSTREAMWITHRANGE \
  NS_IMETHOD CloneWithRange(uint64_t start, uint64_t length, nsIInputStream **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICLONEABLEINPUTSTREAMWITHRANGE \
  nsresult CloneWithRange(uint64_t start, uint64_t length, nsIInputStream **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICLONEABLEINPUTSTREAMWITHRANGE(_to) \
  NS_IMETHOD CloneWithRange(uint64_t start, uint64_t length, nsIInputStream **_retval) override { return _to CloneWithRange(start, length, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICLONEABLEINPUTSTREAMWITHRANGE(_to) \
  NS_IMETHOD CloneWithRange(uint64_t start, uint64_t length, nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CloneWithRange(start, length, _retval); } 


#endif /* __gen_nsICloneableInputStream_h__ */
