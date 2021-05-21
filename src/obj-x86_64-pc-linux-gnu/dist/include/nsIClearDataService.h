/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/cleardata/nsIClearDataService.idl
 */

#ifndef __gen_nsIClearDataService_h__
#define __gen_nsIClearDataService_h__


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

class nsIClearDataCallback; /* forward declaration */


/* starting interface:    nsIClearDataService */
#define NS_ICLEARDATASERVICE_IID_STR "6ef3ef16-a502-4576-9fb4-919f1c40bf61"

#define NS_ICLEARDATASERVICE_IID \
  {0x6ef3ef16, 0xa502, 0x4576, \
    { 0x9f, 0xb4, 0x91, 0x9f, 0x1c, 0x40, 0xbf, 0x61 }}

class NS_NO_VTABLE nsIClearDataService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICLEARDATASERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIClearDataService;

  /* void deleteDataFromLocalFiles (in bool aIsUserRequest, in uint32_t aFlags, in nsIClearDataCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteDataFromLocalFiles(bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) = 0;

  /* void deleteDataFromHost (in AUTF8String aHost, in bool aIsUserRequest, in uint32_t aFlags, in nsIClearDataCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteDataFromHost(const nsACString& aHost, bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) = 0;

  /* void deleteDataFromPrincipal (in nsIPrincipal aPrincipal, in bool aIsUserRequest, in uint32_t aFlags, in nsIClearDataCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteDataFromPrincipal(nsIPrincipal *aPrincipal, bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) = 0;

  /* void deleteDataInTimeRange (in PRTime aFrom, in PRTime aTo, in bool aIsUserRequest, in uint32_t aFlags, in nsIClearDataCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteDataInTimeRange(PRTime aFrom, PRTime aTo, bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) = 0;

  /* void deleteData (in uint32_t aFlags, in nsIClearDataCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteData(uint32_t aFlags, nsIClearDataCallback *aCallback) = 0;

  /* void deleteDataFromOriginAttributesPattern (in jsval aOriginAttributesPattern, [optional] in nsIClearDataCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteDataFromOriginAttributesPattern(JS::HandleValue aOriginAttributesPattern, nsIClearDataCallback *aCallback) = 0;

  /* void deleteUserInteractionForClearingHistory (in Array<nsIPrincipal> aPrincipalsWithStorage, [optional] in PRTime aFrom, [optional] in nsIClearDataCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteUserInteractionForClearingHistory(const nsTArray<RefPtr<nsIPrincipal>>& aPrincipalsWithStorage, PRTime aFrom, nsIClearDataCallback *aCallback) = 0;

  enum {
    CLEAR_COOKIES = 1U,
    CLEAR_NETWORK_CACHE = 2U,
    CLEAR_IMAGE_CACHE = 4U,
    CLEAR_PLUGIN_DATA = 8U,
    CLEAR_DOWNLOADS = 16U,
    CLEAR_PASSWORDS = 32U,
    CLEAR_MEDIA_DEVICES = 64U,
    CLEAR_APPCACHE = 128U,
    CLEAR_DOM_QUOTA = 256U,
    CLEAR_PREDICTOR_NETWORK_DATA = 512U,
    CLEAR_DOM_PUSH_NOTIFICATIONS = 1024U,
    CLEAR_HISTORY = 2048U,
    CLEAR_SESSION_HISTORY = 4096U,
    CLEAR_AUTH_TOKENS = 8192U,
    CLEAR_AUTH_CACHE = 16384U,
    CLEAR_PERMISSIONS = 32768U,
    CLEAR_CONTENT_PREFERENCES = 65536U,
    CLEAR_SECURITY_SETTINGS = 131072U,
    CLEAR_EME = 262144U,
    CLEAR_REPORTS = 524288U,
    CLEAR_STORAGE_ACCESS = 1048576U,
    CLEAR_CERT_EXCEPTIONS = 2097152U,
    CLEAR_CONTENT_BLOCKING_RECORDS = 4194304U,
    CLEAR_CSS_CACHE = 8388608U,
    CLEAR_ALL = 16777215U,
    CLEAR_ALL_CACHES = 8388614U,
    CLEAR_DOM_STORAGES = 525696U,
    CLEAR_FORGET_ABOUT_SITE = 11509695U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIClearDataService, NS_ICLEARDATASERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICLEARDATASERVICE \
  NS_IMETHOD DeleteDataFromLocalFiles(bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) override; \
  NS_IMETHOD DeleteDataFromHost(const nsACString& aHost, bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) override; \
  NS_IMETHOD DeleteDataFromPrincipal(nsIPrincipal *aPrincipal, bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) override; \
  NS_IMETHOD DeleteDataInTimeRange(PRTime aFrom, PRTime aTo, bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) override; \
  NS_IMETHOD DeleteData(uint32_t aFlags, nsIClearDataCallback *aCallback) override; \
  NS_IMETHOD DeleteDataFromOriginAttributesPattern(JS::HandleValue aOriginAttributesPattern, nsIClearDataCallback *aCallback) override; \
  NS_IMETHOD DeleteUserInteractionForClearingHistory(const nsTArray<RefPtr<nsIPrincipal>>& aPrincipalsWithStorage, PRTime aFrom, nsIClearDataCallback *aCallback) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICLEARDATASERVICE \
  nsresult DeleteDataFromLocalFiles(bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback); \
  nsresult DeleteDataFromHost(const nsACString& aHost, bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback); \
  nsresult DeleteDataFromPrincipal(nsIPrincipal *aPrincipal, bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback); \
  nsresult DeleteDataInTimeRange(PRTime aFrom, PRTime aTo, bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback); \
  nsresult DeleteData(uint32_t aFlags, nsIClearDataCallback *aCallback); \
  nsresult DeleteDataFromOriginAttributesPattern(JS::HandleValue aOriginAttributesPattern, nsIClearDataCallback *aCallback); \
  nsresult DeleteUserInteractionForClearingHistory(const nsTArray<RefPtr<nsIPrincipal>>& aPrincipalsWithStorage, PRTime aFrom, nsIClearDataCallback *aCallback); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICLEARDATASERVICE(_to) \
  NS_IMETHOD DeleteDataFromLocalFiles(bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) override { return _to DeleteDataFromLocalFiles(aIsUserRequest, aFlags, aCallback); } \
  NS_IMETHOD DeleteDataFromHost(const nsACString& aHost, bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) override { return _to DeleteDataFromHost(aHost, aIsUserRequest, aFlags, aCallback); } \
  NS_IMETHOD DeleteDataFromPrincipal(nsIPrincipal *aPrincipal, bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) override { return _to DeleteDataFromPrincipal(aPrincipal, aIsUserRequest, aFlags, aCallback); } \
  NS_IMETHOD DeleteDataInTimeRange(PRTime aFrom, PRTime aTo, bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) override { return _to DeleteDataInTimeRange(aFrom, aTo, aIsUserRequest, aFlags, aCallback); } \
  NS_IMETHOD DeleteData(uint32_t aFlags, nsIClearDataCallback *aCallback) override { return _to DeleteData(aFlags, aCallback); } \
  NS_IMETHOD DeleteDataFromOriginAttributesPattern(JS::HandleValue aOriginAttributesPattern, nsIClearDataCallback *aCallback) override { return _to DeleteDataFromOriginAttributesPattern(aOriginAttributesPattern, aCallback); } \
  NS_IMETHOD DeleteUserInteractionForClearingHistory(const nsTArray<RefPtr<nsIPrincipal>>& aPrincipalsWithStorage, PRTime aFrom, nsIClearDataCallback *aCallback) override { return _to DeleteUserInteractionForClearingHistory(aPrincipalsWithStorage, aFrom, aCallback); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICLEARDATASERVICE(_to) \
  NS_IMETHOD DeleteDataFromLocalFiles(bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteDataFromLocalFiles(aIsUserRequest, aFlags, aCallback); } \
  NS_IMETHOD DeleteDataFromHost(const nsACString& aHost, bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteDataFromHost(aHost, aIsUserRequest, aFlags, aCallback); } \
  NS_IMETHOD DeleteDataFromPrincipal(nsIPrincipal *aPrincipal, bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteDataFromPrincipal(aPrincipal, aIsUserRequest, aFlags, aCallback); } \
  NS_IMETHOD DeleteDataInTimeRange(PRTime aFrom, PRTime aTo, bool aIsUserRequest, uint32_t aFlags, nsIClearDataCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteDataInTimeRange(aFrom, aTo, aIsUserRequest, aFlags, aCallback); } \
  NS_IMETHOD DeleteData(uint32_t aFlags, nsIClearDataCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteData(aFlags, aCallback); } \
  NS_IMETHOD DeleteDataFromOriginAttributesPattern(JS::HandleValue aOriginAttributesPattern, nsIClearDataCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteDataFromOriginAttributesPattern(aOriginAttributesPattern, aCallback); } \
  NS_IMETHOD DeleteUserInteractionForClearingHistory(const nsTArray<RefPtr<nsIPrincipal>>& aPrincipalsWithStorage, PRTime aFrom, nsIClearDataCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteUserInteractionForClearingHistory(aPrincipalsWithStorage, aFrom, aCallback); } \


/* starting interface:    nsIClearDataCallback */
#define NS_ICLEARDATACALLBACK_IID_STR "e225517b-24c5-498a-b9fb-9993e341a398"

#define NS_ICLEARDATACALLBACK_IID \
  {0xe225517b, 0x24c5, 0x498a, \
    { 0xb9, 0xfb, 0x99, 0x93, 0xe3, 0x41, 0xa3, 0x98 }}

class NS_NO_VTABLE nsIClearDataCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICLEARDATACALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIClearDataCallback;

  /* void onDataDeleted (in uint32_t aFailedFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnDataDeleted(uint32_t aFailedFlags) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIClearDataCallback, NS_ICLEARDATACALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICLEARDATACALLBACK \
  NS_IMETHOD OnDataDeleted(uint32_t aFailedFlags) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICLEARDATACALLBACK \
  nsresult OnDataDeleted(uint32_t aFailedFlags); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICLEARDATACALLBACK(_to) \
  NS_IMETHOD OnDataDeleted(uint32_t aFailedFlags) override { return _to OnDataDeleted(aFailedFlags); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICLEARDATACALLBACK(_to) \
  NS_IMETHOD OnDataDeleted(uint32_t aFailedFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnDataDeleted(aFailedFlags); } 


#endif /* __gen_nsIClearDataService_h__ */
