/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpActivityObserver.idl
 */

#ifndef __gen_nsIHttpActivityObserver_h__
#define __gen_nsIHttpActivityObserver_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace net {
class HttpActivityArgs;
} // namespace net
} // namespace mozilla

/* starting interface:    nsIHttpActivityObserver */
#define NS_IHTTPACTIVITYOBSERVER_IID_STR "412880c8-6c36-48d8-bf8f-84f91f892503"

#define NS_IHTTPACTIVITYOBSERVER_IID \
  {0x412880c8, 0x6c36, 0x48d8, \
    { 0xbf, 0x8f, 0x84, 0xf9, 0x1f, 0x89, 0x25, 0x03 }}

class NS_NO_VTABLE nsIHttpActivityObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPACTIVITYOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHttpActivityObserver;

  /* [must_use] void observeActivity (in nsISupports aHttpChannel, in uint32_t aActivityType, in uint32_t aActivitySubtype, in PRTime aTimestamp, in uint64_t aExtraSizeData, in ACString aExtraStringData); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ObserveActivity(nsISupports *aHttpChannel, uint32_t aActivityType, uint32_t aActivitySubtype, PRTime aTimestamp, uint64_t aExtraSizeData, const nsACString& aExtraStringData) = 0;

  /* [must_use] readonly attribute boolean isActive; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetIsActive(bool *aIsActive) = 0;

  /* [noscript] void setIsActive (in boolean aActived); */
  NS_IMETHOD SetIsActive(bool aActived) = 0;

  /* [must_use,noscript] void observeActivityWithArgs (in HttpActivityArgs aArgs, in uint32_t aActivityType, in uint32_t aActivitySubtype, in PRTime aTimestamp, in uint64_t aExtraSizeData, in ACString aExtraStringData); */
  [[nodiscard]] NS_IMETHOD ObserveActivityWithArgs(const mozilla::net::HttpActivityArgs & aArgs, uint32_t aActivityType, uint32_t aActivitySubtype, PRTime aTimestamp, uint64_t aExtraSizeData, const nsACString& aExtraStringData) = 0;

  enum {
    ACTIVITY_TYPE_SOCKET_TRANSPORT = 1U,
    ACTIVITY_TYPE_HTTP_TRANSACTION = 2U,
    ACTIVITY_SUBTYPE_REQUEST_HEADER = 20481U,
    ACTIVITY_SUBTYPE_REQUEST_BODY_SENT = 20482U,
    ACTIVITY_SUBTYPE_RESPONSE_START = 20483U,
    ACTIVITY_SUBTYPE_RESPONSE_HEADER = 20484U,
    ACTIVITY_SUBTYPE_RESPONSE_COMPLETE = 20485U,
    ACTIVITY_SUBTYPE_TRANSACTION_CLOSE = 20486U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpActivityObserver, NS_IHTTPACTIVITYOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPACTIVITYOBSERVER \
  [[nodiscard]] NS_IMETHOD ObserveActivity(nsISupports *aHttpChannel, uint32_t aActivityType, uint32_t aActivitySubtype, PRTime aTimestamp, uint64_t aExtraSizeData, const nsACString& aExtraStringData) override; \
  [[nodiscard]] NS_IMETHOD GetIsActive(bool *aIsActive) override; \
  NS_IMETHOD SetIsActive(bool aActived) override; \
  [[nodiscard]] NS_IMETHOD ObserveActivityWithArgs(const mozilla::net::HttpActivityArgs & aArgs, uint32_t aActivityType, uint32_t aActivitySubtype, PRTime aTimestamp, uint64_t aExtraSizeData, const nsACString& aExtraStringData) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPACTIVITYOBSERVER \
  [[nodiscard]] nsresult ObserveActivity(nsISupports *aHttpChannel, uint32_t aActivityType, uint32_t aActivitySubtype, PRTime aTimestamp, uint64_t aExtraSizeData, const nsACString& aExtraStringData); \
  [[nodiscard]] nsresult GetIsActive(bool *aIsActive); \
  nsresult SetIsActive(bool aActived); \
  [[nodiscard]] nsresult ObserveActivityWithArgs(const mozilla::net::HttpActivityArgs & aArgs, uint32_t aActivityType, uint32_t aActivitySubtype, PRTime aTimestamp, uint64_t aExtraSizeData, const nsACString& aExtraStringData); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPACTIVITYOBSERVER(_to) \
  [[nodiscard]] NS_IMETHOD ObserveActivity(nsISupports *aHttpChannel, uint32_t aActivityType, uint32_t aActivitySubtype, PRTime aTimestamp, uint64_t aExtraSizeData, const nsACString& aExtraStringData) override { return _to ObserveActivity(aHttpChannel, aActivityType, aActivitySubtype, aTimestamp, aExtraSizeData, aExtraStringData); } \
  [[nodiscard]] NS_IMETHOD GetIsActive(bool *aIsActive) override { return _to GetIsActive(aIsActive); } \
  NS_IMETHOD SetIsActive(bool aActived) override { return _to SetIsActive(aActived); } \
  [[nodiscard]] NS_IMETHOD ObserveActivityWithArgs(const mozilla::net::HttpActivityArgs & aArgs, uint32_t aActivityType, uint32_t aActivitySubtype, PRTime aTimestamp, uint64_t aExtraSizeData, const nsACString& aExtraStringData) override { return _to ObserveActivityWithArgs(aArgs, aActivityType, aActivitySubtype, aTimestamp, aExtraSizeData, aExtraStringData); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPACTIVITYOBSERVER(_to) \
  [[nodiscard]] NS_IMETHOD ObserveActivity(nsISupports *aHttpChannel, uint32_t aActivityType, uint32_t aActivitySubtype, PRTime aTimestamp, uint64_t aExtraSizeData, const nsACString& aExtraStringData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ObserveActivity(aHttpChannel, aActivityType, aActivitySubtype, aTimestamp, aExtraSizeData, aExtraStringData); } \
  [[nodiscard]] NS_IMETHOD GetIsActive(bool *aIsActive) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsActive(aIsActive); } \
  NS_IMETHOD SetIsActive(bool aActived) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsActive(aActived); } \
  [[nodiscard]] NS_IMETHOD ObserveActivityWithArgs(const mozilla::net::HttpActivityArgs & aArgs, uint32_t aActivityType, uint32_t aActivitySubtype, PRTime aTimestamp, uint64_t aExtraSizeData, const nsACString& aExtraStringData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ObserveActivityWithArgs(aArgs, aActivityType, aActivitySubtype, aTimestamp, aExtraSizeData, aExtraStringData); } \


