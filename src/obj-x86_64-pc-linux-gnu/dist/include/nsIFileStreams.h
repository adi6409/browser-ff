/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIFileStreams.idl
 */

#ifndef __gen_nsIFileStreams_h__
#define __gen_nsIFileStreams_h__


#ifndef __gen_nsIInputStream_h__
#include "nsIInputStream.h"
#endif

#ifndef __gen_nsIOutputStream_h__
#include "nsIOutputStream.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIEventTarget; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIFileMetadataCallback; /* forward declaration */

struct PRFileDesc;

/* starting interface:    nsIFileInputStream */
#define NS_IFILEINPUTSTREAM_IID_STR "e3d56a20-c7ec-11d3-8cda-0060b0fc14a3"

#define NS_IFILEINPUTSTREAM_IID \
  {0xe3d56a20, 0xc7ec, 0x11d3, \
    { 0x8c, 0xda, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3 }}

class NS_NO_VTABLE nsIFileInputStream : public nsIInputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFILEINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFileInputStream;

  /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIFile *file, int32_t ioFlags, int32_t perm, int32_t behaviorFlags) = 0;

  enum {
    CLOSE_ON_EOF = 4,
    REOPEN_ON_REWIND = 8,
    DEFER_OPEN = 16,
    SHARE_DELETE = 32
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFileInputStream, NS_IFILEINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFILEINPUTSTREAM \
  NS_IMETHOD Init(nsIFile *file, int32_t ioFlags, int32_t perm, int32_t behaviorFlags) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFILEINPUTSTREAM \
  nsresult Init(nsIFile *file, int32_t ioFlags, int32_t perm, int32_t behaviorFlags); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFILEINPUTSTREAM(_to) \
  NS_IMETHOD Init(nsIFile *file, int32_t ioFlags, int32_t perm, int32_t behaviorFlags) override { return _to Init(file, ioFlags, perm, behaviorFlags); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFILEINPUTSTREAM(_to) \
  NS_IMETHOD Init(nsIFile *file, int32_t ioFlags, int32_t perm, int32_t behaviorFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(file, ioFlags, perm, behaviorFlags); } \


/* starting interface:    nsIFileOutputStream */
#define NS_IFILEOUTPUTSTREAM_IID_STR "e734cac9-1295-4e6f-9684-3ac4e1f91063"

#define NS_IFILEOUTPUTSTREAM_IID \
  {0xe734cac9, 0x1295, 0x4e6f, \
    { 0x96, 0x84, 0x3a, 0xc4, 0xe1, 0xf9, 0x10, 0x63 }}

class NS_NO_VTABLE nsIFileOutputStream : public nsIOutputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFILEOUTPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFileOutputStream;

  /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIFile *file, int32_t ioFlags, int32_t perm, int32_t behaviorFlags) = 0;

  /* [noscript] void preallocate (in long long length); */
  NS_IMETHOD Preallocate(int64_t length) = 0;

  enum {
    DEFER_OPEN = 1
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFileOutputStream, NS_IFILEOUTPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFILEOUTPUTSTREAM \
  NS_IMETHOD Init(nsIFile *file, int32_t ioFlags, int32_t perm, int32_t behaviorFlags) override; \
  NS_IMETHOD Preallocate(int64_t length) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFILEOUTPUTSTREAM \
  nsresult Init(nsIFile *file, int32_t ioFlags, int32_t perm, int32_t behaviorFlags); \
  nsresult Preallocate(int64_t length); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFILEOUTPUTSTREAM(_to) \
  NS_IMETHOD Init(nsIFile *file, int32_t ioFlags, int32_t perm, int32_t behaviorFlags) override { return _to Init(file, ioFlags, perm, behaviorFlags); } \
  NS_IMETHOD Preallocate(int64_t length) override { return _to Preallocate(length); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFILEOUTPUTSTREAM(_to) \
  NS_IMETHOD Init(nsIFile *file, int32_t ioFlags, int32_t perm, int32_t behaviorFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(file, ioFlags, perm, behaviorFlags); } \
  NS_IMETHOD Preallocate(int64_t length) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Preallocate(length); } \


/* starting interface:    nsIFileStream */
#define NS_IFILESTREAM_IID_STR "82cf605a-8393-4550-83ab-43cd5578e006"

#define NS_IFILESTREAM_IID \
  {0x82cf605a, 0x8393, 0x4550, \
    { 0x83, 0xab, 0x43, 0xcd, 0x55, 0x78, 0xe0, 0x06 }}

class NS_NO_VTABLE nsIFileStream : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFILESTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFileStream;

  /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIFile *file, int32_t ioFlags, int32_t perm, int32_t behaviorFlags) = 0;

  enum {
    DEFER_OPEN = 1
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFileStream, NS_IFILESTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFILESTREAM \
  NS_IMETHOD Init(nsIFile *file, int32_t ioFlags, int32_t perm, int32_t behaviorFlags) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFILESTREAM \
  nsresult Init(nsIFile *file, int32_t ioFlags, int32_t perm, int32_t behaviorFlags); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFILESTREAM(_to) \
  NS_IMETHOD Init(nsIFile *file, int32_t ioFlags, int32_t perm, int32_t behaviorFlags) override { return _to Init(file, ioFlags, perm, behaviorFlags); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFILESTREAM(_to) \
  NS_IMETHOD Init(nsIFile *file, int32_t ioFlags, int32_t perm, int32_t behaviorFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(file, ioFlags, perm, behaviorFlags); } \


/* starting interface:    nsIFileMetadata */
#define NS_IFILEMETADATA_IID_STR "07f679e4-9601-4bd1-b510-cd3852edb881"

#define NS_IFILEMETADATA_IID \
  {0x07f679e4, 0x9601, 0x4bd1, \
    { 0xb5, 0x10, 0xcd, 0x38, 0x52, 0xed, 0xb8, 0x81 }}

class NS_NO_VTABLE nsIFileMetadata : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFILEMETADATA_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFileMetadata;

  /* readonly attribute long long size; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSize(int64_t *aSize) = 0;

  /* readonly attribute long long lastModified; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastModified(int64_t *aLastModified) = 0;

  /* [noscript] PRFileDescPtr getFileDescriptor (); */
  NS_IMETHOD GetFileDescriptor(PRFileDesc * * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFileMetadata, NS_IFILEMETADATA_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFILEMETADATA \
  NS_IMETHOD GetSize(int64_t *aSize) override; \
  NS_IMETHOD GetLastModified(int64_t *aLastModified) override; \
  NS_IMETHOD GetFileDescriptor(PRFileDesc * * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFILEMETADATA \
  nsresult GetSize(int64_t *aSize); \
  nsresult GetLastModified(int64_t *aLastModified); \
  nsresult GetFileDescriptor(PRFileDesc * * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFILEMETADATA(_to) \
  NS_IMETHOD GetSize(int64_t *aSize) override { return _to GetSize(aSize); } \
  NS_IMETHOD GetLastModified(int64_t *aLastModified) override { return _to GetLastModified(aLastModified); } \
  NS_IMETHOD GetFileDescriptor(PRFileDesc * * _retval) override { return _to GetFileDescriptor(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFILEMETADATA(_to) \
  NS_IMETHOD GetSize(int64_t *aSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSize(aSize); } \
  NS_IMETHOD GetLastModified(int64_t *aLastModified) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastModified(aLastModified); } \
  NS_IMETHOD GetFileDescriptor(PRFileDesc * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileDescriptor(_retval); } 


/* starting interface:    nsIAsyncFileMetadata */
#define NS_IASYNCFILEMETADATA_IID_STR "de15b80b-29ba-4b7f-9220-a3d75b17ae8c"

#define NS_IASYNCFILEMETADATA_IID \
  {0xde15b80b, 0x29ba, 0x4b7f, \
    { 0x92, 0x20, 0xa3, 0xd7, 0x5b, 0x17, 0xae, 0x8c }}

class NS_NO_VTABLE nsIAsyncFileMetadata : public nsIFileMetadata {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IASYNCFILEMETADATA_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAsyncFileMetadata;

  /* void asyncFileMetadataWait (in nsIFileMetadataCallback aCallback, in nsIEventTarget aEventTarget); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncFileMetadataWait(nsIFileMetadataCallback *aCallback, nsIEventTarget *aEventTarget) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAsyncFileMetadata, NS_IASYNCFILEMETADATA_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIASYNCFILEMETADATA \
  NS_IMETHOD AsyncFileMetadataWait(nsIFileMetadataCallback *aCallback, nsIEventTarget *aEventTarget) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIASYNCFILEMETADATA \
  nsresult AsyncFileMetadataWait(nsIFileMetadataCallback *aCallback, nsIEventTarget *aEventTarget); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIASYNCFILEMETADATA(_to) \
  NS_IMETHOD AsyncFileMetadataWait(nsIFileMetadataCallback *aCallback, nsIEventTarget *aEventTarget) override { return _to AsyncFileMetadataWait(aCallback, aEventTarget); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIASYNCFILEMETADATA(_to) \
  NS_IMETHOD AsyncFileMetadataWait(nsIFileMetadataCallback *aCallback, nsIEventTarget *aEventTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncFileMetadataWait(aCallback, aEventTarget); } 


/* starting interface:    nsIFileMetadataCallback */
#define NS_IFILEMETADATACALLBACK_IID_STR "d01c7ead-7ba3-4726-b399-618ec8ec7057"

#define NS_IFILEMETADATACALLBACK_IID \
  {0xd01c7ead, 0x7ba3, 0x4726, \
    { 0xb3, 0x99, 0x61, 0x8e, 0xc8, 0xec, 0x70, 0x57 }}

class NS_NO_VTABLE nsIFileMetadataCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFILEMETADATACALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFileMetadataCallback;

  /* void onFileMetadataReady (in nsIAsyncFileMetadata aObject); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnFileMetadataReady(nsIAsyncFileMetadata *aObject) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFileMetadataCallback, NS_IFILEMETADATACALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFILEMETADATACALLBACK \
  NS_IMETHOD OnFileMetadataReady(nsIAsyncFileMetadata *aObject) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFILEMETADATACALLBACK \
  nsresult OnFileMetadataReady(nsIAsyncFileMetadata *aObject); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFILEMETADATACALLBACK(_to) \
  NS_IMETHOD OnFileMetadataReady(nsIAsyncFileMetadata *aObject) override { return _to OnFileMetadataReady(aObject); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFILEMETADATACALLBACK(_to) \
  NS_IMETHOD OnFileMetadataReady(nsIAsyncFileMetadata *aObject) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnFileMetadataReady(aObject); } 


#endif /* __gen_nsIFileStreams_h__ */
