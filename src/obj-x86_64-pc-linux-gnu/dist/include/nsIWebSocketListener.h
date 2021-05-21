/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/websocket/nsIWebSocketListener.idl
 */

#ifndef __gen_nsIWebSocketListener_h__
#define __gen_nsIWebSocketListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIWebSocketListener */
#define NS_IWEBSOCKETLISTENER_IID_STR "d74c96b2-65b3-4e39-9e39-c577de5d7a73"

#define NS_IWEBSOCKETLISTENER_IID \
  {0xd74c96b2, 0x65b3, 0x4e39, \
    { 0x9e, 0x39, 0xc5, 0x77, 0xde, 0x5d, 0x7a, 0x73 }}

class NS_NO_VTABLE nsIWebSocketListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBSOCKETLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebSocketListener;

  /* [must_use] void onStart (in nsISupports aContext); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD OnStart(nsISupports *aContext) = 0;

  /* [must_use] void onStop (in nsISupports aContext, in nsresult aStatusCode); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD OnStop(nsISupports *aContext, nsresult aStatusCode) = 0;

  /* [must_use] void onMessageAvailable (in nsISupports aContext, in AUTF8String aMsg); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD OnMessageAvailable(nsISupports *aContext, const nsACString& aMsg) = 0;

  /* [must_use] void onBinaryMessageAvailable (in nsISupports aContext, in ACString aMsg); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD OnBinaryMessageAvailable(nsISupports *aContext, const nsACString& aMsg) = 0;

  /* [must_use] void onAcknowledge (in nsISupports aContext, in uint32_t aSize); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD OnAcknowledge(nsISupports *aContext, uint32_t aSize) = 0;

  /* [must_use] void onServerClose (in nsISupports aContext, in unsigned short aCode, in AUTF8String aReason); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD OnServerClose(nsISupports *aContext, uint16_t aCode, const nsACString& aReason) = 0;

  /* [must_use] void OnError (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD OnError(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebSocketListener, NS_IWEBSOCKETLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBSOCKETLISTENER \
  [[nodiscard]] NS_IMETHOD OnStart(nsISupports *aContext) override; \
  [[nodiscard]] NS_IMETHOD OnStop(nsISupports *aContext, nsresult aStatusCode) override; \
  [[nodiscard]] NS_IMETHOD OnMessageAvailable(nsISupports *aContext, const nsACString& aMsg) override; \
  [[nodiscard]] NS_IMETHOD OnBinaryMessageAvailable(nsISupports *aContext, const nsACString& aMsg) override; \
  [[nodiscard]] NS_IMETHOD OnAcknowledge(nsISupports *aContext, uint32_t aSize) override; \
  [[nodiscard]] NS_IMETHOD OnServerClose(nsISupports *aContext, uint16_t aCode, const nsACString& aReason) override; \
  [[nodiscard]] NS_IMETHOD OnError(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBSOCKETLISTENER \
  [[nodiscard]] nsresult OnStart(nsISupports *aContext); \
  [[nodiscard]] nsresult OnStop(nsISupports *aContext, nsresult aStatusCode); \
  [[nodiscard]] nsresult OnMessageAvailable(nsISupports *aContext, const nsACString& aMsg); \
  [[nodiscard]] nsresult OnBinaryMessageAvailable(nsISupports *aContext, const nsACString& aMsg); \
  [[nodiscard]] nsresult OnAcknowledge(nsISupports *aContext, uint32_t aSize); \
  [[nodiscard]] nsresult OnServerClose(nsISupports *aContext, uint16_t aCode, const nsACString& aReason); \
  [[nodiscard]] nsresult OnError(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBSOCKETLISTENER(_to) \
  [[nodiscard]] NS_IMETHOD OnStart(nsISupports *aContext) override { return _to OnStart(aContext); } \
  [[nodiscard]] NS_IMETHOD OnStop(nsISupports *aContext, nsresult aStatusCode) override { return _to OnStop(aContext, aStatusCode); } \
  [[nodiscard]] NS_IMETHOD OnMessageAvailable(nsISupports *aContext, const nsACString& aMsg) override { return _to OnMessageAvailable(aContext, aMsg); } \
  [[nodiscard]] NS_IMETHOD OnBinaryMessageAvailable(nsISupports *aContext, const nsACString& aMsg) override { return _to OnBinaryMessageAvailable(aContext, aMsg); } \
  [[nodiscard]] NS_IMETHOD OnAcknowledge(nsISupports *aContext, uint32_t aSize) override { return _to OnAcknowledge(aContext, aSize); } \
  [[nodiscard]] NS_IMETHOD OnServerClose(nsISupports *aContext, uint16_t aCode, const nsACString& aReason) override { return _to OnServerClose(aContext, aCode, aReason); } \
  [[nodiscard]] NS_IMETHOD OnError(void) override { return _to OnError(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBSOCKETLISTENER(_to) \
  [[nodiscard]] NS_IMETHOD OnStart(nsISupports *aContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnStart(aContext); } \
  [[nodiscard]] NS_IMETHOD OnStop(nsISupports *aContext, nsresult aStatusCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnStop(aContext, aStatusCode); } \
  [[nodiscard]] NS_IMETHOD OnMessageAvailable(nsISupports *aContext, const nsACString& aMsg) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnMessageAvailable(aContext, aMsg); } \
  [[nodiscard]] NS_IMETHOD OnBinaryMessageAvailable(nsISupports *aContext, const nsACString& aMsg) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnBinaryMessageAvailable(aContext, aMsg); } \
  [[nodiscard]] NS_IMETHOD OnAcknowledge(nsISupports *aContext, uint32_t aSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnAcknowledge(aContext, aSize); } \
  [[nodiscard]] NS_IMETHOD OnServerClose(nsISupports *aContext, uint16_t aCode, const nsACString& aReason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnServerClose(aContext, aCode, aReason); } \
  [[nodiscard]] NS_IMETHOD OnError(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnError(); } 


#endif /* __gen_nsIWebSocketListener_h__ */
