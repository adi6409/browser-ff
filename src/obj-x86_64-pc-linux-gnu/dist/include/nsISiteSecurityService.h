/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsISiteSecurityService.idl
 */

#ifndef __gen_nsISiteSecurityService_h__
#define __gen_nsISiteSecurityService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIObserver; /* forward declaration */

class nsIHttpChannel; /* forward declaration */

class nsITransportSecurityInfo; /* forward declaration */

class nsISimpleEnumerator; /* forward declaration */

#include "nsStringFwd.h"
#include "nsTArrayForwardDeclare.h"
namespace mozilla
{
  namespace pkix
  {
    class Time;
  }
}

/* starting interface:    nsISiteSecurityState */
#define NS_ISITESECURITYSTATE_IID_STR "31313372-842c-4110-bdf1-6aea17c845ad"

#define NS_ISITESECURITYSTATE_IID \
  {0x31313372, 0x842c, 0x4110, \
    { 0xbd, 0xf1, 0x6a, 0xea, 0x17, 0xc8, 0x45, 0xad }}

class NS_NO_VTABLE nsISiteSecurityState : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISITESECURITYSTATE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISiteSecurityState;

  /* [must_use] readonly attribute ACString hostname; */
  [[nodiscard]] NS_IMETHOD GetHostname(nsACString& aHostname) = 0;

  /* [infallible] readonly attribute long long expireTime; */
  NS_IMETHOD GetExpireTime(int64_t *aExpireTime) = 0;
  inline int64_t  GetExpireTime()
  {
    int64_t result;
    mozilla::DebugOnly<nsresult> rv = GetExpireTime(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute short securityPropertyState; */
  NS_IMETHOD GetSecurityPropertyState(int16_t *aSecurityPropertyState) = 0;
  inline int16_t  GetSecurityPropertyState()
  {
    int16_t result;
    mozilla::DebugOnly<nsresult> rv = GetSecurityPropertyState(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean includeSubdomains; */
  NS_IMETHOD GetIncludeSubdomains(bool *aIncludeSubdomains) = 0;
  inline bool  GetIncludeSubdomains()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIncludeSubdomains(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [implicit_jscontext,must_use] readonly attribute jsval originAttributes; */
  [[nodiscard]] NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) = 0;

  enum {
    SECURITY_PROPERTY_UNSET = 0,
    SECURITY_PROPERTY_SET = 1,
    SECURITY_PROPERTY_KNOCKOUT = 2,
    SECURITY_PROPERTY_NEGATIVE = 3
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISiteSecurityState, NS_ISITESECURITYSTATE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISITESECURITYSTATE \
  [[nodiscard]] NS_IMETHOD GetHostname(nsACString& aHostname) override; \
  using nsISiteSecurityState::GetExpireTime; \
  NS_IMETHOD GetExpireTime(int64_t *aExpireTime) override; \
  using nsISiteSecurityState::GetSecurityPropertyState; \
  NS_IMETHOD GetSecurityPropertyState(int16_t *aSecurityPropertyState) override; \
  using nsISiteSecurityState::GetIncludeSubdomains; \
  NS_IMETHOD GetIncludeSubdomains(bool *aIncludeSubdomains) override; \
  [[nodiscard]] NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISITESECURITYSTATE \
  [[nodiscard]] nsresult GetHostname(nsACString& aHostname); \
  using nsISiteSecurityState::GetExpireTime; \
  nsresult GetExpireTime(int64_t *aExpireTime); \
  using nsISiteSecurityState::GetSecurityPropertyState; \
  nsresult GetSecurityPropertyState(int16_t *aSecurityPropertyState); \
  using nsISiteSecurityState::GetIncludeSubdomains; \
  nsresult GetIncludeSubdomains(bool *aIncludeSubdomains); \
  [[nodiscard]] nsresult GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISITESECURITYSTATE(_to) \
  [[nodiscard]] NS_IMETHOD GetHostname(nsACString& aHostname) override { return _to GetHostname(aHostname); } \
  using nsISiteSecurityState::GetExpireTime; \
  NS_IMETHOD GetExpireTime(int64_t *aExpireTime) override { return _to GetExpireTime(aExpireTime); } \
  using nsISiteSecurityState::GetSecurityPropertyState; \
  NS_IMETHOD GetSecurityPropertyState(int16_t *aSecurityPropertyState) override { return _to GetSecurityPropertyState(aSecurityPropertyState); } \
  using nsISiteSecurityState::GetIncludeSubdomains; \
  NS_IMETHOD GetIncludeSubdomains(bool *aIncludeSubdomains) override { return _to GetIncludeSubdomains(aIncludeSubdomains); } \
  [[nodiscard]] NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return _to GetOriginAttributes(cx, aOriginAttributes); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISITESECURITYSTATE(_to) \
  [[nodiscard]] NS_IMETHOD GetHostname(nsACString& aHostname) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHostname(aHostname); } \
  NS_IMETHOD GetExpireTime(int64_t *aExpireTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExpireTime(aExpireTime); } \
  NS_IMETHOD GetSecurityPropertyState(int16_t *aSecurityPropertyState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecurityPropertyState(aSecurityPropertyState); } \
  NS_IMETHOD GetIncludeSubdomains(bool *aIncludeSubdomains) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIncludeSubdomains(aIncludeSubdomains); } \
  [[nodiscard]] NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginAttributes(cx, aOriginAttributes); } \


/* starting interface:    nsISiteHSTSState */
#define NS_ISITEHSTSSTATE_IID_STR "9ff16e40-1029-496c-95c2-bc819872b216"

#define NS_ISITEHSTSSTATE_IID \
  {0x9ff16e40, 0x1029, 0x496c, \
    { 0x95, 0xc2, 0xbc, 0x81, 0x98, 0x72, 0xb2, 0x16 }}

class NS_NO_VTABLE nsISiteHSTSState : public nsISiteSecurityState {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISITEHSTSSTATE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISiteHSTSState;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISiteHSTSState, NS_ISITEHSTSSTATE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISITEHSTSSTATE \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISITEHSTSSTATE \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISITEHSTSSTATE(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISITEHSTSSTATE(_to) \
  /* no methods! */


/* starting interface:    nsISiteSecurityService */
#define NS_ISITESECURITYSERVICE_IID_STR "275127f8-dbd7-4681-afbf-6df0c6587a01"

#define NS_ISITESECURITYSERVICE_IID \
  {0x275127f8, 0xdbd7, 0x4681, \
    { 0xaf, 0xbf, 0x6d, 0xf0, 0xc6, 0x58, 0x7a, 0x01 }}

class NS_NO_VTABLE nsISiteSecurityService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISITESECURITYSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISiteSecurityService;

  enum {
    HEADER_HSTS = 0U,
    STATIC_PINNING = 1U,
    Success = 0U,
    ERROR_UNKNOWN = 1U,
    ERROR_UNTRUSTWORTHY_CONNECTION = 2U,
    ERROR_COULD_NOT_PARSE_HEADER = 3U,
    ERROR_NO_MAX_AGE = 4U,
    ERROR_MULTIPLE_MAX_AGES = 5U,
    ERROR_INVALID_MAX_AGE = 6U,
    ERROR_MULTIPLE_INCLUDE_SUBDOMAINS = 7U,
    ERROR_INVALID_INCLUDE_SUBDOMAINS = 8U,
    ERROR_COULD_NOT_SAVE_STATE = 13U,
    SOURCE_UNKNOWN = 0U,
    SOURCE_PRELOAD_LIST = 1U,
    SOURCE_ORGANIC_REQUEST = 2U
  };

  /* [binaryname(ProcessHeader),must_use,noscript] void processHeaderNative (in uint32_t aType, in nsIURI aSourceURI, in ACString aHeader, in nsITransportSecurityInfo aSecInfo, in uint32_t aFlags, in uint32_t aSource, in const_OriginAttributesRef aOriginAttributes, [optional] out unsigned long long aMaxAge, [optional] out boolean aIncludeSubdomains, [optional] out uint32_t aFailureResult); */
  [[nodiscard]] NS_IMETHOD ProcessHeader(uint32_t aType, nsIURI *aSourceURI, const nsACString& aHeader, nsITransportSecurityInfo *aSecInfo, uint32_t aFlags, uint32_t aSource, const mozilla::OriginAttributes & aOriginAttributes, uint64_t *aMaxAge = nullptr, bool *aIncludeSubdomains = nullptr, uint32_t *aFailureResult = nullptr) = 0;

  /* [binaryname(ProcessHeaderScriptable),implicit_jscontext,must_use,optional_argc] void processHeader (in uint32_t aType, in nsIURI aSourceURI, in ACString aHeader, in nsITransportSecurityInfo aSecInfo, in uint32_t aFlags, in uint32_t aSource, [optional] in jsval aOriginAttributes, [optional] out unsigned long long aMaxAge, [optional] out boolean aIncludeSubdomains, [optional] out uint32_t aFailureResult); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ProcessHeaderScriptable(uint32_t aType, nsIURI *aSourceURI, const nsACString& aHeader, nsITransportSecurityInfo *aSecInfo, uint32_t aFlags, uint32_t aSource, JS::HandleValue aOriginAttributes, uint64_t *aMaxAge, bool *aIncludeSubdomains, uint32_t *aFailureResult, JSContext* cx, uint8_t _argc) = 0;

  /* [implicit_jscontext,must_use,optional_argc] void resetState (in uint32_t aType, in nsIURI aURI, in uint32_t aFlags, [optional] in jsval aOriginAttributes); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ResetState(uint32_t aType, nsIURI *aURI, uint32_t aFlags, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc) = 0;

  /* [binaryname(IsSecureURI),must_use,noscript] boolean isSecureURINative (in uint32_t aType, in nsIURI aURI, in uint32_t aFlags, in const_OriginAttributesRef aOriginAttributes, [optional] out boolean aCached, [optional] out uint32_t aSource); */
  [[nodiscard]] NS_IMETHOD IsSecureURI(uint32_t aType, nsIURI *aURI, uint32_t aFlags, const mozilla::OriginAttributes & aOriginAttributes, bool *aCached, uint32_t *aSource, bool *_retval) = 0;

  /* [binaryname(IsSecureURIScriptable),implicit_jscontext,must_use,optional_argc] boolean isSecureURI (in uint32_t aType, in nsIURI aURI, in uint32_t aFlags, [optional] in jsval aOriginAttributes, [optional] out boolean aCached, [optional] out uint32_t aSource); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD IsSecureURIScriptable(uint32_t aType, nsIURI *aURI, uint32_t aFlags, JS::HandleValue aOriginAttributes, bool *aCached, uint32_t *aSource, JSContext* cx, uint8_t _argc, bool *_retval) = 0;

  /* [must_use] void clearAll (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ClearAll(void) = 0;

  /* [must_use] void clearPreloads (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ClearPreloads(void) = 0;

  /* [must_use] boolean setHSTSPreload (in ACString aHost, in boolean aIncludesSubdomains, in int64_t aExpires); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD SetHSTSPreload(const nsACString& aHost, bool aIncludesSubdomains, int64_t aExpires, bool *_retval) = 0;

  /* [must_use] nsISimpleEnumerator enumerate (in uint32_t aType); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD Enumerate(uint32_t aType, nsISimpleEnumerator **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISiteSecurityService, NS_ISITESECURITYSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISITESECURITYSERVICE \
  [[nodiscard]] NS_IMETHOD ProcessHeader(uint32_t aType, nsIURI *aSourceURI, const nsACString& aHeader, nsITransportSecurityInfo *aSecInfo, uint32_t aFlags, uint32_t aSource, const mozilla::OriginAttributes & aOriginAttributes, uint64_t *aMaxAge = nullptr, bool *aIncludeSubdomains = nullptr, uint32_t *aFailureResult = nullptr) override; \
  [[nodiscard]] NS_IMETHOD ProcessHeaderScriptable(uint32_t aType, nsIURI *aSourceURI, const nsACString& aHeader, nsITransportSecurityInfo *aSecInfo, uint32_t aFlags, uint32_t aSource, JS::HandleValue aOriginAttributes, uint64_t *aMaxAge, bool *aIncludeSubdomains, uint32_t *aFailureResult, JSContext* cx, uint8_t _argc) override; \
  [[nodiscard]] NS_IMETHOD ResetState(uint32_t aType, nsIURI *aURI, uint32_t aFlags, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc) override; \
  [[nodiscard]] NS_IMETHOD IsSecureURI(uint32_t aType, nsIURI *aURI, uint32_t aFlags, const mozilla::OriginAttributes & aOriginAttributes, bool *aCached, uint32_t *aSource, bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD IsSecureURIScriptable(uint32_t aType, nsIURI *aURI, uint32_t aFlags, JS::HandleValue aOriginAttributes, bool *aCached, uint32_t *aSource, JSContext* cx, uint8_t _argc, bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD ClearAll(void) override; \
  [[nodiscard]] NS_IMETHOD ClearPreloads(void) override; \
  [[nodiscard]] NS_IMETHOD SetHSTSPreload(const nsACString& aHost, bool aIncludesSubdomains, int64_t aExpires, bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD Enumerate(uint32_t aType, nsISimpleEnumerator **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISITESECURITYSERVICE \
  [[nodiscard]] nsresult ProcessHeader(uint32_t aType, nsIURI *aSourceURI, const nsACString& aHeader, nsITransportSecurityInfo *aSecInfo, uint32_t aFlags, uint32_t aSource, const mozilla::OriginAttributes & aOriginAttributes, uint64_t *aMaxAge = nullptr, bool *aIncludeSubdomains = nullptr, uint32_t *aFailureResult = nullptr); \
  [[nodiscard]] nsresult ProcessHeaderScriptable(uint32_t aType, nsIURI *aSourceURI, const nsACString& aHeader, nsITransportSecurityInfo *aSecInfo, uint32_t aFlags, uint32_t aSource, JS::HandleValue aOriginAttributes, uint64_t *aMaxAge, bool *aIncludeSubdomains, uint32_t *aFailureResult, JSContext* cx, uint8_t _argc); \
  [[nodiscard]] nsresult ResetState(uint32_t aType, nsIURI *aURI, uint32_t aFlags, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc); \
  [[nodiscard]] nsresult IsSecureURI(uint32_t aType, nsIURI *aURI, uint32_t aFlags, const mozilla::OriginAttributes & aOriginAttributes, bool *aCached, uint32_t *aSource, bool *_retval); \
  [[nodiscard]] nsresult IsSecureURIScriptable(uint32_t aType, nsIURI *aURI, uint32_t aFlags, JS::HandleValue aOriginAttributes, bool *aCached, uint32_t *aSource, JSContext* cx, uint8_t _argc, bool *_retval); \
  [[nodiscard]] nsresult ClearAll(void); \
  [[nodiscard]] nsresult ClearPreloads(void); \
  [[nodiscard]] nsresult SetHSTSPreload(const nsACString& aHost, bool aIncludesSubdomains, int64_t aExpires, bool *_retval); \
  [[nodiscard]] nsresult Enumerate(uint32_t aType, nsISimpleEnumerator **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISITESECURITYSERVICE(_to) \
  [[nodiscard]] NS_IMETHOD ProcessHeader(uint32_t aType, nsIURI *aSourceURI, const nsACString& aHeader, nsITransportSecurityInfo *aSecInfo, uint32_t aFlags, uint32_t aSource, const mozilla::OriginAttributes & aOriginAttributes, uint64_t *aMaxAge = nullptr, bool *aIncludeSubdomains = nullptr, uint32_t *aFailureResult = nullptr) override { return _to ProcessHeader(aType, aSourceURI, aHeader, aSecInfo, aFlags, aSource, aOriginAttributes, aMaxAge, aIncludeSubdomains, aFailureResult); } \
  [[nodiscard]] NS_IMETHOD ProcessHeaderScriptable(uint32_t aType, nsIURI *aSourceURI, const nsACString& aHeader, nsITransportSecurityInfo *aSecInfo, uint32_t aFlags, uint32_t aSource, JS::HandleValue aOriginAttributes, uint64_t *aMaxAge, bool *aIncludeSubdomains, uint32_t *aFailureResult, JSContext* cx, uint8_t _argc) override { return _to ProcessHeaderScriptable(aType, aSourceURI, aHeader, aSecInfo, aFlags, aSource, aOriginAttributes, aMaxAge, aIncludeSubdomains, aFailureResult, cx, _argc); } \
  [[nodiscard]] NS_IMETHOD ResetState(uint32_t aType, nsIURI *aURI, uint32_t aFlags, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc) override { return _to ResetState(aType, aURI, aFlags, aOriginAttributes, cx, _argc); } \
  [[nodiscard]] NS_IMETHOD IsSecureURI(uint32_t aType, nsIURI *aURI, uint32_t aFlags, const mozilla::OriginAttributes & aOriginAttributes, bool *aCached, uint32_t *aSource, bool *_retval) override { return _to IsSecureURI(aType, aURI, aFlags, aOriginAttributes, aCached, aSource, _retval); } \
  [[nodiscard]] NS_IMETHOD IsSecureURIScriptable(uint32_t aType, nsIURI *aURI, uint32_t aFlags, JS::HandleValue aOriginAttributes, bool *aCached, uint32_t *aSource, JSContext* cx, uint8_t _argc, bool *_retval) override { return _to IsSecureURIScriptable(aType, aURI, aFlags, aOriginAttributes, aCached, aSource, cx, _argc, _retval); } \
  [[nodiscard]] NS_IMETHOD ClearAll(void) override { return _to ClearAll(); } \
  [[nodiscard]] NS_IMETHOD ClearPreloads(void) override { return _to ClearPreloads(); } \
  [[nodiscard]] NS_IMETHOD SetHSTSPreload(const nsACString& aHost, bool aIncludesSubdomains, int64_t aExpires, bool *_retval) override { return _to SetHSTSPreload(aHost, aIncludesSubdomains, aExpires, _retval); } \
  [[nodiscard]] NS_IMETHOD Enumerate(uint32_t aType, nsISimpleEnumerator **_retval) override { return _to Enumerate(aType, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISITESECURITYSERVICE(_to) \
  [[nodiscard]] NS_IMETHOD ProcessHeader(uint32_t aType, nsIURI *aSourceURI, const nsACString& aHeader, nsITransportSecurityInfo *aSecInfo, uint32_t aFlags, uint32_t aSource, const mozilla::OriginAttributes & aOriginAttributes, uint64_t *aMaxAge = nullptr, bool *aIncludeSubdomains = nullptr, uint32_t *aFailureResult = nullptr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ProcessHeader(aType, aSourceURI, aHeader, aSecInfo, aFlags, aSource, aOriginAttributes, aMaxAge, aIncludeSubdomains, aFailureResult); } \
  [[nodiscard]] NS_IMETHOD ProcessHeaderScriptable(uint32_t aType, nsIURI *aSourceURI, const nsACString& aHeader, nsITransportSecurityInfo *aSecInfo, uint32_t aFlags, uint32_t aSource, JS::HandleValue aOriginAttributes, uint64_t *aMaxAge, bool *aIncludeSubdomains, uint32_t *aFailureResult, JSContext* cx, uint8_t _argc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ProcessHeaderScriptable(aType, aSourceURI, aHeader, aSecInfo, aFlags, aSource, aOriginAttributes, aMaxAge, aIncludeSubdomains, aFailureResult, cx, _argc); } \
  [[nodiscard]] NS_IMETHOD ResetState(uint32_t aType, nsIURI *aURI, uint32_t aFlags, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetState(aType, aURI, aFlags, aOriginAttributes, cx, _argc); } \
  [[nodiscard]] NS_IMETHOD IsSecureURI(uint32_t aType, nsIURI *aURI, uint32_t aFlags, const mozilla::OriginAttributes & aOriginAttributes, bool *aCached, uint32_t *aSource, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsSecureURI(aType, aURI, aFlags, aOriginAttributes, aCached, aSource, _retval); } \
  [[nodiscard]] NS_IMETHOD IsSecureURIScriptable(uint32_t aType, nsIURI *aURI, uint32_t aFlags, JS::HandleValue aOriginAttributes, bool *aCached, uint32_t *aSource, JSContext* cx, uint8_t _argc, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsSecureURIScriptable(aType, aURI, aFlags, aOriginAttributes, aCached, aSource, cx, _argc, _retval); } \
  [[nodiscard]] NS_IMETHOD ClearAll(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearAll(); } \
  [[nodiscard]] NS_IMETHOD ClearPreloads(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearPreloads(); } \
  [[nodiscard]] NS_IMETHOD SetHSTSPreload(const nsACString& aHost, bool aIncludesSubdomains, int64_t aExpires, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHSTSPreload(aHost, aIncludesSubdomains, aExpires, _retval); } \
  [[nodiscard]] NS_IMETHOD Enumerate(uint32_t aType, nsISimpleEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Enumerate(aType, _retval); } 

#define NS_SSSERVICE_CONTRACTID "@mozilla.org/ssservice;1"

#endif /* __gen_nsISiteSecurityService_h__ */
