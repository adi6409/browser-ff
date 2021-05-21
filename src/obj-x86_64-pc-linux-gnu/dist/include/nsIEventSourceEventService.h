/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIEventSourceEventService.idl
 */

#ifndef __gen_nsIEventSourceEventService_h__
#define __gen_nsIEventSourceEventService_h__


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

/* starting interface:    nsIEventSourceEventListener */
#define NS_IEVENTSOURCEEVENTLISTENER_IID_STR "d2cc6222-b7f2-490d-adc2-497d89878fa2"

#define NS_IEVENTSOURCEEVENTLISTENER_IID \
  {0xd2cc6222, 0xb7f2, 0x490d, \
    { 0xad, 0xc2, 0x49, 0x7d, 0x89, 0x87, 0x8f, 0xa2 }}

class NS_NO_VTABLE nsIEventSourceEventListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEVENTSOURCEEVENTLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEventSourceEventListener;

  /* [must_use] void eventSourceConnectionOpened (in uint64_t aHttpChannelId); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD EventSourceConnectionOpened(uint64_t aHttpChannelId) = 0;

  /* [must_use] void eventSourceConnectionClosed (in uint64_t aHttpChannelId); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD EventSourceConnectionClosed(uint64_t aHttpChannelId) = 0;

  /* [must_use] void eventReceived (in uint64_t aHttpChannelId, in AString aEventName, in AString aLastEventID, in AString aData, in uint32_t aRetry, in DOMHighResTimeStamp aTimeStamp); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD EventReceived(uint64_t aHttpChannelId, const nsAString& aEventName, const nsAString& aLastEventID, const nsAString& aData, uint32_t aRetry, DOMHighResTimeStamp aTimeStamp) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEventSourceEventListener, NS_IEVENTSOURCEEVENTLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEVENTSOURCEEVENTLISTENER \
  [[nodiscard]] NS_IMETHOD EventSourceConnectionOpened(uint64_t aHttpChannelId) override; \
  [[nodiscard]] NS_IMETHOD EventSourceConnectionClosed(uint64_t aHttpChannelId) override; \
  [[nodiscard]] NS_IMETHOD EventReceived(uint64_t aHttpChannelId, const nsAString& aEventName, const nsAString& aLastEventID, const nsAString& aData, uint32_t aRetry, DOMHighResTimeStamp aTimeStamp) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEVENTSOURCEEVENTLISTENER \
  [[nodiscard]] nsresult EventSourceConnectionOpened(uint64_t aHttpChannelId); \
  [[nodiscard]] nsresult EventSourceConnectionClosed(uint64_t aHttpChannelId); \
  [[nodiscard]] nsresult EventReceived(uint64_t aHttpChannelId, const nsAString& aEventName, const nsAString& aLastEventID, const nsAString& aData, uint32_t aRetry, DOMHighResTimeStamp aTimeStamp); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEVENTSOURCEEVENTLISTENER(_to) \
  [[nodiscard]] NS_IMETHOD EventSourceConnectionOpened(uint64_t aHttpChannelId) override { return _to EventSourceConnectionOpened(aHttpChannelId); } \
  [[nodiscard]] NS_IMETHOD EventSourceConnectionClosed(uint64_t aHttpChannelId) override { return _to EventSourceConnectionClosed(aHttpChannelId); } \
  [[nodiscard]] NS_IMETHOD EventReceived(uint64_t aHttpChannelId, const nsAString& aEventName, const nsAString& aLastEventID, const nsAString& aData, uint32_t aRetry, DOMHighResTimeStamp aTimeStamp) override { return _to EventReceived(aHttpChannelId, aEventName, aLastEventID, aData, aRetry, aTimeStamp); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEVENTSOURCEEVENTLISTENER(_to) \
  [[nodiscard]] NS_IMETHOD EventSourceConnectionOpened(uint64_t aHttpChannelId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EventSourceConnectionOpened(aHttpChannelId); } \
  [[nodiscard]] NS_IMETHOD EventSourceConnectionClosed(uint64_t aHttpChannelId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EventSourceConnectionClosed(aHttpChannelId); } \
  [[nodiscard]] NS_IMETHOD EventReceived(uint64_t aHttpChannelId, const nsAString& aEventName, const nsAString& aLastEventID, const nsAString& aData, uint32_t aRetry, DOMHighResTimeStamp aTimeStamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EventReceived(aHttpChannelId, aEventName, aLastEventID, aData, aRetry, aTimeStamp); } 


/* starting interface:    nsIEventSourceEventService */
#define NS_IEVENTSOURCEEVENTSERVICE_IID_STR "c0378840-8a74-4b0a-9225-c3a0ac1fac41"

