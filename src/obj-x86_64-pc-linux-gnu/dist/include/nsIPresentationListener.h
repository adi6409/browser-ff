/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationListener.idl
 */

#ifndef __gen_nsIPresentationListener_h__
#define __gen_nsIPresentationListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "nsTArray.h"

/* starting interface:    nsIPresentationAvailabilityListener */
#define NS_IPRESENTATIONAVAILABILITYLISTENER_IID_STR "0105f837-4279-4715-9d5b-2dc3f8b65353"

#define NS_IPRESENTATIONAVAILABILITYLISTENER_IID \
  {0x0105f837, 0x4279, 0x4715, \
    { 0x9d, 0x5b, 0x2d, 0xc3, 0xf8, 0xb6, 0x53, 0x53 }}

class NS_NO_VTABLE nsIPresentationAvailabilityListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONAVAILABILITYLISTENER_IID)

  /* [noscript] void notifyAvailableChange (in URLArrayRef urls, in bool available); */
  NS_IMETHOD NotifyAvailableChange(const nsTArray<nsString> & urls, bool available) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationAvailabilityListener, NS_IPRESENTATIONAVAILABILITYLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONAVAILABILITYLISTENER \
  NS_IMETHOD NotifyAvailableChange(const nsTArray<nsString> & urls, bool available) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONAVAILABILITYLISTENER \
  nsresult NotifyAvailableChange(const nsTArray<nsString> & urls, bool available); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONAVAILABILITYLISTENER(_to) \
  NS_IMETHOD NotifyAvailableChange(const nsTArray<nsString> & urls, bool available) override { return _to NotifyAvailableChange(urls, available); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONAVAILABILITYLISTENER(_to) \
  NS_IMETHOD NotifyAvailableChange(const nsTArray<nsString> & urls, bool available) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyAvailableChange(urls, available); } 


/* starting interface:    nsIPresentationSessionListener */
#define NS_IPRESENTATIONSESSIONLISTENER_IID_STR "7dd48df8-8f8c-48c7-ac37-7b9fd1acf2f8"

#define NS_IPRESENTATIONSESSIONLISTENER_IID \
  {0x7dd48df8, 0x8f8c, 0x48c7, \
    { 0xac, 0x37, 0x7b, 0x9f, 0xd1, 0xac, 0xf2, 0xf8 }}

class NS_NO_VTABLE nsIPresentationSessionListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONSESSIONLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationSessionListener;

  enum {
    STATE_CONNECTING = 0U,
    STATE_CONNECTED = 1U,
    STATE_CLOSED = 2U,
    STATE_TERMINATED = 3U
  };

  /* void notifyStateChange (in AString sessionId, in unsigned short state, in nsresult reason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyStateChange(const nsAString& sessionId, uint16_t state, nsresult reason) = 0;

  /* void notifyMessage (in AString sessionId, in ACString data, in boolean isBinary); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyMessage(const nsAString& sessionId, const nsACString& data, bool isBinary) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationSessionListener, NS_IPRESENTATIONSESSIONLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONSESSIONLISTENER \
  NS_IMETHOD NotifyStateChange(const nsAString& sessionId, uint16_t state, nsresult reason) override; \
  NS_IMETHOD NotifyMessage(const nsAString& sessionId, const nsACString& data, bool isBinary) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONSESSIONLISTENER \
  nsresult NotifyStateChange(const nsAString& sessionId, uint16_t state, nsresult reason); \
  nsresult NotifyMessage(const nsAString& sessionId, const nsACString& data, bool isBinary); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONSESSIONLISTENER(_to) \
  NS_IMETHOD NotifyStateChange(const nsAString& sessionId, uint16_t state, nsresult reason) override { return _to NotifyStateChange(sessionId, state, reason); } \
  NS_IMETHOD NotifyMessage(const nsAString& sessionId, const nsACString& data, bool isBinary) override { return _to NotifyMessage(sessionId, data, isBinary); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONSESSIONLISTENER(_to) \
  NS_IMETHOD NotifyStateChange(const nsAString& sessionId, uint16_t state, nsresult reason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyStateChange(sessionId, state, reason); } \
  NS_IMETHOD NotifyMessage(const nsAString& sessionId, const nsACString& data, bool isBinary) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyMessage(sessionId, data, isBinary); } 


/* starting interface:    nsIPresentationRespondingListener */
#define NS_IPRESENTATIONRESPONDINGLISTENER_IID_STR "27f101d7-9ed1-429e-b4f8-43b00e8e111c"

#define NS_IPRESENTATIONRESPONDINGLISTENER_IID \
  {0x27f101d7, 0x9ed1, 0x429e, \
    { 0xb4, 0xf8, 0x43, 0xb0, 0x0e, 0x8e, 0x11, 0x1c }}

class NS_NO_VTABLE nsIPresentationRespondingListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONRESPONDINGLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationRespondingListener;

  /* void notifySessionConnect (in unsigned long long windowId, in AString sessionId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotifySessionConnect(uint64_t windowId, const nsAString& sessionId) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationRespondingListener, NS_IPRESENTATIONRESPONDINGLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONRESPONDINGLISTENER \
  NS_IMETHOD NotifySessionConnect(uint64_t windowId, const nsAString& sessionId) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONRESPONDINGLISTENER \
  nsresult NotifySessionConnect(uint64_t windowId, const nsAString& sessionId); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONRESPONDINGLISTENER(_to) \
  NS_IMETHOD NotifySessionConnect(uint64_t windowId, const nsAString& sessionId) override { return _to NotifySessionConnect(windowId, sessionId); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONRESPONDINGLISTENER(_to) \
  NS_IMETHOD NotifySessionConnect(uint64_t windowId, const nsAString& sessionId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifySessionConnect(windowId, sessionId); } 


#endif /* __gen_nsIPresentationListener_h__ */
