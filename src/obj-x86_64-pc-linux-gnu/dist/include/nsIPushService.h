/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/push/nsIPushService.idl
 */

#ifndef __gen_nsIPushService_h__
#define __gen_nsIPushService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */


/* starting interface:    nsIPushSubscription */
#define NS_IPUSHSUBSCRIPTION_IID_STR "1de32d5c-ea88-4c9e-9626-b032bd87f415"

#define NS_IPUSHSUBSCRIPTION_IID \
  {0x1de32d5c, 0xea88, 0x4c9e, \
    { 0x96, 0x26, 0xb0, 0x32, 0xbd, 0x87, 0xf4, 0x15 }}

class NS_NO_VTABLE nsIPushSubscription : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPUSHSUBSCRIPTION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPushSubscription;

  /* readonly attribute AString endpoint; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEndpoint(nsAString& aEndpoint) = 0;

  /* readonly attribute long long pushCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPushCount(int64_t *aPushCount) = 0;

  /* readonly attribute long long lastPush; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastPush(int64_t *aLastPush) = 0;

  /* readonly attribute long quota; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetQuota(int32_t *aQuota) = 0;

  /* readonly attribute bool isSystemSubscription; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsSystemSubscription(bool *aIsSystemSubscription) = 0;

  /* readonly attribute jsval p256dhPrivateKey; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetP256dhPrivateKey(JS::MutableHandleValue aP256dhPrivateKey) = 0;

  /* bool quotaApplies (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD QuotaApplies(bool *_retval) = 0;

  /* bool isExpired (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsExpired(bool *_retval) = 0;

  /* Array<uint8_t> getKey (in AString name); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetKey(const nsAString& name, nsTArray<uint8_t >& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPushSubscription, NS_IPUSHSUBSCRIPTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPUSHSUBSCRIPTION \
  NS_IMETHOD GetEndpoint(nsAString& aEndpoint) override; \
  NS_IMETHOD GetPushCount(int64_t *aPushCount) override; \
  NS_IMETHOD GetLastPush(int64_t *aLastPush) override; \
  NS_IMETHOD GetQuota(int32_t *aQuota) override; \
  NS_IMETHOD GetIsSystemSubscription(bool *aIsSystemSubscription) override; \
  NS_IMETHOD GetP256dhPrivateKey(JS::MutableHandleValue aP256dhPrivateKey) override; \
  NS_IMETHOD QuotaApplies(bool *_retval) override; \
  NS_IMETHOD IsExpired(bool *_retval) override; \
  NS_IMETHOD GetKey(const nsAString& name, nsTArray<uint8_t >& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPUSHSUBSCRIPTION \
  nsresult GetEndpoint(nsAString& aEndpoint); \
  nsresult GetPushCount(int64_t *aPushCount); \
  nsresult GetLastPush(int64_t *aLastPush); \
  nsresult GetQuota(int32_t *aQuota); \
  nsresult GetIsSystemSubscription(bool *aIsSystemSubscription); \
  nsresult GetP256dhPrivateKey(JS::MutableHandleValue aP256dhPrivateKey); \
  nsresult QuotaApplies(bool *_retval); \
  nsresult IsExpired(bool *_retval); \
  nsresult GetKey(const nsAString& name, nsTArray<uint8_t >& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPUSHSUBSCRIPTION(_to) \
  NS_IMETHOD GetEndpoint(nsAString& aEndpoint) override { return _to GetEndpoint(aEndpoint); } \
  NS_IMETHOD GetPushCount(int64_t *aPushCount) override { return _to GetPushCount(aPushCount); } \
  NS_IMETHOD GetLastPush(int64_t *aLastPush) override { return _to GetLastPush(aLastPush); } \
  NS_IMETHOD GetQuota(int32_t *aQuota) override { return _to GetQuota(aQuota); } \
  NS_IMETHOD GetIsSystemSubscription(bool *aIsSystemSubscription) override { return _to GetIsSystemSubscription(aIsSystemSubscription); } \
  NS_IMETHOD GetP256dhPrivateKey(JS::MutableHandleValue aP256dhPrivateKey) override { return _to GetP256dhPrivateKey(aP256dhPrivateKey); } \
  NS_IMETHOD QuotaApplies(bool *_retval) override { return _to QuotaApplies(_retval); } \
  NS_IMETHOD IsExpired(bool *_retval) override { return _to IsExpired(_retval); } \
  NS_IMETHOD GetKey(const nsAString& name, nsTArray<uint8_t >& _retval) override { return _to GetKey(name, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPUSHSUBSCRIPTION(_to) \
  NS_IMETHOD GetEndpoint(nsAString& aEndpoint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEndpoint(aEndpoint); } \
  NS_IMETHOD GetPushCount(int64_t *aPushCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPushCount(aPushCount); } \
  NS_IMETHOD GetLastPush(int64_t *aLastPush) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastPush(aLastPush); } \
  NS_IMETHOD GetQuota(int32_t *aQuota) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetQuota(aQuota); } \
  NS_IMETHOD GetIsSystemSubscription(bool *aIsSystemSubscription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsSystemSubscription(aIsSystemSubscription); } \
  NS_IMETHOD GetP256dhPrivateKey(JS::MutableHandleValue aP256dhPrivateKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetP256dhPrivateKey(aP256dhPrivateKey); } \
  NS_IMETHOD QuotaApplies(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->QuotaApplies(_retval); } \
  NS_IMETHOD IsExpired(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsExpired(_retval); } \
  NS_IMETHOD GetKey(const nsAString& name, nsTArray<uint8_t >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKey(name, _retval); } 


/* starting interface:    nsIPushSubscriptionCallback */
#define NS_IPUSHSUBSCRIPTIONCALLBACK_IID_STR "1799c074-9d52-46b0-ab3c-c09790732f6f"

