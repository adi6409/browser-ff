/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/url-classifier/nsIChannelClassifierService.idl
 */

#ifndef __gen_nsIChannelClassifierService_h__
#define __gen_nsIChannelClassifierService_h__


#ifndef __gen_nsIContentPolicy_h__
#include "nsIContentPolicy.h"
#endif

#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIObserver; /* forward declaration */


/* starting interface:    nsIUrlClassifierBlockedChannel */
#define NS_IURLCLASSIFIERBLOCKEDCHANNEL_IID_STR "9b0353a7-ab46-4914-9178-2215ee221e4e"

#define NS_IURLCLASSIFIERBLOCKEDCHANNEL_IID \
  {0x9b0353a7, 0xab46, 0x4914, \
    { 0x91, 0x78, 0x22, 0x15, 0xee, 0x22, 0x1e, 0x4e }}

class NS_NO_VTABLE nsIUrlClassifierBlockedChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERBLOCKEDCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierBlockedChannel;

  enum {
    TRACKING_PROTECTION = 0U,
    SOCIAL_TRACKING_PROTECTION = 1U,
    FINGERPRINTING_PROTECTION = 2U,
    CRYPTOMINING_PROTECTION = 3U
  };

  /* readonly attribute uint8_t reason; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetReason(uint8_t *aReason) = 0;

  /* readonly attribute ACString tables; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTables(nsACString& aTables) = 0;

  /* readonly attribute AString url; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUrl(nsAString& aUrl) = 0;

  /* readonly attribute uint64_t tabId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTabId(uint64_t *aTabId) = 0;

  /* readonly attribute uint64_t channelId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetChannelId(uint64_t *aChannelId) = 0;

  /* readonly attribute boolean isPrivateBrowsing; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsPrivateBrowsing(bool *aIsPrivateBrowsing) = 0;

  /* readonly attribute AString topLevelUrl; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTopLevelUrl(nsAString& aTopLevelUrl) = 0;

  /* void unblock (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Unblock(void) = 0;

  /* void allow (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Allow(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierBlockedChannel, NS_IURLCLASSIFIERBLOCKEDCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERBLOCKEDCHANNEL \
  NS_IMETHOD GetReason(uint8_t *aReason) override; \
  NS_IMETHOD GetTables(nsACString& aTables) override; \
  NS_IMETHOD GetUrl(nsAString& aUrl) override; \
  NS_IMETHOD GetTabId(uint64_t *aTabId) override; \
  NS_IMETHOD GetChannelId(uint64_t *aChannelId) override; \
  NS_IMETHOD GetIsPrivateBrowsing(bool *aIsPrivateBrowsing) override; \
  NS_IMETHOD GetTopLevelUrl(nsAString& aTopLevelUrl) override; \
  NS_IMETHOD Unblock(void) override; \
  NS_IMETHOD Allow(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERBLOCKEDCHANNEL \
  nsresult GetReason(uint8_t *aReason); \
  nsresult GetTables(nsACString& aTables); \
  nsresult GetUrl(nsAString& aUrl); \
  nsresult GetTabId(uint64_t *aTabId); \
  nsresult GetChannelId(uint64_t *aChannelId); \
  nsresult GetIsPrivateBrowsing(bool *aIsPrivateBrowsing); \
  nsresult GetTopLevelUrl(nsAString& aTopLevelUrl); \
  nsresult Unblock(void); \
  nsresult Allow(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERBLOCKEDCHANNEL(_to) \
  NS_IMETHOD GetReason(uint8_t *aReason) override { return _to GetReason(aReason); } \
  NS_IMETHOD GetTables(nsACString& aTables) override { return _to GetTables(aTables); } \
  NS_IMETHOD GetUrl(nsAString& aUrl) override { return _to GetUrl(aUrl); } \
  NS_IMETHOD GetTabId(uint64_t *aTabId) override { return _to GetTabId(aTabId); } \
  NS_IMETHOD GetChannelId(uint64_t *aChannelId) override { return _to GetChannelId(aChannelId); } \
  NS_IMETHOD GetIsPrivateBrowsing(bool *aIsPrivateBrowsing) override { return _to GetIsPrivateBrowsing(aIsPrivateBrowsing); } \
  NS_IMETHOD GetTopLevelUrl(nsAString& aTopLevelUrl) override { return _to GetTopLevelUrl(aTopLevelUrl); } \
  NS_IMETHOD Unblock(void) override { return _to Unblock(); } \
  NS_IMETHOD Allow(void) override { return _to Allow(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERBLOCKEDCHANNEL(_to) \
  NS_IMETHOD GetReason(uint8_t *aReason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReason(aReason); } \
  NS_IMETHOD GetTables(nsACString& aTables) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTables(aTables); } \
  NS_IMETHOD GetUrl(nsAString& aUrl) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUrl(aUrl); } \
  NS_IMETHOD GetTabId(uint64_t *aTabId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTabId(aTabId); } \
  NS_IMETHOD GetChannelId(uint64_t *aChannelId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChannelId(aChannelId); } \
  NS_IMETHOD GetIsPrivateBrowsing(bool *aIsPrivateBrowsing) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsPrivateBrowsing(aIsPrivateBrowsing); } \
  NS_IMETHOD GetTopLevelUrl(nsAString& aTopLevelUrl) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTopLevelUrl(aTopLevelUrl); } \
  NS_IMETHOD Unblock(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Unblock(); } \
  NS_IMETHOD Allow(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Allow(); } 


/* starting interface:    nsIChannelClassifierService */
#define NS_ICHANNELCLASSIFIERSERVICE_IID_STR "9411409c-5dac-40b9-ba36-2738a7237a4c"

#define NS_ICHANNELCLASSIFIERSERVICE_IID \
  {0x9411409c, 0x5dac, 0x40b9, \
    { 0xba, 0x36, 0x27, 0x38, 0xa7, 0x23, 0x7a, 0x4c }}

class NS_NO_VTABLE nsIChannelClassifierService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICHANNELCLASSIFIERSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIChannelClassifierService;

  /* void addListener (in nsIObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddListener(nsIObserver *aObserver) = 0;

  /* void removeListener (in nsIObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveListener(nsIObserver *aObserver) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIChannelClassifierService, NS_ICHANNELCLASSIFIERSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICHANNELCLASSIFIERSERVICE \
  NS_IMETHOD AddListener(nsIObserver *aObserver) override; \
  NS_IMETHOD RemoveListener(nsIObserver *aObserver) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICHANNELCLASSIFIERSERVICE \
  nsresult AddListener(nsIObserver *aObserver); \
  nsresult RemoveListener(nsIObserver *aObserver); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICHANNELCLASSIFIERSERVICE(_to) \
  NS_IMETHOD AddListener(nsIObserver *aObserver) override { return _to AddListener(aObserver); } \
  NS_IMETHOD RemoveListener(nsIObserver *aObserver) override { return _to RemoveListener(aObserver); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICHANNELCLASSIFIERSERVICE(_to) \
  NS_IMETHOD AddListener(nsIObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddListener(aObserver); } \
  NS_IMETHOD RemoveListener(nsIObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveListener(aObserver); } 


#endif /* __gen_nsIChannelClassifierService_h__ */