#define NS_HTTP_ACTIVITY_TYPE_SOCKET_TRANSPORT    \
            nsIHttpActivityObserver::ACTIVITY_TYPE_SOCKET_TRANSPORT
#define NS_HTTP_ACTIVITY_TYPE_HTTP_TRANSACTION    \
            nsIHttpActivityObserver::ACTIVITY_TYPE_HTTP_TRANSACTION
#define NS_HTTP_ACTIVITY_SUBTYPE_REQUEST_HEADER    \
            nsIHttpActivityObserver::ACTIVITY_SUBTYPE_REQUEST_HEADER
#define NS_HTTP_ACTIVITY_SUBTYPE_REQUEST_BODY_SENT \
            nsIHttpActivityObserver::ACTIVITY_SUBTYPE_REQUEST_BODY_SENT
#define NS_HTTP_ACTIVITY_SUBTYPE_RESPONSE_START    \
            nsIHttpActivityObserver::ACTIVITY_SUBTYPE_RESPONSE_START
#define NS_HTTP_ACTIVITY_SUBTYPE_RESPONSE_HEADER   \
            nsIHttpActivityObserver::ACTIVITY_SUBTYPE_RESPONSE_HEADER
#define NS_HTTP_ACTIVITY_SUBTYPE_RESPONSE_COMPLETE \
            nsIHttpActivityObserver::ACTIVITY_SUBTYPE_RESPONSE_COMPLETE
#define NS_HTTP_ACTIVITY_SUBTYPE_TRANSACTION_CLOSE \
            nsIHttpActivityObserver::ACTIVITY_SUBTYPE_TRANSACTION_CLOSE

/* starting interface:    nsIHttpActivityDistributor */
#define NS_IHTTPACTIVITYDISTRIBUTOR_IID_STR "7c512cb8-582a-4625-b5b6-8639755271b5"

#define NS_IHTTPACTIVITYDISTRIBUTOR_IID \
  {0x7c512cb8, 0x582a, 0x4625, \
    { 0xb5, 0xb6, 0x86, 0x39, 0x75, 0x52, 0x71, 0xb5 }}

class NS_NO_VTABLE nsIHttpActivityDistributor : public nsIHttpActivityObserver {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPACTIVITYDISTRIBUTOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHttpActivityDistributor;

  /* void addObserver (in nsIHttpActivityObserver aObserver); */
  NS_IMETHOD AddObserver(nsIHttpActivityObserver *aObserver) = 0;

  /* void removeObserver (in nsIHttpActivityObserver aObserver); */
  NS_IMETHOD RemoveObserver(nsIHttpActivityObserver *aObserver) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpActivityDistributor, NS_IHTTPACTIVITYDISTRIBUTOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPACTIVITYDISTRIBUTOR \
  NS_IMETHOD AddObserver(nsIHttpActivityObserver *aObserver) override; \
  NS_IMETHOD RemoveObserver(nsIHttpActivityObserver *aObserver) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPACTIVITYDISTRIBUTOR \
  nsresult AddObserver(nsIHttpActivityObserver *aObserver); \
  nsresult RemoveObserver(nsIHttpActivityObserver *aObserver); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPACTIVITYDISTRIBUTOR(_to) \
  NS_IMETHOD AddObserver(nsIHttpActivityObserver *aObserver) override { return _to AddObserver(aObserver); } \
  NS_IMETHOD RemoveObserver(nsIHttpActivityObserver *aObserver) override { return _to RemoveObserver(aObserver); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPACTIVITYDISTRIBUTOR(_to) \
  NS_IMETHOD AddObserver(nsIHttpActivityObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddObserver(aObserver); } \
  NS_IMETHOD RemoveObserver(nsIHttpActivityObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveObserver(aObserver); } 


#endif /* __gen_nsIHttpActivityObserver_h__ */
