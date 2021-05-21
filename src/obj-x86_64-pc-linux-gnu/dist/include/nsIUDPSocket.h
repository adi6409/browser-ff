/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIUDPSocket.idl
 */

#ifndef __gen_nsIUDPSocket_h__
#define __gen_nsIUDPSocket_h__


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
class nsINetAddr; /* forward declaration */

class nsIUDPSocketListener; /* forward declaration */

class nsIUDPMessage; /* forward declaration */

class nsISocketTransport; /* forward declaration */

class nsIOutputStream; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsIPrincipal; /* forward declaration */

#include "nsTArrayForwardDeclare.h"
namespace mozilla {
namespace net {
union NetAddr;
}
}

/* starting interface:    nsIUDPSocket */
#define NS_IUDPSOCKET_IID_STR "d423bf4e-4499-40cf-bc03-153e2bf206d1"

#define NS_IUDPSOCKET_IID \
  {0xd423bf4e, 0x4499, 0x40cf, \
    { 0xbc, 0x03, 0x15, 0x3e, 0x2b, 0xf2, 0x06, 0xd1 }}

class NS_NO_VTABLE nsIUDPSocket : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUDPSOCKET_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUDPSocket;

  /* [optional_argc] void init (in long aPort, in boolean aLoopbackOnly, in nsIPrincipal aPrincipal, [optional] in boolean aAddressReuse); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(int32_t aPort, bool aLoopbackOnly, nsIPrincipal *aPrincipal, bool aAddressReuse, uint8_t _argc) = 0;

  /* [optional_argc] void init2 (in AUTF8String aAddr, in long aPort, in nsIPrincipal aPrincipal, [optional] in boolean aAddressReuse); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init2(const nsACString& aAddr, int32_t aPort, nsIPrincipal *aPrincipal, bool aAddressReuse, uint8_t _argc) = 0;

  /* [noscript,optional_argc] void initWithAddress ([const] in NetAddrPtr aAddr, in nsIPrincipal aPrincipal, [optional] in boolean aAddressReuse); */
  NS_IMETHOD InitWithAddress(const mozilla::net::NetAddr * aAddr, nsIPrincipal *aPrincipal, bool aAddressReuse, uint8_t _argc) = 0;

  /* void close (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Close(void) = 0;

  /* void asyncListen (in nsIUDPSocketListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncListen(nsIUDPSocketListener *aListener) = 0;

  /* void connect ([const] in NetAddrPtr aAddr); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Connect(const mozilla::net::NetAddr * aAddr) = 0;

  /* readonly attribute nsINetAddr localAddr; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLocalAddr(nsINetAddr **aLocalAddr) = 0;

  /* readonly attribute long port; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPort(int32_t *aPort) = 0;

  /* [noscript] NetAddr getAddress (); */
  NS_IMETHOD GetAddress(mozilla::net::NetAddr * _retval) = 0;

  /* unsigned long send (in AUTF8String host, in unsigned short port, in Array<uint8_t> data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Send(const nsACString& host, uint16_t port, const nsTArray<uint8_t >& data, uint32_t *_retval) = 0;

  /* unsigned long sendWithAddr (in nsINetAddr addr, in Array<uint8_t> data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SendWithAddr(nsINetAddr *addr, const nsTArray<uint8_t >& data, uint32_t *_retval) = 0;

  /* [noscript] unsigned long sendWithAddress ([const] in NetAddrPtr addr, in Array<uint8_t> data); */
  NS_IMETHOD SendWithAddress(const mozilla::net::NetAddr * addr, const nsTArray<uint8_t >& data, uint32_t *_retval) = 0;

  /* void sendBinaryStream (in AUTF8String host, in unsigned short port, in nsIInputStream stream); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SendBinaryStream(const nsACString& host, uint16_t port, nsIInputStream *stream) = 0;

  /* void sendBinaryStreamWithAddress ([const] in NetAddrPtr addr, in nsIInputStream stream); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SendBinaryStreamWithAddress(const mozilla::net::NetAddr * addr, nsIInputStream *stream) = 0;

  /* void joinMulticast (in AUTF8String addr, [optional] in AUTF8String iface); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD JoinMulticast(const nsACString& addr, const nsACString& iface) = 0;

  /* [noscript] void joinMulticastAddr ([const] in NetAddr addr, [const, optional] in NetAddrPtr iface); */
  NS_IMETHOD JoinMulticastAddr(const mozilla::net::NetAddr addr, const mozilla::net::NetAddr * iface) = 0;

  /* void leaveMulticast (in AUTF8String addr, [optional] in AUTF8String iface); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LeaveMulticast(const nsACString& addr, const nsACString& iface) = 0;

  /* [noscript] void leaveMulticastAddr ([const] in NetAddr addr, [const, optional] in NetAddrPtr iface); */
  NS_IMETHOD LeaveMulticastAddr(const mozilla::net::NetAddr addr, const mozilla::net::NetAddr * iface) = 0;

  /* attribute boolean multicastLoopback; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMulticastLoopback(bool *aMulticastLoopback) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMulticastLoopback(bool aMulticastLoopback) = 0;

  /* attribute AUTF8String multicastInterface; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMulticastInterface(nsACString& aMulticastInterface) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMulticastInterface(const nsACString& aMulticastInterface) = 0;

  /* [noscript] attribute NetAddr multicastInterfaceAddr; */
  NS_IMETHOD GetMulticastInterfaceAddr(mozilla::net::NetAddr * aMulticastInterfaceAddr) = 0;
  NS_IMETHOD SetMulticastInterfaceAddr(mozilla::net::NetAddr aMulticastInterfaceAddr) = 0;

  /* [noscript] attribute long recvBufferSize; */
  NS_IMETHOD GetRecvBufferSize(int32_t *aRecvBufferSize) = 0;
  NS_IMETHOD SetRecvBufferSize(int32_t aRecvBufferSize) = 0;

  /* [noscript] attribute long sendBufferSize; */
  NS_IMETHOD GetSendBufferSize(int32_t *aSendBufferSize) = 0;
  NS_IMETHOD SetSendBufferSize(int32_t aSendBufferSize) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUDPSocket, NS_IUDPSOCKET_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUDPSOCKET \
  NS_IMETHOD Init(int32_t aPort, bool aLoopbackOnly, nsIPrincipal *aPrincipal, bool aAddressReuse, uint8_t _argc) override; \
  NS_IMETHOD Init2(const nsACString& aAddr, int32_t aPort, nsIPrincipal *aPrincipal, bool aAddressReuse, uint8_t _argc) override; \
  NS_IMETHOD InitWithAddress(const mozilla::net::NetAddr * aAddr, nsIPrincipal *aPrincipal, bool aAddressReuse, uint8_t _argc) override; \
  NS_IMETHOD Close(void) override; \
  NS_IMETHOD AsyncListen(nsIUDPSocketListener *aListener) override; \
  NS_IMETHOD Connect(const mozilla::net::NetAddr * aAddr) override; \
  NS_IMETHOD GetLocalAddr(nsINetAddr **aLocalAddr) override; \
  NS_IMETHOD GetPort(int32_t *aPort) override; \
  NS_IMETHOD GetAddress(mozilla::net::NetAddr * _retval) override; \
  NS_IMETHOD Send(const nsACString& host, uint16_t port, const nsTArray<uint8_t >& data, uint32_t *_retval) override; \
  NS_IMETHOD SendWithAddr(nsINetAddr *addr, const nsTArray<uint8_t >& data, uint32_t *_retval) override; \
  NS_IMETHOD SendWithAddress(const mozilla::net::NetAddr * addr, const nsTArray<uint8_t >& data, uint32_t *_retval) override; \
  NS_IMETHOD SendBinaryStream(const nsACString& host, uint16_t port, nsIInputStream *stream) override; \
  NS_IMETHOD SendBinaryStreamWithAddress(const mozilla::net::NetAddr * addr, nsIInputStream *stream) override; \
  NS_IMETHOD JoinMulticast(const nsACString& addr, const nsACString& iface) override; \
  NS_IMETHOD JoinMulticastAddr(const mozilla::net::NetAddr addr, const mozilla::net::NetAddr * iface) override; \
  NS_IMETHOD LeaveMulticast(const nsACString& addr, const nsACString& iface) override; \
  NS_IMETHOD LeaveMulticastAddr(const mozilla::net::NetAddr addr, const mozilla::net::NetAddr * iface) override; \
  NS_IMETHOD GetMulticastLoopback(bool *aMulticastLoopback) override; \
  NS_IMETHOD SetMulticastLoopback(bool aMulticastLoopback) override; \
  NS_IMETHOD GetMulticastInterface(nsACString& aMulticastInterface) override; \
  NS_IMETHOD SetMulticastInterface(const nsACString& aMulticastInterface) override; \
  NS_IMETHOD GetMulticastInterfaceAddr(mozilla::net::NetAddr * aMulticastInterfaceAddr) override; \
  NS_IMETHOD SetMulticastInterfaceAddr(mozilla::net::NetAddr aMulticastInterfaceAddr) override; \
  NS_IMETHOD GetRecvBufferSize(int32_t *aRecvBufferSize) override; \
  NS_IMETHOD SetRecvBufferSize(int32_t aRecvBufferSize) override; \
  NS_IMETHOD GetSendBufferSize(int32_t *aSendBufferSize) override; \
  NS_IMETHOD SetSendBufferSize(int32_t aSendBufferSize) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUDPSOCKET \
  nsresult Init(int32_t aPort, bool aLoopbackOnly, nsIPrincipal *aPrincipal, bool aAddressReuse, uint8_t _argc); \
  nsresult Init2(const nsACString& aAddr, int32_t aPort, nsIPrincipal *aPrincipal, bool aAddressReuse, uint8_t _argc); \
  nsresult InitWithAddress(const mozilla::net::NetAddr * aAddr, nsIPrincipal *aPrincipal, bool aAddressReuse, uint8_t _argc); \
  nsresult Close(void); \
  nsresult AsyncListen(nsIUDPSocketListener *aListener); \
  nsresult Connect(const mozilla::net::NetAddr * aAddr); \
  nsresult GetLocalAddr(nsINetAddr **aLocalAddr); \
  nsresult GetPort(int32_t *aPort); \
  nsresult GetAddress(mozilla::net::NetAddr * _retval); \
  nsresult Send(const nsACString& host, uint16_t port, const nsTArray<uint8_t >& data, uint32_t *_retval); \
  nsresult SendWithAddr(nsINetAddr *addr, const nsTArray<uint8_t >& data, uint32_t *_retval); \
  nsresult SendWithAddress(const mozilla::net::NetAddr * addr, const nsTArray<uint8_t >& data, uint32_t *_retval); \
  nsresult SendBinaryStream(const nsACString& host, uint16_t port, nsIInputStream *stream); \
  nsresult SendBinaryStreamWithAddress(const mozilla::net::NetAddr * addr, nsIInputStream *stream); \
  nsresult JoinMulticast(const nsACString& addr, const nsACString& iface); \
  nsresult JoinMulticastAddr(const mozilla::net::NetAddr addr, const mozilla::net::NetAddr * iface); \
  nsresult LeaveMulticast(const nsACString& addr, const nsACString& iface); \
  nsresult LeaveMulticastAddr(const mozilla::net::NetAddr addr, const mozilla::net::NetAddr * iface); \
  nsresult GetMulticastLoopback(bool *aMulticastLoopback); \
  nsresult SetMulticastLoopback(bool aMulticastLoopback); \
  nsresult GetMulticastInterface(nsACString& aMulticastInterface); \
  nsresult SetMulticastInterface(const nsACString& aMulticastInterface); \
  nsresult GetMulticastInterfaceAddr(mozilla::net::NetAddr * aMulticastInterfaceAddr); \
  nsresult SetMulticastInterfaceAddr(mozilla::net::NetAddr aMulticastInterfaceAddr); \
  nsresult GetRecvBufferSize(int32_t *aRecvBufferSize); \
  nsresult SetRecvBufferSize(int32_t aRecvBufferSize); \
  nsresult GetSendBufferSize(int32_t *aSendBufferSize); \
  nsresult SetSendBufferSize(int32_t aSendBufferSize); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUDPSOCKET(_to) \
  NS_IMETHOD Init(int32_t aPort, bool aLoopbackOnly, nsIPrincipal *aPrincipal, bool aAddressReuse, uint8_t _argc) override { return _to Init(aPort, aLoopbackOnly, aPrincipal, aAddressReuse, _argc); } \
  NS_IMETHOD Init2(const nsACString& aAddr, int32_t aPort, nsIPrincipal *aPrincipal, bool aAddressReuse, uint8_t _argc) override { return _to Init2(aAddr, aPort, aPrincipal, aAddressReuse, _argc); } \
  NS_IMETHOD InitWithAddress(const mozilla::net::NetAddr * aAddr, nsIPrincipal *aPrincipal, bool aAddressReuse, uint8_t _argc) override { return _to InitWithAddress(aAddr, aPrincipal, aAddressReuse, _argc); } \
  NS_IMETHOD Close(void) override { return _to Close(); } \
  NS_IMETHOD AsyncListen(nsIUDPSocketListener *aListener) override { return _to AsyncListen(aListener); } \
  NS_IMETHOD Connect(const mozilla::net::NetAddr * aAddr) override { return _to Connect(aAddr); } \
  NS_IMETHOD GetLocalAddr(nsINetAddr **aLocalAddr) override { return _to GetLocalAddr(aLocalAddr); } \
  NS_IMETHOD GetPort(int32_t *aPort) override { return _to GetPort(aPort); } \
  NS_IMETHOD GetAddress(mozilla::net::NetAddr * _retval) override { return _to GetAddress(_retval); } \
  NS_IMETHOD Send(const nsACString& host, uint16_t port, const nsTArray<uint8_t >& data, uint32_t *_retval) override { return _to Send(host, port, data, _retval); } \
  NS_IMETHOD SendWithAddr(nsINetAddr *addr, const nsTArray<uint8_t >& data, uint32_t *_retval) override { return _to SendWithAddr(addr, data, _retval); } \
  NS_IMETHOD SendWithAddress(const mozilla::net::NetAddr * addr, const nsTArray<uint8_t >& data, uint32_t *_retval) override { return _to SendWithAddress(addr, data, _retval); } \
  NS_IMETHOD SendBinaryStream(const nsACString& host, uint16_t port, nsIInputStream *stream) override { return _to SendBinaryStream(host, port, stream); } \
  NS_IMETHOD SendBinaryStreamWithAddress(const mozilla::net::NetAddr * addr, nsIInputStream *stream) override { return _to SendBinaryStreamWithAddress(addr, stream); } \
  NS_IMETHOD JoinMulticast(const nsACString& addr, const nsACString& iface) override { return _to JoinMulticast(addr, iface); } \
  NS_IMETHOD JoinMulticastAddr(const mozilla::net::NetAddr addr, const mozilla::net::NetAddr * iface) override { return _to JoinMulticastAddr(addr, iface); } \
  NS_IMETHOD LeaveMulticast(const nsACString& addr, const nsACString& iface) override { return _to LeaveMulticast(addr, iface); } \
  NS_IMETHOD LeaveMulticastAddr(const mozilla::net::NetAddr addr, const mozilla::net::NetAddr * iface) override { return _to LeaveMulticastAddr(addr, iface); } \
  NS_IMETHOD GetMulticastLoopback(bool *aMulticastLoopback) override { return _to GetMulticastLoopback(aMulticastLoopback); } \
  NS_IMETHOD SetMulticastLoopback(bool aMulticastLoopback) override { return _to SetMulticastLoopback(aMulticastLoopback); } \
  NS_IMETHOD GetMulticastInterface(nsACString& aMulticastInterface) override { return _to GetMulticastInterface(aMulticastInterface); } \
  NS_IMETHOD SetMulticastInterface(const nsACString& aMulticastInterface) override { return _to SetMulticastInterface(aMulticastInterface); } \
  NS_IMETHOD GetMulticastInterfaceAddr(mozilla::net::NetAddr * aMulticastInterfaceAddr) override { return _to GetMulticastInterfaceAddr(aMulticastInterfaceAddr); } \
  NS_IMETHOD SetMulticastInterfaceAddr(mozilla::net::NetAddr aMulticastInterfaceAddr) override { return _to SetMulticastInterfaceAddr(aMulticastInterfaceAddr); } \
  NS_IMETHOD GetRecvBufferSize(int32_t *aRecvBufferSize) override { return _to GetRecvBufferSize(aRecvBufferSize); } \
  NS_IMETHOD SetRecvBufferSize(int32_t aRecvBufferSize) override { return _to SetRecvBufferSize(aRecvBufferSize); } \
  NS_IMETHOD GetSendBufferSize(int32_t *aSendBufferSize) override { return _to GetSendBufferSize(aSendBufferSize); } \
  NS_IMETHOD SetSendBufferSize(int32_t aSendBufferSize) override { return _to SetSendBufferSize(aSendBufferSize); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUDPSOCKET(_to) \
  NS_IMETHOD Init(int32_t aPort, bool aLoopbackOnly, nsIPrincipal *aPrincipal, bool aAddressReuse, uint8_t _argc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aPort, aLoopbackOnly, aPrincipal, aAddressReuse, _argc); } \
  NS_IMETHOD Init2(const nsACString& aAddr, int32_t aPort, nsIPrincipal *aPrincipal, bool aAddressReuse, uint8_t _argc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init2(aAddr, aPort, aPrincipal, aAddressReuse, _argc); } \
  NS_IMETHOD InitWithAddress(const mozilla::net::NetAddr * aAddr, nsIPrincipal *aPrincipal, bool aAddressReuse, uint8_t _argc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithAddress(aAddr, aPrincipal, aAddressReuse, _argc); } \
  NS_IMETHOD Close(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(); } \
  NS_IMETHOD AsyncListen(nsIUDPSocketListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncListen(aListener); } \
  NS_IMETHOD Connect(const mozilla::net::NetAddr * aAddr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Connect(aAddr); } \
  NS_IMETHOD GetLocalAddr(nsINetAddr **aLocalAddr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLocalAddr(aLocalAddr); } \
  NS_IMETHOD GetPort(int32_t *aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPort(aPort); } \
  NS_IMETHOD GetAddress(mozilla::net::NetAddr * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAddress(_retval); } \
  NS_IMETHOD Send(const nsACString& host, uint16_t port, const nsTArray<uint8_t >& data, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Send(host, port, data, _retval); } \
  NS_IMETHOD SendWithAddr(nsINetAddr *addr, const nsTArray<uint8_t >& data, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendWithAddr(addr, data, _retval); } \
  NS_IMETHOD SendWithAddress(const mozilla::net::NetAddr * addr, const nsTArray<uint8_t >& data, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendWithAddress(addr, data, _retval); } \
  NS_IMETHOD SendBinaryStream(const nsACString& host, uint16_t port, nsIInputStream *stream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendBinaryStream(host, port, stream); } \
  NS_IMETHOD SendBinaryStreamWithAddress(const mozilla::net::NetAddr * addr, nsIInputStream *stream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendBinaryStreamWithAddress(addr, stream); } \
  NS_IMETHOD JoinMulticast(const nsACString& addr, const nsACString& iface) override { return !_to ? NS_ERROR_NULL_POINTER : _to->JoinMulticast(addr, iface); } \
  NS_IMETHOD JoinMulticastAddr(const mozilla::net::NetAddr addr, const mozilla::net::NetAddr * iface) override { return !_to ? NS_ERROR_NULL_POINTER : _to->JoinMulticastAddr(addr, iface); } \
  NS_IMETHOD LeaveMulticast(const nsACString& addr, const nsACString& iface) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LeaveMulticast(addr, iface); } \
  NS_IMETHOD LeaveMulticastAddr(const mozilla::net::NetAddr addr, const mozilla::net::NetAddr * iface) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LeaveMulticastAddr(addr, iface); } \
  NS_IMETHOD GetMulticastLoopback(bool *aMulticastLoopback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMulticastLoopback(aMulticastLoopback); } \
  NS_IMETHOD SetMulticastLoopback(bool aMulticastLoopback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMulticastLoopback(aMulticastLoopback); } \
  NS_IMETHOD GetMulticastInterface(nsACString& aMulticastInterface) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMulticastInterface(aMulticastInterface); } \
  NS_IMETHOD SetMulticastInterface(const nsACString& aMulticastInterface) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMulticastInterface(aMulticastInterface); } \
  NS_IMETHOD GetMulticastInterfaceAddr(mozilla::net::NetAddr * aMulticastInterfaceAddr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMulticastInterfaceAddr(aMulticastInterfaceAddr); } \
  NS_IMETHOD SetMulticastInterfaceAddr(mozilla::net::NetAddr aMulticastInterfaceAddr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMulticastInterfaceAddr(aMulticastInterfaceAddr); } \
  NS_IMETHOD GetRecvBufferSize(int32_t *aRecvBufferSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRecvBufferSize(aRecvBufferSize); } \
  NS_IMETHOD SetRecvBufferSize(int32_t aRecvBufferSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRecvBufferSize(aRecvBufferSize); } \
  NS_IMETHOD GetSendBufferSize(int32_t *aSendBufferSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSendBufferSize(aSendBufferSize); } \
  NS_IMETHOD SetSendBufferSize(int32_t aSendBufferSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSendBufferSize(aSendBufferSize); } 


