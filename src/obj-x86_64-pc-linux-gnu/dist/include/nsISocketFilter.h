/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISocketFilter.idl
 */

#ifndef __gen_nsISocketFilter_h__
#define __gen_nsISocketFilter_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsINetAddr_h__
#include "nsINetAddr.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsISocketFilter */
#define NS_ISOCKETFILTER_IID_STR "afe2c40c-b9b9-4207-b898-e5cde18c6139"

#define NS_ISOCKETFILTER_IID \
  {0xafe2c40c, 0xb9b9, 0x4207, \
    { 0xb8, 0x98, 0xe5, 0xcd, 0xe1, 0x8c, 0x61, 0x39 }}

class NS_NO_VTABLE nsISocketFilter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISOCKETFILTER_IID)

  enum {
    SF_INCOMING = 0,
    SF_OUTGOING = 1
  };

  /* bool filterPacket ([const] in NetAddrPtr remote_addr, [array, size_is (len), const] in uint8_t data, in unsigned long len, in long direction); */
  NS_IMETHOD FilterPacket(const mozilla::net::NetAddr * remote_addr, const uint8_t *data, uint32_t len, int32_t direction, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISocketFilter, NS_ISOCKETFILTER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISOCKETFILTER \
  NS_IMETHOD FilterPacket(const mozilla::net::NetAddr * remote_addr, const uint8_t *data, uint32_t len, int32_t direction, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISOCKETFILTER \
  nsresult FilterPacket(const mozilla::net::NetAddr * remote_addr, const uint8_t *data, uint32_t len, int32_t direction, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISOCKETFILTER(_to) \
  NS_IMETHOD FilterPacket(const mozilla::net::NetAddr * remote_addr, const uint8_t *data, uint32_t len, int32_t direction, bool *_retval) override { return _to FilterPacket(remote_addr, data, len, direction, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISOCKETFILTER(_to) \
  NS_IMETHOD FilterPacket(const mozilla::net::NetAddr * remote_addr, const uint8_t *data, uint32_t len, int32_t direction, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FilterPacket(remote_addr, data, len, direction, _retval); } 


/* starting interface:    nsISocketFilterHandler */
#define NS_ISOCKETFILTERHANDLER_IID_STR "81ee76c6-4753-4125-9c8c-290ed9ba62fb"

#define NS_ISOCKETFILTERHANDLER_IID \
  {0x81ee76c6, 0x4753, 0x4125, \
    { 0x9c, 0x8c, 0x29, 0x0e, 0xd9, 0xba, 0x62, 0xfb }}

class NS_NO_VTABLE nsISocketFilterHandler : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISOCKETFILTERHANDLER_IID)

  /* nsISocketFilter newFilter (); */
  NS_IMETHOD NewFilter(nsISocketFilter **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISocketFilterHandler, NS_ISOCKETFILTERHANDLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISOCKETFILTERHANDLER \
  NS_IMETHOD NewFilter(nsISocketFilter **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISOCKETFILTERHANDLER \
  nsresult NewFilter(nsISocketFilter **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISOCKETFILTERHANDLER(_to) \
  NS_IMETHOD NewFilter(nsISocketFilter **_retval) override { return _to NewFilter(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISOCKETFILTERHANDLER(_to) \
  NS_IMETHOD NewFilter(nsISocketFilter **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewFilter(_retval); } 

/**
 * Filter handlers are registered with XPCOM under the following CONTRACTID prefix:
 */
#define NS_NETWORK_UDP_SOCKET_FILTER_HANDLER_PREFIX "@mozilla.org/network/udp-filter-handler;1?name="
#define NS_NETWORK_TCP_SOCKET_FILTER_HANDLER_PREFIX "@mozilla.org/network/tcp-filter-handler;1?name="
#define NS_NETWORK_SOCKET_FILTER_HANDLER_STUN_SUFFIX "stun"
#define NS_STUN_UDP_SOCKET_FILTER_HANDLER_CONTRACTID NS_NETWORK_UDP_SOCKET_FILTER_HANDLER_PREFIX NS_NETWORK_SOCKET_FILTER_HANDLER_STUN_SUFFIX
#define NS_STUN_TCP_SOCKET_FILTER_HANDLER_CONTRACTID NS_NETWORK_TCP_SOCKET_FILTER_HANDLER_PREFIX NS_NETWORK_SOCKET_FILTER_HANDLER_STUN_SUFFIX

#endif /* __gen_nsISocketFilter_h__ */