#define NS_IPUSHSUBSCRIPTIONCALLBACK_IID \
  {0x1799c074, 0x9d52, 0x46b0, \
    { 0xab, 0x3c, 0xc0, 0x97, 0x90, 0x73, 0x2f, 0x6f }}

class NS_NO_VTABLE nsIPushSubscriptionCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPUSHSUBSCRIPTIONCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPushSubscriptionCallback;

  /* void onPushSubscription (in nsresult status, in nsIPushSubscription subscription); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnPushSubscription(nsresult status, nsIPushSubscription *subscription) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPushSubscriptionCallback, NS_IPUSHSUBSCRIPTIONCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPUSHSUBSCRIPTIONCALLBACK \
  NS_IMETHOD OnPushSubscription(nsresult status, nsIPushSubscription *subscription) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPUSHSUBSCRIPTIONCALLBACK \
  nsresult OnPushSubscription(nsresult status, nsIPushSubscription *subscription); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPUSHSUBSCRIPTIONCALLBACK(_to) \
  NS_IMETHOD OnPushSubscription(nsresult status, nsIPushSubscription *subscription) override { return _to OnPushSubscription(status, subscription); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPUSHSUBSCRIPTIONCALLBACK(_to) \
  NS_IMETHOD OnPushSubscription(nsresult status, nsIPushSubscription *subscription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnPushSubscription(status, subscription); } 


/* starting interface:    nsIUnsubscribeResultCallback */
#define NS_IUNSUBSCRIBERESULTCALLBACK_IID_STR "d574118f-61a9-4270-b1f6-4461aa85c4f5"

#define NS_IUNSUBSCRIBERESULTCALLBACK_IID \
  {0xd574118f, 0x61a9, 0x4270, \
    { 0xb1, 0xf6, 0x44, 0x61, 0xaa, 0x85, 0xc4, 0xf5 }}

class NS_NO_VTABLE nsIUnsubscribeResultCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUNSUBSCRIBERESULTCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUnsubscribeResultCallback;

  /* void onUnsubscribe (in nsresult status, in bool success); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnUnsubscribe(nsresult status, bool success) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUnsubscribeResultCallback, NS_IUNSUBSCRIBERESULTCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUNSUBSCRIBERESULTCALLBACK \
  NS_IMETHOD OnUnsubscribe(nsresult status, bool success) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUNSUBSCRIBERESULTCALLBACK \
  nsresult OnUnsubscribe(nsresult status, bool success); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUNSUBSCRIBERESULTCALLBACK(_to) \
  NS_IMETHOD OnUnsubscribe(nsresult status, bool success) override { return _to OnUnsubscribe(status, success); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUNSUBSCRIBERESULTCALLBACK(_to) \
  NS_IMETHOD OnUnsubscribe(nsresult status, bool success) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnUnsubscribe(status, success); } 


/* starting interface:    nsIPushClearResultCallback */
#define NS_IPUSHCLEARRESULTCALLBACK_IID_STR "bd47b38e-8bfa-4f92-834e-832a4431e05e"

#define NS_IPUSHCLEARRESULTCALLBACK_IID \
  {0xbd47b38e, 0x8bfa, 0x4f92, \
    { 0x83, 0x4e, 0x83, 0x2a, 0x44, 0x31, 0xe0, 0x5e }}

