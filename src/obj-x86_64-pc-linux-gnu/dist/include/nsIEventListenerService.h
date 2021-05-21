/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/events/nsIEventListenerService.idl
 */

#ifndef __gen_nsIEventListenerService_h__
#define __gen_nsIEventListenerService_h__


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
namespace mozilla {
namespace dom {
class EventTarget; /* webidl EventTarget */
} // namespace dom
} // namespace mozilla

class nsIArray; /* forward declaration */


/* starting interface:    nsIEventListenerChange */
#define NS_IEVENTLISTENERCHANGE_IID_STR "07222b02-da12-4cf4-b2f7-761da007a8d8"

#define NS_IEVENTLISTENERCHANGE_IID \
  {0x07222b02, 0xda12, 0x4cf4, \
    { 0xb2, 0xf7, 0x76, 0x1d, 0xa0, 0x07, 0xa8, 0xd8 }}

class NS_NO_VTABLE nsIEventListenerChange : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEVENTLISTENERCHANGE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEventListenerChange;

  /* readonly attribute EventTarget target; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTarget(mozilla::dom::EventTarget **aTarget) = 0;

  /* [noscript] readonly attribute uint32_t countOfEventListenerChangesAffectingAccessibility; */
  NS_IMETHOD GetCountOfEventListenerChangesAffectingAccessibility(uint32_t *aCountOfEventListenerChangesAffectingAccessibility) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEventListenerChange, NS_IEVENTLISTENERCHANGE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEVENTLISTENERCHANGE \
  NS_IMETHOD GetTarget(mozilla::dom::EventTarget **aTarget) override; \
  NS_IMETHOD GetCountOfEventListenerChangesAffectingAccessibility(uint32_t *aCountOfEventListenerChangesAffectingAccessibility) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEVENTLISTENERCHANGE \
  nsresult GetTarget(mozilla::dom::EventTarget **aTarget); \
  nsresult GetCountOfEventListenerChangesAffectingAccessibility(uint32_t *aCountOfEventListenerChangesAffectingAccessibility); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEVENTLISTENERCHANGE(_to) \
  NS_IMETHOD GetTarget(mozilla::dom::EventTarget **aTarget) override { return _to GetTarget(aTarget); } \
  NS_IMETHOD GetCountOfEventListenerChangesAffectingAccessibility(uint32_t *aCountOfEventListenerChangesAffectingAccessibility) override { return _to GetCountOfEventListenerChangesAffectingAccessibility(aCountOfEventListenerChangesAffectingAccessibility); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEVENTLISTENERCHANGE(_to) \
  NS_IMETHOD GetTarget(mozilla::dom::EventTarget **aTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTarget(aTarget); } \
  NS_IMETHOD GetCountOfEventListenerChangesAffectingAccessibility(uint32_t *aCountOfEventListenerChangesAffectingAccessibility) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCountOfEventListenerChangesAffectingAccessibility(aCountOfEventListenerChangesAffectingAccessibility); } 


/* starting interface:    nsIListenerChangeListener */
#define NS_ILISTENERCHANGELISTENER_IID_STR "aa7c95f6-d3b5-44b3-9597-1d9f19b9c5f2"

#define NS_ILISTENERCHANGELISTENER_IID \
  {0xaa7c95f6, 0xd3b5, 0x44b3, \
    { 0x95, 0x97, 0x1d, 0x9f, 0x19, 0xb9, 0xc5, 0xf2 }}

class NS_NO_VTABLE nsIListenerChangeListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILISTENERCHANGELISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIListenerChangeListener;

  /* void listenersChanged (in nsIArray aEventListenerChanges); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ListenersChanged(nsIArray *aEventListenerChanges) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIListenerChangeListener, NS_ILISTENERCHANGELISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILISTENERCHANGELISTENER \
  NS_IMETHOD ListenersChanged(nsIArray *aEventListenerChanges) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILISTENERCHANGELISTENER \
  nsresult ListenersChanged(nsIArray *aEventListenerChanges); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILISTENERCHANGELISTENER(_to) \
  NS_IMETHOD ListenersChanged(nsIArray *aEventListenerChanges) override { return _to ListenersChanged(aEventListenerChanges); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILISTENERCHANGELISTENER(_to) \
  NS_IMETHOD ListenersChanged(nsIArray *aEventListenerChanges) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ListenersChanged(aEventListenerChanges); } 


/* starting interface:    nsIEventListenerInfo */
#define NS_IEVENTLISTENERINFO_IID_STR "11ba5fd7-8db2-4b1a-9f67-342cfa11afad"

