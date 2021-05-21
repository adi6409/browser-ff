/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIServerSocket.idl
 */

#ifndef __gen_nsIServerSocket_h__
#define __gen_nsIServerSocket_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */

class nsIServerSocketListener; /* forward declaration */

class nsISocketTransport; /* forward declaration */

typedef uint32_t  nsServerSocketFlag;


/* starting interface:    nsIServerSocket */
#define NS_ISERVERSOCKET_IID_STR "7a9c39cb-a13f-4eef-9bdf-a74301628742"

#define NS_ISERVERSOCKET_IID \
  {0x7a9c39cb, 0xa13f, 0x4eef, \
    { 0x9b, 0xdf, 0xa7, 0x43, 0x01, 0x62, 0x87, 0x42 }}

class NS_NO_VTABLE nsIServerSocket : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISERVERSOCKET_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIServerSocket;

  enum {
    LoopbackOnly = 1U,
    KeepWhenOffline = 2U
  };

  /* void init (in long aPort, in boolean aLoopbackOnly, in long aBackLog); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(int32_t aPort, bool aLoopbackOnly, int32_t aBackLog) = 0;

  /* void initIPv6 (in long aPort, in boolean aLoopbackOnly, in long aBackLog); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitIPv6(int32_t aPort, bool aLoopbackOnly, int32_t aBackLog) = 0;

  /* void initSpecialConnection (in long aPort, in nsServerSocketFlag aFlags, in long aBackLog); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitSpecialConnection(int32_t aPort, nsServerSocketFlag aFlags, int32_t aBackLog) = 0;

  /* [noscript] void initWithAddress ([const] in PRNetAddrPtr aAddr, in long aBackLog); */
  NS_IMETHOD InitWithAddress(const union PRNetAddr * aAddr, int32_t aBackLog) = 0;

  /* void initWithFilename (in nsIFile aPath, in unsigned long aPermissions, in long aBacklog); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitWithFilename(nsIFile *aPath, uint32_t aPermissions, int32_t aBacklog) = 0;

  /* void initWithAbstractAddress (in AUTF8String aName, in long aBacklog); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitWithAbstractAddress(const nsACString& aName, int32_t aBacklog) = 0;

  /* void close (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Close(void) = 0;

  /* void asyncListen (in nsIServerSocketListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncListen(nsIServerSocketListener *aListener) = 0;

  /* readonly attribute long port; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPort(int32_t *aPort) = 0;

  /* [noscript] PRNetAddr getAddress (); */
  NS_IMETHOD GetAddress(union PRNetAddr * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIServerSocket, NS_ISERVERSOCKET_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISERVERSOCKET \
  NS_IMETHOD Init(int32_t aPort, bool aLoopbackOnly, int32_t aBackLog) override; \
  NS_IMETHOD InitIPv6(int32_t aPort, bool aLoopbackOnly, int32_t aBackLog) override; \
  NS_IMETHOD InitSpecialConnection(int32_t aPort, nsServerSocketFlag aFlags, int32_t aBackLog) override; \
  NS_IMETHOD InitWithAddress(const union PRNetAddr * aAddr, int32_t aBackLog) override; \
  NS_IMETHOD InitWithFilename(nsIFile *aPath, uint32_t aPermissions, int32_t aBacklog) override; \
  NS_IMETHOD InitWithAbstractAddress(const nsACString& aName, int32_t aBacklog) override; \
  NS_IMETHOD Close(void) override; \
  NS_IMETHOD AsyncListen(nsIServerSocketListener *aListener) override; \
  NS_IMETHOD GetPort(int32_t *aPort) override; \
  NS_IMETHOD GetAddress(union PRNetAddr * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISERVERSOCKET \
  nsresult Init(int32_t aPort, bool aLoopbackOnly, int32_t aBackLog); \
  nsresult InitIPv6(int32_t aPort, bool aLoopbackOnly, int32_t aBackLog); \
  nsresult InitSpecialConnection(int32_t aPort, nsServerSocketFlag aFlags, int32_t aBackLog); \
  nsresult InitWithAddress(const union PRNetAddr * aAddr, int32_t aBackLog); \
  nsresult InitWithFilename(nsIFile *aPath, uint32_t aPermissions, int32_t aBacklog); \
  nsresult InitWithAbstractAddress(const nsACString& aName, int32_t aBacklog); \
  nsresult Close(void); \
  nsresult AsyncListen(nsIServerSocketListener *aListener); \
  nsresult GetPort(int32_t *aPort); \
  nsresult GetAddress(union PRNetAddr * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISERVERSOCKET(_to) \
  NS_IMETHOD Init(int32_t aPort, bool aLoopbackOnly, int32_t aBackLog) override { return _to Init(aPort, aLoopbackOnly, aBackLog); } \
  NS_IMETHOD InitIPv6(int32_t aPort, bool aLoopbackOnly, int32_t aBackLog) override { return _to InitIPv6(aPort, aLoopbackOnly, aBackLog); } \
  NS_IMETHOD InitSpecialConnection(int32_t aPort, nsServerSocketFlag aFlags, int32_t aBackLog) override { return _to InitSpecialConnection(aPort, aFlags, aBackLog); } \
  NS_IMETHOD InitWithAddress(const union PRNetAddr * aAddr, int32_t aBackLog) override { return _to InitWithAddress(aAddr, aBackLog); } \
  NS_IMETHOD InitWithFilename(nsIFile *aPath, uint32_t aPermissions, int32_t aBacklog) override { return _to InitWithFilename(aPath, aPermissions, aBacklog); } \
  NS_IMETHOD InitWithAbstractAddress(const nsACString& aName, int32_t aBacklog) override { return _to InitWithAbstractAddress(aName, aBacklog); } \
  NS_IMETHOD Close(void) override { return _to Close(); } \
  NS_IMETHOD AsyncListen(nsIServerSocketListener *aListener) override { return _to AsyncListen(aListener); } \
  NS_IMETHOD GetPort(int32_t *aPort) override { return _to GetPort(aPort); } \
  NS_IMETHOD GetAddress(union PRNetAddr * _retval) override { return _to GetAddress(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISERVERSOCKET(_to) \
  NS_IMETHOD Init(int32_t aPort, bool aLoopbackOnly, int32_t aBackLog) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aPort, aLoopbackOnly, aBackLog); } \
  NS_IMETHOD InitIPv6(int32_t aPort, bool aLoopbackOnly, int32_t aBackLog) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitIPv6(aPort, aLoopbackOnly, aBackLog); } \
  NS_IMETHOD InitSpecialConnection(int32_t aPort, nsServerSocketFlag aFlags, int32_t aBackLog) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitSpecialConnection(aPort, aFlags, aBackLog); } \
  NS_IMETHOD InitWithAddress(const union PRNetAddr * aAddr, int32_t aBackLog) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithAddress(aAddr, aBackLog); } \
  NS_IMETHOD InitWithFilename(nsIFile *aPath, uint32_t aPermissions, int32_t aBacklog) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithFilename(aPath, aPermissions, aBacklog); } \
  NS_IMETHOD InitWithAbstractAddress(const nsACString& aName, int32_t aBacklog) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithAbstractAddress(aName, aBacklog); } \
  NS_IMETHOD Close(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(); } \
  NS_IMETHOD AsyncListen(nsIServerSocketListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncListen(aListener); } \
  NS_IMETHOD GetPort(int32_t *aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPort(aPort); } \
  NS_IMETHOD GetAddress(union PRNetAddr * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAddress(_retval); } 


