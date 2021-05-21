/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libjar/nsIZipReader.idl
 */

#ifndef __gen_nsIZipReader_h__
#define __gen_nsIZipReader_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
struct PRFileDesc;
class nsIUTF8StringEnumerator; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsIFile; /* forward declaration */


/* starting interface:    nsIZipEntry */
#define NS_IZIPENTRY_IID_STR "fad6f72f-13d8-4e26-9173-53007a4afe71"

#define NS_IZIPENTRY_IID \
  {0xfad6f72f, 0x13d8, 0x4e26, \
    { 0x91, 0x73, 0x53, 0x00, 0x7a, 0x4a, 0xfe, 0x71 }}

class NS_NO_VTABLE nsIZipEntry : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IZIPENTRY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIZipEntry;

  /* readonly attribute unsigned short compression; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCompression(uint16_t *aCompression) = 0;

  /* readonly attribute unsigned long size; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSize(uint32_t *aSize) = 0;

  /* readonly attribute unsigned long realSize; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRealSize(uint32_t *aRealSize) = 0;

  /* readonly attribute unsigned long CRC32; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCRC32(uint32_t *aCRC32) = 0;

  /* readonly attribute boolean isDirectory; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsDirectory(bool *aIsDirectory) = 0;

  /* readonly attribute PRTime lastModifiedTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) = 0;

  /* readonly attribute boolean isSynthetic; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsSynthetic(bool *aIsSynthetic) = 0;

  /* readonly attribute unsigned long permissions; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPermissions(uint32_t *aPermissions) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIZipEntry, NS_IZIPENTRY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIZIPENTRY \
  NS_IMETHOD GetCompression(uint16_t *aCompression) override; \
  NS_IMETHOD GetSize(uint32_t *aSize) override; \
  NS_IMETHOD GetRealSize(uint32_t *aRealSize) override; \
  NS_IMETHOD GetCRC32(uint32_t *aCRC32) override; \
  NS_IMETHOD GetIsDirectory(bool *aIsDirectory) override; \
  NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) override; \
  NS_IMETHOD GetIsSynthetic(bool *aIsSynthetic) override; \
  NS_IMETHOD GetPermissions(uint32_t *aPermissions) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIZIPENTRY \
  nsresult GetCompression(uint16_t *aCompression); \
  nsresult GetSize(uint32_t *aSize); \
  nsresult GetRealSize(uint32_t *aRealSize); \
  nsresult GetCRC32(uint32_t *aCRC32); \
  nsresult GetIsDirectory(bool *aIsDirectory); \
  nsresult GetLastModifiedTime(PRTime *aLastModifiedTime); \
  nsresult GetIsSynthetic(bool *aIsSynthetic); \
  nsresult GetPermissions(uint32_t *aPermissions); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIZIPENTRY(_to) \
  NS_IMETHOD GetCompression(uint16_t *aCompression) override { return _to GetCompression(aCompression); } \
  NS_IMETHOD GetSize(uint32_t *aSize) override { return _to GetSize(aSize); } \
  NS_IMETHOD GetRealSize(uint32_t *aRealSize) override { return _to GetRealSize(aRealSize); } \
  NS_IMETHOD GetCRC32(uint32_t *aCRC32) override { return _to GetCRC32(aCRC32); } \
  NS_IMETHOD GetIsDirectory(bool *aIsDirectory) override { return _to GetIsDirectory(aIsDirectory); } \
  NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) override { return _to GetLastModifiedTime(aLastModifiedTime); } \
  NS_IMETHOD GetIsSynthetic(bool *aIsSynthetic) override { return _to GetIsSynthetic(aIsSynthetic); } \
  NS_IMETHOD GetPermissions(uint32_t *aPermissions) override { return _to GetPermissions(aPermissions); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIZIPENTRY(_to) \
  NS_IMETHOD GetCompression(uint16_t *aCompression) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCompression(aCompression); } \
  NS_IMETHOD GetSize(uint32_t *aSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSize(aSize); } \
  NS_IMETHOD GetRealSize(uint32_t *aRealSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRealSize(aRealSize); } \
  NS_IMETHOD GetCRC32(uint32_t *aCRC32) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCRC32(aCRC32); } \
  NS_IMETHOD GetIsDirectory(bool *aIsDirectory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsDirectory(aIsDirectory); } \
  NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastModifiedTime(aLastModifiedTime); } \
  NS_IMETHOD GetIsSynthetic(bool *aIsSynthetic) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsSynthetic(aIsSynthetic); } \
  NS_IMETHOD GetPermissions(uint32_t *aPermissions) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPermissions(aPermissions); } 


/* starting interface:    nsIZipReader */
#define NS_IZIPREADER_IID_STR "9ba4ef54-e0a0-4f65-9d23-128482448885"