class NS_NO_VTABLE nsIPushClearResultCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPUSHCLEARRESULTCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPushClearResultCallback;

  /* void onClear (in nsresult status); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnClear(nsresult status) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPushClearResultCallback, NS_IPUSHCLEARRESULTCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPUSHCLEARRESULTCALLBACK \
  NS_IMETHOD OnClear(nsresult status) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPUSHCLEARRESULTCALLBACK \
  nsresult OnClear(nsresult status); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPUSHCLEARRESULTCALLBACK(_to) \
  NS_IMETHOD OnClear(nsresult status) override { return _to OnClear(status); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPUSHCLEARRESULTCALLBACK(_to) \
  NS_IMETHOD OnClear(nsresult status) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnClear(status); } 


/* starting interface:    nsIPushService */
#define NS_IPUSHSERVICE_IID_STR "678ef584-bf25-47aa-ac84-03efc0865b68"

#define NS_IPUSHSERVICE_IID \
  {0x678ef584, 0xbf25, 0x47aa, \
    { 0xac, 0x84, 0x03, 0xef, 0xc0, 0x86, 0x5b, 0x68 }}

class NS_NO_VTABLE nsIPushService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPUSHSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPushService;

  /* readonly attribute AString pushTopic; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPushTopic(nsAString& aPushTopic) = 0;

  /* readonly attribute AString subscriptionChangeTopic; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSubscriptionChangeTopic(nsAString& aSubscriptionChangeTopic) = 0;

  /* readonly attribute AString subscriptionModifiedTopic; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSubscriptionModifiedTopic(nsAString& aSubscriptionModifiedTopic) = 0;

  /* void subscribe (in AString scope, in nsIPrincipal principal, in nsIPushSubscriptionCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Subscribe(const nsAString& scope, nsIPrincipal *principal, nsIPushSubscriptionCallback *callback) = 0;

  /* void subscribeWithKey (in AString scope, in nsIPrincipal principal, in Array<uint8_t> key, in nsIPushSubscriptionCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SubscribeWithKey(const nsAString& scope, nsIPrincipal *principal, const nsTArray<uint8_t >& key, nsIPushSubscriptionCallback *callback) = 0;

  /* void unsubscribe (in AString scope, in nsIPrincipal principal, in nsIUnsubscribeResultCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Unsubscribe(const nsAString& scope, nsIPrincipal *principal, nsIUnsubscribeResultCallback *callback) = 0;

  /* void getSubscription (in AString scope, in nsIPrincipal principal, in nsIPushSubscriptionCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSubscription(const nsAString& scope, nsIPrincipal *principal, nsIPushSubscriptionCallback *callback) = 0;

  /* void clearForDomain (in AString domain, in nsIPushClearResultCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearForDomain(const nsAString& domain, nsIPushClearResultCallback *callback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPushService, NS_IPUSHSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPUSHSERVICE \
  NS_IMETHOD GetPushTopic(nsAString& aPushTopic) override; \
  NS_IMETHOD GetSubscriptionChangeTopic(nsAString& aSubscriptionChangeTopic) override; \
  NS_IMETHOD GetSubscriptionModifiedTopic(nsAString& aSubscriptionModifiedTopic) override; \
  NS_IMETHOD Subscribe(const nsAString& scope, nsIPrincipal *principal, nsIPushSubscriptionCallback *callback) override; \
  NS_IMETHOD SubscribeWithKey(const nsAString& scope, nsIPrincipal *principal, const nsTArray<uint8_t >& key, nsIPushSubscriptionCallback *callback) override; \
  NS_IMETHOD Unsubscribe(const nsAString& scope, nsIPrincipal *principal, nsIUnsubscribeResultCallback *callback) override; \
  NS_IMETHOD GetSubscription(const nsAString& scope, nsIPrincipal *principal, nsIPushSubscriptionCallback *callback) override; \
  NS_IMETHOD ClearForDomain(const nsAString& domain, nsIPushClearResultCallback *callback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPUSHSERVICE \
  nsresult GetPushTopic(nsAString& aPushTopic); \
  nsresult GetSubscriptionChangeTopic(nsAString& aSubscriptionChangeTopic); \
  nsresult GetSubscriptionModifiedTopic(nsAString& aSubscriptionModifiedTopic); \
  nsresult Subscribe(const nsAString& scope, nsIPrincipal *principal, nsIPushSubscriptionCallback *callback); \
  nsresult SubscribeWithKey(const nsAString& scope, nsIPrincipal *principal, const nsTArray<uint8_t >& key, nsIPushSubscriptionCallback *callback); \
  nsresult Unsubscribe(const nsAString& scope, nsIPrincipal *principal, nsIUnsubscribeResultCallback *callback); \
  nsresult GetSubscription(const nsAString& scope, nsIPrincipal *principal, nsIPushSubscriptionCallback *callback); \
  nsresult ClearForDomain(const nsAString& domain, nsIPushClearResultCallback *callback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPUSHSERVICE(_to) \
  NS_IMETHOD GetPushTopic(nsAString& aPushTopic) override { return _to GetPushTopic(aPushTopic); } \
  NS_IMETHOD GetSubscriptionChangeTopic(nsAString& aSubscriptionChangeTopic) override { return _to GetSubscriptionChangeTopic(aSubscriptionChangeTopic); } \
  NS_IMETHOD GetSubscriptionModifiedTopic(nsAString& aSubscriptionModifiedTopic) override { return _to GetSubscriptionModifiedTopic(aSubscriptionModifiedTopic); } \
  NS_IMETHOD Subscribe(const nsAString& scope, nsIPrincipal *principal, nsIPushSubscriptionCallback *callback) override { return _to Subscribe(scope, principal, callback); } \
  NS_IMETHOD SubscribeWithKey(const nsAString& scope, nsIPrincipal *principal, const nsTArray<uint8_t >& key, nsIPushSubscriptionCallback *callback) override { return _to SubscribeWithKey(scope, principal, key, callback); } \
  NS_IMETHOD Unsubscribe(const nsAString& scope, nsIPrincipal *principal, nsIUnsubscribeResultCallback *callback) override { return _to Unsubscribe(scope, principal, callback); } \
  NS_IMETHOD GetSubscription(const nsAString& scope, nsIPrincipal *principal, nsIPushSubscriptionCallback *callback) override { return _to GetSubscription(scope, principal, callback); } \
  NS_IMETHOD ClearForDomain(const nsAString& domain, nsIPushClearResultCallback *callback) override { return _to ClearForDomain(domain, callback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPUSHSERVICE(_to) \
  NS_IMETHOD GetPushTopic(nsAString& aPushTopic) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPushTopic(aPushTopic); } \
  NS_IMETHOD GetSubscriptionChangeTopic(nsAString& aSubscriptionChangeTopic) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSubscriptionChangeTopic(aSubscriptionChangeTopic); } \
  NS_IMETHOD GetSubscriptionModifiedTopic(nsAString& aSubscriptionModifiedTopic) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSubscriptionModifiedTopic(aSubscriptionModifiedTopic); } \
  NS_IMETHOD Subscribe(const nsAString& scope, nsIPrincipal *principal, nsIPushSubscriptionCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Subscribe(scope, principal, callback); } \
  NS_IMETHOD SubscribeWithKey(const nsAString& scope, nsIPrincipal *principal, const nsTArray<uint8_t >& key, nsIPushSubscriptionCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SubscribeWithKey(scope, principal, key, callback); } \
  NS_IMETHOD Unsubscribe(const nsAString& scope, nsIPrincipal *principal, nsIUnsubscribeResultCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Unsubscribe(scope, principal, callback); } \
  NS_IMETHOD GetSubscription(const nsAString& scope, nsIPrincipal *principal, nsIPushSubscriptionCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSubscription(scope, principal, callback); } \
  NS_IMETHOD ClearForDomain(const nsAString& domain, nsIPushClearResultCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearForDomain(domain, callback); } 


/* starting interface:    nsIPushQuotaManager */
#define NS_IPUSHQUOTAMANAGER_IID_STR "a2555e70-46f8-4b52-bf02-d978b979d143"

