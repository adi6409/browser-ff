/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationService.idl
 */

#ifndef __gen_nsIPresentationService_h__
#define __gen_nsIPresentationService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */

class nsIPresentationAvailabilityListener; /* forward declaration */

class nsIPresentationRespondingListener; /* forward declaration */

class nsIPresentationSessionListener; /* forward declaration */

class nsIPresentationTransportBuilderConstructor; /* forward declaration */

class nsIPrincipal; /* forward declaration */

namespace mozilla {
namespace dom {
class Blob; /* webidl Blob */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class EventTarget; /* webidl EventTarget */
} // namespace dom
} // namespace mozilla

#define PRESENTATION_SERVICE_CID \
  { 0x1d9bb10c, 0xc0ab, 0x4fe8, \
    { 0x9e, 0x4f, 0x40, 0x58, 0xb8, 0x51, 0x98, 0x32 } }
#define PRESENTATION_SERVICE_CONTRACTID \
  "@mozilla.org/presentation/presentationservice;1"
#include "nsStringFwd.h"
#include "nsTArray.h"

/* starting interface:    nsIPresentationServiceCallback */
#define NS_IPRESENTATIONSERVICECALLBACK_IID_STR "12073206-0065-4b10-9488-a6eb9b23e65b"

#define NS_IPRESENTATIONSERVICECALLBACK_IID \
  {0x12073206, 0x0065, 0x4b10, \
    { 0x94, 0x88, 0xa6, 0xeb, 0x9b, 0x23, 0xe6, 0x5b }}

class NS_NO_VTABLE nsIPresentationServiceCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONSERVICECALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationServiceCallback;

  /* void notifySuccess (in AString url); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotifySuccess(const nsAString& url) = 0;

  /* void notifyError (in nsresult error); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyError(nsresult error) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationServiceCallback, NS_IPRESENTATIONSERVICECALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONSERVICECALLBACK \
  NS_IMETHOD NotifySuccess(const nsAString& url) override; \
  NS_IMETHOD NotifyError(nsresult error) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONSERVICECALLBACK \
  nsresult NotifySuccess(const nsAString& url); \
  nsresult NotifyError(nsresult error); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONSERVICECALLBACK(_to) \
  NS_IMETHOD NotifySuccess(const nsAString& url) override { return _to NotifySuccess(url); } \
  NS_IMETHOD NotifyError(nsresult error) override { return _to NotifyError(error); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONSERVICECALLBACK(_to) \
  NS_IMETHOD NotifySuccess(const nsAString& url) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifySuccess(url); } \
  NS_IMETHOD NotifyError(nsresult error) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyError(error); } 


/* starting interface:    nsIPresentationService */
#define NS_IPRESENTATIONSERVICE_IID_STR "de42b741-5619-4650-b961-c2cebb572c95"

#define NS_IPRESENTATIONSERVICE_IID \
  {0xde42b741, 0x5619, 0x4650, \
    { 0xb9, 0x61, 0xc2, 0xce, 0xbb, 0x57, 0x2c, 0x95 }}