#define NS_IZIPREADER_IID \
  {0x9ba4ef54, 0xe0a0, 0x4f65, \
    { 0x9d, 0x23, 0x12, 0x84, 0x82, 0x44, 0x88, 0x85 }}

class NS_NO_VTABLE nsIZipReader : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IZIPREADER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIZipReader;

  /* void open (in nsIFile zipFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Open(nsIFile *zipFile) = 0;

  /* void openInner (in nsIZipReader zipReader, in AUTF8String zipEntry); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenInner(nsIZipReader *zipReader, const nsACString& zipEntry) = 0;

  /* void openMemory (in voidPtr aData, in unsigned long aLength); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenMemory(void * aData, uint32_t aLength) = 0;

  /* readonly attribute nsIFile file; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFile(nsIFile **aFile) = 0;

  /* void close (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Close(void) = 0;

  /* void test (in AUTF8String aEntryName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Test(const nsACString& aEntryName) = 0;

  /* void extract (in AUTF8String zipEntry, in nsIFile outFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Extract(const nsACString& zipEntry, nsIFile *outFile) = 0;

  /* nsIZipEntry getEntry (in AUTF8String zipEntry); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEntry(const nsACString& zipEntry, nsIZipEntry **_retval) = 0;

  /* boolean hasEntry (in AUTF8String zipEntry); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasEntry(const nsACString& zipEntry, bool *_retval) = 0;

  /* nsIUTF8StringEnumerator findEntries (in AUTF8String aPattern); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FindEntries(const nsACString& aPattern, nsIUTF8StringEnumerator **_retval) = 0;

  /* nsIInputStream getInputStream (in AUTF8String zipEntry); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInputStream(const nsACString& zipEntry, nsIInputStream **_retval) = 0;

  /* nsIInputStream getInputStreamWithSpec (in AUTF8String aJarSpec, in AUTF8String zipEntry); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInputStreamWithSpec(const nsACString& aJarSpec, const nsACString& zipEntry, nsIInputStream **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIZipReader, NS_IZIPREADER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIZIPREADER \
  NS_IMETHOD Open(nsIFile *zipFile) override; \
  NS_IMETHOD OpenInner(nsIZipReader *zipReader, const nsACString& zipEntry) override; \
  NS_IMETHOD OpenMemory(void * aData, uint32_t aLength) override; \
  NS_IMETHOD GetFile(nsIFile **aFile) override; \
  NS_IMETHOD Close(void) override; \
  NS_IMETHOD Test(const nsACString& aEntryName) override; \
  NS_IMETHOD Extract(const nsACString& zipEntry, nsIFile *outFile) override; \
  NS_IMETHOD GetEntry(const nsACString& zipEntry, nsIZipEntry **_retval) override; \
  NS_IMETHOD HasEntry(const nsACString& zipEntry, bool *_retval) override; \
  NS_IMETHOD FindEntries(const nsACString& aPattern, nsIUTF8StringEnumerator **_retval) override; \
  NS_IMETHOD GetInputStream(const nsACString& zipEntry, nsIInputStream **_retval) override; \
  NS_IMETHOD GetInputStreamWithSpec(const nsACString& aJarSpec, const nsACString& zipEntry, nsIInputStream **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIZIPREADER \
  nsresult Open(nsIFile *zipFile); \
  nsresult OpenInner(nsIZipReader *zipReader, const nsACString& zipEntry); \
  nsresult OpenMemory(void * aData, uint32_t aLength); \
  nsresult GetFile(nsIFile **aFile); \
  nsresult Close(void); \
  nsresult Test(const nsACString& aEntryName); \
  nsresult Extract(const nsACString& zipEntry, nsIFile *outFile); \
  nsresult GetEntry(const nsACString& zipEntry, nsIZipEntry **_retval); \
  nsresult HasEntry(const nsACString& zipEntry, bool *_retval); \
  nsresult FindEntries(const nsACString& aPattern, nsIUTF8StringEnumerator **_retval); \
  nsresult GetInputStream(const nsACString& zipEntry, nsIInputStream **_retval); \
  nsresult GetInputStreamWithSpec(const nsACString& aJarSpec, const nsACString& zipEntry, nsIInputStream **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIZIPREADER(_to) \
  NS_IMETHOD Open(nsIFile *zipFile) override { return _to Open(zipFile); } \
  NS_IMETHOD OpenInner(nsIZipReader *zipReader, const nsACString& zipEntry) override { return _to OpenInner(zipReader, zipEntry); } \
  NS_IMETHOD OpenMemory(void * aData, uint32_t aLength) override { return _to OpenMemory(aData, aLength); } \
  NS_IMETHOD GetFile(nsIFile **aFile) override { return _to GetFile(aFile); } \
  NS_IMETHOD Close(void) override { return _to Close(); } \
  NS_IMETHOD Test(const nsACString& aEntryName) override { return _to Test(aEntryName); } \
  NS_IMETHOD Extract(const nsACString& zipEntry, nsIFile *outFile) override { return _to Extract(zipEntry, outFile); } \
  NS_IMETHOD GetEntry(const nsACString& zipEntry, nsIZipEntry **_retval) override { return _to GetEntry(zipEntry, _retval); } \
  NS_IMETHOD HasEntry(const nsACString& zipEntry, bool *_retval) override { return _to HasEntry(zipEntry, _retval); } \
  NS_IMETHOD FindEntries(const nsACString& aPattern, nsIUTF8StringEnumerator **_retval) override { return _to FindEntries(aPattern, _retval); } \
  NS_IMETHOD GetInputStream(const nsACString& zipEntry, nsIInputStream **_retval) override { return _to GetInputStream(zipEntry, _retval); } \
  NS_IMETHOD GetInputStreamWithSpec(const nsACString& aJarSpec, const nsACString& zipEntry, nsIInputStream **_retval) override { return _to GetInputStreamWithSpec(aJarSpec, zipEntry, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIZIPREADER(_to) \
  NS_IMETHOD Open(nsIFile *zipFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Open(zipFile); } \
  NS_IMETHOD OpenInner(nsIZipReader *zipReader, const nsACString& zipEntry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenInner(zipReader, zipEntry); } \
  NS_IMETHOD OpenMemory(void * aData, uint32_t aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenMemory(aData, aLength); } \
  NS_IMETHOD GetFile(nsIFile **aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFile(aFile); } \
  NS_IMETHOD Close(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(); } \
  NS_IMETHOD Test(const nsACString& aEntryName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Test(aEntryName); } \
  NS_IMETHOD Extract(const nsACString& zipEntry, nsIFile *outFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Extract(zipEntry, outFile); } \
  NS_IMETHOD GetEntry(const nsACString& zipEntry, nsIZipEntry **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEntry(zipEntry, _retval); } \
  NS_IMETHOD HasEntry(const nsACString& zipEntry, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasEntry(zipEntry, _retval); } \
  NS_IMETHOD FindEntries(const nsACString& aPattern, nsIUTF8StringEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FindEntries(aPattern, _retval); } \
  NS_IMETHOD GetInputStream(const nsACString& zipEntry, nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInputStream(zipEntry, _retval); } \
  NS_IMETHOD GetInputStreamWithSpec(const nsACString& aJarSpec, const nsACString& zipEntry, nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInputStreamWithSpec(aJarSpec, zipEntry, _retval); } 


/* starting interface:    nsIZipReaderCache */
#define NS_IZIPREADERCACHE_IID_STR "31179807-9fcd-46c4-befa-2ade209a394b"

