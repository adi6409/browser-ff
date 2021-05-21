/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationControlChannel.idl
 */

#ifndef __gen_nsIPresentationControlChannel_h__
#define __gen_nsIPresentationControlChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */

class nsIInputStream; /* forward declaration */


/* starting interface:    nsIPresentationChannelDescription */
#define NS_IPRESENTATIONCHANNELDESCRIPTION_IID_STR "ae318e05-2a4e-4f85-95c0-e8b191ad812c"

#define NS_IPRESENTATIONCHANNELDESCRIPTION_IID \
  {0xae318e05, 0x2a4e, 0x4f85, \
    { 0x95, 0xc0, 0xe8, 0xb1, 0x91, 0xad, 0x81, 0x2c }}

class NS_NO_VTABLE nsIPresentationChannelDescription : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONCHANNELDESCRIPTION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationChannelDescription;

  enum {
    TYPE_TCP = 1U,
    TYPE_DATACHANNEL = 2U
  };

  /* readonly attribute uint8_t type; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetType(uint8_t *aType) = 0;

  /* readonly attribute nsIArray tcpAddress; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTcpAddress(nsIArray **aTcpAddress) = 0;

  /* readonly attribute uint16_t tcpPort; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTcpPort(uint16_t *aTcpPort) = 0;

  /* readonly attribute AString dataChannelSDP; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDataChannelSDP(nsAString& aDataChannelSDP) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationChannelDescription, NS_IPRESENTATIONCHANNELDESCRIPTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONCHANNELDESCRIPTION \
  NS_IMETHOD GetType(uint8_t *aType) override; \
  NS_IMETHOD GetTcpAddress(nsIArray **aTcpAddress) override; \
  NS_IMETHOD GetTcpPort(uint16_t *aTcpPort) override; \
  NS_IMETHOD GetDataChannelSDP(nsAString& aDataChannelSDP) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONCHANNELDESCRIPTION \
  nsresult GetType(uint8_t *aType); \
  nsresult GetTcpAddress(nsIArray **aTcpAddress); \
  nsresult GetTcpPort(uint16_t *aTcpPort); \
  nsresult GetDataChannelSDP(nsAString& aDataChannelSDP); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONCHANNELDESCRIPTION(_to) \
  NS_IMETHOD GetType(uint8_t *aType) override { return _to GetType(aType); } \
  NS_IMETHOD GetTcpAddress(nsIArray **aTcpAddress) override { return _to GetTcpAddress(aTcpAddress); } \
  NS_IMETHOD GetTcpPort(uint16_t *aTcpPort) override { return _to GetTcpPort(aTcpPort); } \
  NS_IMETHOD GetDataChannelSDP(nsAString& aDataChannelSDP) override { return _to GetDataChannelSDP(aDataChannelSDP); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONCHANNELDESCRIPTION(_to) \
  NS_IMETHOD GetType(uint8_t *aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD GetTcpAddress(nsIArray **aTcpAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTcpAddress(aTcpAddress); } \
  NS_IMETHOD GetTcpPort(uint16_t *aTcpPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTcpPort(aTcpPort); } \
  NS_IMETHOD GetDataChannelSDP(nsAString& aDataChannelSDP) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDataChannelSDP(aDataChannelSDP); } 


/* starting interface:    nsIPresentationControlChannelListener */
#define NS_IPRESENTATIONCONTROLCHANNELLISTENER_IID_STR "96dd548f-7d0f-43c1-b1ad-28e666cf1e82"

#define NS_IPRESENTATIONCONTROLCHANNELLISTENER_IID \
  {0x96dd548f, 0x7d0f, 0x43c1, \
    { 0xb1, 0xad, 0x28, 0xe6, 0x66, 0xcf, 0x1e, 0x82 }}