class NS_NO_VTABLE nsIPresentationService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRESENTATIONSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPresentationService;

  enum {
    ROLE_CONTROLLER = 1U,
    ROLE_RECEIVER = 2U,
    CLOSED_REASON_ERROR = 1U,
    CLOSED_REASON_CLOSED = 2U,
    CLOSED_REASON_WENTAWAY = 3U
  };

  /* [noscript] void startSession (in URLArrayRef urls, in AString sessionId, in AString origin, in AString deviceId, in unsigned long long windowId, in EventTarget eventTarget, in nsIPrincipal principal, in nsIPresentationServiceCallback callback, in nsIPresentationTransportBuilderConstructor constructor); */
  NS_IMETHOD StartSession(const nsTArray<nsString> & urls, const nsAString& sessionId, const nsAString& origin, const nsAString& deviceId, uint64_t windowId, mozilla::dom::EventTarget *eventTarget, nsIPrincipal *principal, nsIPresentationServiceCallback *callback, nsIPresentationTransportBuilderConstructor *constructor) = 0;

  /* void sendSessionMessage (in AString sessionId, in uint8_t role, in AString data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SendSessionMessage(const nsAString& sessionId, uint8_t role, const nsAString& data) = 0;

  /* void sendSessionBinaryMsg (in AString sessionId, in uint8_t role, in ACString data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SendSessionBinaryMsg(const nsAString& sessionId, uint8_t role, const nsACString& data) = 0;

  /* void sendSessionBlob (in AString sessionId, in uint8_t role, in Blob blob); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SendSessionBlob(const nsAString& sessionId, uint8_t role, mozilla::dom::Blob *blob) = 0;

  /* void closeSession (in AString sessionId, in uint8_t role, in uint8_t closedReason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CloseSession(const nsAString& sessionId, uint8_t role, uint8_t closedReason) = 0;

  /* void terminateSession (in AString sessionId, in uint8_t role); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD TerminateSession(const nsAString& sessionId, uint8_t role) = 0;

  /* [noscript] void reconnectSession (in URLArrayRef urls, in AString sessionId, in uint8_t role, in nsIPresentationServiceCallback callback); */
  NS_IMETHOD ReconnectSession(const nsTArray<nsString> & urls, const nsAString& sessionId, uint8_t role, nsIPresentationServiceCallback *callback) = 0;

  /* [noscript] void registerAvailabilityListener (in URLArrayRef availabilityUrls, in nsIPresentationAvailabilityListener listener); */
  NS_IMETHOD RegisterAvailabilityListener(const nsTArray<nsString> & availabilityUrls, nsIPresentationAvailabilityListener *listener) = 0;

  /* [noscript] void unregisterAvailabilityListener (in URLArrayRef availabilityUrls, in nsIPresentationAvailabilityListener listener); */
  NS_IMETHOD UnregisterAvailabilityListener(const nsTArray<nsString> & availabilityUrls, nsIPresentationAvailabilityListener *listener) = 0;

  /* void registerSessionListener (in AString sessionId, in uint8_t role, in nsIPresentationSessionListener listener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterSessionListener(const nsAString& sessionId, uint8_t role, nsIPresentationSessionListener *listener) = 0;

  /* void unregisterSessionListener (in AString sessionId, in uint8_t role); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnregisterSessionListener(const nsAString& sessionId, uint8_t role) = 0;

  /* void registerRespondingListener (in unsigned long long windowId, in nsIPresentationRespondingListener listener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterRespondingListener(uint64_t windowId, nsIPresentationRespondingListener *listener) = 0;

  /* void unregisterRespondingListener (in unsigned long long windowId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnregisterRespondingListener(uint64_t windowId) = 0;

  /* void notifyReceiverReady (in AString sessionId, in unsigned long long windowId, in boolean isLoading, in nsIPresentationTransportBuilderConstructor constructor); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyReceiverReady(const nsAString& sessionId, uint64_t windowId, bool isLoading, nsIPresentationTransportBuilderConstructor *constructor) = 0;

  /* void NotifyTransportClosed (in AString sessionId, in uint8_t role, in nsresult reason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyTransportClosed(const nsAString& sessionId, uint8_t role, nsresult reason) = 0;

  /* void untrackSessionInfo (in AString sessionId, in uint8_t role); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UntrackSessionInfo(const nsAString& sessionId, uint8_t role) = 0;

  /* unsigned long long getWindowIdBySessionId (in AString sessionId, in uint8_t role); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWindowIdBySessionId(const nsAString& sessionId, uint8_t role, uint64_t *_retval) = 0;

  /* void updateWindowIdBySessionId (in AString sessionId, in uint8_t role, in unsigned long long windowId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateWindowIdBySessionId(const nsAString& sessionId, uint8_t role, uint64_t windowId) = 0;

  /* void buildTransport (in AString sessionId, in uint8_t role); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD BuildTransport(const nsAString& sessionId, uint8_t role) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPresentationService, NS_IPRESENTATIONSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRESENTATIONSERVICE \
  NS_IMETHOD StartSession(const nsTArray<nsString> & urls, const nsAString& sessionId, const nsAString& origin, const nsAString& deviceId, uint64_t windowId, mozilla::dom::EventTarget *eventTarget, nsIPrincipal *principal, nsIPresentationServiceCallback *callback, nsIPresentationTransportBuilderConstructor *constructor) override; \
  NS_IMETHOD SendSessionMessage(const nsAString& sessionId, uint8_t role, const nsAString& data) override; \
  NS_IMETHOD SendSessionBinaryMsg(const nsAString& sessionId, uint8_t role, const nsACString& data) override; \
  NS_IMETHOD SendSessionBlob(const nsAString& sessionId, uint8_t role, mozilla::dom::Blob *blob) override; \
  NS_IMETHOD CloseSession(const nsAString& sessionId, uint8_t role, uint8_t closedReason) override; \
  NS_IMETHOD TerminateSession(const nsAString& sessionId, uint8_t role) override; \
  NS_IMETHOD ReconnectSession(const nsTArray<nsString> & urls, const nsAString& sessionId, uint8_t role, nsIPresentationServiceCallback *callback) override; \
  NS_IMETHOD RegisterAvailabilityListener(const nsTArray<nsString> & availabilityUrls, nsIPresentationAvailabilityListener *listener) override; \
  NS_IMETHOD UnregisterAvailabilityListener(const nsTArray<nsString> & availabilityUrls, nsIPresentationAvailabilityListener *listener) override; \
  NS_IMETHOD RegisterSessionListener(const nsAString& sessionId, uint8_t role, nsIPresentationSessionListener *listener) override; \
  NS_IMETHOD UnregisterSessionListener(const nsAString& sessionId, uint8_t role) override; \
  NS_IMETHOD RegisterRespondingListener(uint64_t windowId, nsIPresentationRespondingListener *listener) override; \
  NS_IMETHOD UnregisterRespondingListener(uint64_t windowId) override; \
  NS_IMETHOD NotifyReceiverReady(const nsAString& sessionId, uint64_t windowId, bool isLoading, nsIPresentationTransportBuilderConstructor *constructor) override; \
  NS_IMETHOD NotifyTransportClosed(const nsAString& sessionId, uint8_t role, nsresult reason) override; \
  NS_IMETHOD UntrackSessionInfo(const nsAString& sessionId, uint8_t role) override; \
  NS_IMETHOD GetWindowIdBySessionId(const nsAString& sessionId, uint8_t role, uint64_t *_retval) override; \
  NS_IMETHOD UpdateWindowIdBySessionId(const nsAString& sessionId, uint8_t role, uint64_t windowId) override; \
  NS_IMETHOD BuildTransport(const nsAString& sessionId, uint8_t role) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRESENTATIONSERVICE \
  nsresult StartSession(const nsTArray<nsString> & urls, const nsAString& sessionId, const nsAString& origin, const nsAString& deviceId, uint64_t windowId, mozilla::dom::EventTarget *eventTarget, nsIPrincipal *principal, nsIPresentationServiceCallback *callback, nsIPresentationTransportBuilderConstructor *constructor); \
  nsresult SendSessionMessage(const nsAString& sessionId, uint8_t role, const nsAString& data); \
  nsresult SendSessionBinaryMsg(const nsAString& sessionId, uint8_t role, const nsACString& data); \
  nsresult SendSessionBlob(const nsAString& sessionId, uint8_t role, mozilla::dom::Blob *blob); \
  nsresult CloseSession(const nsAString& sessionId, uint8_t role, uint8_t closedReason); \
  nsresult TerminateSession(const nsAString& sessionId, uint8_t role); \
  nsresult ReconnectSession(const nsTArray<nsString> & urls, const nsAString& sessionId, uint8_t role, nsIPresentationServiceCallback *callback); \
  nsresult RegisterAvailabilityListener(const nsTArray<nsString> & availabilityUrls, nsIPresentationAvailabilityListener *listener); \
  nsresult UnregisterAvailabilityListener(const nsTArray<nsString> & availabilityUrls, nsIPresentationAvailabilityListener *listener); \
  nsresult RegisterSessionListener(const nsAString& sessionId, uint8_t role, nsIPresentationSessionListener *listener); \
  nsresult UnregisterSessionListener(const nsAString& sessionId, uint8_t role); \
  nsresult RegisterRespondingListener(uint64_t windowId, nsIPresentationRespondingListener *listener); \
  nsresult UnregisterRespondingListener(uint64_t windowId); \
  nsresult NotifyReceiverReady(const nsAString& sessionId, uint64_t windowId, bool isLoading, nsIPresentationTransportBuilderConstructor *constructor); \
  nsresult NotifyTransportClosed(const nsAString& sessionId, uint8_t role, nsresult reason); \
  nsresult UntrackSessionInfo(const nsAString& sessionId, uint8_t role); \
  nsresult GetWindowIdBySessionId(const nsAString& sessionId, uint8_t role, uint64_t *_retval); \
  nsresult UpdateWindowIdBySessionId(const nsAString& sessionId, uint8_t role, uint64_t windowId); \
  nsresult BuildTransport(const nsAString& sessionId, uint8_t role); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRESENTATIONSERVICE(_to) \
  NS_IMETHOD StartSession(const nsTArray<nsString> & urls, const nsAString& sessionId, const nsAString& origin, const nsAString& deviceId, uint64_t windowId, mozilla::dom::EventTarget *eventTarget, nsIPrincipal *principal, nsIPresentationServiceCallback *callback, nsIPresentationTransportBuilderConstructor *constructor) override { return _to StartSession(urls, sessionId, origin, deviceId, windowId, eventTarget, principal, callback, constructor); } \
  NS_IMETHOD SendSessionMessage(const nsAString& sessionId, uint8_t role, const nsAString& data) override { return _to SendSessionMessage(sessionId, role, data); } \
  NS_IMETHOD SendSessionBinaryMsg(const nsAString& sessionId, uint8_t role, const nsACString& data) override { return _to SendSessionBinaryMsg(sessionId, role, data); } \
  NS_IMETHOD SendSessionBlob(const nsAString& sessionId, uint8_t role, mozilla::dom::Blob *blob) override { return _to SendSessionBlob(sessionId, role, blob); } \
  NS_IMETHOD CloseSession(const nsAString& sessionId, uint8_t role, uint8_t closedReason) override { return _to CloseSession(sessionId, role, closedReason); } \
  NS_IMETHOD TerminateSession(const nsAString& sessionId, uint8_t role) override { return _to TerminateSession(sessionId, role); } \
  NS_IMETHOD ReconnectSession(const nsTArray<nsString> & urls, const nsAString& sessionId, uint8_t role, nsIPresentationServiceCallback *callback) override { return _to ReconnectSession(urls, sessionId, role, callback); } \
  NS_IMETHOD RegisterAvailabilityListener(const nsTArray<nsString> & availabilityUrls, nsIPresentationAvailabilityListener *listener) override { return _to RegisterAvailabilityListener(availabilityUrls, listener); } \
  NS_IMETHOD UnregisterAvailabilityListener(const nsTArray<nsString> & availabilityUrls, nsIPresentationAvailabilityListener *listener) override { return _to UnregisterAvailabilityListener(availabilityUrls, listener); } \
  NS_IMETHOD RegisterSessionListener(const nsAString& sessionId, uint8_t role, nsIPresentationSessionListener *listener) override { return _to RegisterSessionListener(sessionId, role, listener); } \
  NS_IMETHOD UnregisterSessionListener(const nsAString& sessionId, uint8_t role) override { return _to UnregisterSessionListener(sessionId, role); } \
  NS_IMETHOD RegisterRespondingListener(uint64_t windowId, nsIPresentationRespondingListener *listener) override { return _to RegisterRespondingListener(windowId, listener); } \
  NS_IMETHOD UnregisterRespondingListener(uint64_t windowId) override { return _to UnregisterRespondingListener(windowId); } \
  NS_IMETHOD NotifyReceiverReady(const nsAString& sessionId, uint64_t windowId, bool isLoading, nsIPresentationTransportBuilderConstructor *constructor) override { return _to NotifyReceiverReady(sessionId, windowId, isLoading, constructor); } \
  NS_IMETHOD NotifyTransportClosed(const nsAString& sessionId, uint8_t role, nsresult reason) override { return _to NotifyTransportClosed(sessionId, role, reason); } \
  NS_IMETHOD UntrackSessionInfo(const nsAString& sessionId, uint8_t role) override { return _to UntrackSessionInfo(sessionId, role); } \
  NS_IMETHOD GetWindowIdBySessionId(const nsAString& sessionId, uint8_t role, uint64_t *_retval) override { return _to GetWindowIdBySessionId(sessionId, role, _retval); } \
  NS_IMETHOD UpdateWindowIdBySessionId(const nsAString& sessionId, uint8_t role, uint64_t windowId) override { return _to UpdateWindowIdBySessionId(sessionId, role, windowId); } \
  NS_IMETHOD BuildTransport(const nsAString& sessionId, uint8_t role) override { return _to BuildTransport(sessionId, role); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRESENTATIONSERVICE(_to) \
  NS_IMETHOD StartSession(const nsTArray<nsString> & urls, const nsAString& sessionId, const nsAString& origin, const nsAString& deviceId, uint64_t windowId, mozilla::dom::EventTarget *eventTarget, nsIPrincipal *principal, nsIPresentationServiceCallback *callback, nsIPresentationTransportBuilderConstructor *constructor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartSession(urls, sessionId, origin, deviceId, windowId, eventTarget, principal, callback, constructor); } \
  NS_IMETHOD SendSessionMessage(const nsAString& sessionId, uint8_t role, const nsAString& data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendSessionMessage(sessionId, role, data); } \
  NS_IMETHOD SendSessionBinaryMsg(const nsAString& sessionId, uint8_t role, const nsACString& data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendSessionBinaryMsg(sessionId, role, data); } \
  NS_IMETHOD SendSessionBlob(const nsAString& sessionId, uint8_t role, mozilla::dom::Blob *blob) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendSessionBlob(sessionId, role, blob); } \
  NS_IMETHOD CloseSession(const nsAString& sessionId, uint8_t role, uint8_t closedReason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CloseSession(sessionId, role, closedReason); } \
  NS_IMETHOD TerminateSession(const nsAString& sessionId, uint8_t role) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TerminateSession(sessionId, role); } \
  NS_IMETHOD ReconnectSession(const nsTArray<nsString> & urls, const nsAString& sessionId, uint8_t role, nsIPresentationServiceCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReconnectSession(urls, sessionId, role, callback); } \
  NS_IMETHOD RegisterAvailabilityListener(const nsTArray<nsString> & availabilityUrls, nsIPresentationAvailabilityListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterAvailabilityListener(availabilityUrls, listener); } \
  NS_IMETHOD UnregisterAvailabilityListener(const nsTArray<nsString> & availabilityUrls, nsIPresentationAvailabilityListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterAvailabilityListener(availabilityUrls, listener); } \
  NS_IMETHOD RegisterSessionListener(const nsAString& sessionId, uint8_t role, nsIPresentationSessionListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterSessionListener(sessionId, role, listener); } \
  NS_IMETHOD UnregisterSessionListener(const nsAString& sessionId, uint8_t role) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterSessionListener(sessionId, role); } \
  NS_IMETHOD RegisterRespondingListener(uint64_t windowId, nsIPresentationRespondingListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterRespondingListener(windowId, listener); } \
  NS_IMETHOD UnregisterRespondingListener(uint64_t windowId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterRespondingListener(windowId); } \
  NS_IMETHOD NotifyReceiverReady(const nsAString& sessionId, uint64_t windowId, bool isLoading, nsIPresentationTransportBuilderConstructor *constructor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyReceiverReady(sessionId, windowId, isLoading, constructor); } \
  NS_IMETHOD NotifyTransportClosed(const nsAString& sessionId, uint8_t role, nsresult reason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyTransportClosed(sessionId, role, reason); } \
  NS_IMETHOD UntrackSessionInfo(const nsAString& sessionId, uint8_t role) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UntrackSessionInfo(sessionId, role); } \
  NS_IMETHOD GetWindowIdBySessionId(const nsAString& sessionId, uint8_t role, uint64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWindowIdBySessionId(sessionId, role, _retval); } \
  NS_IMETHOD UpdateWindowIdBySessionId(const nsAString& sessionId, uint8_t role, uint64_t windowId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateWindowIdBySessionId(sessionId, role, windowId); } \
  NS_IMETHOD BuildTransport(const nsAString& sessionId, uint8_t role) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BuildTransport(sessionId, role); } 


#endif /* __gen_nsIPresentationService_h__ */