#define NS_IZIPREADERCACHE_IID \
  {0x31179807, 0x9fcd, 0x46c4, \
    { 0xbe, 0xfa, 0x2a, 0xde, 0x20, 0x9a, 0x39, 0x4b }}

class NS_NO_VTABLE nsIZipReaderCache : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IZIPREADERCACHE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIZipReaderCache;

  /* void init (in unsigned long cacheSize); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(uint32_t cacheSize) = 0;

  /* nsIZipReader getZip (in nsIFile zipFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetZip(nsIFile *zipFile, nsIZipReader **_retval) = 0;

  /* nsIZipReader getZipIfCached (in nsIFile zipFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetZipIfCached(nsIFile *zipFile, nsIZipReader **_retval) = 0;

  /* bool isCached (in nsIFile zipFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsCached(nsIFile *zipFile, bool *_retval) = 0;

  /* nsIZipReader getInnerZip (in nsIFile zipFile, in AUTF8String zipEntry); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInnerZip(nsIFile *zipFile, const nsACString& zipEntry, nsIZipReader **_retval) = 0;

  /* PRFileDescStar getFd (in nsIFile zipFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFd(nsIFile *zipFile, PRFileDesc * * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIZipReaderCache, NS_IZIPREADERCACHE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIZIPREADERCACHE \
  NS_IMETHOD Init(uint32_t cacheSize) override; \
  NS_IMETHOD GetZip(nsIFile *zipFile, nsIZipReader **_retval) override; \
  NS_IMETHOD GetZipIfCached(nsIFile *zipFile, nsIZipReader **_retval) override; \
  NS_IMETHOD IsCached(nsIFile *zipFile, bool *_retval) override; \
  NS_IMETHOD GetInnerZip(nsIFile *zipFile, const nsACString& zipEntry, nsIZipReader **_retval) override; \
  NS_IMETHOD GetFd(nsIFile *zipFile, PRFileDesc * * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIZIPREADERCACHE \
  nsresult Init(uint32_t cacheSize); \
  nsresult GetZip(nsIFile *zipFile, nsIZipReader **_retval); \
  nsresult GetZipIfCached(nsIFile *zipFile, nsIZipReader **_retval); \
  nsresult IsCached(nsIFile *zipFile, bool *_retval); \
  nsresult GetInnerZip(nsIFile *zipFile, const nsACString& zipEntry, nsIZipReader **_retval); \
  nsresult GetFd(nsIFile *zipFile, PRFileDesc * * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIZIPREADERCACHE(_to) \
  NS_IMETHOD Init(uint32_t cacheSize) override { return _to Init(cacheSize); } \
  NS_IMETHOD GetZip(nsIFile *zipFile, nsIZipReader **_retval) override { return _to GetZip(zipFile, _retval); } \
  NS_IMETHOD GetZipIfCached(nsIFile *zipFile, nsIZipReader **_retval) override { return _to GetZipIfCached(zipFile, _retval); } \
  NS_IMETHOD IsCached(nsIFile *zipFile, bool *_retval) override { return _to IsCached(zipFile, _retval); } \
  NS_IMETHOD GetInnerZip(nsIFile *zipFile, const nsACString& zipEntry, nsIZipReader **_retval) override { return _to GetInnerZip(zipFile, zipEntry, _retval); } \
  NS_IMETHOD GetFd(nsIFile *zipFile, PRFileDesc * * _retval) override { return _to GetFd(zipFile, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIZIPREADERCACHE(_to) \
  NS_IMETHOD Init(uint32_t cacheSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(cacheSize); } \
  NS_IMETHOD GetZip(nsIFile *zipFile, nsIZipReader **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetZip(zipFile, _retval); } \
  NS_IMETHOD GetZipIfCached(nsIFile *zipFile, nsIZipReader **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetZipIfCached(zipFile, _retval); } \
  NS_IMETHOD IsCached(nsIFile *zipFile, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCached(zipFile, _retval); } \
  NS_IMETHOD GetInnerZip(nsIFile *zipFile, const nsACString& zipEntry, nsIZipReader **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInnerZip(zipFile, zipEntry, _retval); } \
  NS_IMETHOD GetFd(nsIFile *zipFile, PRFileDesc * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFd(zipFile, _retval); } 


#define NS_ZIPREADER_CID                             \
{ /* 88e2fd0b-f7f4-480c-9483-7846b00e8dad */         \
   0x88e2fd0b, 0xf7f4, 0x480c,                       \
  { 0x94, 0x83, 0x78, 0x46, 0xb0, 0x0e, 0x8d, 0xad } \
}
#define NS_ZIPREADERCACHE_CID                        \
{ /* 608b7f6f-4b60-40d6-87ed-d933bf53d8c1 */         \
   0x608b7f6f, 0x4b60, 0x40d6,                       \
  { 0x87, 0xed, 0xd9, 0x33, 0xbf, 0x53, 0xd8, 0xc1 } \
}

#endif /* __gen_nsIZipReader_h__ */
