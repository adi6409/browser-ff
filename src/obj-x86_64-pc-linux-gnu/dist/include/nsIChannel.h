/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIChannel.idl
 */

#ifndef __gen_nsIChannel_h__
#define __gen_nsIChannel_h__


#ifndef __gen_nsIRequest_h__
#include "nsIRequest.h"
#endif

#ifndef __gen_nsILoadInfo_h__
#include "nsILoadInfo.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIInterfaceRequestor; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsIStreamListener; /* forward declaration */

#include "nsCOMPtr.h"

/* starting interface:    nsIChannel */
#define NS_ICHANNEL_IID_STR "2c389865-23db-4aa7-9fe5-60cc7b00697e"

#define NS_ICHANNEL_IID \
  {0x2c389865, 0x23db, 0x4aa7, \
    { 0x9f, 0xe5, 0x60, 0xcc, 0x7b, 0x00, 0x69, 0x7e }}

class nsIChannel : public nsIRequest {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIChannel;

  /* attribute nsIURI originalURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOriginalURI(nsIURI **aOriginalURI) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetOriginalURI(nsIURI *aOriginalURI) = 0;

  /* readonly attribute nsIURI URI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetURI(nsIURI **aURI) = 0;

  /* attribute nsISupports owner; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOwner(nsISupports **aOwner) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetOwner(nsISupports *aOwner) = 0;

  /* attribute nsIInterfaceRequestor notificationCallbacks; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetNotificationCallbacks(nsIInterfaceRequestor *aNotificationCallbacks) = 0;

  /* readonly attribute nsISupports securityInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) = 0;

  /* attribute ACString contentType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContentType(nsACString& aContentType) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetContentType(const nsACString& aContentType) = 0;

  /* attribute ACString contentCharset; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContentCharset(nsACString& aContentCharset) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetContentCharset(const nsACString& aContentCharset) = 0;

  /* attribute int64_t contentLength; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContentLength(int64_t *aContentLength) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetContentLength(int64_t aContentLength) = 0;

  /* nsIInputStream open (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Open(nsIInputStream **_retval) = 0;

  /* void asyncOpen (in nsIStreamListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncOpen(nsIStreamListener *aListener) = 0;

  /* [must_use] readonly attribute boolean canceled; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetCanceled(bool *aCanceled) = 0;

  enum {
    LOAD_DOCUMENT_URI = 65536U,
    LOAD_RETARGETED_DOCUMENT_URI = 131072U,
    LOAD_REPLACE = 262144U,
    LOAD_INITIAL_DOCUMENT_URI = 524288U,
    LOAD_TARGETED = 1048576U,
    LOAD_CALL_CONTENT_SNIFFERS = 2097152U,
    LOAD_BYPASS_URL_CLASSIFIER = 4194304U,
    LOAD_MEDIA_SNIFFER_OVERRIDES_CONTENT_TYPE = 8388608U,
    LOAD_EXPLICIT_CREDENTIALS = 16777216U,
    LOAD_BYPASS_SERVICE_WORKER = 33554432U
  };

  /* attribute unsigned long contentDisposition; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContentDisposition(uint32_t *aContentDisposition) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetContentDisposition(uint32_t aContentDisposition) = 0;

  enum {
    DISPOSITION_INLINE = 0U,
    DISPOSITION_ATTACHMENT = 1U
  };

  /* attribute AString contentDispositionFilename; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContentDispositionFilename(nsAString& aContentDispositionFilename) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetContentDispositionFilename(const nsAString& aContentDispositionFilename) = 0;

  /* readonly attribute ACString contentDispositionHeader; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContentDispositionHeader(nsACString& aContentDispositionHeader) = 0;

  /* attribute nsILoadInfo loadInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLoadInfo(nsILoadInfo **aLoadInfo) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLoadInfo(nsILoadInfo *aLoadInfo) = 0;

  /* readonly attribute bool isDocument; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsDocument(bool *aIsDocument) = 0;

   inline bool IsDocument()
  {
    bool isDocument = false;
    if (NS_SUCCEEDED(GetIsDocument(&isDocument)) && isDocument) {
      return true;
    }
    return false;
  }
  inline already_AddRefed<nsILoadInfo> LoadInfo()
  {
    nsCOMPtr<nsILoadInfo> result;
    mozilla::DebugOnly<nsresult> rv = GetLoadInfo(getter_AddRefs(result));
    MOZ_ASSERT(NS_SUCCEEDED(rv) && result);
    return result.forget();
  }
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIChannel, NS_ICHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICHANNEL \
  NS_IMETHOD GetOriginalURI(nsIURI **aOriginalURI) override; \
  NS_IMETHOD SetOriginalURI(nsIURI *aOriginalURI) override; \
  NS_IMETHOD GetURI(nsIURI **aURI) override; \
  NS_IMETHOD GetOwner(nsISupports **aOwner) override; \
  NS_IMETHOD SetOwner(nsISupports *aOwner) override; \
  NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) override; \
  NS_IMETHOD SetNotificationCallbacks(nsIInterfaceRequestor *aNotificationCallbacks) override; \
  NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) override; \
  NS_IMETHOD GetContentType(nsACString& aContentType) override; \
  NS_IMETHOD SetContentType(const nsACString& aContentType) override; \
  NS_IMETHOD GetContentCharset(nsACString& aContentCharset) override; \
  NS_IMETHOD SetContentCharset(const nsACString& aContentCharset) override; \
  NS_IMETHOD GetContentLength(int64_t *aContentLength) override; \
  NS_IMETHOD SetContentLength(int64_t aContentLength) override; \
  NS_IMETHOD Open(nsIInputStream **_retval) override; \
  NS_IMETHOD AsyncOpen(nsIStreamListener *aListener) override; \
  [[nodiscard]] NS_IMETHOD GetCanceled(bool *aCanceled) override; \
  NS_IMETHOD GetContentDisposition(uint32_t *aContentDisposition) override; \
  NS_IMETHOD SetContentDisposition(uint32_t aContentDisposition) override; \
  NS_IMETHOD GetContentDispositionFilename(nsAString& aContentDispositionFilename) override; \
  NS_IMETHOD SetContentDispositionFilename(const nsAString& aContentDispositionFilename) override; \
  NS_IMETHOD GetContentDispositionHeader(nsACString& aContentDispositionHeader) override; \
  NS_IMETHOD GetLoadInfo(nsILoadInfo **aLoadInfo) override; \
  NS_IMETHOD SetLoadInfo(nsILoadInfo *aLoadInfo) override; \
  NS_IMETHOD GetIsDocument(bool *aIsDocument) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICHANNEL \
  nsresult GetOriginalURI(nsIURI **aOriginalURI); \
  nsresult SetOriginalURI(nsIURI *aOriginalURI); \
  nsresult GetURI(nsIURI **aURI); \
  nsresult GetOwner(nsISupports **aOwner); \
  nsresult SetOwner(nsISupports *aOwner); \
  nsresult GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks); \
  nsresult SetNotificationCallbacks(nsIInterfaceRequestor *aNotificationCallbacks); \
  nsresult GetSecurityInfo(nsISupports **aSecurityInfo); \
  nsresult GetContentType(nsACString& aContentType); \
  nsresult SetContentType(const nsACString& aContentType); \
  nsresult GetContentCharset(nsACString& aContentCharset); \
  nsresult SetContentCharset(const nsACString& aContentCharset); \
  nsresult GetContentLength(int64_t *aContentLength); \
  nsresult SetContentLength(int64_t aContentLength); \
  nsresult Open(nsIInputStream **_retval); \
  nsresult AsyncOpen(nsIStreamListener *aListener); \
  [[nodiscard]] nsresult GetCanceled(bool *aCanceled); \
  nsresult GetContentDisposition(uint32_t *aContentDisposition); \
  nsresult SetContentDisposition(uint32_t aContentDisposition); \
  nsresult GetContentDispositionFilename(nsAString& aContentDispositionFilename); \
  nsresult SetContentDispositionFilename(const nsAString& aContentDispositionFilename); \
  nsresult GetContentDispositionHeader(nsACString& aContentDispositionHeader); \
  nsresult GetLoadInfo(nsILoadInfo **aLoadInfo); \
  nsresult SetLoadInfo(nsILoadInfo *aLoadInfo); \
  nsresult GetIsDocument(bool *aIsDocument); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICHANNEL(_to) \
  NS_IMETHOD GetOriginalURI(nsIURI **aOriginalURI) override { return _to GetOriginalURI(aOriginalURI); } \
  NS_IMETHOD SetOriginalURI(nsIURI *aOriginalURI) override { return _to SetOriginalURI(aOriginalURI); } \
  NS_IMETHOD GetURI(nsIURI **aURI) override { return _to GetURI(aURI); } \
  NS_IMETHOD GetOwner(nsISupports **aOwner) override { return _to GetOwner(aOwner); } \
  NS_IMETHOD SetOwner(nsISupports *aOwner) override { return _to SetOwner(aOwner); } \
  NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) override { return _to GetNotificationCallbacks(aNotificationCallbacks); } \
  NS_IMETHOD SetNotificationCallbacks(nsIInterfaceRequestor *aNotificationCallbacks) override { return _to SetNotificationCallbacks(aNotificationCallbacks); } \
  NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) override { return _to GetSecurityInfo(aSecurityInfo); } \
  NS_IMETHOD GetContentType(nsACString& aContentType) override { return _to GetContentType(aContentType); } \
  NS_IMETHOD SetContentType(const nsACString& aContentType) override { return _to SetContentType(aContentType); } \
  NS_IMETHOD GetContentCharset(nsACString& aContentCharset) override { return _to GetContentCharset(aContentCharset); } \
  NS_IMETHOD SetContentCharset(const nsACString& aContentCharset) override { return _to SetContentCharset(aContentCharset); } \
  NS_IMETHOD GetContentLength(int64_t *aContentLength) override { return _to GetContentLength(aContentLength); } \
  NS_IMETHOD SetContentLength(int64_t aContentLength) override { return _to SetContentLength(aContentLength); } \
  NS_IMETHOD Open(nsIInputStream **_retval) override { return _to Open(_retval); } \
  NS_IMETHOD AsyncOpen(nsIStreamListener *aListener) override { return _to AsyncOpen(aListener); } \
  [[nodiscard]] NS_IMETHOD GetCanceled(bool *aCanceled) override { return _to GetCanceled(aCanceled); } \
  NS_IMETHOD GetContentDisposition(uint32_t *aContentDisposition) override { return _to GetContentDisposition(aContentDisposition); } \
  NS_IMETHOD SetContentDisposition(uint32_t aContentDisposition) override { return _to SetContentDisposition(aContentDisposition); } \
  NS_IMETHOD GetContentDispositionFilename(nsAString& aContentDispositionFilename) override { return _to GetContentDispositionFilename(aContentDispositionFilename); } \
  NS_IMETHOD SetContentDispositionFilename(const nsAString& aContentDispositionFilename) override { return _to SetContentDispositionFilename(aContentDispositionFilename); } \
  NS_IMETHOD GetContentDispositionHeader(nsACString& aContentDispositionHeader) override { return _to GetContentDispositionHeader(aContentDispositionHeader); } \
  NS_IMETHOD GetLoadInfo(nsILoadInfo **aLoadInfo) override { return _to GetLoadInfo(aLoadInfo); } \
  NS_IMETHOD SetLoadInfo(nsILoadInfo *aLoadInfo) override { return _to SetLoadInfo(aLoadInfo); } \
  NS_IMETHOD GetIsDocument(bool *aIsDocument) override { return _to GetIsDocument(aIsDocument); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICHANNEL(_to) \
  NS_IMETHOD GetOriginalURI(nsIURI **aOriginalURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginalURI(aOriginalURI); } \
  NS_IMETHOD SetOriginalURI(nsIURI *aOriginalURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOriginalURI(aOriginalURI); } \
  NS_IMETHOD GetURI(nsIURI **aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURI(aURI); } \
  NS_IMETHOD GetOwner(nsISupports **aOwner) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOwner(aOwner); } \
  NS_IMETHOD SetOwner(nsISupports *aOwner) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOwner(aOwner); } \
  NS_IMETHOD GetNotificationCallbacks(nsIInterfaceRequestor **aNotificationCallbacks) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNotificationCallbacks(aNotificationCallbacks); } \
  NS_IMETHOD SetNotificationCallbacks(nsIInterfaceRequestor *aNotificationCallbacks) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNotificationCallbacks(aNotificationCallbacks); } \
  NS_IMETHOD GetSecurityInfo(nsISupports **aSecurityInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecurityInfo(aSecurityInfo); } \
  NS_IMETHOD GetContentType(nsACString& aContentType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentType(aContentType); } \
  NS_IMETHOD SetContentType(const nsACString& aContentType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContentType(aContentType); } \
  NS_IMETHOD GetContentCharset(nsACString& aContentCharset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentCharset(aContentCharset); } \
  NS_IMETHOD SetContentCharset(const nsACString& aContentCharset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContentCharset(aContentCharset); } \
  NS_IMETHOD GetContentLength(int64_t *aContentLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentLength(aContentLength); } \
  NS_IMETHOD SetContentLength(int64_t aContentLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContentLength(aContentLength); } \
  NS_IMETHOD Open(nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Open(_retval); } \
  NS_IMETHOD AsyncOpen(nsIStreamListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncOpen(aListener); } \
  [[nodiscard]] NS_IMETHOD GetCanceled(bool *aCanceled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanceled(aCanceled); } \
  NS_IMETHOD GetContentDisposition(uint32_t *aContentDisposition) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentDisposition(aContentDisposition); } \
  NS_IMETHOD SetContentDisposition(uint32_t aContentDisposition) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContentDisposition(aContentDisposition); } \
  NS_IMETHOD GetContentDispositionFilename(nsAString& aContentDispositionFilename) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentDispositionFilename(aContentDispositionFilename); } \
  NS_IMETHOD SetContentDispositionFilename(const nsAString& aContentDispositionFilename) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContentDispositionFilename(aContentDispositionFilename); } \
  NS_IMETHOD GetContentDispositionHeader(nsACString& aContentDispositionHeader) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentDispositionHeader(aContentDispositionHeader); } \
  NS_IMETHOD GetLoadInfo(nsILoadInfo **aLoadInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadInfo(aLoadInfo); } \
  NS_IMETHOD SetLoadInfo(nsILoadInfo *aLoadInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLoadInfo(aLoadInfo); } \
  NS_IMETHOD GetIsDocument(bool *aIsDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsDocument(aIsDocument); } \


/* starting interface:    nsIIdentChannel */
#define NS_IIDENTCHANNEL_IID_STR "1ebbff64-d742-4f4a-aad5-4df2d1eb937a"

#define NS_IIDENTCHANNEL_IID \
  {0x1ebbff64, 0xd742, 0x4f4a, \
    { 0xaa, 0xd5, 0x4d, 0xf2, 0xd1, 0xeb, 0x93, 0x7a }}

class nsIIdentChannel : public nsIChannel {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IIDENTCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIIdentChannel;

