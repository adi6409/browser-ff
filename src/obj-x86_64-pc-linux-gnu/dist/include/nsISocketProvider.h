/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/socket/nsISocketProvider.idl
 */

#ifndef __gen_nsISocketProvider_h__
#define __gen_nsISocketProvider_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIProxyInfo; /* forward declaration */

#include "mozilla/BasePrincipal.h"

/* starting interface:    nsISocketProvider */
#define NS_ISOCKETPROVIDER_IID_STR "508d5469-9e1e-4a08-b5b0-7cfebba1e51a"

#define NS_ISOCKETPROVIDER_IID \
  {0x508d5469, 0x9e1e, 0x4a08, \
    { 0xb5, 0xb0, 0x7c, 0xfe, 0xbb, 0xa1, 0xe5, 0x1a }}

class NS_NO_VTABLE nsISocketProvider : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISOCKETPROVIDER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISocketProvider;

  /* [noscript] void newSocket (in long aFamily, in string aHost, in long aPort, in nsIProxyInfo aProxy, in const_OriginAttributesRef aOriginAttributes, in unsigned long aFlags, in unsigned long aTlsFlags, out PRFileDescStar aFileDesc, out nsISupports aSecurityInfo); */
  NS_IMETHOD NewSocket(int32_t aFamily, const char * aHost, int32_t aPort, nsIProxyInfo *aProxy, const mozilla::OriginAttributes & aOriginAttributes, uint32_t aFlags, uint32_t aTlsFlags, struct PRFileDesc * * aFileDesc, nsISupports **aSecurityInfo) = 0;

  /* [noscript] void addToSocket (in long aFamily, in string aHost, in long aPort, in nsIProxyInfo aProxy, in const_OriginAttributesRef aOriginAttributes, in unsigned long aFlags, in unsigned long aTlsFlags, in PRFileDescStar aFileDesc, out nsISupports aSecurityInfo); */
  NS_IMETHOD AddToSocket(int32_t aFamily, const char * aHost, int32_t aPort, nsIProxyInfo *aProxy, const mozilla::OriginAttributes & aOriginAttributes, uint32_t aFlags, uint32_t aTlsFlags, struct PRFileDesc * aFileDesc, nsISupports **aSecurityInfo) = 0;

  enum {
    PROXY_RESOLVES_HOST = 1,
    ANONYMOUS_CONNECT = 2,
    NO_PERMANENT_STORAGE = 4U,
    BE_CONSERVATIVE = 8U,
    ANONYMOUS_CONNECT_ALLOW_CLIENT_CERT = 16
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISocketProvider, NS_ISOCKETPROVIDER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISOCKETPROVIDER \
  NS_IMETHOD NewSocket(int32_t aFamily, const char * aHost, int32_t aPort, nsIProxyInfo *aProxy, const mozilla::OriginAttributes & aOriginAttributes, uint32_t aFlags, uint32_t aTlsFlags, struct PRFileDesc * * aFileDesc, nsISupports **aSecurityInfo) override; \
  NS_IMETHOD AddToSocket(int32_t aFamily, const char * aHost, int32_t aPort, nsIProxyInfo *aProxy, const mozilla::OriginAttributes & aOriginAttributes, uint32_t aFlags, uint32_t aTlsFlags, struct PRFileDesc * aFileDesc, nsISupports **aSecurityInfo) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISOCKETPROVIDER \
  nsresult NewSocket(int32_t aFamily, const char * aHost, int32_t aPort, nsIProxyInfo *aProxy, const mozilla::OriginAttributes & aOriginAttributes, uint32_t aFlags, uint32_t aTlsFlags, struct PRFileDesc * * aFileDesc, nsISupports **aSecurityInfo); \
  nsresult AddToSocket(int32_t aFamily, const char * aHost, int32_t aPort, nsIProxyInfo *aProxy, const mozilla::OriginAttributes & aOriginAttributes, uint32_t aFlags, uint32_t aTlsFlags, struct PRFileDesc * aFileDesc, nsISupports **aSecurityInfo); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISOCKETPROVIDER(_to) \
  NS_IMETHOD NewSocket(int32_t aFamily, const char * aHost, int32_t aPort, nsIProxyInfo *aProxy, const mozilla::OriginAttributes & aOriginAttributes, uint32_t aFlags, uint32_t aTlsFlags, struct PRFileDesc * * aFileDesc, nsISupports **aSecurityInfo) override { return _to NewSocket(aFamily, aHost, aPort, aProxy, aOriginAttributes, aFlags, aTlsFlags, aFileDesc, aSecurityInfo); } \
  NS_IMETHOD AddToSocket(int32_t aFamily, const char * aHost, int32_t aPort, nsIProxyInfo *aProxy, const mozilla::OriginAttributes & aOriginAttributes, uint32_t aFlags, uint32_t aTlsFlags, struct PRFileDesc * aFileDesc, nsISupports **aSecurityInfo) override { return _to AddToSocket(aFamily, aHost, aPort, aProxy, aOriginAttributes, aFlags, aTlsFlags, aFileDesc, aSecurityInfo); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISOCKETPROVIDER(_to) \
  NS_IMETHOD NewSocket(int32_t aFamily, const char * aHost, int32_t aPort, nsIProxyInfo *aProxy, const mozilla::OriginAttributes & aOriginAttributes, uint32_t aFlags, uint32_t aTlsFlags, struct PRFileDesc * * aFileDesc, nsISupports **aSecurityInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewSocket(aFamily, aHost, aPort, aProxy, aOriginAttributes, aFlags, aTlsFlags, aFileDesc, aSecurityInfo); } \
  NS_IMETHOD AddToSocket(int32_t aFamily, const char * aHost, int32_t aPort, nsIProxyInfo *aProxy, const mozilla::OriginAttributes & aOriginAttributes, uint32_t aFlags, uint32_t aTlsFlags, struct PRFileDesc * aFileDesc, nsISupports **aSecurityInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddToSocket(aFamily, aHost, aPort, aProxy, aOriginAttributes, aFlags, aTlsFlags, aFileDesc, aSecurityInfo); } \


#endif /* __gen_nsISocketProvider_h__ */
