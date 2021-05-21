/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cookie/nsICookie.idl
 */

#ifndef __gen_nsICookie_h__
#define __gen_nsICookie_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
typedef int32_t  nsCookieStatus;

typedef int32_t  nsCookiePolicy;


/* starting interface:    nsICookie */
#define NS_ICOOKIE_IID_STR "adf0db5e-211e-45a3-be14-4486ac430a58"

#define NS_ICOOKIE_IID \
  {0xadf0db5e, 0x211e, 0x45a3, \
    { 0xbe, 0x14, 0x44, 0x86, 0xac, 0x43, 0x0a, 0x58 }}

class NS_NO_VTABLE nsICookie : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOOKIE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICookie;

  enum {
    SAMESITE_NONE = 0U,
    SAMESITE_LAX = 1U,
    SAMESITE_STRICT = 2U
  };

  /* readonly attribute ACString name; */
  NS_IMETHOD GetName(nsACString& aName) = 0;

  /* readonly attribute AUTF8String value; */
  NS_IMETHOD GetValue(nsACString& aValue) = 0;

  /* readonly attribute boolean isDomain; */
  NS_IMETHOD GetIsDomain(bool *aIsDomain) = 0;

  /* readonly attribute AUTF8String host; */
  NS_IMETHOD GetHost(nsACString& aHost) = 0;

  /* readonly attribute AUTF8String rawHost; */
  NS_IMETHOD GetRawHost(nsACString& aRawHost) = 0;

  /* readonly attribute AUTF8String path; */
  NS_IMETHOD GetPath(nsACString& aPath) = 0;

  /* readonly attribute boolean isSecure; */
  NS_IMETHOD GetIsSecure(bool *aIsSecure) = 0;

  /* readonly attribute uint64_t expires; */
  NS_IMETHOD GetExpires(uint64_t *aExpires) = 0;

  /* readonly attribute int64_t expiry; */
  NS_IMETHOD GetExpiry(int64_t *aExpiry) = 0;

  /* [implicit_jscontext] readonly attribute jsval originAttributes; */
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) = 0;

  /* readonly attribute boolean isSession; */
  NS_IMETHOD GetIsSession(bool *aIsSession) = 0;

  /* readonly attribute boolean isHttpOnly; */
  NS_IMETHOD GetIsHttpOnly(bool *aIsHttpOnly) = 0;

  /* readonly attribute int64_t creationTime; */
  NS_IMETHOD GetCreationTime(int64_t *aCreationTime) = 0;

  /* readonly attribute int64_t lastAccessed; */
  NS_IMETHOD GetLastAccessed(int64_t *aLastAccessed) = 0;

  /* readonly attribute int32_t sameSite; */
  NS_IMETHOD GetSameSite(int32_t *aSameSite) = 0;

  enum schemeType : uint8_t {
    SCHEME_UNSET = 0,
    SCHEME_HTTP = 1,
    SCHEME_HTTPS = 2,
    SCHEME_FILE = 4,
  };

  /* readonly attribute nsICookie_schemeType schemeMap; */
  NS_IMETHOD GetSchemeMap(nsICookie::schemeType *aSchemeMap) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICookie, NS_ICOOKIE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOOKIE \
  NS_IMETHOD GetName(nsACString& aName) override; \
  NS_IMETHOD GetValue(nsACString& aValue) override; \
  NS_IMETHOD GetIsDomain(bool *aIsDomain) override; \
  NS_IMETHOD GetHost(nsACString& aHost) override; \
  NS_IMETHOD GetRawHost(nsACString& aRawHost) override; \
  NS_IMETHOD GetPath(nsACString& aPath) override; \
  NS_IMETHOD GetIsSecure(bool *aIsSecure) override; \
  NS_IMETHOD GetExpires(uint64_t *aExpires) override; \
  NS_IMETHOD GetExpiry(int64_t *aExpiry) override; \
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override; \
  NS_IMETHOD GetIsSession(bool *aIsSession) override; \
  NS_IMETHOD GetIsHttpOnly(bool *aIsHttpOnly) override; \
  NS_IMETHOD GetCreationTime(int64_t *aCreationTime) override; \
  NS_IMETHOD GetLastAccessed(int64_t *aLastAccessed) override; \
  NS_IMETHOD GetSameSite(int32_t *aSameSite) override; \
  NS_IMETHOD GetSchemeMap(nsICookie::schemeType *aSchemeMap) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOOKIE \
  nsresult GetName(nsACString& aName); \
  nsresult GetValue(nsACString& aValue); \
  nsresult GetIsDomain(bool *aIsDomain); \
  nsresult GetHost(nsACString& aHost); \
  nsresult GetRawHost(nsACString& aRawHost); \
  nsresult GetPath(nsACString& aPath); \
  nsresult GetIsSecure(bool *aIsSecure); \
  nsresult GetExpires(uint64_t *aExpires); \
  nsresult GetExpiry(int64_t *aExpiry); \
  nsresult GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes); \
  nsresult GetIsSession(bool *aIsSession); \
  nsresult GetIsHttpOnly(bool *aIsHttpOnly); \
  nsresult GetCreationTime(int64_t *aCreationTime); \
  nsresult GetLastAccessed(int64_t *aLastAccessed); \
  nsresult GetSameSite(int32_t *aSameSite); \
  nsresult GetSchemeMap(nsICookie::schemeType *aSchemeMap); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOOKIE(_to) \
  NS_IMETHOD GetName(nsACString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD GetValue(nsACString& aValue) override { return _to GetValue(aValue); } \
  NS_IMETHOD GetIsDomain(bool *aIsDomain) override { return _to GetIsDomain(aIsDomain); } \
  NS_IMETHOD GetHost(nsACString& aHost) override { return _to GetHost(aHost); } \
  NS_IMETHOD GetRawHost(nsACString& aRawHost) override { return _to GetRawHost(aRawHost); } \
  NS_IMETHOD GetPath(nsACString& aPath) override { return _to GetPath(aPath); } \
  NS_IMETHOD GetIsSecure(bool *aIsSecure) override { return _to GetIsSecure(aIsSecure); } \
  NS_IMETHOD GetExpires(uint64_t *aExpires) override { return _to GetExpires(aExpires); } \
  NS_IMETHOD GetExpiry(int64_t *aExpiry) override { return _to GetExpiry(aExpiry); } \
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return _to GetOriginAttributes(cx, aOriginAttributes); } \
  NS_IMETHOD GetIsSession(bool *aIsSession) override { return _to GetIsSession(aIsSession); } \
  NS_IMETHOD GetIsHttpOnly(bool *aIsHttpOnly) override { return _to GetIsHttpOnly(aIsHttpOnly); } \
  NS_IMETHOD GetCreationTime(int64_t *aCreationTime) override { return _to GetCreationTime(aCreationTime); } \
  NS_IMETHOD GetLastAccessed(int64_t *aLastAccessed) override { return _to GetLastAccessed(aLastAccessed); } \
  NS_IMETHOD GetSameSite(int32_t *aSameSite) override { return _to GetSameSite(aSameSite); } \
  NS_IMETHOD GetSchemeMap(nsICookie::schemeType *aSchemeMap) override { return _to GetSchemeMap(aSchemeMap); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOOKIE(_to) \
  NS_IMETHOD GetName(nsACString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD GetValue(nsACString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } \
  NS_IMETHOD GetIsDomain(bool *aIsDomain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsDomain(aIsDomain); } \
  NS_IMETHOD GetHost(nsACString& aHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHost(aHost); } \
  NS_IMETHOD GetRawHost(nsACString& aRawHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRawHost(aRawHost); } \
  NS_IMETHOD GetPath(nsACString& aPath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPath(aPath); } \
  NS_IMETHOD GetIsSecure(bool *aIsSecure) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsSecure(aIsSecure); } \
  NS_IMETHOD GetExpires(uint64_t *aExpires) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExpires(aExpires); } \
  NS_IMETHOD GetExpiry(int64_t *aExpiry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExpiry(aExpiry); } \
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginAttributes(cx, aOriginAttributes); } \
  NS_IMETHOD GetIsSession(bool *aIsSession) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsSession(aIsSession); } \
  NS_IMETHOD GetIsHttpOnly(bool *aIsHttpOnly) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsHttpOnly(aIsHttpOnly); } \
  NS_IMETHOD GetCreationTime(int64_t *aCreationTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCreationTime(aCreationTime); } \
  NS_IMETHOD GetLastAccessed(int64_t *aLastAccessed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastAccessed(aLastAccessed); } \
  NS_IMETHOD GetSameSite(int32_t *aSameSite) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSameSite(aSameSite); } \
  NS_IMETHOD GetSchemeMap(nsICookie::schemeType *aSchemeMap) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSchemeMap(aSchemeMap); } 


#endif /* __gen_nsICookie_h__ */