  /* [must_use] attribute uint64_t channelId; */
  [[nodiscard]] NS_IMETHOD GetChannelId(uint64_t *aChannelId) = 0;
  [[nodiscard]] NS_IMETHOD SetChannelId(uint64_t aChannelId) = 0;

   inline uint64_t ChannelId()
  {
    uint64_t value = 0;
    if (NS_SUCCEEDED(GetChannelId(&value))) {
      return value;
    }
    return 0;
  }
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIIdentChannel, NS_IIDENTCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIIDENTCHANNEL \
  [[nodiscard]] NS_IMETHOD GetChannelId(uint64_t *aChannelId) override; \
  [[nodiscard]] NS_IMETHOD SetChannelId(uint64_t aChannelId) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIIDENTCHANNEL \
  [[nodiscard]] nsresult GetChannelId(uint64_t *aChannelId); \
  [[nodiscard]] nsresult SetChannelId(uint64_t aChannelId); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIIDENTCHANNEL(_to) \
  [[nodiscard]] NS_IMETHOD GetChannelId(uint64_t *aChannelId) override { return _to GetChannelId(aChannelId); } \
  [[nodiscard]] NS_IMETHOD SetChannelId(uint64_t aChannelId) override { return _to SetChannelId(aChannelId); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIIDENTCHANNEL(_to) \
  [[nodiscard]] NS_IMETHOD GetChannelId(uint64_t *aChannelId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChannelId(aChannelId); } \
  [[nodiscard]] NS_IMETHOD SetChannelId(uint64_t aChannelId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetChannelId(aChannelId); } \


#endif /* __gen_nsIChannel_h__ */
