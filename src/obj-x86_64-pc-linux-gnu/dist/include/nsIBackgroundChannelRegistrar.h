/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIBackgroundChannelRegistrar.idl
 */

#ifndef __gen_nsIBackgroundChannelRegistrar_h__
#define __gen_nsIBackgroundChannelRegistrar_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace net {
class HttpBackgroundChannelParent;
class HttpChannelParent;
}
}

/* starting interface:    nsIBackgroundChannelRegistrar */
#define NS_IBACKGROUNDCHANNELREGISTRAR_IID_STR "8acaa9b1-f0c4-4ade-baeb-39b0d4b96e5b"

#define NS_IBACKGROUNDCHANNELREGISTRAR_IID \
  {0x8acaa9b1, 0xf0c4, 0x4ade, \
    { 0xba, 0xeb, 0x39, 0xb0, 0xd4, 0xb9, 0x6e, 0x5b }}

class NS_NO_VTABLE nsIBackgroundChannelRegistrar : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBACKGROUNDCHANNELREGISTRAR_IID)

  /* [noscript,nostdcall,notxpcom] void linkHttpChannel (in uint64_t aKey, in HttpChannelParent aChannel); */
  virtual void LinkHttpChannel(uint64_t aKey, mozilla::net::HttpChannelParent * aChannel) = 0;

  /* [noscript,nostdcall,notxpcom] void linkBackgroundChannel (in uint64_t aKey, in HttpBackgroundChannelParent aBgChannel); */
  virtual void LinkBackgroundChannel(uint64_t aKey, mozilla::net::HttpBackgroundChannelParent * aBgChannel) = 0;

  /* [noscript,nostdcall,notxpcom] void deleteChannel (in uint64_t aKey); */
  virtual void DeleteChannel(uint64_t aKey) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBackgroundChannelRegistrar, NS_IBACKGROUNDCHANNELREGISTRAR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBACKGROUNDCHANNELREGISTRAR \
  virtual void LinkHttpChannel(uint64_t aKey, mozilla::net::HttpChannelParent * aChannel) override; \
  virtual void LinkBackgroundChannel(uint64_t aKey, mozilla::net::HttpBackgroundChannelParent * aBgChannel) override; \
  virtual void DeleteChannel(uint64_t aKey) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBACKGROUNDCHANNELREGISTRAR \
  void LinkHttpChannel(uint64_t aKey, mozilla::net::HttpChannelParent * aChannel); \
  void LinkBackgroundChannel(uint64_t aKey, mozilla::net::HttpBackgroundChannelParent * aBgChannel); \
  void DeleteChannel(uint64_t aKey); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBACKGROUNDCHANNELREGISTRAR(_to) \
  virtual void LinkHttpChannel(uint64_t aKey, mozilla::net::HttpChannelParent * aChannel) override { return _to LinkHttpChannel(aKey, aChannel); } \
  virtual void LinkBackgroundChannel(uint64_t aKey, mozilla::net::HttpBackgroundChannelParent * aBgChannel) override { return _to LinkBackgroundChannel(aKey, aBgChannel); } \
  virtual void DeleteChannel(uint64_t aKey) override { return _to DeleteChannel(aKey); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBACKGROUNDCHANNELREGISTRAR(_to) \
  virtual void LinkHttpChannel(uint64_t aKey, mozilla::net::HttpChannelParent * aChannel) override; \
  virtual void LinkBackgroundChannel(uint64_t aKey, mozilla::net::HttpBackgroundChannelParent * aBgChannel) override; \
  virtual void DeleteChannel(uint64_t aKey) override; 


#endif /* __gen_nsIBackgroundChannelRegistrar_h__ */
