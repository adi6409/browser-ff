/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/websocket/nsIWebSocketEventService.idl
 */

#ifndef __gen_nsIWebSocketEventService_h__
#define __gen_nsIWebSocketEventService_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIWeakReference; /* forward declaration */


/* starting interface:    nsIWebSocketFrame */
#define NS_IWEBSOCKETFRAME_IID_STR "6714a6be-2265-4f73-a988-d78a12416037"

#define NS_IWEBSOCKETFRAME_IID \
  {0x6714a6be, 0x2265, 0x4f73, \
    { 0xa9, 0x88, 0xd7, 0x8a, 0x12, 0x41, 0x60, 0x37 }}

class NS_NO_VTABLE nsIWebSocketFrame : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBSOCKETFRAME_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebSocketFrame;

  /* [must_use] readonly attribute DOMHighResTimeStamp timeStamp; */
  [[nodiscard]] NS_IMETHOD GetTimeStamp(DOMHighResTimeStamp *aTimeStamp) = 0;

  /* [must_use] readonly attribute boolean finBit; */
  [[nodiscard]] NS_IMETHOD GetFinBit(bool *aFinBit) = 0;

  /* [must_use] readonly attribute boolean rsvBit1; */
  [[nodiscard]] NS_IMETHOD GetRsvBit1(bool *aRsvBit1) = 0;

  /* [must_use] readonly attribute boolean rsvBit2; */
  [[nodiscard]] NS_IMETHOD GetRsvBit2(bool *aRsvBit2) = 0;

  /* [must_use] readonly attribute boolean rsvBit3; */
  [[nodiscard]] NS_IMETHOD GetRsvBit3(bool *aRsvBit3) = 0;

  /* [must_use] readonly attribute unsigned short opCode; */
  [[nodiscard]] NS_IMETHOD GetOpCode(uint16_t *aOpCode) = 0;

  /* [must_use] readonly attribute boolean maskBit; */
  [[nodiscard]] NS_IMETHOD GetMaskBit(bool *aMaskBit) = 0;

  /* [must_use] readonly attribute unsigned long mask; */
  [[nodiscard]] NS_IMETHOD GetMask(uint32_t *aMask) = 0;

  /* [must_use] readonly attribute ACString payload; */
  [[nodiscard]] NS_IMETHOD GetPayload(nsACString& aPayload) = 0;

  enum {
    OPCODE_CONTINUATION = 0U,
    OPCODE_TEXT = 1U,
    OPCODE_BINARY = 2U,
    OPCODE_CLOSE = 8U,
    OPCODE_PING = 9U,
    OPCODE_PONG = 10U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebSocketFrame, NS_IWEBSOCKETFRAME_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBSOCKETFRAME \
  [[nodiscard]] NS_IMETHOD GetTimeStamp(DOMHighResTimeStamp *aTimeStamp) override; \
  [[nodiscard]] NS_IMETHOD GetFinBit(bool *aFinBit) override; \
  [[nodiscard]] NS_IMETHOD GetRsvBit1(bool *aRsvBit1) override; \
  [[nodiscard]] NS_IMETHOD GetRsvBit2(bool *aRsvBit2) override; \
  [[nodiscard]] NS_IMETHOD GetRsvBit3(bool *aRsvBit3) override; \
  [[nodiscard]] NS_IMETHOD GetOpCode(uint16_t *aOpCode) override; \
  [[nodiscard]] NS_IMETHOD GetMaskBit(bool *aMaskBit) override; \
  [[nodiscard]] NS_IMETHOD GetMask(uint32_t *aMask) override; \
  [[nodiscard]] NS_IMETHOD GetPayload(nsACString& aPayload) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBSOCKETFRAME \
  [[nodiscard]] nsresult GetTimeStamp(DOMHighResTimeStamp *aTimeStamp); \
  [[nodiscard]] nsresult GetFinBit(bool *aFinBit); \
  [[nodiscard]] nsresult GetRsvBit1(bool *aRsvBit1); \
  [[nodiscard]] nsresult GetRsvBit2(bool *aRsvBit2); \
  [[nodiscard]] nsresult GetRsvBit3(bool *aRsvBit3); \
  [[nodiscard]] nsresult GetOpCode(uint16_t *aOpCode); \
  [[nodiscard]] nsresult GetMaskBit(bool *aMaskBit); \
  [[nodiscard]] nsresult GetMask(uint32_t *aMask); \
  [[nodiscard]] nsresult GetPayload(nsACString& aPayload); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBSOCKETFRAME(_to) \
  [[nodiscard]] NS_IMETHOD GetTimeStamp(DOMHighResTimeStamp *aTimeStamp) override { return _to GetTimeStamp(aTimeStamp); } \
  [[nodiscard]] NS_IMETHOD GetFinBit(bool *aFinBit) override { return _to GetFinBit(aFinBit); } \
  [[nodiscard]] NS_IMETHOD GetRsvBit1(bool *aRsvBit1) override { return _to GetRsvBit1(aRsvBit1); } \
  [[nodiscard]] NS_IMETHOD GetRsvBit2(bool *aRsvBit2) override { return _to GetRsvBit2(aRsvBit2); } \
  [[nodiscard]] NS_IMETHOD GetRsvBit3(bool *aRsvBit3) override { return _to GetRsvBit3(aRsvBit3); } \
  [[nodiscard]] NS_IMETHOD GetOpCode(uint16_t *aOpCode) override { return _to GetOpCode(aOpCode); } \
  [[nodiscard]] NS_IMETHOD GetMaskBit(bool *aMaskBit) override { return _to GetMaskBit(aMaskBit); } \
  [[nodiscard]] NS_IMETHOD GetMask(uint32_t *aMask) override { return _to GetMask(aMask); } \
  [[nodiscard]] NS_IMETHOD GetPayload(nsACString& aPayload) override { return _to GetPayload(aPayload); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBSOCKETFRAME(_to) \
  [[nodiscard]] NS_IMETHOD GetTimeStamp(DOMHighResTimeStamp *aTimeStamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTimeStamp(aTimeStamp); } \
  [[nodiscard]] NS_IMETHOD GetFinBit(bool *aFinBit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFinBit(aFinBit); } \
  [[nodiscard]] NS_IMETHOD GetRsvBit1(bool *aRsvBit1) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRsvBit1(aRsvBit1); } \
  [[nodiscard]] NS_IMETHOD GetRsvBit2(bool *aRsvBit2) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRsvBit2(aRsvBit2); } \
  [[nodiscard]] NS_IMETHOD GetRsvBit3(bool *aRsvBit3) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRsvBit3(aRsvBit3); } \
  [[nodiscard]] NS_IMETHOD GetOpCode(uint16_t *aOpCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOpCode(aOpCode); } \
  [[nodiscard]] NS_IMETHOD GetMaskBit(bool *aMaskBit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMaskBit(aMaskBit); } \
  [[nodiscard]] NS_IMETHOD GetMask(uint32_t *aMask) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMask(aMask); } \
  [[nodiscard]] NS_IMETHOD GetPayload(nsACString& aPayload) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPayload(aPayload); } \


