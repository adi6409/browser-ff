/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cookie/nsICookieJarSettings.idl
 */

#ifndef __gen_nsICookieJarSettings_h__
#define __gen_nsICookieJarSettings_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsISerializable_h__
#include "nsISerializable.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */


/* starting interface:    nsICookieJarSettings */
#define NS_ICOOKIEJARSETTINGS_IID_STR "3ec40331-7cf0-4b71-ba2a-2265aab8f6bc"

#define NS_ICOOKIEJARSETTINGS_IID \
  {0x3ec40331, 0x7cf0, 0x4b71, \
    { 0xba, 0x2a, 0x22, 0x65, 0xaa, 0xb8, 0xf6, 0xbc }}

class NS_NO_VTABLE nsICookieJarSettings : public nsISerializable {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOOKIEJARSETTINGS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICookieJarSettings;

  /* [infallible] readonly attribute unsigned long cookieBehavior; */
  NS_IMETHOD GetCookieBehavior(uint32_t *aCookieBehavior) = 0;
  inline uint32_t  GetCookieBehavior()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetCookieBehavior(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean isFirstPartyIsolated; */
  NS_IMETHOD GetIsFirstPartyIsolated(bool *aIsFirstPartyIsolated) = 0;
  inline bool  GetIsFirstPartyIsolated()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsFirstPartyIsolated(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean rejectThirdPartyContexts; */
  NS_IMETHOD GetRejectThirdPartyContexts(bool *aRejectThirdPartyContexts) = 0;
  inline bool  GetRejectThirdPartyContexts()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetRejectThirdPartyContexts(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean limitForeignContexts; */
  NS_IMETHOD GetLimitForeignContexts(bool *aLimitForeignContexts) = 0;
  inline bool  GetLimitForeignContexts()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetLimitForeignContexts(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] attribute boolean partitionForeign; */
  NS_IMETHOD GetPartitionForeign(bool *aPartitionForeign) = 0;
  inline bool  GetPartitionForeign()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetPartitionForeign(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetPartitionForeign(bool aPartitionForeign) = 0;

  /* [infallible] readonly attribute boolean isOnContentBlockingAllowList; */
  NS_IMETHOD GetIsOnContentBlockingAllowList(bool *aIsOnContentBlockingAllowList) = 0;
  inline bool  GetIsOnContentBlockingAllowList()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsOnContentBlockingAllowList(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* readonly attribute AString partitionKey; */
  NS_IMETHOD GetPartitionKey(nsAString& aPartitionKey) = 0;

  /* unsigned long cookiePermission (in nsIPrincipal aPrincipal); */
  NS_IMETHOD CookiePermission(nsIPrincipal *aPrincipal, uint32_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICookieJarSettings, NS_ICOOKIEJARSETTINGS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOOKIEJARSETTINGS \
  using nsICookieJarSettings::GetCookieBehavior; \
  NS_IMETHOD GetCookieBehavior(uint32_t *aCookieBehavior) override; \
  using nsICookieJarSettings::GetIsFirstPartyIsolated; \
  NS_IMETHOD GetIsFirstPartyIsolated(bool *aIsFirstPartyIsolated) override; \
  using nsICookieJarSettings::GetRejectThirdPartyContexts; \
  NS_IMETHOD GetRejectThirdPartyContexts(bool *aRejectThirdPartyContexts) override; \
  using nsICookieJarSettings::GetLimitForeignContexts; \
  NS_IMETHOD GetLimitForeignContexts(bool *aLimitForeignContexts) override; \
  using nsICookieJarSettings::GetPartitionForeign; \
  NS_IMETHOD GetPartitionForeign(bool *aPartitionForeign) override; \
  NS_IMETHOD SetPartitionForeign(bool aPartitionForeign) override; \
  using nsICookieJarSettings::GetIsOnContentBlockingAllowList; \
  NS_IMETHOD GetIsOnContentBlockingAllowList(bool *aIsOnContentBlockingAllowList) override; \
  NS_IMETHOD GetPartitionKey(nsAString& aPartitionKey) override; \
  NS_IMETHOD CookiePermission(nsIPrincipal *aPrincipal, uint32_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOOKIEJARSETTINGS \
  using nsICookieJarSettings::GetCookieBehavior; \
  nsresult GetCookieBehavior(uint32_t *aCookieBehavior); \
  using nsICookieJarSettings::GetIsFirstPartyIsolated; \
  nsresult GetIsFirstPartyIsolated(bool *aIsFirstPartyIsolated); \
  using nsICookieJarSettings::GetRejectThirdPartyContexts; \
  nsresult GetRejectThirdPartyContexts(bool *aRejectThirdPartyContexts); \
  using nsICookieJarSettings::GetLimitForeignContexts; \
  nsresult GetLimitForeignContexts(bool *aLimitForeignContexts); \
  using nsICookieJarSettings::GetPartitionForeign; \
  nsresult GetPartitionForeign(bool *aPartitionForeign); \
  nsresult SetPartitionForeign(bool aPartitionForeign); \
  using nsICookieJarSettings::GetIsOnContentBlockingAllowList; \
  nsresult GetIsOnContentBlockingAllowList(bool *aIsOnContentBlockingAllowList); \
  nsresult GetPartitionKey(nsAString& aPartitionKey); \
  nsresult CookiePermission(nsIPrincipal *aPrincipal, uint32_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOOKIEJARSETTINGS(_to) \
  using nsICookieJarSettings::GetCookieBehavior; \
  NS_IMETHOD GetCookieBehavior(uint32_t *aCookieBehavior) override { return _to GetCookieBehavior(aCookieBehavior); } \
  using nsICookieJarSettings::GetIsFirstPartyIsolated; \
  NS_IMETHOD GetIsFirstPartyIsolated(bool *aIsFirstPartyIsolated) override { return _to GetIsFirstPartyIsolated(aIsFirstPartyIsolated); } \
  using nsICookieJarSettings::GetRejectThirdPartyContexts; \
  NS_IMETHOD GetRejectThirdPartyContexts(bool *aRejectThirdPartyContexts) override { return _to GetRejectThirdPartyContexts(aRejectThirdPartyContexts); } \
  using nsICookieJarSettings::GetLimitForeignContexts; \
  NS_IMETHOD GetLimitForeignContexts(bool *aLimitForeignContexts) override { return _to GetLimitForeignContexts(aLimitForeignContexts); } \
  using nsICookieJarSettings::GetPartitionForeign; \
  NS_IMETHOD GetPartitionForeign(bool *aPartitionForeign) override { return _to GetPartitionForeign(aPartitionForeign); } \
  NS_IMETHOD SetPartitionForeign(bool aPartitionForeign) override { return _to SetPartitionForeign(aPartitionForeign); } \
  using nsICookieJarSettings::GetIsOnContentBlockingAllowList; \
  NS_IMETHOD GetIsOnContentBlockingAllowList(bool *aIsOnContentBlockingAllowList) override { return _to GetIsOnContentBlockingAllowList(aIsOnContentBlockingAllowList); } \
  NS_IMETHOD GetPartitionKey(nsAString& aPartitionKey) override { return _to GetPartitionKey(aPartitionKey); } \
  NS_IMETHOD CookiePermission(nsIPrincipal *aPrincipal, uint32_t *_retval) override { return _to CookiePermission(aPrincipal, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOOKIEJARSETTINGS(_to) \
  NS_IMETHOD GetCookieBehavior(uint32_t *aCookieBehavior) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCookieBehavior(aCookieBehavior); } \
  NS_IMETHOD GetIsFirstPartyIsolated(bool *aIsFirstPartyIsolated) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsFirstPartyIsolated(aIsFirstPartyIsolated); } \
  NS_IMETHOD GetRejectThirdPartyContexts(bool *aRejectThirdPartyContexts) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRejectThirdPartyContexts(aRejectThirdPartyContexts); } \
  NS_IMETHOD GetLimitForeignContexts(bool *aLimitForeignContexts) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLimitForeignContexts(aLimitForeignContexts); } \
  NS_IMETHOD GetPartitionForeign(bool *aPartitionForeign) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPartitionForeign(aPartitionForeign); } \
  NS_IMETHOD SetPartitionForeign(bool aPartitionForeign) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPartitionForeign(aPartitionForeign); } \
  NS_IMETHOD GetIsOnContentBlockingAllowList(bool *aIsOnContentBlockingAllowList) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsOnContentBlockingAllowList(aIsOnContentBlockingAllowList); } \
  NS_IMETHOD GetPartitionKey(nsAString& aPartitionKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPartitionKey(aPartitionKey); } \
  NS_IMETHOD CookiePermission(nsIPrincipal *aPrincipal, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CookiePermission(aPrincipal, _retval); } 


#endif /* __gen_nsICookieJarSettings_h__ */
