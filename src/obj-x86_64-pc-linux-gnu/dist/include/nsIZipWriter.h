/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libjar/zipwriter/nsIZipWriter.idl
 */

#ifndef __gen_nsIZipWriter_h__
#define __gen_nsIZipWriter_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsIRequestObserver; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIZipEntry; /* forward declaration */


/* starting interface:    nsIZipWriter */
#define NS_IZIPWRITER_IID_STR "3ca10750-797e-4a22-bcfe-66170b5e96dd"

#define NS_IZIPWRITER_IID \
  {0x3ca10750, 0x797e, 0x4a22, \
    { 0xbc, 0xfe, 0x66, 0x17, 0x0b, 0x5e, 0x96, 0xdd }}

class NS_NO_VTABLE nsIZipWriter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IZIPWRITER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIZipWriter;

  enum {
    COMPRESSION_NONE = 0U,
    COMPRESSION_FASTEST = 1U,
    COMPRESSION_DEFAULT = 6U,
    COMPRESSION_BEST = 9U
  };

  /* attribute ACString comment; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetComment(nsACString& aComment) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetComment(const nsACString& aComment) = 0;

  /* readonly attribute boolean inQueue; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInQueue(bool *aInQueue) = 0;

  /* readonly attribute nsIFile file; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFile(nsIFile **aFile) = 0;

  /* void open (in nsIFile aFile, in int32_t aIoFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Open(nsIFile *aFile, int32_t aIoFlags) = 0;

  /* nsIZipEntry getEntry (in AUTF8String aZipEntry); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEntry(const nsACString& aZipEntry, nsIZipEntry **_retval) = 0;

  /* boolean hasEntry (in AUTF8String aZipEntry); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasEntry(const nsACString& aZipEntry, bool *_retval) = 0;

  /* void addEntryDirectory (in AUTF8String aZipEntry, in PRTime aModTime, in boolean aQueue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddEntryDirectory(const nsACString& aZipEntry, PRTime aModTime, bool aQueue) = 0;

  /* void addEntryFile (in AUTF8String aZipEntry, in int32_t aCompression, in nsIFile aFile, in boolean aQueue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddEntryFile(const nsACString& aZipEntry, int32_t aCompression, nsIFile *aFile, bool aQueue) = 0;

  /* void addEntryChannel (in AUTF8String aZipEntry, in PRTime aModTime, in int32_t aCompression, in nsIChannel aChannel, in boolean aQueue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddEntryChannel(const nsACString& aZipEntry, PRTime aModTime, int32_t aCompression, nsIChannel *aChannel, bool aQueue) = 0;

  /* void addEntryStream (in AUTF8String aZipEntry, in PRTime aModTime, in int32_t aCompression, in nsIInputStream aStream, in boolean aQueue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddEntryStream(const nsACString& aZipEntry, PRTime aModTime, int32_t aCompression, nsIInputStream *aStream, bool aQueue) = 0;

  /* void removeEntry (in AUTF8String aZipEntry, in boolean aQueue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveEntry(const nsACString& aZipEntry, bool aQueue) = 0;

  /* void processQueue (in nsIRequestObserver aObserver, in nsISupports aContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ProcessQueue(nsIRequestObserver *aObserver, nsISupports *aContext) = 0;

  /* void close (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Close(void) = 0;

  /* void alignStoredFiles (in uint16_t aAlignSize); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AlignStoredFiles(uint16_t aAlignSize) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIZipWriter, NS_IZIPWRITER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIZIPWRITER \
  NS_IMETHOD GetComment(nsACString& aComment) override; \
  NS_IMETHOD SetComment(const nsACString& aComment) override; \
  NS_IMETHOD GetInQueue(bool *aInQueue) override; \
  NS_IMETHOD GetFile(nsIFile **aFile) override; \
  NS_IMETHOD Open(nsIFile *aFile, int32_t aIoFlags) override; \
  NS_IMETHOD GetEntry(const nsACString& aZipEntry, nsIZipEntry **_retval) override; \
  NS_IMETHOD HasEntry(const nsACString& aZipEntry, bool *_retval) override; \
  NS_IMETHOD AddEntryDirectory(const nsACString& aZipEntry, PRTime aModTime, bool aQueue) override; \
  NS_IMETHOD AddEntryFile(const nsACString& aZipEntry, int32_t aCompression, nsIFile *aFile, bool aQueue) override; \
  NS_IMETHOD AddEntryChannel(const nsACString& aZipEntry, PRTime aModTime, int32_t aCompression, nsIChannel *aChannel, bool aQueue) override; \
  NS_IMETHOD AddEntryStream(const nsACString& aZipEntry, PRTime aModTime, int32_t aCompression, nsIInputStream *aStream, bool aQueue) override; \
  NS_IMETHOD RemoveEntry(const nsACString& aZipEntry, bool aQueue) override; \
  NS_IMETHOD ProcessQueue(nsIRequestObserver *aObserver, nsISupports *aContext) override; \
  NS_IMETHOD Close(void) override; \
  NS_IMETHOD AlignStoredFiles(uint16_t aAlignSize) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIZIPWRITER \
  nsresult GetComment(nsACString& aComment); \
  nsresult SetComment(const nsACString& aComment); \
  nsresult GetInQueue(bool *aInQueue); \
  nsresult GetFile(nsIFile **aFile); \
  nsresult Open(nsIFile *aFile, int32_t aIoFlags); \
  nsresult GetEntry(const nsACString& aZipEntry, nsIZipEntry **_retval); \
  nsresult HasEntry(const nsACString& aZipEntry, bool *_retval); \
  nsresult AddEntryDirectory(const nsACString& aZipEntry, PRTime aModTime, bool aQueue); \
  nsresult AddEntryFile(const nsACString& aZipEntry, int32_t aCompression, nsIFile *aFile, bool aQueue); \
  nsresult AddEntryChannel(const nsACString& aZipEntry, PRTime aModTime, int32_t aCompression, nsIChannel *aChannel, bool aQueue); \
  nsresult AddEntryStream(const nsACString& aZipEntry, PRTime aModTime, int32_t aCompression, nsIInputStream *aStream, bool aQueue); \
  nsresult RemoveEntry(const nsACString& aZipEntry, bool aQueue); \
  nsresult ProcessQueue(nsIRequestObserver *aObserver, nsISupports *aContext); \
  nsresult Close(void); \
  nsresult AlignStoredFiles(uint16_t aAlignSize); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIZIPWRITER(_to) \
  NS_IMETHOD GetComment(nsACString& aComment) override { return _to GetComment(aComment); } \
  NS_IMETHOD SetComment(const nsACString& aComment) override { return _to SetComment(aComment); } \
  NS_IMETHOD GetInQueue(bool *aInQueue) override { return _to GetInQueue(aInQueue); } \
  NS_IMETHOD GetFile(nsIFile **aFile) override { return _to GetFile(aFile); } \
  NS_IMETHOD Open(nsIFile *aFile, int32_t aIoFlags) override { return _to Open(aFile, aIoFlags); } \
  NS_IMETHOD GetEntry(const nsACString& aZipEntry, nsIZipEntry **_retval) override { return _to GetEntry(aZipEntry, _retval); } \
  NS_IMETHOD HasEntry(const nsACString& aZipEntry, bool *_retval) override { return _to HasEntry(aZipEntry, _retval); } \
  NS_IMETHOD AddEntryDirectory(const nsACString& aZipEntry, PRTime aModTime, bool aQueue) override { return _to AddEntryDirectory(aZipEntry, aModTime, aQueue); } \
  NS_IMETHOD AddEntryFile(const nsACString& aZipEntry, int32_t aCompression, nsIFile *aFile, bool aQueue) override { return _to AddEntryFile(aZipEntry, aCompression, aFile, aQueue); } \
  NS_IMETHOD AddEntryChannel(const nsACString& aZipEntry, PRTime aModTime, int32_t aCompression, nsIChannel *aChannel, bool aQueue) override { return _to AddEntryChannel(aZipEntry, aModTime, aCompression, aChannel, aQueue); } \
  NS_IMETHOD AddEntryStream(const nsACString& aZipEntry, PRTime aModTime, int32_t aCompression, nsIInputStream *aStream, bool aQueue) override { return _to AddEntryStream(aZipEntry, aModTime, aCompression, aStream, aQueue); } \
  NS_IMETHOD RemoveEntry(const nsACString& aZipEntry, bool aQueue) override { return _to RemoveEntry(aZipEntry, aQueue); } \
  NS_IMETHOD ProcessQueue(nsIRequestObserver *aObserver, nsISupports *aContext) override { return _to ProcessQueue(aObserver, aContext); } \
  NS_IMETHOD Close(void) override { return _to Close(); } \
  NS_IMETHOD AlignStoredFiles(uint16_t aAlignSize) override { return _to AlignStoredFiles(aAlignSize); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIZIPWRITER(_to) \
  NS_IMETHOD GetComment(nsACString& aComment) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetComment(aComment); } \
  NS_IMETHOD SetComment(const nsACString& aComment) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetComment(aComment); } \
  NS_IMETHOD GetInQueue(bool *aInQueue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInQueue(aInQueue); } \
  NS_IMETHOD GetFile(nsIFile **aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFile(aFile); } \
  NS_IMETHOD Open(nsIFile *aFile, int32_t aIoFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Open(aFile, aIoFlags); } \
  NS_IMETHOD GetEntry(const nsACString& aZipEntry, nsIZipEntry **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEntry(aZipEntry, _retval); } \
  NS_IMETHOD HasEntry(const nsACString& aZipEntry, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasEntry(aZipEntry, _retval); } \
  NS_IMETHOD AddEntryDirectory(const nsACString& aZipEntry, PRTime aModTime, bool aQueue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddEntryDirectory(aZipEntry, aModTime, aQueue); } \
  NS_IMETHOD AddEntryFile(const nsACString& aZipEntry, int32_t aCompression, nsIFile *aFile, bool aQueue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddEntryFile(aZipEntry, aCompression, aFile, aQueue); } \
  NS_IMETHOD AddEntryChannel(const nsACString& aZipEntry, PRTime aModTime, int32_t aCompression, nsIChannel *aChannel, bool aQueue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddEntryChannel(aZipEntry, aModTime, aCompression, aChannel, aQueue); } \
  NS_IMETHOD AddEntryStream(const nsACString& aZipEntry, PRTime aModTime, int32_t aCompression, nsIInputStream *aStream, bool aQueue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddEntryStream(aZipEntry, aModTime, aCompression, aStream, aQueue); } \
  NS_IMETHOD RemoveEntry(const nsACString& aZipEntry, bool aQueue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveEntry(aZipEntry, aQueue); } \
  NS_IMETHOD ProcessQueue(nsIRequestObserver *aObserver, nsISupports *aContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ProcessQueue(aObserver, aContext); } \
  NS_IMETHOD Close(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(); } \
  NS_IMETHOD AlignStoredFiles(uint16_t aAlignSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AlignStoredFiles(aAlignSize); } 


#endif /* __gen_nsIZipWriter_h__ */