#define NS_IPUSHQUOTAMANAGER_IID \
  {0xa2555e70, 0x46f8, 0x4b52, \
    { 0xbf, 0x02, 0xd9, 0x78, 0xb9, 0x79, 0xd1, 0x43 }}

class NS_NO_VTABLE nsIPushQuotaManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPUSHQUOTAMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPushQuotaManager;

  /* void notificationForOriginShown (in string origin); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotificationForOriginShown(const char * origin) = 0;

  /* void notificationForOriginClosed (in string origin); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotificationForOriginClosed(const char * origin) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPushQuotaManager, NS_IPUSHQUOTAMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPUSHQUOTAMANAGER \
  NS_IMETHOD NotificationForOriginShown(const char * origin) override; \
  NS_IMETHOD NotificationForOriginClosed(const char * origin) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPUSHQUOTAMANAGER \
  nsresult NotificationForOriginShown(const char * origin); \
  nsresult NotificationForOriginClosed(const char * origin); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPUSHQUOTAMANAGER(_to) \
  NS_IMETHOD NotificationForOriginShown(const char * origin) override { return _to NotificationForOriginShown(origin); } \
  NS_IMETHOD NotificationForOriginClosed(const char * origin) override { return _to NotificationForOriginClosed(origin); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPUSHQUOTAMANAGER(_to) \
  NS_IMETHOD NotificationForOriginShown(const char * origin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotificationForOriginShown(origin); } \
  NS_IMETHOD NotificationForOriginClosed(const char * origin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotificationForOriginClosed(origin); } 


#endif /* __gen_nsIPushService_h__ */