#define NS_IEVENTLISTENERINFO_IID \
  {0x11ba5fd7, 0x8db2, 0x4b1a, \
    { 0x9f, 0x67, 0x34, 0x2c, 0xfa, 0x11, 0xaf, 0xad }}

class NS_NO_VTABLE nsIEventListenerInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEVENTLISTENERINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEventListenerInfo;

  /* readonly attribute AString type; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetType(nsAString& aType) = 0;

  /* readonly attribute boolean capturing; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCapturing(bool *aCapturing) = 0;

  /* readonly attribute boolean allowsUntrusted; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAllowsUntrusted(bool *aAllowsUntrusted) = 0;

  /* readonly attribute boolean inSystemEventGroup; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInSystemEventGroup(bool *aInSystemEventGroup) = 0;

  /* [implicit_jscontext] readonly attribute jsval listenerObject; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetListenerObject(JSContext* cx, JS::MutableHandleValue aListenerObject) = 0;

  /* AString toSource (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ToSource(nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEventListenerInfo, NS_IEVENTLISTENERINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEVENTLISTENERINFO \
  NS_IMETHOD GetType(nsAString& aType) override; \
  NS_IMETHOD GetCapturing(bool *aCapturing) override; \
  NS_IMETHOD GetAllowsUntrusted(bool *aAllowsUntrusted) override; \
  NS_IMETHOD GetInSystemEventGroup(bool *aInSystemEventGroup) override; \
  NS_IMETHOD GetListenerObject(JSContext* cx, JS::MutableHandleValue aListenerObject) override; \
  NS_IMETHOD ToSource(nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEVENTLISTENERINFO \
  nsresult GetType(nsAString& aType); \
  nsresult GetCapturing(bool *aCapturing); \
  nsresult GetAllowsUntrusted(bool *aAllowsUntrusted); \
  nsresult GetInSystemEventGroup(bool *aInSystemEventGroup); \
  nsresult GetListenerObject(JSContext* cx, JS::MutableHandleValue aListenerObject); \
  nsresult ToSource(nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEVENTLISTENERINFO(_to) \
  NS_IMETHOD GetType(nsAString& aType) override { return _to GetType(aType); } \
  NS_IMETHOD GetCapturing(bool *aCapturing) override { return _to GetCapturing(aCapturing); } \
  NS_IMETHOD GetAllowsUntrusted(bool *aAllowsUntrusted) override { return _to GetAllowsUntrusted(aAllowsUntrusted); } \
  NS_IMETHOD GetInSystemEventGroup(bool *aInSystemEventGroup) override { return _to GetInSystemEventGroup(aInSystemEventGroup); } \
  NS_IMETHOD GetListenerObject(JSContext* cx, JS::MutableHandleValue aListenerObject) override { return _to GetListenerObject(cx, aListenerObject); } \
  NS_IMETHOD ToSource(nsAString& _retval) override { return _to ToSource(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEVENTLISTENERINFO(_to) \
  NS_IMETHOD GetType(nsAString& aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD GetCapturing(bool *aCapturing) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCapturing(aCapturing); } \
  NS_IMETHOD GetAllowsUntrusted(bool *aAllowsUntrusted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowsUntrusted(aAllowsUntrusted); } \
  NS_IMETHOD GetInSystemEventGroup(bool *aInSystemEventGroup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInSystemEventGroup(aInSystemEventGroup); } \
  NS_IMETHOD GetListenerObject(JSContext* cx, JS::MutableHandleValue aListenerObject) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetListenerObject(cx, aListenerObject); } \
  NS_IMETHOD ToSource(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ToSource(_retval); } 


/* starting interface:    nsIEventListenerService */
#define NS_IEVENTLISTENERSERVICE_IID_STR "77aab5f7-213d-4db4-9f22-e46dfb774f15"

#define NS_IEVENTLISTENERSERVICE_IID \
  {0x77aab5f7, 0x213d, 0x4db4, \
    { 0x9f, 0x22, 0xe4, 0x6d, 0xfb, 0x77, 0x4f, 0x15 }}

