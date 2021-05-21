/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIStorageStream.idl
 */

#ifndef __gen_nsIStorageStream_h__
#define __gen_nsIStorageStream_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */

class nsIOutputStream; /* forward declaration */


/* starting interface:    nsIStorageStream */
#define NS_ISTORAGESTREAM_IID_STR "44a200fe-6c2b-4b41-b4e3-63e8c14e7c0d"

#define NS_ISTORAGESTREAM_IID \
  {0x44a200fe, 0x6c2b, 0x4b41, \
    { 0xb4, 0xe3, 0x63, 0xe8, 0xc1, 0x4e, 0x7c, 0x0d }}

class NS_NO_VTABLE nsIStorageStream : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTORAGESTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStorageStream;

  /* void init (in uint32_t segmentSize, in uint32_t maxSize); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(uint32_t segmentSize, uint32_t maxSize) = 0;

  /* nsIOutputStream getOutputStream (in int32_t startPosition); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOutputStream(int32_t startPosition, nsIOutputStream **_retval) = 0;

  /* nsIInputStream newInputStream (in int32_t startPosition); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NewInputStream(int32_t startPosition, nsIInputStream **_retval) = 0;

  /* attribute uint32_t length; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLength(uint32_t *aLength) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLength(uint32_t aLength) = 0;

  /* readonly attribute boolean writeInProgress; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWriteInProgress(bool *aWriteInProgress) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStorageStream, NS_ISTORAGESTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTORAGESTREAM \
  NS_IMETHOD Init(uint32_t segmentSize, uint32_t maxSize) override; \
  NS_IMETHOD GetOutputStream(int32_t startPosition, nsIOutputStream **_retval) override; \
  NS_IMETHOD NewInputStream(int32_t startPosition, nsIInputStream **_retval) override; \
  NS_IMETHOD GetLength(uint32_t *aLength) override; \
  NS_IMETHOD SetLength(uint32_t aLength) override; \
  NS_IMETHOD GetWriteInProgress(bool *aWriteInProgress) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTORAGESTREAM \
  nsresult Init(uint32_t segmentSize, uint32_t maxSize); \
  nsresult GetOutputStream(int32_t startPosition, nsIOutputStream **_retval); \
  nsresult NewInputStream(int32_t startPosition, nsIInputStream **_retval); \
  nsresult GetLength(uint32_t *aLength); \
  nsresult SetLength(uint32_t aLength); \
  nsresult GetWriteInProgress(bool *aWriteInProgress); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTORAGESTREAM(_to) \
  NS_IMETHOD Init(uint32_t segmentSize, uint32_t maxSize) override { return _to Init(segmentSize, maxSize); } \
  NS_IMETHOD GetOutputStream(int32_t startPosition, nsIOutputStream **_retval) override { return _to GetOutputStream(startPosition, _retval); } \
  NS_IMETHOD NewInputStream(int32_t startPosition, nsIInputStream **_retval) override { return _to NewInputStream(startPosition, _retval); } \
  NS_IMETHOD GetLength(uint32_t *aLength) override { return _to GetLength(aLength); } \
  NS_IMETHOD SetLength(uint32_t aLength) override { return _to SetLength(aLength); } \
  NS_IMETHOD GetWriteInProgress(bool *aWriteInProgress) override { return _to GetWriteInProgress(aWriteInProgress); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTORAGESTREAM(_to) \
  NS_IMETHOD Init(uint32_t segmentSize, uint32_t maxSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(segmentSize, maxSize); } \
  NS_IMETHOD GetOutputStream(int32_t startPosition, nsIOutputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOutputStream(startPosition, _retval); } \
  NS_IMETHOD NewInputStream(int32_t startPosition, nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewInputStream(startPosition, _retval); } \
  NS_IMETHOD GetLength(uint32_t *aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLength(aLength); } \
  NS_IMETHOD SetLength(uint32_t aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLength(aLength); } \
  NS_IMETHOD GetWriteInProgress(bool *aWriteInProgress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWriteInProgress(aWriteInProgress); } 

// Factory method
nsresult
NS_NewStorageStream(uint32_t segmentSize, uint32_t maxSize, nsIStorageStream **result);

#endif /* __gen_nsIStorageStream_h__ */
