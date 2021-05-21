/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cookie/nsICookieManager.idl
 */

#ifndef __gen_nsICookieManager_h__
#define __gen_nsICookieManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsICookie_h__
#include "nsICookie.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
class OriginAttributes;
} // mozilla namespace

/* starting interface:    nsICookieManager */
#define NS_ICOOKIEMANAGER_IID_STR "aaab6710-0f2c-11d5-a53b-0010a401eb10"

#define NS_ICOOKIEMANAGER_IID \
  {0xaaab6710, 0x0f2c, 0x11d5, \
    { 0xa5, 0x3b, 0x00, 0x10, 0xa4, 0x01, 0xeb, 0x10 }}

class nsICookieManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOOKIEMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICookieManager;

  /* void removeAll (); */
  NS_IMETHOD RemoveAll(void) = 0;

  /* readonly attribute Array<nsICookie> cookies; */
  NS_IMETHOD GetCookies(nsTArray<RefPtr<nsICookie>>& aCookies) = 0;

  /* readonly attribute Array<nsICookie> sessionCookies; */
  NS_IMETHOD GetSessionCookies(nsTArray<RefPtr<nsICookie>>& aSessionCookies) = 0;

  /* readonly attribute uint32_t cookieBehavior; */
  NS_IMETHOD GetCookieBehavior(uint32_t *aCookieBehavior) = 0;

     static uint32_t GetCookieBehavior();
    /* [implicit_jscontext] void remove (in AUTF8String aHost, in ACString aName, in AUTF8String aPath, in jsval aOriginAttributes); */
  NS_IMETHOD Remove(const nsACString& aHost, const nsACString& aName, const nsACString& aPath, JS::HandleValue aOriginAttributes, JSContext* cx) = 0;

  /* [notxpcom] nsresult removeNative (in AUTF8String aHost, in ACString aName, in AUTF8String aPath, in OriginAttributesPtr aOriginAttributes); */
  NS_IMETHOD RemoveNative(const nsACString& aHost, const nsACString& aName, const nsACString& aPath, mozilla::OriginAttributes * aOriginAttributes) = 0;

  /* [implicit_jscontext] void add (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in AUTF8String aValue, in boolean aIsSecure, in boolean aIsHttpOnly, in boolean aIsSession, in int64_t aExpiry, in jsval aOriginAttributes, in int32_t aSameSite, in nsICookie_schemeType aSchemeMap); */
  NS_IMETHOD Add(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, const nsACString& aValue, bool aIsSecure, bool aIsHttpOnly, bool aIsSession, int64_t aExpiry, JS::HandleValue aOriginAttributes, int32_t aSameSite, nsICookie::schemeType aSchemeMap, JSContext* cx) = 0;

  /* [notxpcom] nsresult addNative (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in AUTF8String aValue, in boolean aIsSecure, in boolean aIsHttpOnly, in boolean aIsSession, in int64_t aExpiry, in OriginAttributesPtr aOriginAttributes, in int32_t aSameSite, in nsICookie_schemeType aSchemeMap); */
  NS_IMETHOD AddNative(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, const nsACString& aValue, bool aIsSecure, bool aIsHttpOnly, bool aIsSession, int64_t aExpiry, mozilla::OriginAttributes * aOriginAttributes, int32_t aSameSite, nsICookie::schemeType aSchemeMap) = 0;

  /* [implicit_jscontext] boolean cookieExists (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in jsval aOriginAttributes); */
  NS_IMETHOD CookieExists(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, JS::HandleValue aOriginAttributes, JSContext* cx, bool *_retval) = 0;

  /* [notxpcom] nsresult cookieExistsNative (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in OriginAttributesPtr aOriginAttributes, out boolean aExists); */
  NS_IMETHOD CookieExistsNative(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, mozilla::OriginAttributes * aOriginAttributes, bool *aExists) = 0;

  /* unsigned long countCookiesFromHost (in AUTF8String aHost); */
  NS_IMETHOD CountCookiesFromHost(const nsACString& aHost, uint32_t *_retval) = 0;

  /* [implicit_jscontext] Array<nsICookie> getCookiesFromHost (in AUTF8String aHost, in jsval aOriginAttributes); */
  NS_IMETHOD GetCookiesFromHost(const nsACString& aHost, JS::HandleValue aOriginAttributes, JSContext* cx, nsTArray<RefPtr<nsICookie>>& _retval) = 0;

  /* Array<nsICookie> getCookiesWithOriginAttributes (in AString aPattern, [optional] in AUTF8String aHost); */
  NS_IMETHOD GetCookiesWithOriginAttributes(const nsAString& aPattern, const nsACString& aHost, nsTArray<RefPtr<nsICookie>>& _retval) = 0;

  /* void removeCookiesWithOriginAttributes (in AString aPattern, [optional] in AUTF8String aHost); */
  NS_IMETHOD RemoveCookiesWithOriginAttributes(const nsAString& aPattern, const nsACString& aHost) = 0;

  /* void removeCookiesFromExactHost (in AUTF8String aHost, in AString aPattern); */
  NS_IMETHOD RemoveCookiesFromExactHost(const nsACString& aHost, const nsAString& aPattern) = 0;

  /* [implicit_jscontext] Promise removeAllSince (in int64_t aSinceWhen); */
  NS_IMETHOD RemoveAllSince(int64_t aSinceWhen, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* Array<nsICookie> getCookiesSince (in int64_t aSinceWhen); */
  NS_IMETHOD GetCookiesSince(int64_t aSinceWhen, nsTArray<RefPtr<nsICookie>>& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICookieManager, NS_ICOOKIEMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOOKIEMANAGER \
  NS_IMETHOD RemoveAll(void) override; \
  NS_IMETHOD GetCookies(nsTArray<RefPtr<nsICookie>>& aCookies) override; \
  NS_IMETHOD GetSessionCookies(nsTArray<RefPtr<nsICookie>>& aSessionCookies) override; \
  NS_IMETHOD GetCookieBehavior(uint32_t *aCookieBehavior) override; \
  NS_IMETHOD Remove(const nsACString& aHost, const nsACString& aName, const nsACString& aPath, JS::HandleValue aOriginAttributes, JSContext* cx) override; \
  NS_IMETHOD RemoveNative(const nsACString& aHost, const nsACString& aName, const nsACString& aPath, mozilla::OriginAttributes * aOriginAttributes) override; \
  NS_IMETHOD Add(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, const nsACString& aValue, bool aIsSecure, bool aIsHttpOnly, bool aIsSession, int64_t aExpiry, JS::HandleValue aOriginAttributes, int32_t aSameSite, nsICookie::schemeType aSchemeMap, JSContext* cx) override; \
  NS_IMETHOD AddNative(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, const nsACString& aValue, bool aIsSecure, bool aIsHttpOnly, bool aIsSession, int64_t aExpiry, mozilla::OriginAttributes * aOriginAttributes, int32_t aSameSite, nsICookie::schemeType aSchemeMap) override; \
  NS_IMETHOD CookieExists(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, JS::HandleValue aOriginAttributes, JSContext* cx, bool *_retval) override; \
  NS_IMETHOD CookieExistsNative(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, mozilla::OriginAttributes * aOriginAttributes, bool *aExists) override; \
  NS_IMETHOD CountCookiesFromHost(const nsACString& aHost, uint32_t *_retval) override; \
  NS_IMETHOD GetCookiesFromHost(const nsACString& aHost, JS::HandleValue aOriginAttributes, JSContext* cx, nsTArray<RefPtr<nsICookie>>& _retval) override; \
  NS_IMETHOD GetCookiesWithOriginAttributes(const nsAString& aPattern, const nsACString& aHost, nsTArray<RefPtr<nsICookie>>& _retval) override; \
  NS_IMETHOD RemoveCookiesWithOriginAttributes(const nsAString& aPattern, const nsACString& aHost) override; \
  NS_IMETHOD RemoveCookiesFromExactHost(const nsACString& aHost, const nsAString& aPattern) override; \
  NS_IMETHOD RemoveAllSince(int64_t aSinceWhen, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetCookiesSince(int64_t aSinceWhen, nsTArray<RefPtr<nsICookie>>& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOOKIEMANAGER \
  nsresult RemoveAll(void); \
  nsresult GetCookies(nsTArray<RefPtr<nsICookie>>& aCookies); \
  nsresult GetSessionCookies(nsTArray<RefPtr<nsICookie>>& aSessionCookies); \
  nsresult GetCookieBehavior(uint32_t *aCookieBehavior); \
  nsresult Remove(const nsACString& aHost, const nsACString& aName, const nsACString& aPath, JS::HandleValue aOriginAttributes, JSContext* cx); \
  nsresult RemoveNative(const nsACString& aHost, const nsACString& aName, const nsACString& aPath, mozilla::OriginAttributes * aOriginAttributes); \
  nsresult Add(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, const nsACString& aValue, bool aIsSecure, bool aIsHttpOnly, bool aIsSession, int64_t aExpiry, JS::HandleValue aOriginAttributes, int32_t aSameSite, nsICookie::schemeType aSchemeMap, JSContext* cx); \
  nsresult AddNative(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, const nsACString& aValue, bool aIsSecure, bool aIsHttpOnly, bool aIsSession, int64_t aExpiry, mozilla::OriginAttributes * aOriginAttributes, int32_t aSameSite, nsICookie::schemeType aSchemeMap); \
  nsresult CookieExists(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, JS::HandleValue aOriginAttributes, JSContext* cx, bool *_retval); \
  nsresult CookieExistsNative(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, mozilla::OriginAttributes * aOriginAttributes, bool *aExists); \
  nsresult CountCookiesFromHost(const nsACString& aHost, uint32_t *_retval); \
  nsresult GetCookiesFromHost(const nsACString& aHost, JS::HandleValue aOriginAttributes, JSContext* cx, nsTArray<RefPtr<nsICookie>>& _retval); \
  nsresult GetCookiesWithOriginAttributes(const nsAString& aPattern, const nsACString& aHost, nsTArray<RefPtr<nsICookie>>& _retval); \
  nsresult RemoveCookiesWithOriginAttributes(const nsAString& aPattern, const nsACString& aHost); \
  nsresult RemoveCookiesFromExactHost(const nsACString& aHost, const nsAString& aPattern); \
  nsresult RemoveAllSince(int64_t aSinceWhen, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult GetCookiesSince(int64_t aSinceWhen, nsTArray<RefPtr<nsICookie>>& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOOKIEMANAGER(_to) \
  NS_IMETHOD RemoveAll(void) override { return _to RemoveAll(); } \
  NS_IMETHOD GetCookies(nsTArray<RefPtr<nsICookie>>& aCookies) override { return _to GetCookies(aCookies); } \
  NS_IMETHOD GetSessionCookies(nsTArray<RefPtr<nsICookie>>& aSessionCookies) override { return _to GetSessionCookies(aSessionCookies); } \
  NS_IMETHOD GetCookieBehavior(uint32_t *aCookieBehavior) override { return _to GetCookieBehavior(aCookieBehavior); } \
  NS_IMETHOD Remove(const nsACString& aHost, const nsACString& aName, const nsACString& aPath, JS::HandleValue aOriginAttributes, JSContext* cx) override { return _to Remove(aHost, aName, aPath, aOriginAttributes, cx); } \
  NS_IMETHOD RemoveNative(const nsACString& aHost, const nsACString& aName, const nsACString& aPath, mozilla::OriginAttributes * aOriginAttributes) override { return _to RemoveNative(aHost, aName, aPath, aOriginAttributes); } \
  NS_IMETHOD Add(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, const nsACString& aValue, bool aIsSecure, bool aIsHttpOnly, bool aIsSession, int64_t aExpiry, JS::HandleValue aOriginAttributes, int32_t aSameSite, nsICookie::schemeType aSchemeMap, JSContext* cx) override { return _to Add(aHost, aPath, aName, aValue, aIsSecure, aIsHttpOnly, aIsSession, aExpiry, aOriginAttributes, aSameSite, aSchemeMap, cx); } \
  NS_IMETHOD AddNative(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, const nsACString& aValue, bool aIsSecure, bool aIsHttpOnly, bool aIsSession, int64_t aExpiry, mozilla::OriginAttributes * aOriginAttributes, int32_t aSameSite, nsICookie::schemeType aSchemeMap) override { return _to AddNative(aHost, aPath, aName, aValue, aIsSecure, aIsHttpOnly, aIsSession, aExpiry, aOriginAttributes, aSameSite, aSchemeMap); } \
  NS_IMETHOD CookieExists(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, JS::HandleValue aOriginAttributes, JSContext* cx, bool *_retval) override { return _to CookieExists(aHost, aPath, aName, aOriginAttributes, cx, _retval); } \
  NS_IMETHOD CookieExistsNative(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, mozilla::OriginAttributes * aOriginAttributes, bool *aExists) override { return _to CookieExistsNative(aHost, aPath, aName, aOriginAttributes, aExists); } \
  NS_IMETHOD CountCookiesFromHost(const nsACString& aHost, uint32_t *_retval) override { return _to CountCookiesFromHost(aHost, _retval); } \
  NS_IMETHOD GetCookiesFromHost(const nsACString& aHost, JS::HandleValue aOriginAttributes, JSContext* cx, nsTArray<RefPtr<nsICookie>>& _retval) override { return _to GetCookiesFromHost(aHost, aOriginAttributes, cx, _retval); } \
  NS_IMETHOD GetCookiesWithOriginAttributes(const nsAString& aPattern, const nsACString& aHost, nsTArray<RefPtr<nsICookie>>& _retval) override { return _to GetCookiesWithOriginAttributes(aPattern, aHost, _retval); } \
  NS_IMETHOD RemoveCookiesWithOriginAttributes(const nsAString& aPattern, const nsACString& aHost) override { return _to RemoveCookiesWithOriginAttributes(aPattern, aHost); } \
  NS_IMETHOD RemoveCookiesFromExactHost(const nsACString& aHost, const nsAString& aPattern) override { return _to RemoveCookiesFromExactHost(aHost, aPattern); } \
  NS_IMETHOD RemoveAllSince(int64_t aSinceWhen, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to RemoveAllSince(aSinceWhen, cx, _retval); } \
  NS_IMETHOD GetCookiesSince(int64_t aSinceWhen, nsTArray<RefPtr<nsICookie>>& _retval) override { return _to GetCookiesSince(aSinceWhen, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOOKIEMANAGER(_to) \
  NS_IMETHOD RemoveAll(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveAll(); } \
  NS_IMETHOD GetCookies(nsTArray<RefPtr<nsICookie>>& aCookies) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCookies(aCookies); } \
  NS_IMETHOD GetSessionCookies(nsTArray<RefPtr<nsICookie>>& aSessionCookies) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSessionCookies(aSessionCookies); } \
  NS_IMETHOD GetCookieBehavior(uint32_t *aCookieBehavior) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCookieBehavior(aCookieBehavior); } \
  NS_IMETHOD Remove(const nsACString& aHost, const nsACString& aName, const nsACString& aPath, JS::HandleValue aOriginAttributes, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Remove(aHost, aName, aPath, aOriginAttributes, cx); } \
  NS_IMETHOD RemoveNative(const nsACString& aHost, const nsACString& aName, const nsACString& aPath, mozilla::OriginAttributes * aOriginAttributes) override; \
  NS_IMETHOD Add(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, const nsACString& aValue, bool aIsSecure, bool aIsHttpOnly, bool aIsSession, int64_t aExpiry, JS::HandleValue aOriginAttributes, int32_t aSameSite, nsICookie::schemeType aSchemeMap, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Add(aHost, aPath, aName, aValue, aIsSecure, aIsHttpOnly, aIsSession, aExpiry, aOriginAttributes, aSameSite, aSchemeMap, cx); } \
  NS_IMETHOD AddNative(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, const nsACString& aValue, bool aIsSecure, bool aIsHttpOnly, bool aIsSession, int64_t aExpiry, mozilla::OriginAttributes * aOriginAttributes, int32_t aSameSite, nsICookie::schemeType aSchemeMap) override; \
  NS_IMETHOD CookieExists(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, JS::HandleValue aOriginAttributes, JSContext* cx, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CookieExists(aHost, aPath, aName, aOriginAttributes, cx, _retval); } \
  NS_IMETHOD CookieExistsNative(const nsACString& aHost, const nsACString& aPath, const nsACString& aName, mozilla::OriginAttributes * aOriginAttributes, bool *aExists) override; \
  NS_IMETHOD CountCookiesFromHost(const nsACString& aHost, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CountCookiesFromHost(aHost, _retval); } \
  NS_IMETHOD GetCookiesFromHost(const nsACString& aHost, JS::HandleValue aOriginAttributes, JSContext* cx, nsTArray<RefPtr<nsICookie>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCookiesFromHost(aHost, aOriginAttributes, cx, _retval); } \
  NS_IMETHOD GetCookiesWithOriginAttributes(const nsAString& aPattern, const nsACString& aHost, nsTArray<RefPtr<nsICookie>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCookiesWithOriginAttributes(aPattern, aHost, _retval); } \
  NS_IMETHOD RemoveCookiesWithOriginAttributes(const nsAString& aPattern, const nsACString& aHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveCookiesWithOriginAttributes(aPattern, aHost); } \
  NS_IMETHOD RemoveCookiesFromExactHost(const nsACString& aHost, const nsAString& aPattern) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveCookiesFromExactHost(aHost, aPattern); } \
  NS_IMETHOD RemoveAllSince(int64_t aSinceWhen, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveAllSince(aSinceWhen, cx, _retval); } \
  NS_IMETHOD GetCookiesSince(int64_t aSinceWhen, nsTArray<RefPtr<nsICookie>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCookiesSince(aSinceWhen, _retval); } 


#endif /* __gen_nsICookieManager_h__ */