/* starting interface:    nsIWebSocketEventListener */
#define NS_IWEBSOCKETEVENTLISTENER_IID_STR "e7c005ab-e694-489b-b741-96db43ffb16f"

#define NS_IWEBSOCKETEVENTLISTENER_IID \
  {0xe7c005ab, 0xe694, 0x489b, \
    { 0xb7, 0x41, 0x96, 0xdb, 0x43, 0xff, 0xb1, 0x6f }}

class NS_NO_VTABLE nsIWebSocketEventListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBSOCKETEVENTLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebSocketEventListener;

  /* [must_use] void webSocketCreated (in unsigned long aWebSocketSerialID, in AString aURI, in ACString aProtocols); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD WebSocketCreated(uint32_t aWebSocketSerialID, const nsAString& aURI, const nsACString& aProtocols) = 0;

  /* [must_use] void webSocketOpened (in unsigned long aWebSocketSerialID, in AString aEffectiveURI, in ACString aProtocols, in ACString aExtensions, in uint64_t aHttpChannelId); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD WebSocketOpened(uint32_t aWebSocketSerialID, const nsAString& aEffectiveURI, const nsACString& aProtocols, const nsACString& aExtensions, uint64_t aHttpChannelId) = 0;

  enum {
    TYPE_STRING = 0U,
    TYPE_BLOB = 1U,
    TYPE_ARRAYBUFFER = 2U
  };

  /* [must_use] void webSocketMessageAvailable (in unsigned long aWebSocketSerialID, in ACString aMessage, in unsigned short aType); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD WebSocketMessageAvailable(uint32_t aWebSocketSerialID, const nsACString& aMessage, uint16_t aType) = 0;

  /* [must_use] void webSocketClosed (in unsigned long aWebSocketSerialID, in boolean aWasClean, in unsigned short aCode, in AString aReason); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD WebSocketClosed(uint32_t aWebSocketSerialID, bool aWasClean, uint16_t aCode, const nsAString& aReason) = 0;

  /* [must_use] void frameReceived (in unsigned long aWebSocketSerialID, in nsIWebSocketFrame aFrame); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD FrameReceived(uint32_t aWebSocketSerialID, nsIWebSocketFrame *aFrame) = 0;

  /* [must_use] void frameSent (in unsigned long aWebSocketSerialID, in nsIWebSocketFrame aFrame); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD FrameSent(uint32_t aWebSocketSerialID, nsIWebSocketFrame *aFrame) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebSocketEventListener, NS_IWEBSOCKETEVENTLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBSOCKETEVENTLISTENER \
  [[nodiscard]] NS_IMETHOD WebSocketCreated(uint32_t aWebSocketSerialID, const nsAString& aURI, const nsACString& aProtocols) override; \
  [[nodiscard]] NS_IMETHOD WebSocketOpened(uint32_t aWebSocketSerialID, const nsAString& aEffectiveURI, const nsACString& aProtocols, const nsACString& aExtensions, uint64_t aHttpChannelId) override; \
  [[nodiscard]] NS_IMETHOD WebSocketMessageAvailable(uint32_t aWebSocketSerialID, const nsACString& aMessage, uint16_t aType) override; \
  [[nodiscard]] NS_IMETHOD WebSocketClosed(uint32_t aWebSocketSerialID, bool aWasClean, uint16_t aCode, const nsAString& aReason) override; \
  [[nodiscard]] NS_IMETHOD FrameReceived(uint32_t aWebSocketSerialID, nsIWebSocketFrame *aFrame) override; \
  [[nodiscard]] NS_IMETHOD FrameSent(uint32_t aWebSocketSerialID, nsIWebSocketFrame *aFrame) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBSOCKETEVENTLISTENER \
  [[nodiscard]] nsresult WebSocketCreated(uint32_t aWebSocketSerialID, const nsAString& aURI, const nsACString& aProtocols); \
  [[nodiscard]] nsresult WebSocketOpened(uint32_t aWebSocketSerialID, const nsAString& aEffectiveURI, const nsACString& aProtocols, const nsACString& aExtensions, uint64_t aHttpChannelId); \
  [[nodiscard]] nsresult WebSocketMessageAvailable(uint32_t aWebSocketSerialID, const nsACString& aMessage, uint16_t aType); \
  [[nodiscard]] nsresult WebSocketClosed(uint32_t aWebSocketSerialID, bool aWasClean, uint16_t aCode, const nsAString& aReason); \
  [[nodiscard]] nsresult FrameReceived(uint32_t aWebSocketSerialID, nsIWebSocketFrame *aFrame); \
  [[nodiscard]] nsresult FrameSent(uint32_t aWebSocketSerialID, nsIWebSocketFrame *aFrame); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBSOCKETEVENTLISTENER(_to) \
  [[nodiscard]] NS_IMETHOD WebSocketCreated(uint32_t aWebSocketSerialID, const nsAString& aURI, const nsACString& aProtocols) override { return _to WebSocketCreated(aWebSocketSerialID, aURI, aProtocols); } \
  [[nodiscard]] NS_IMETHOD WebSocketOpened(uint32_t aWebSocketSerialID, const nsAString& aEffectiveURI, const nsACString& aProtocols, const nsACString& aExtensions, uint64_t aHttpChannelId) override { return _to WebSocketOpened(aWebSocketSerialID, aEffectiveURI, aProtocols, aExtensions, aHttpChannelId); } \
  [[nodiscard]] NS_IMETHOD WebSocketMessageAvailable(uint32_t aWebSocketSerialID, const nsACString& aMessage, uint16_t aType) override { return _to WebSocketMessageAvailable(aWebSocketSerialID, aMessage, aType); } \
  [[nodiscard]] NS_IMETHOD WebSocketClosed(uint32_t aWebSocketSerialID, bool aWasClean, uint16_t aCode, const nsAString& aReason) override { return _to WebSocketClosed(aWebSocketSerialID, aWasClean, aCode, aReason); } \
  [[nodiscard]] NS_IMETHOD FrameReceived(uint32_t aWebSocketSerialID, nsIWebSocketFrame *aFrame) override { return _to FrameReceived(aWebSocketSerialID, aFrame); } \
  [[nodiscard]] NS_IMETHOD FrameSent(uint32_t aWebSocketSerialID, nsIWebSocketFrame *aFrame) override { return _to FrameSent(aWebSocketSerialID, aFrame); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBSOCKETEVENTLISTENER(_to) \
  [[nodiscard]] NS_IMETHOD WebSocketCreated(uint32_t aWebSocketSerialID, const nsAString& aURI, const nsACString& aProtocols) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WebSocketCreated(aWebSocketSerialID, aURI, aProtocols); } \
  [[nodiscard]] NS_IMETHOD WebSocketOpened(uint32_t aWebSocketSerialID, const nsAString& aEffectiveURI, const nsACString& aProtocols, const nsACString& aExtensions, uint64_t aHttpChannelId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WebSocketOpened(aWebSocketSerialID, aEffectiveURI, aProtocols, aExtensions, aHttpChannelId); } \
  [[nodiscard]] NS_IMETHOD WebSocketMessageAvailable(uint32_t aWebSocketSerialID, const nsACString& aMessage, uint16_t aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WebSocketMessageAvailable(aWebSocketSerialID, aMessage, aType); } \
  [[nodiscard]] NS_IMETHOD WebSocketClosed(uint32_t aWebSocketSerialID, bool aWasClean, uint16_t aCode, const nsAString& aReason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WebSocketClosed(aWebSocketSerialID, aWasClean, aCode, aReason); } \
  [[nodiscard]] NS_IMETHOD FrameReceived(uint32_t aWebSocketSerialID, nsIWebSocketFrame *aFrame) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FrameReceived(aWebSocketSerialID, aFrame); } \
  [[nodiscard]] NS_IMETHOD FrameSent(uint32_t aWebSocketSerialID, nsIWebSocketFrame *aFrame) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FrameSent(aWebSocketSerialID, aFrame); } 


/* starting interface:    nsIWebSocketEventService */
#define NS_IWEBSOCKETEVENTSERVICE_IID_STR "b89d1b90-2cf3-4d8f-ac21-5aedfb25c760"