class NS_NO_VTABLE nsIPresentationControlChannelListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONCONTROLCHANNELLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationControlChannelListener;

  /* void onOffer (in nsIPresentationChannelDescription offer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnOffer(nsIPresentationChannelDescription *offer) = 0;

  /* void onAnswer (in nsIPresentationChannelDescription answer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnAnswer(nsIPresentationChannelDescription *answer) = 0;

  /* void onIceCandidate (in AString candidate); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnIceCandidate(const nsAString& candidate) = 0;

  /* void notifyConnected (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyConnected(void) = 0;

  /* void notifyDisconnected (in nsresult reason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyDisconnected(nsresult reason) = 0;

  /* void notifyReconnected (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyReconnected(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationControlChannelListener, NS_IPRESENTATIONCONTROLCHANNELLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONCONTROLCHANNELLISTENER \
  NS_IMETHOD OnOffer(nsIPresentationChannelDescription *offer) override; \
  NS_IMETHOD OnAnswer(nsIPresentationChannelDescription *answer) override; \
  NS_IMETHOD OnIceCandidate(const nsAString& candidate) override; \
  NS_IMETHOD NotifyConnected(void) override; \
  NS_IMETHOD NotifyDisconnected(nsresult reason) override; \
  NS_IMETHOD NotifyReconnected(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONCONTROLCHANNELLISTENER \
  nsresult OnOffer(nsIPresentationChannelDescription *offer); \
  nsresult OnAnswer(nsIPresentationChannelDescription *answer); \
  nsresult OnIceCandidate(const nsAString& candidate); \
  nsresult NotifyConnected(void); \
  nsresult NotifyDisconnected(nsresult reason); \
  nsresult NotifyReconnected(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONCONTROLCHANNELLISTENER(_to) \
  NS_IMETHOD OnOffer(nsIPresentationChannelDescription *offer) override { return _to OnOffer(offer); } \
  NS_IMETHOD OnAnswer(nsIPresentationChannelDescription *answer) override { return _to OnAnswer(answer); } \
  NS_IMETHOD OnIceCandidate(const nsAString& candidate) override { return _to OnIceCandidate(candidate); } \
  NS_IMETHOD NotifyConnected(void) override { return _to NotifyConnected(); } \
  NS_IMETHOD NotifyDisconnected(nsresult reason) override { return _to NotifyDisconnected(reason); } \
  NS_IMETHOD NotifyReconnected(void) override { return _to NotifyReconnected(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONCONTROLCHANNELLISTENER(_to) \
  NS_IMETHOD OnOffer(nsIPresentationChannelDescription *offer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnOffer(offer); } \
  NS_IMETHOD OnAnswer(nsIPresentationChannelDescription *answer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnAnswer(answer); } \
  NS_IMETHOD OnIceCandidate(const nsAString& candidate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnIceCandidate(candidate); } \
  NS_IMETHOD NotifyConnected(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyConnected(); } \
  NS_IMETHOD NotifyDisconnected(nsresult reason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyDisconnected(reason); } \
  NS_IMETHOD NotifyReconnected(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyReconnected(); } 


/* starting interface:    nsIPresentationControlChannel */
#define NS_IPRESENTATIONCONTROLCHANNEL_IID_STR "e60e208c-a9f5-4bc6-9a3e-47f3e4ae9c57"

#define NS_IPRESENTATIONCONTROLCHANNEL_IID \
  {0xe60e208c, 0xa9f5, 0x4bc6, \
    { 0x9a, 0x3e, 0x47, 0xf3, 0xe4, 0xae, 0x9c, 0x57 }}

