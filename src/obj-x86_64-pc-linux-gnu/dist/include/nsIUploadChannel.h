/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIUploadChannel.idl
 */

#ifndef __gen_nsIUploadChannel_h__
#define __gen_nsIUploadChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */


/* starting interface:    nsIUploadChannel */
#define NS_IUPLOADCHANNEL_IID_STR "5cfe15bd-5adb-4a7f-9e55-4f5a67d15794"

#define NS_IUPLOADCHANNEL_IID \
  {0x5cfe15bd, 0x5adb, 0x4a7f, \
    { 0x9e, 0x55, 0x4f, 0x5a, 0x67, 0xd1, 0x57, 0x94 }}

class NS_NO_VTABLE nsIUploadChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUPLOADCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUploadChannel;

  /* void setUploadStream (in nsIInputStream aStream, in ACString aContentType, in long long aContentLength); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetUploadStream(nsIInputStream *aStream, const nsACString& aContentType, int64_t aContentLength) = 0;

  /* readonly attribute nsIInputStream uploadStream; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUploadStream(nsIInputStream **aUploadStream) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUploadChannel, NS_IUPLOADCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUPLOADCHANNEL \
  NS_IMETHOD SetUploadStream(nsIInputStream *aStream, const nsACString& aContentType, int64_t aContentLength) override; \
  NS_IMETHOD GetUploadStream(nsIInputStream **aUploadStream) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUPLOADCHANNEL \
  nsresult SetUploadStream(nsIInputStream *aStream, const nsACString& aContentType, int64_t aContentLength); \
  nsresult GetUploadStream(nsIInputStream **aUploadStream); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUPLOADCHANNEL(_to) \
  NS_IMETHOD SetUploadStream(nsIInputStream *aStream, const nsACString& aContentType, int64_t aContentLength) override { return _to SetUploadStream(aStream, aContentType, aContentLength); } \
  NS_IMETHOD GetUploadStream(nsIInputStream **aUploadStream) override { return _to GetUploadStream(aUploadStream); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUPLOADCHANNEL(_to) \
  NS_IMETHOD SetUploadStream(nsIInputStream *aStream, const nsACString& aContentType, int64_t aContentLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUploadStream(aStream, aContentType, aContentLength); } \
  NS_IMETHOD GetUploadStream(nsIInputStream **aUploadStream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUploadStream(aUploadStream); } 


#endif /* __gen_nsIUploadChannel_h__ */