#define NS_IWEBSOCKETEVENTSERVICE_IID \
  {0xb89d1b90, 0x2cf3, 0x4d8f, \
    { 0xac, 0x21, 0x5a, 0xed, 0xfb, 0x25, 0xc7, 0x60 }}

class NS_NO_VTABLE nsIWebSocketEventService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBSOCKETEVENTSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebSocketEventService;

  /* [must_use] void sendMessage (in unsigned long aWebSocketSerialID, in AString aMessage); */
  [[nodiscard]] NS_IMETHOD SendMessage(uint32_t aWebSocketSerialID, const nsAString& aMessage) = 0;

  /* [must_use] void addListener (in unsigned long long aInnerWindowID, in nsIWebSocketEventListener aListener); */
  [[nodiscard]] NS_IMETHOD AddListener(uint64_t aInnerWindowID, nsIWebSocketEventListener *aListener) = 0;

  /* [must_use] void removeListener (in unsigned long long aInnerWindowID, in nsIWebSocketEventListener aListener); */
  [[nodiscard]] NS_IMETHOD RemoveListener(uint64_t aInnerWindowID, nsIWebSocketEventListener *aListener) = 0;

  /* [must_use] bool hasListenerFor (in unsigned long long aInnerWindowID); */
  [[nodiscard]] NS_IMETHOD HasListenerFor(uint64_t aInnerWindowID, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebSocketEventService, NS_IWEBSOCKETEVENTSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBSOCKETEVENTSERVICE \
  [[nodiscard]] NS_IMETHOD SendMessage(uint32_t aWebSocketSerialID, const nsAString& aMessage) override; \
  [[nodiscard]] NS_IMETHOD AddListener(uint64_t aInnerWindowID, nsIWebSocketEventListener *aListener) override; \
  [[nodiscard]] NS_IMETHOD RemoveListener(uint64_t aInnerWindowID, nsIWebSocketEventListener *aListener) override; \
  [[nodiscard]] NS_IMETHOD HasListenerFor(uint64_t aInnerWindowID, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBSOCKETEVENTSERVICE \
  [[nodiscard]] nsresult SendMessage(uint32_t aWebSocketSerialID, const nsAString& aMessage); \
  [[nodiscard]] nsresult AddListener(uint64_t aInnerWindowID, nsIWebSocketEventListener *aListener); \
  [[nodiscard]] nsresult RemoveListener(uint64_t aInnerWindowID, nsIWebSocketEventListener *aListener); \
  [[nodiscard]] nsresult HasListenerFor(uint64_t aInnerWindowID, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBSOCKETEVENTSERVICE(_to) \
  [[nodiscard]] NS_IMETHOD SendMessage(uint32_t aWebSocketSerialID, const nsAString& aMessage) override { return _to SendMessage(aWebSocketSerialID, aMessage); } \
  [[nodiscard]] NS_IMETHOD AddListener(uint64_t aInnerWindowID, nsIWebSocketEventListener *aListener) override { return _to AddListener(aInnerWindowID, aListener); } \
  [[nodiscard]] NS_IMETHOD RemoveListener(uint64_t aInnerWindowID, nsIWebSocketEventListener *aListener) override { return _to RemoveListener(aInnerWindowID, aListener); } \
  [[nodiscard]] NS_IMETHOD HasListenerFor(uint64_t aInnerWindowID, bool *_retval) override { return _to HasListenerFor(aInnerWindowID, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBSOCKETEVENTSERVICE(_to) \
  [[nodiscard]] NS_IMETHOD SendMessage(uint32_t aWebSocketSerialID, const nsAString& aMessage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendMessage(aWebSocketSerialID, aMessage); } \
  [[nodiscard]] NS_IMETHOD AddListener(uint64_t aInnerWindowID, nsIWebSocketEventListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddListener(aInnerWindowID, aListener); } \
  [[nodiscard]] NS_IMETHOD RemoveListener(uint64_t aInnerWindowID, nsIWebSocketEventListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveListener(aInnerWindowID, aListener); } \
  [[nodiscard]] NS_IMETHOD HasListenerFor(uint64_t aInnerWindowID, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasListenerFor(aInnerWindowID, _retval); } 


#endif /* __gen_nsIWebSocketEventService_h__ */
