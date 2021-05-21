/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpChannelChild.idl
 */

#ifndef __gen_nsIHttpChannelChild_h__
#define __gen_nsIHttpChannelChild_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
class OriginAttributes;
} // mozilla namespace
class nsIPrincipal; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsIHttpChannelChild */
#define NS_IHTTPCHANNELCHILD_IID_STR "d02b96ed-2789-4f42-a25c-7abe63de7c18"

#define NS_IHTTPCHANNELCHILD_IID \
  {0xd02b96ed, 0x2789, 0x4f42, \
    { 0xa2, 0x5c, 0x7a, 0xbe, 0x63, 0xde, 0x7c, 0x18 }}

class NS_NO_VTABLE nsIHttpChannelChild : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPCHANNELCHILD_IID)

  /* [must_use] void addCookiesToRequest (); */
  [[nodiscard]] NS_IMETHOD AddCookiesToRequest(void) = 0;

  /* [must_use] readonly attribute RequestHeaderTuples clientSetRequestHeaders; */
  [[nodiscard]] NS_IMETHOD GetClientSetRequestHeaders(mozilla::net::RequestHeaderTuples * * aClientSetRequestHeaders) = 0;

  /* [nostdcall,notxpcom] void GetClientSetCorsPreflightParameters (in MaybeCorsPreflightArgsRef args); */
  virtual void GetClientSetCorsPreflightParameters(mozilla::Maybe<mozilla::net::CorsPreflightArgs> & args) = 0;

  /* [must_use] void removeCorsPreflightCacheEntry (in nsIURI aURI, in nsIPrincipal aRequestingPrincipal, in const_OriginAttributes aOriginAttributes); */
  [[nodiscard]] NS_IMETHOD RemoveCorsPreflightCacheEntry(nsIURI *aURI, nsIPrincipal *aRequestingPrincipal, const mozilla::OriginAttributes & aOriginAttributes) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpChannelChild, NS_IHTTPCHANNELCHILD_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPCHANNELCHILD \
  [[nodiscard]] NS_IMETHOD AddCookiesToRequest(void) override; \
  [[nodiscard]] NS_IMETHOD GetClientSetRequestHeaders(mozilla::net::RequestHeaderTuples * * aClientSetRequestHeaders) override; \
  virtual void GetClientSetCorsPreflightParameters(mozilla::Maybe<mozilla::net::CorsPreflightArgs> & args) override; \
  [[nodiscard]] NS_IMETHOD RemoveCorsPreflightCacheEntry(nsIURI *aURI, nsIPrincipal *aRequestingPrincipal, const mozilla::OriginAttributes & aOriginAttributes) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPCHANNELCHILD \
  [[nodiscard]] nsresult AddCookiesToRequest(void); \
  [[nodiscard]] nsresult GetClientSetRequestHeaders(mozilla::net::RequestHeaderTuples * * aClientSetRequestHeaders); \
  void GetClientSetCorsPreflightParameters(mozilla::Maybe<mozilla::net::CorsPreflightArgs> & args); \
  [[nodiscard]] nsresult RemoveCorsPreflightCacheEntry(nsIURI *aURI, nsIPrincipal *aRequestingPrincipal, const mozilla::OriginAttributes & aOriginAttributes); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPCHANNELCHILD(_to) \
  [[nodiscard]] NS_IMETHOD AddCookiesToRequest(void) override { return _to AddCookiesToRequest(); } \
  [[nodiscard]] NS_IMETHOD GetClientSetRequestHeaders(mozilla::net::RequestHeaderTuples * * aClientSetRequestHeaders) override { return _to GetClientSetRequestHeaders(aClientSetRequestHeaders); } \
  virtual void GetClientSetCorsPreflightParameters(mozilla::Maybe<mozilla::net::CorsPreflightArgs> & args) override { return _to GetClientSetCorsPreflightParameters(args); } \
  [[nodiscard]] NS_IMETHOD RemoveCorsPreflightCacheEntry(nsIURI *aURI, nsIPrincipal *aRequestingPrincipal, const mozilla::OriginAttributes & aOriginAttributes) override { return _to RemoveCorsPreflightCacheEntry(aURI, aRequestingPrincipal, aOriginAttributes); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPCHANNELCHILD(_to) \
  [[nodiscard]] NS_IMETHOD AddCookiesToRequest(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddCookiesToRequest(); } \
  [[nodiscard]] NS_IMETHOD GetClientSetRequestHeaders(mozilla::net::RequestHeaderTuples * * aClientSetRequestHeaders) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClientSetRequestHeaders(aClientSetRequestHeaders); } \
  virtual void GetClientSetCorsPreflightParameters(mozilla::Maybe<mozilla::net::CorsPreflightArgs> & args) override; \
  [[nodiscard]] NS_IMETHOD RemoveCorsPreflightCacheEntry(nsIURI *aURI, nsIPrincipal *aRequestingPrincipal, const mozilla::OriginAttributes & aOriginAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveCorsPreflightCacheEntry(aURI, aRequestingPrincipal, aOriginAttributes); } 


#endif /* __gen_nsIHttpChannelChild_h__ */