/* starting interface:    nsIUDPSocketListener */
#define NS_IUDPSOCKETLISTENER_IID_STR "2e4b5dd3-7358-4281-b81f-10c62ef39cb5"

#define NS_IUDPSOCKETLISTENER_IID \
  {0x2e4b5dd3, 0x7358, 0x4281, \
    { 0xb8, 0x1f, 0x10, 0xc6, 0x2e, 0xf3, 0x9c, 0xb5 }}

class NS_NO_VTABLE nsIUDPSocketListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUDPSOCKETLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUDPSocketListener;

  /* void onPacketReceived (in nsIUDPSocket aSocket, in nsIUDPMessage aMessage); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnPacketReceived(nsIUDPSocket *aSocket, nsIUDPMessage *aMessage) = 0;

  /* void onStopListening (in nsIUDPSocket aSocket, in nsresult aStatus); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnStopListening(nsIUDPSocket *aSocket, nsresult aStatus) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUDPSocketListener, NS_IUDPSOCKETLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUDPSOCKETLISTENER \
  NS_IMETHOD OnPacketReceived(nsIUDPSocket *aSocket, nsIUDPMessage *aMessage) override; \
  NS_IMETHOD OnStopListening(nsIUDPSocket *aSocket, nsresult aStatus) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUDPSOCKETLISTENER \
  nsresult OnPacketReceived(nsIUDPSocket *aSocket, nsIUDPMessage *aMessage); \
  nsresult OnStopListening(nsIUDPSocket *aSocket, nsresult aStatus); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUDPSOCKETLISTENER(_to) \
  NS_IMETHOD OnPacketReceived(nsIUDPSocket *aSocket, nsIUDPMessage *aMessage) override { return _to OnPacketReceived(aSocket, aMessage); } \
  NS_IMETHOD OnStopListening(nsIUDPSocket *aSocket, nsresult aStatus) override { return _to OnStopListening(aSocket, aStatus); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUDPSOCKETLISTENER(_to) \
  NS_IMETHOD OnPacketReceived(nsIUDPSocket *aSocket, nsIUDPMessage *aMessage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnPacketReceived(aSocket, aMessage); } \
  NS_IMETHOD OnStopListening(nsIUDPSocket *aSocket, nsresult aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnStopListening(aSocket, aStatus); } 


/* starting interface:    nsIUDPMessage */
#define NS_IUDPMESSAGE_IID_STR "afdc743f-9cc0-40d8-b442-695dc54bbb74"