#define NS_IEVENTSOURCEEVENTSERVICE_IID \
  {0xc0378840, 0x8a74, 0x4b0a, \
    { 0x92, 0x25, 0xc3, 0xa0, 0xac, 0x1f, 0xac, 0x41 }}

class NS_NO_VTABLE nsIEventSourceEventService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEVENTSOURCEEVENTSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEventSourceEventService;

  /* [must_use] void addListener (in unsigned long long aInnerWindowID, in nsIEventSourceEventListener aListener); */
  [[nodiscard]] NS_IMETHOD AddListener(uint64_t aInnerWindowID, nsIEventSourceEventListener *aListener) = 0;

  /* [must_use] void removeListener (in unsigned long long aInnerWindowID, in nsIEventSourceEventListener aListener); */
  [[nodiscard]] NS_IMETHOD RemoveListener(uint64_t aInnerWindowID, nsIEventSourceEventListener *aListener) = 0;

  /* [must_use] bool hasListenerFor (in unsigned long long aInnerWindowID); */
  [[nodiscard]] NS_IMETHOD HasListenerFor(uint64_t aInnerWindowID, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEventSourceEventService, NS_IEVENTSOURCEEVENTSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEVENTSOURCEEVENTSERVICE \
  [[nodiscard]] NS_IMETHOD AddListener(uint64_t aInnerWindowID, nsIEventSourceEventListener *aListener) override; \
  [[nodiscard]] NS_IMETHOD RemoveListener(uint64_t aInnerWindowID, nsIEventSourceEventListener *aListener) override; \
  [[nodiscard]] NS_IMETHOD HasListenerFor(uint64_t aInnerWindowID, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEVENTSOURCEEVENTSERVICE \
  [[nodiscard]] nsresult AddListener(uint64_t aInnerWindowID, nsIEventSourceEventListener *aListener); \
  [[nodiscard]] nsresult RemoveListener(uint64_t aInnerWindowID, nsIEventSourceEventListener *aListener); \
  [[nodiscard]] nsresult HasListenerFor(uint64_t aInnerWindowID, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEVENTSOURCEEVENTSERVICE(_to) \
  [[nodiscard]] NS_IMETHOD AddListener(uint64_t aInnerWindowID, nsIEventSourceEventListener *aListener) override { return _to AddListener(aInnerWindowID, aListener); } \
  [[nodiscard]] NS_IMETHOD RemoveListener(uint64_t aInnerWindowID, nsIEventSourceEventListener *aListener) override { return _to RemoveListener(aInnerWindowID, aListener); } \
  [[nodiscard]] NS_IMETHOD HasListenerFor(uint64_t aInnerWindowID, bool *_retval) override { return _to HasListenerFor(aInnerWindowID, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEVENTSOURCEEVENTSERVICE(_to) \
  [[nodiscard]] NS_IMETHOD AddListener(uint64_t aInnerWindowID, nsIEventSourceEventListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddListener(aInnerWindowID, aListener); } \
  [[nodiscard]] NS_IMETHOD RemoveListener(uint64_t aInnerWindowID, nsIEventSourceEventListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveListener(aInnerWindowID, aListener); } \
  [[nodiscard]] NS_IMETHOD HasListenerFor(uint64_t aInnerWindowID, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasListenerFor(aInnerWindowID, _retval); } 


#endif /* __gen_nsIEventSourceEventService_h__ */
