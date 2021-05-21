/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libjar/nsIJARChannel.idl
 */

#ifndef __gen_nsIJARChannel_h__
#define __gen_nsIJARChannel_h__


#ifndef __gen_nsIChannel_h__
#include "nsIChannel.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */

class nsIZipEntry; /* forward declaration */


/* starting interface:    nsIJARChannel */
#define NS_IJARCHANNEL_IID_STR "e72b179b-d5df-4d87-b5de-fd73a65c60f6"

#define NS_IJARCHANNEL_IID \
  {0xe72b179b, 0xd5df, 0x4d87, \
    { 0xb5, 0xde, 0xfd, 0x73, 0xa6, 0x5c, 0x60, 0xf6 }}

class NS_NO_VTABLE nsIJARChannel : public nsIChannel {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IJARCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIJARChannel;

  /* attribute nsIFile jarFile; */
  NS_IMETHOD GetJarFile(nsIFile **aJarFile) = 0;
  NS_IMETHOD SetJarFile(nsIFile *aJarFile) = 0;

  /* readonly attribute nsIZipEntry zipEntry; */
  NS_IMETHOD GetZipEntry(nsIZipEntry **aZipEntry) = 0;

  /* boolean ensureCached (); */
  NS_IMETHOD EnsureCached(bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIJARChannel, NS_IJARCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIJARCHANNEL \
  NS_IMETHOD GetJarFile(nsIFile **aJarFile) override; \
  NS_IMETHOD SetJarFile(nsIFile *aJarFile) override; \
  NS_IMETHOD GetZipEntry(nsIZipEntry **aZipEntry) override; \
  NS_IMETHOD EnsureCached(bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIJARCHANNEL \
  nsresult GetJarFile(nsIFile **aJarFile); \
  nsresult SetJarFile(nsIFile *aJarFile); \
  nsresult GetZipEntry(nsIZipEntry **aZipEntry); \
  nsresult EnsureCached(bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIJARCHANNEL(_to) \
  NS_IMETHOD GetJarFile(nsIFile **aJarFile) override { return _to GetJarFile(aJarFile); } \
  NS_IMETHOD SetJarFile(nsIFile *aJarFile) override { return _to SetJarFile(aJarFile); } \
  NS_IMETHOD GetZipEntry(nsIZipEntry **aZipEntry) override { return _to GetZipEntry(aZipEntry); } \
  NS_IMETHOD EnsureCached(bool *_retval) override { return _to EnsureCached(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIJARCHANNEL(_to) \
  NS_IMETHOD GetJarFile(nsIFile **aJarFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetJarFile(aJarFile); } \
  NS_IMETHOD SetJarFile(nsIFile *aJarFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetJarFile(aJarFile); } \
  NS_IMETHOD GetZipEntry(nsIZipEntry **aZipEntry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetZipEntry(aZipEntry); } \
  NS_IMETHOD EnsureCached(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnsureCached(_retval); } 


#endif /* __gen_nsIJARChannel_h__ */