class NS_NO_VTABLE nsIPresentationControlChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONCONTROLCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationControlChannel;

  /* attribute nsIPresentationControlChannelListener listener; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetListener(nsIPresentationControlChannelListener **aListener) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetListener(nsIPresentationControlChannelListener *aListener) = 0;

  /* void sendOffer (in nsIPresentationChannelDescription offer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SendOffer(nsIPresentationChannelDescription *offer) = 0;

  /* void sendAnswer (in nsIPresentationChannelDescription answer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SendAnswer(nsIPresentationChannelDescription *answer) = 0;

  /* void sendIceCandidate (in AString candidate); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SendIceCandidate(const nsAString& candidate) = 0;

  /* void launch (in AString presentationId, in AString url); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Launch(const nsAString& presentationId, const nsAString& url) = 0;

  /* void terminate (in AString presentationId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Terminate(const nsAString& presentationId) = 0;

  /* void disconnect (in nsresult reason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Disconnect(nsresult reason) = 0;

  /* void reconnect (in AString presentationId, in AString url); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Reconnect(const nsAString& presentationId, const nsAString& url) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationControlChannel, NS_IPRESENTATIONCONTROLCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONCONTROLCHANNEL \
  NS_IMETHOD GetListener(nsIPresentationControlChannelListener **aListener) override; \
  NS_IMETHOD SetListener(nsIPresentationControlChannelListener *aListener) override; \
  NS_IMETHOD SendOffer(nsIPresentationChannelDescription *offer) override; \
  NS_IMETHOD SendAnswer(nsIPresentationChannelDescription *answer) override; \
  NS_IMETHOD SendIceCandidate(const nsAString& candidate) override; \
  NS_IMETHOD Launch(const nsAString& presentationId, const nsAString& url) override; \
  NS_IMETHOD Terminate(const nsAString& presentationId) override; \
  NS_IMETHOD Disconnect(nsresult reason) override; \
  NS_IMETHOD Reconnect(const nsAString& presentationId, const nsAString& url) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONCONTROLCHANNEL \
  nsresult GetListener(nsIPresentationControlChannelListener **aListener); \
  nsresult SetListener(nsIPresentationControlChannelListener *aListener); \
  nsresult SendOffer(nsIPresentationChannelDescription *offer); \
  nsresult SendAnswer(nsIPresentationChannelDescription *answer); \
  nsresult SendIceCandidate(const nsAString& candidate); \
  nsresult Launch(const nsAString& presentationId, const nsAString& url); \
  nsresult Terminate(const nsAString& presentationId); \
  nsresult Disconnect(nsresult reason); \
  nsresult Reconnect(const nsAString& presentationId, const nsAString& url); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONCONTROLCHANNEL(_to) \
  NS_IMETHOD GetListener(nsIPresentationControlChannelListener **aListener) override { return _to GetListener(aListener); } \
  NS_IMETHOD SetListener(nsIPresentationControlChannelListener *aListener) override { return _to SetListener(aListener); } \
  NS_IMETHOD SendOffer(nsIPresentationChannelDescription *offer) override { return _to SendOffer(offer); } \
  NS_IMETHOD SendAnswer(nsIPresentationChannelDescription *answer) override { return _to SendAnswer(answer); } \
  NS_IMETHOD SendIceCandidate(const nsAString& candidate) override { return _to SendIceCandidate(candidate); } \
  NS_IMETHOD Launch(const nsAString& presentationId, const nsAString& url) override { return _to Launch(presentationId, url); } \
  NS_IMETHOD Terminate(const nsAString& presentationId) override { return _to Terminate(presentationId); } \
  NS_IMETHOD Disconnect(nsresult reason) override { return _to Disconnect(reason); } \
  NS_IMETHOD Reconnect(const nsAString& presentationId, const nsAString& url) override { return _to Reconnect(presentationId, url); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONCONTROLCHANNEL(_to) \
  NS_IMETHOD GetListener(nsIPresentationControlChannelListener **aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetListener(aListener); } \
  NS_IMETHOD SetListener(nsIPresentationControlChannelListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetListener(aListener); } \
  NS_IMETHOD SendOffer(nsIPresentationChannelDescription *offer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendOffer(offer); } \
  NS_IMETHOD SendAnswer(nsIPresentationChannelDescription *answer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendAnswer(answer); } \
  NS_IMETHOD SendIceCandidate(const nsAString& candidate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendIceCandidate(candidate); } \
  NS_IMETHOD Launch(const nsAString& presentationId, const nsAString& url) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Launch(presentationId, url); } \
  NS_IMETHOD Terminate(const nsAString& presentationId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Terminate(presentationId); } \
  NS_IMETHOD Disconnect(nsresult reason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Disconnect(reason); } \
  NS_IMETHOD Reconnect(const nsAString& presentationId, const nsAString& url) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reconnect(presentationId, url); } 


#endif /* __gen_nsIPresentationControlChannel_h__ */