#define NS_IUDPMESSAGE_IID \
  {0xafdc743f, 0x9cc0, 0x40d8, \
    { 0xb4, 0x42, 0x69, 0x5d, 0xc5, 0x4b, 0xbb, 0x74 }}

class NS_NO_VTABLE nsIUDPMessage : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUDPMESSAGE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUDPMessage;

  /* readonly attribute nsINetAddr fromAddr; */
  NS_IMETHOD GetFromAddr(nsINetAddr **aFromAddr) = 0;

  /* readonly attribute ACString data; */
  NS_IMETHOD GetData(nsACString& aData) = 0;

  /* readonly attribute nsIOutputStream outputStream; */
  NS_IMETHOD GetOutputStream(nsIOutputStream **aOutputStream) = 0;

  /* [implicit_jscontext] readonly attribute jsval rawData; */
  NS_IMETHOD GetRawData(JSContext* cx, JS::MutableHandleValue aRawData) = 0;

  /* [noscript,nostdcall,notxpcom] Uint8TArrayRef getDataAsTArray (); */
  virtual FallibleTArray<uint8_t> & GetDataAsTArray(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUDPMessage, NS_IUDPMESSAGE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUDPMESSAGE \
  NS_IMETHOD GetFromAddr(nsINetAddr **aFromAddr) override; \
  NS_IMETHOD GetData(nsACString& aData) override; \
  NS_IMETHOD GetOutputStream(nsIOutputStream **aOutputStream) override; \
  NS_IMETHOD GetRawData(JSContext* cx, JS::MutableHandleValue aRawData) override; \
  virtual FallibleTArray<uint8_t> & GetDataAsTArray(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUDPMESSAGE \
  nsresult GetFromAddr(nsINetAddr **aFromAddr); \
  nsresult GetData(nsACString& aData); \
  nsresult GetOutputStream(nsIOutputStream **aOutputStream); \
  nsresult GetRawData(JSContext* cx, JS::MutableHandleValue aRawData); \
  FallibleTArray<uint8_t> & GetDataAsTArray(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUDPMESSAGE(_to) \
  NS_IMETHOD GetFromAddr(nsINetAddr **aFromAddr) override { return _to GetFromAddr(aFromAddr); } \
  NS_IMETHOD GetData(nsACString& aData) override { return _to GetData(aData); } \
  NS_IMETHOD GetOutputStream(nsIOutputStream **aOutputStream) override { return _to GetOutputStream(aOutputStream); } \
  NS_IMETHOD GetRawData(JSContext* cx, JS::MutableHandleValue aRawData) override { return _to GetRawData(cx, aRawData); } \
  virtual FallibleTArray<uint8_t> & GetDataAsTArray(void) override { return _to GetDataAsTArray(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUDPMESSAGE(_to) \
  NS_IMETHOD GetFromAddr(nsINetAddr **aFromAddr) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFromAddr(aFromAddr); } \
  NS_IMETHOD GetData(nsACString& aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetData(aData); } \
  NS_IMETHOD GetOutputStream(nsIOutputStream **aOutputStream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOutputStream(aOutputStream); } \
  NS_IMETHOD GetRawData(JSContext* cx, JS::MutableHandleValue aRawData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRawData(cx, aRawData); } \
  virtual FallibleTArray<uint8_t> & GetDataAsTArray(void) override; 


#endif /* __gen_nsIUDPSocket_h__ */