class NS_NO_VTABLE nsIEventListenerService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEVENTLISTENERSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEventListenerService;

  /* Array<nsIEventListenerInfo> getListenerInfoFor (in EventTarget aEventTarget); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetListenerInfoFor(mozilla::dom::EventTarget *aEventTarget, nsTArray<RefPtr<nsIEventListenerInfo>>& _retval) = 0;

  /* Array<EventTarget> getEventTargetChainFor (in EventTarget aEventTarget, in boolean composed); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEventTargetChainFor(mozilla::dom::EventTarget *aEventTarget, bool composed, nsTArray<RefPtr<mozilla::dom::EventTarget>>& _retval) = 0;

  /* boolean hasListenersFor (in EventTarget aEventTarget, in AString aType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasListenersFor(mozilla::dom::EventTarget *aEventTarget, const nsAString& aType, bool *_retval) = 0;

  /* [implicit_jscontext] void addSystemEventListener (in EventTarget target, in AString type, in jsval listener, in boolean useCapture); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddSystemEventListener(mozilla::dom::EventTarget *target, const nsAString& type, JS::HandleValue listener, bool useCapture, JSContext* cx) = 0;

  /* [implicit_jscontext] void removeSystemEventListener (in EventTarget target, in AString type, in jsval listener, in boolean useCapture); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveSystemEventListener(mozilla::dom::EventTarget *target, const nsAString& type, JS::HandleValue listener, bool useCapture, JSContext* cx) = 0;

  /* [implicit_jscontext] void addListenerForAllEvents (in EventTarget target, in jsval listener, [optional] in boolean aUseCapture, [optional] in boolean aWantsUntrusted, [optional] in boolean aSystemEventGroup); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddListenerForAllEvents(mozilla::dom::EventTarget *target, JS::HandleValue listener, bool aUseCapture, bool aWantsUntrusted, bool aSystemEventGroup, JSContext* cx) = 0;

  /* [implicit_jscontext] void removeListenerForAllEvents (in EventTarget target, in jsval listener, [optional] in boolean aUseCapture, [optional] in boolean aSystemEventGroup); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveListenerForAllEvents(mozilla::dom::EventTarget *target, JS::HandleValue listener, bool aUseCapture, bool aSystemEventGroup, JSContext* cx) = 0;

  /* void addListenerChangeListener (in nsIListenerChangeListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddListenerChangeListener(nsIListenerChangeListener *aListener) = 0;

  /* void removeListenerChangeListener (in nsIListenerChangeListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveListenerChangeListener(nsIListenerChangeListener *aListener) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEventListenerService, NS_IEVENTLISTENERSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEVENTLISTENERSERVICE \
  NS_IMETHOD GetListenerInfoFor(mozilla::dom::EventTarget *aEventTarget, nsTArray<RefPtr<nsIEventListenerInfo>>& _retval) override; \
  NS_IMETHOD GetEventTargetChainFor(mozilla::dom::EventTarget *aEventTarget, bool composed, nsTArray<RefPtr<mozilla::dom::EventTarget>>& _retval) override; \
  NS_IMETHOD HasListenersFor(mozilla::dom::EventTarget *aEventTarget, const nsAString& aType, bool *_retval) override; \
  NS_IMETHOD AddSystemEventListener(mozilla::dom::EventTarget *target, const nsAString& type, JS::HandleValue listener, bool useCapture, JSContext* cx) override; \
  NS_IMETHOD RemoveSystemEventListener(mozilla::dom::EventTarget *target, const nsAString& type, JS::HandleValue listener, bool useCapture, JSContext* cx) override; \
  NS_IMETHOD AddListenerForAllEvents(mozilla::dom::EventTarget *target, JS::HandleValue listener, bool aUseCapture, bool aWantsUntrusted, bool aSystemEventGroup, JSContext* cx) override; \
  NS_IMETHOD RemoveListenerForAllEvents(mozilla::dom::EventTarget *target, JS::HandleValue listener, bool aUseCapture, bool aSystemEventGroup, JSContext* cx) override; \
  NS_IMETHOD AddListenerChangeListener(nsIListenerChangeListener *aListener) override; \
  NS_IMETHOD RemoveListenerChangeListener(nsIListenerChangeListener *aListener) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEVENTLISTENERSERVICE \
  nsresult GetListenerInfoFor(mozilla::dom::EventTarget *aEventTarget, nsTArray<RefPtr<nsIEventListenerInfo>>& _retval); \
  nsresult GetEventTargetChainFor(mozilla::dom::EventTarget *aEventTarget, bool composed, nsTArray<RefPtr<mozilla::dom::EventTarget>>& _retval); \
  nsresult HasListenersFor(mozilla::dom::EventTarget *aEventTarget, const nsAString& aType, bool *_retval); \
  nsresult AddSystemEventListener(mozilla::dom::EventTarget *target, const nsAString& type, JS::HandleValue listener, bool useCapture, JSContext* cx); \
  nsresult RemoveSystemEventListener(mozilla::dom::EventTarget *target, const nsAString& type, JS::HandleValue listener, bool useCapture, JSContext* cx); \
  nsresult AddListenerForAllEvents(mozilla::dom::EventTarget *target, JS::HandleValue listener, bool aUseCapture, bool aWantsUntrusted, bool aSystemEventGroup, JSContext* cx); \
  nsresult RemoveListenerForAllEvents(mozilla::dom::EventTarget *target, JS::HandleValue listener, bool aUseCapture, bool aSystemEventGroup, JSContext* cx); \
  nsresult AddListenerChangeListener(nsIListenerChangeListener *aListener); \
  nsresult RemoveListenerChangeListener(nsIListenerChangeListener *aListener); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEVENTLISTENERSERVICE(_to) \
  NS_IMETHOD GetListenerInfoFor(mozilla::dom::EventTarget *aEventTarget, nsTArray<RefPtr<nsIEventListenerInfo>>& _retval) override { return _to GetListenerInfoFor(aEventTarget, _retval); } \
  NS_IMETHOD GetEventTargetChainFor(mozilla::dom::EventTarget *aEventTarget, bool composed, nsTArray<RefPtr<mozilla::dom::EventTarget>>& _retval) override { return _to GetEventTargetChainFor(aEventTarget, composed, _retval); } \
  NS_IMETHOD HasListenersFor(mozilla::dom::EventTarget *aEventTarget, const nsAString& aType, bool *_retval) override { return _to HasListenersFor(aEventTarget, aType, _retval); } \
  NS_IMETHOD AddSystemEventListener(mozilla::dom::EventTarget *target, const nsAString& type, JS::HandleValue listener, bool useCapture, JSContext* cx) override { return _to AddSystemEventListener(target, type, listener, useCapture, cx); } \
  NS_IMETHOD RemoveSystemEventListener(mozilla::dom::EventTarget *target, const nsAString& type, JS::HandleValue listener, bool useCapture, JSContext* cx) override { return _to RemoveSystemEventListener(target, type, listener, useCapture, cx); } \
  NS_IMETHOD AddListenerForAllEvents(mozilla::dom::EventTarget *target, JS::HandleValue listener, bool aUseCapture, bool aWantsUntrusted, bool aSystemEventGroup, JSContext* cx) override { return _to AddListenerForAllEvents(target, listener, aUseCapture, aWantsUntrusted, aSystemEventGroup, cx); } \
  NS_IMETHOD RemoveListenerForAllEvents(mozilla::dom::EventTarget *target, JS::HandleValue listener, bool aUseCapture, bool aSystemEventGroup, JSContext* cx) override { return _to RemoveListenerForAllEvents(target, listener, aUseCapture, aSystemEventGroup, cx); } \
  NS_IMETHOD AddListenerChangeListener(nsIListenerChangeListener *aListener) override { return _to AddListenerChangeListener(aListener); } \
  NS_IMETHOD RemoveListenerChangeListener(nsIListenerChangeListener *aListener) override { return _to RemoveListenerChangeListener(aListener); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEVENTLISTENERSERVICE(_to) \
  NS_IMETHOD GetListenerInfoFor(mozilla::dom::EventTarget *aEventTarget, nsTArray<RefPtr<nsIEventListenerInfo>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetListenerInfoFor(aEventTarget, _retval); } \
  NS_IMETHOD GetEventTargetChainFor(mozilla::dom::EventTarget *aEventTarget, bool composed, nsTArray<RefPtr<mozilla::dom::EventTarget>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEventTargetChainFor(aEventTarget, composed, _retval); } \
  NS_IMETHOD HasListenersFor(mozilla::dom::EventTarget *aEventTarget, const nsAString& aType, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasListenersFor(aEventTarget, aType, _retval); } \
  NS_IMETHOD AddSystemEventListener(mozilla::dom::EventTarget *target, const nsAString& type, JS::HandleValue listener, bool useCapture, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddSystemEventListener(target, type, listener, useCapture, cx); } \
  NS_IMETHOD RemoveSystemEventListener(mozilla::dom::EventTarget *target, const nsAString& type, JS::HandleValue listener, bool useCapture, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveSystemEventListener(target, type, listener, useCapture, cx); } \
  NS_IMETHOD AddListenerForAllEvents(mozilla::dom::EventTarget *target, JS::HandleValue listener, bool aUseCapture, bool aWantsUntrusted, bool aSystemEventGroup, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddListenerForAllEvents(target, listener, aUseCapture, aWantsUntrusted, aSystemEventGroup, cx); } \
  NS_IMETHOD RemoveListenerForAllEvents(mozilla::dom::EventTarget *target, JS::HandleValue listener, bool aUseCapture, bool aSystemEventGroup, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveListenerForAllEvents(target, listener, aUseCapture, aSystemEventGroup, cx); } \
  NS_IMETHOD AddListenerChangeListener(nsIListenerChangeListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddListenerChangeListener(aListener); } \
  NS_IMETHOD RemoveListenerChangeListener(nsIListenerChangeListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveListenerChangeListener(aListener); } 


#endif /* __gen_nsIEventListenerService_h__ */