/* starting interface:    nsIServerSocketListener */
#define NS_ISERVERSOCKETLISTENER_IID_STR "836d98ec-fee2-4bde-b609-abd5e966eabd"

#define NS_ISERVERSOCKETLISTENER_IID \
  {0x836d98ec, 0xfee2, 0x4bde, \
    { 0xb6, 0x09, 0xab, 0xd5, 0xe9, 0x66, 0xea, 0xbd }}

class NS_NO_VTABLE nsIServerSocketListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISERVERSOCKETLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIServerSocketListener;

  /* void onSocketAccepted (in nsIServerSocket aServ, in nsISocketTransport aTransport); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnSocketAccepted(nsIServerSocket *aServ, nsISocketTransport *aTransport) = 0;

  /* void onStopListening (in nsIServerSocket aServ, in nsresult aStatus); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnStopListening(nsIServerSocket *aServ, nsresult aStatus) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIServerSocketListener, NS_ISERVERSOCKETLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISERVERSOCKETLISTENER \
  NS_IMETHOD OnSocketAccepted(nsIServerSocket *aServ, nsISocketTransport *aTransport) override; \
  NS_IMETHOD OnStopListening(nsIServerSocket *aServ, nsresult aStatus) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISERVERSOCKETLISTENER \
  nsresult OnSocketAccepted(nsIServerSocket *aServ, nsISocketTransport *aTransport); \
  nsresult OnStopListening(nsIServerSocket *aServ, nsresult aStatus); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISERVERSOCKETLISTENER(_to) \
  NS_IMETHOD OnSocketAccepted(nsIServerSocket *aServ, nsISocketTransport *aTransport) override { return _to OnSocketAccepted(aServ, aTransport); } \
  NS_IMETHOD OnStopListening(nsIServerSocket *aServ, nsresult aStatus) override { return _to OnStopListening(aServ, aStatus); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISERVERSOCKETLISTENER(_to) \
  NS_IMETHOD OnSocketAccepted(nsIServerSocket *aServ, nsISocketTransport *aTransport) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnSocketAccepted(aServ, aTransport); } \
  NS_IMETHOD OnStopListening(nsIServerSocket *aServ, nsresult aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnStopListening(aServ, aStatus); } 


#endif /* __gen_nsIServerSocket_h__ */
