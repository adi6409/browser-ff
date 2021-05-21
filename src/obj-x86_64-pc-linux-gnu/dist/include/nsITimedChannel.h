/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsITimedChannel.idl
 */

#ifndef __gen_nsITimedChannel_h__
#define __gen_nsITimedChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */

class nsIPrincipal; /* forward declaration */

namespace mozilla {
class TimeStamp;
}
#include "nsTArrayForwardDeclare.h"
#include "nsCOMPtr.h"

/* starting interface:    nsIServerTiming */
#define NS_ISERVERTIMING_IID_STR "c2d9e95b-9cc9-4f47-9ef6-1de0cf7ebc75"

#define NS_ISERVERTIMING_IID \
  {0xc2d9e95b, 0x9cc9, 0x4f47, \
    { 0x9e, 0xf6, 0x1d, 0xe0, 0xcf, 0x7e, 0xbc, 0x75 }}

class NS_NO_VTABLE nsIServerTiming : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISERVERTIMING_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIServerTiming;

  /* [must_use] readonly attribute ACString name; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetName(nsACString& aName) = 0;

  /* [must_use] readonly attribute double duration; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetDuration(double *aDuration) = 0;

  /* [must_use] readonly attribute ACString description; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetDescription(nsACString& aDescription) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIServerTiming, NS_ISERVERTIMING_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISERVERTIMING \
  [[nodiscard]] NS_IMETHOD GetName(nsACString& aName) override; \
  [[nodiscard]] NS_IMETHOD GetDuration(double *aDuration) override; \
  [[nodiscard]] NS_IMETHOD GetDescription(nsACString& aDescription) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISERVERTIMING \
  [[nodiscard]] nsresult GetName(nsACString& aName); \
  [[nodiscard]] nsresult GetDuration(double *aDuration); \
  [[nodiscard]] nsresult GetDescription(nsACString& aDescription); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISERVERTIMING(_to) \
  [[nodiscard]] NS_IMETHOD GetName(nsACString& aName) override { return _to GetName(aName); } \
  [[nodiscard]] NS_IMETHOD GetDuration(double *aDuration) override { return _to GetDuration(aDuration); } \
  [[nodiscard]] NS_IMETHOD GetDescription(nsACString& aDescription) override { return _to GetDescription(aDescription); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISERVERTIMING(_to) \
  [[nodiscard]] NS_IMETHOD GetName(nsACString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  [[nodiscard]] NS_IMETHOD GetDuration(double *aDuration) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDuration(aDuration); } \
  [[nodiscard]] NS_IMETHOD GetDescription(nsACString& aDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDescription(aDescription); } 


/* starting interface:    nsITimedChannel */
#define NS_ITIMEDCHANNEL_IID_STR "ca63784d-959c-4c3a-9a59-234a2a520de0"

#define NS_ITIMEDCHANNEL_IID \
  {0xca63784d, 0x959c, 0x4c3a, \
    { 0x9a, 0x59, 0x23, 0x4a, 0x2a, 0x52, 0x0d, 0xe0 }}

class nsITimedChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITIMEDCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITimedChannel;

  /* attribute boolean timingEnabled; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTimingEnabled(bool *aTimingEnabled) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTimingEnabled(bool aTimingEnabled) = 0;

  /* attribute uint8_t redirectCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRedirectCount(uint8_t *aRedirectCount) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetRedirectCount(uint8_t aRedirectCount) = 0;

  /* attribute uint8_t internalRedirectCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInternalRedirectCount(uint8_t *aInternalRedirectCount) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetInternalRedirectCount(uint8_t aInternalRedirectCount) = 0;

  /* [noscript] attribute TimeStamp channelCreation; */
  NS_IMETHOD GetChannelCreation(mozilla::TimeStamp * aChannelCreation) = 0;
  NS_IMETHOD SetChannelCreation(mozilla::TimeStamp aChannelCreation) = 0;

  /* [noscript] attribute TimeStamp asyncOpen; */
  NS_IMETHOD GetAsyncOpen(mozilla::TimeStamp * aAsyncOpen) = 0;
  NS_IMETHOD SetAsyncOpen(mozilla::TimeStamp aAsyncOpen) = 0;

  /* [noscript] attribute TimeStamp launchServiceWorkerStart; */
  NS_IMETHOD GetLaunchServiceWorkerStart(mozilla::TimeStamp * aLaunchServiceWorkerStart) = 0;
  NS_IMETHOD SetLaunchServiceWorkerStart(mozilla::TimeStamp aLaunchServiceWorkerStart) = 0;

  /* [noscript] attribute TimeStamp launchServiceWorkerEnd; */
  NS_IMETHOD GetLaunchServiceWorkerEnd(mozilla::TimeStamp * aLaunchServiceWorkerEnd) = 0;
  NS_IMETHOD SetLaunchServiceWorkerEnd(mozilla::TimeStamp aLaunchServiceWorkerEnd) = 0;

  /* [noscript] attribute TimeStamp dispatchFetchEventStart; */
  NS_IMETHOD GetDispatchFetchEventStart(mozilla::TimeStamp * aDispatchFetchEventStart) = 0;
  NS_IMETHOD SetDispatchFetchEventStart(mozilla::TimeStamp aDispatchFetchEventStart) = 0;

  /* [noscript] attribute TimeStamp dispatchFetchEventEnd; */
  NS_IMETHOD GetDispatchFetchEventEnd(mozilla::TimeStamp * aDispatchFetchEventEnd) = 0;
  NS_IMETHOD SetDispatchFetchEventEnd(mozilla::TimeStamp aDispatchFetchEventEnd) = 0;

  /* [noscript] attribute TimeStamp handleFetchEventStart; */
  NS_IMETHOD GetHandleFetchEventStart(mozilla::TimeStamp * aHandleFetchEventStart) = 0;
  NS_IMETHOD SetHandleFetchEventStart(mozilla::TimeStamp aHandleFetchEventStart) = 0;

  /* [noscript] attribute TimeStamp handleFetchEventEnd; */
  NS_IMETHOD GetHandleFetchEventEnd(mozilla::TimeStamp * aHandleFetchEventEnd) = 0;
  NS_IMETHOD SetHandleFetchEventEnd(mozilla::TimeStamp aHandleFetchEventEnd) = 0;

  /* [noscript] readonly attribute TimeStamp domainLookupStart; */
  NS_IMETHOD GetDomainLookupStart(mozilla::TimeStamp * aDomainLookupStart) = 0;

  /* [noscript] readonly attribute TimeStamp domainLookupEnd; */
  NS_IMETHOD GetDomainLookupEnd(mozilla::TimeStamp * aDomainLookupEnd) = 0;

  /* [noscript] readonly attribute TimeStamp connectStart; */
  NS_IMETHOD GetConnectStart(mozilla::TimeStamp * aConnectStart) = 0;

  /* [noscript] readonly attribute TimeStamp tcpConnectEnd; */
  NS_IMETHOD GetTcpConnectEnd(mozilla::TimeStamp * aTcpConnectEnd) = 0;

  /* [noscript] readonly attribute TimeStamp secureConnectionStart; */
  NS_IMETHOD GetSecureConnectionStart(mozilla::TimeStamp * aSecureConnectionStart) = 0;

  /* [noscript] readonly attribute TimeStamp connectEnd; */
  NS_IMETHOD GetConnectEnd(mozilla::TimeStamp * aConnectEnd) = 0;

  /* [noscript] readonly attribute TimeStamp requestStart; */
  NS_IMETHOD GetRequestStart(mozilla::TimeStamp * aRequestStart) = 0;

  /* [noscript] readonly attribute TimeStamp responseStart; */
  NS_IMETHOD GetResponseStart(mozilla::TimeStamp * aResponseStart) = 0;

  /* [noscript] readonly attribute TimeStamp responseEnd; */
  NS_IMETHOD GetResponseEnd(mozilla::TimeStamp * aResponseEnd) = 0;

  /* [noscript] attribute TimeStamp redirectStart; */
  NS_IMETHOD GetRedirectStart(mozilla::TimeStamp * aRedirectStart) = 0;
  NS_IMETHOD SetRedirectStart(mozilla::TimeStamp aRedirectStart) = 0;

  /* [noscript] attribute TimeStamp redirectEnd; */
  NS_IMETHOD GetRedirectEnd(mozilla::TimeStamp * aRedirectEnd) = 0;
  NS_IMETHOD SetRedirectEnd(mozilla::TimeStamp aRedirectEnd) = 0;

  /* [noscript] attribute AString initiatorType; */
  NS_IMETHOD GetInitiatorType(nsAString& aInitiatorType) = 0;
  NS_IMETHOD SetInitiatorType(const nsAString& aInitiatorType) = 0;

  /* [noscript] attribute boolean allRedirectsSameOrigin; */
  NS_IMETHOD GetAllRedirectsSameOrigin(bool *aAllRedirectsSameOrigin) = 0;
  NS_IMETHOD SetAllRedirectsSameOrigin(bool aAllRedirectsSameOrigin) = 0;

  /* [noscript] attribute boolean allRedirectsPassTimingAllowCheck; */
  NS_IMETHOD GetAllRedirectsPassTimingAllowCheck(bool *aAllRedirectsPassTimingAllowCheck) = 0;
  NS_IMETHOD SetAllRedirectsPassTimingAllowCheck(bool aAllRedirectsPassTimingAllowCheck) = 0;

  /* [noscript] boolean timingAllowCheck (in nsIPrincipal origin); */
  NS_IMETHOD TimingAllowCheck(nsIPrincipal *origin, bool *_retval) = 0;

   inline bool TimingAllowCheck(nsIPrincipal* aOrigin) {
    bool allowed = false;
    return NS_SUCCEEDED(TimingAllowCheck(aOrigin, &allowed)) && allowed;
  }
    /* [noscript] readonly attribute TimeStamp cacheReadStart; */
  NS_IMETHOD GetCacheReadStart(mozilla::TimeStamp * aCacheReadStart) = 0;

  /* [noscript] readonly attribute TimeStamp cacheReadEnd; */
  NS_IMETHOD GetCacheReadEnd(mozilla::TimeStamp * aCacheReadEnd) = 0;

  /* readonly attribute PRTime channelCreationTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetChannelCreationTime(PRTime *aChannelCreationTime) = 0;

  /* readonly attribute PRTime asyncOpenTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAsyncOpenTime(PRTime *aAsyncOpenTime) = 0;

  /* readonly attribute PRTime launchServiceWorkerStartTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLaunchServiceWorkerStartTime(PRTime *aLaunchServiceWorkerStartTime) = 0;

  /* readonly attribute PRTime launchServiceWorkerEndTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLaunchServiceWorkerEndTime(PRTime *aLaunchServiceWorkerEndTime) = 0;

  /* readonly attribute PRTime dispatchFetchEventStartTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDispatchFetchEventStartTime(PRTime *aDispatchFetchEventStartTime) = 0;

  /* readonly attribute PRTime dispatchFetchEventEndTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDispatchFetchEventEndTime(PRTime *aDispatchFetchEventEndTime) = 0;

  /* readonly attribute PRTime handleFetchEventStartTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHandleFetchEventStartTime(PRTime *aHandleFetchEventStartTime) = 0;

  /* readonly attribute PRTime handleFetchEventEndTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHandleFetchEventEndTime(PRTime *aHandleFetchEventEndTime) = 0;

  /* readonly attribute PRTime domainLookupStartTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDomainLookupStartTime(PRTime *aDomainLookupStartTime) = 0;

  /* readonly attribute PRTime domainLookupEndTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDomainLookupEndTime(PRTime *aDomainLookupEndTime) = 0;

  /* readonly attribute PRTime connectStartTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetConnectStartTime(PRTime *aConnectStartTime) = 0;

  /* readonly attribute PRTime tcpConnectEndTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTcpConnectEndTime(PRTime *aTcpConnectEndTime) = 0;

  /* readonly attribute PRTime secureConnectionStartTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSecureConnectionStartTime(PRTime *aSecureConnectionStartTime) = 0;

  /* readonly attribute PRTime connectEndTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetConnectEndTime(PRTime *aConnectEndTime) = 0;

  /* readonly attribute PRTime requestStartTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRequestStartTime(PRTime *aRequestStartTime) = 0;

  /* readonly attribute PRTime responseStartTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetResponseStartTime(PRTime *aResponseStartTime) = 0;

  /* readonly attribute PRTime responseEndTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetResponseEndTime(PRTime *aResponseEndTime) = 0;

  /* readonly attribute PRTime cacheReadStartTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCacheReadStartTime(PRTime *aCacheReadStartTime) = 0;

  /* readonly attribute PRTime cacheReadEndTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCacheReadEndTime(PRTime *aCacheReadEndTime) = 0;

  /* readonly attribute PRTime redirectStartTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRedirectStartTime(PRTime *aRedirectStartTime) = 0;

  /* readonly attribute PRTime redirectEndTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRedirectEndTime(PRTime *aRedirectEndTime) = 0;

  /* [noscript] attribute boolean reportResourceTiming; */
  NS_IMETHOD GetReportResourceTiming(bool *aReportResourceTiming) = 0;
  NS_IMETHOD SetReportResourceTiming(bool aReportResourceTiming) = 0;

  /* readonly attribute nsIArray serverTiming; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetServerTiming(nsIArray **aServerTiming) = 0;

  /* nsServerTimingArrayRef getNativeServerTiming (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNativeServerTiming(nsTArray<nsCOMPtr<nsIServerTiming>> & _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITimedChannel, NS_ITIMEDCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITIMEDCHANNEL \
  NS_IMETHOD GetTimingEnabled(bool *aTimingEnabled) override; \
  NS_IMETHOD SetTimingEnabled(bool aTimingEnabled) override; \
  NS_IMETHOD GetRedirectCount(uint8_t *aRedirectCount) override; \
  NS_IMETHOD SetRedirectCount(uint8_t aRedirectCount) override; \
  NS_IMETHOD GetInternalRedirectCount(uint8_t *aInternalRedirectCount) override; \
  NS_IMETHOD SetInternalRedirectCount(uint8_t aInternalRedirectCount) override; \
  NS_IMETHOD GetChannelCreation(mozilla::TimeStamp * aChannelCreation) override; \
  NS_IMETHOD SetChannelCreation(mozilla::TimeStamp aChannelCreation) override; \
  NS_IMETHOD GetAsyncOpen(mozilla::TimeStamp * aAsyncOpen) override; \
  NS_IMETHOD SetAsyncOpen(mozilla::TimeStamp aAsyncOpen) override; \
  NS_IMETHOD GetLaunchServiceWorkerStart(mozilla::TimeStamp * aLaunchServiceWorkerStart) override; \
  NS_IMETHOD SetLaunchServiceWorkerStart(mozilla::TimeStamp aLaunchServiceWorkerStart) override; \
  NS_IMETHOD GetLaunchServiceWorkerEnd(mozilla::TimeStamp * aLaunchServiceWorkerEnd) override; \
  NS_IMETHOD SetLaunchServiceWorkerEnd(mozilla::TimeStamp aLaunchServiceWorkerEnd) override; \
  NS_IMETHOD GetDispatchFetchEventStart(mozilla::TimeStamp * aDispatchFetchEventStart) override; \
  NS_IMETHOD SetDispatchFetchEventStart(mozilla::TimeStamp aDispatchFetchEventStart) override; \
  NS_IMETHOD GetDispatchFetchEventEnd(mozilla::TimeStamp * aDispatchFetchEventEnd) override; \
  NS_IMETHOD SetDispatchFetchEventEnd(mozilla::TimeStamp aDispatchFetchEventEnd) override; \
  NS_IMETHOD GetHandleFetchEventStart(mozilla::TimeStamp * aHandleFetchEventStart) override; \
  NS_IMETHOD SetHandleFetchEventStart(mozilla::TimeStamp aHandleFetchEventStart) override; \
  NS_IMETHOD GetHandleFetchEventEnd(mozilla::TimeStamp * aHandleFetchEventEnd) override; \
  NS_IMETHOD SetHandleFetchEventEnd(mozilla::TimeStamp aHandleFetchEventEnd) override; \
  NS_IMETHOD GetDomainLookupStart(mozilla::TimeStamp * aDomainLookupStart) override; \
  NS_IMETHOD GetDomainLookupEnd(mozilla::TimeStamp * aDomainLookupEnd) override; \
  NS_IMETHOD GetConnectStart(mozilla::TimeStamp * aConnectStart) override; \
  NS_IMETHOD GetTcpConnectEnd(mozilla::TimeStamp * aTcpConnectEnd) override; \
  NS_IMETHOD GetSecureConnectionStart(mozilla::TimeStamp * aSecureConnectionStart) override; \
  NS_IMETHOD GetConnectEnd(mozilla::TimeStamp * aConnectEnd) override; \
  NS_IMETHOD GetRequestStart(mozilla::TimeStamp * aRequestStart) override; \
  NS_IMETHOD GetResponseStart(mozilla::TimeStamp * aResponseStart) override; \
  NS_IMETHOD GetResponseEnd(mozilla::TimeStamp * aResponseEnd) override; \
  NS_IMETHOD GetRedirectStart(mozilla::TimeStamp * aRedirectStart) override; \
  NS_IMETHOD SetRedirectStart(mozilla::TimeStamp aRedirectStart) override; \
  NS_IMETHOD GetRedirectEnd(mozilla::TimeStamp * aRedirectEnd) override; \
  NS_IMETHOD SetRedirectEnd(mozilla::TimeStamp aRedirectEnd) override; \
  NS_IMETHOD GetInitiatorType(nsAString& aInitiatorType) override; \
  NS_IMETHOD SetInitiatorType(const nsAString& aInitiatorType) override; \
  NS_IMETHOD GetAllRedirectsSameOrigin(bool *aAllRedirectsSameOrigin) override; \
  NS_IMETHOD SetAllRedirectsSameOrigin(bool aAllRedirectsSameOrigin) override; \
  NS_IMETHOD GetAllRedirectsPassTimingAllowCheck(bool *aAllRedirectsPassTimingAllowCheck) override; \
  NS_IMETHOD SetAllRedirectsPassTimingAllowCheck(bool aAllRedirectsPassTimingAllowCheck) override; \
  NS_IMETHOD TimingAllowCheck(nsIPrincipal *origin, bool *_retval) override; \
  NS_IMETHOD GetCacheReadStart(mozilla::TimeStamp * aCacheReadStart) override; \
  NS_IMETHOD GetCacheReadEnd(mozilla::TimeStamp * aCacheReadEnd) override; \
  NS_IMETHOD GetChannelCreationTime(PRTime *aChannelCreationTime) override; \
  NS_IMETHOD GetAsyncOpenTime(PRTime *aAsyncOpenTime) override; \
  NS_IMETHOD GetLaunchServiceWorkerStartTime(PRTime *aLaunchServiceWorkerStartTime) override; \
  NS_IMETHOD GetLaunchServiceWorkerEndTime(PRTime *aLaunchServiceWorkerEndTime) override; \
  NS_IMETHOD GetDispatchFetchEventStartTime(PRTime *aDispatchFetchEventStartTime) override; \
  NS_IMETHOD GetDispatchFetchEventEndTime(PRTime *aDispatchFetchEventEndTime) override; \
  NS_IMETHOD GetHandleFetchEventStartTime(PRTime *aHandleFetchEventStartTime) override; \
  NS_IMETHOD GetHandleFetchEventEndTime(PRTime *aHandleFetchEventEndTime) override; \
  NS_IMETHOD GetDomainLookupStartTime(PRTime *aDomainLookupStartTime) override; \
  NS_IMETHOD GetDomainLookupEndTime(PRTime *aDomainLookupEndTime) override; \
  NS_IMETHOD GetConnectStartTime(PRTime *aConnectStartTime) override; \
  NS_IMETHOD GetTcpConnectEndTime(PRTime *aTcpConnectEndTime) override; \
  NS_IMETHOD GetSecureConnectionStartTime(PRTime *aSecureConnectionStartTime) override; \
  NS_IMETHOD GetConnectEndTime(PRTime *aConnectEndTime) override; \
  NS_IMETHOD GetRequestStartTime(PRTime *aRequestStartTime) override; \
  NS_IMETHOD GetResponseStartTime(PRTime *aResponseStartTime) override; \
  NS_IMETHOD GetResponseEndTime(PRTime *aResponseEndTime) override; \
  NS_IMETHOD GetCacheReadStartTime(PRTime *aCacheReadStartTime) override; \
  NS_IMETHOD GetCacheReadEndTime(PRTime *aCacheReadEndTime) override; \
  NS_IMETHOD GetRedirectStartTime(PRTime *aRedirectStartTime) override; \
  NS_IMETHOD GetRedirectEndTime(PRTime *aRedirectEndTime) override; \
  NS_IMETHOD GetReportResourceTiming(bool *aReportResourceTiming) override; \
  NS_IMETHOD SetReportResourceTiming(bool aReportResourceTiming) override; \
  NS_IMETHOD GetServerTiming(nsIArray **aServerTiming) override; \
  NS_IMETHOD GetNativeServerTiming(nsTArray<nsCOMPtr<nsIServerTiming>> & _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITIMEDCHANNEL \
  nsresult GetTimingEnabled(bool *aTimingEnabled); \
  nsresult SetTimingEnabled(bool aTimingEnabled); \
  nsresult GetRedirectCount(uint8_t *aRedirectCount); \
  nsresult SetRedirectCount(uint8_t aRedirectCount); \
  nsresult GetInternalRedirectCount(uint8_t *aInternalRedirectCount); \
  nsresult SetInternalRedirectCount(uint8_t aInternalRedirectCount); \
  nsresult GetChannelCreation(mozilla::TimeStamp * aChannelCreation); \
  nsresult SetChannelCreation(mozilla::TimeStamp aChannelCreation); \
  nsresult GetAsyncOpen(mozilla::TimeStamp * aAsyncOpen); \
  nsresult SetAsyncOpen(mozilla::TimeStamp aAsyncOpen); \
  nsresult GetLaunchServiceWorkerStart(mozilla::TimeStamp * aLaunchServiceWorkerStart); \
  nsresult SetLaunchServiceWorkerStart(mozilla::TimeStamp aLaunchServiceWorkerStart); \
  nsresult GetLaunchServiceWorkerEnd(mozilla::TimeStamp * aLaunchServiceWorkerEnd); \
  nsresult SetLaunchServiceWorkerEnd(mozilla::TimeStamp aLaunchServiceWorkerEnd); \
  nsresult GetDispatchFetchEventStart(mozilla::TimeStamp * aDispatchFetchEventStart); \
  nsresult SetDispatchFetchEventStart(mozilla::TimeStamp aDispatchFetchEventStart); \
  nsresult GetDispatchFetchEventEnd(mozilla::TimeStamp * aDispatchFetchEventEnd); \
  nsresult SetDispatchFetchEventEnd(mozilla::TimeStamp aDispatchFetchEventEnd); \
  nsresult GetHandleFetchEventStart(mozilla::TimeStamp * aHandleFetchEventStart); \
  nsresult SetHandleFetchEventStart(mozilla::TimeStamp aHandleFetchEventStart); \
  nsresult GetHandleFetchEventEnd(mozilla::TimeStamp * aHandleFetchEventEnd); \
  nsresult SetHandleFetchEventEnd(mozilla::TimeStamp aHandleFetchEventEnd); \
  nsresult GetDomainLookupStart(mozilla::TimeStamp * aDomainLookupStart); \
  nsresult GetDomainLookupEnd(mozilla::TimeStamp * aDomainLookupEnd); \
  nsresult GetConnectStart(mozilla::TimeStamp * aConnectStart); \
  nsresult GetTcpConnectEnd(mozilla::TimeStamp * aTcpConnectEnd); \
  nsresult GetSecureConnectionStart(mozilla::TimeStamp * aSecureConnectionStart); \
  nsresult GetConnectEnd(mozilla::TimeStamp * aConnectEnd); \
  nsresult GetRequestStart(mozilla::TimeStamp * aRequestStart); \
  nsresult GetResponseStart(mozilla::TimeStamp * aResponseStart); \
  nsresult GetResponseEnd(mozilla::TimeStamp * aResponseEnd); \
  nsresult GetRedirectStart(mozilla::TimeStamp * aRedirectStart); \
  nsresult SetRedirectStart(mozilla::TimeStamp aRedirectStart); \
  nsresult GetRedirectEnd(mozilla::TimeStamp * aRedirectEnd); \
  nsresult SetRedirectEnd(mozilla::TimeStamp aRedirectEnd); \
  nsresult GetInitiatorType(nsAString& aInitiatorType); \
  nsresult SetInitiatorType(const nsAString& aInitiatorType); \
  nsresult GetAllRedirectsSameOrigin(bool *aAllRedirectsSameOrigin); \
  nsresult SetAllRedirectsSameOrigin(bool aAllRedirectsSameOrigin); \
  nsresult GetAllRedirectsPassTimingAllowCheck(bool *aAllRedirectsPassTimingAllowCheck); \
  nsresult SetAllRedirectsPassTimingAllowCheck(bool aAllRedirectsPassTimingAllowCheck); \
  nsresult TimingAllowCheck(nsIPrincipal *origin, bool *_retval); \
  nsresult GetCacheReadStart(mozilla::TimeStamp * aCacheReadStart); \
  nsresult GetCacheReadEnd(mozilla::TimeStamp * aCacheReadEnd); \
  nsresult GetChannelCreationTime(PRTime *aChannelCreationTime); \
  nsresult GetAsyncOpenTime(PRTime *aAsyncOpenTime); \
  nsresult GetLaunchServiceWorkerStartTime(PRTime *aLaunchServiceWorkerStartTime); \
  nsresult GetLaunchServiceWorkerEndTime(PRTime *aLaunchServiceWorkerEndTime); \
  nsresult GetDispatchFetchEventStartTime(PRTime *aDispatchFetchEventStartTime); \
  nsresult GetDispatchFetchEventEndTime(PRTime *aDispatchFetchEventEndTime); \
  nsresult GetHandleFetchEventStartTime(PRTime *aHandleFetchEventStartTime); \
  nsresult GetHandleFetchEventEndTime(PRTime *aHandleFetchEventEndTime); \
  nsresult GetDomainLookupStartTime(PRTime *aDomainLookupStartTime); \
  nsresult GetDomainLookupEndTime(PRTime *aDomainLookupEndTime); \
  nsresult GetConnectStartTime(PRTime *aConnectStartTime); \
  nsresult GetTcpConnectEndTime(PRTime *aTcpConnectEndTime); \
  nsresult GetSecureConnectionStartTime(PRTime *aSecureConnectionStartTime); \
  nsresult GetConnectEndTime(PRTime *aConnectEndTime); \
  nsresult GetRequestStartTime(PRTime *aRequestStartTime); \
  nsresult GetResponseStartTime(PRTime *aResponseStartTime); \
  nsresult GetResponseEndTime(PRTime *aResponseEndTime); \
  nsresult GetCacheReadStartTime(PRTime *aCacheReadStartTime); \
  nsresult GetCacheReadEndTime(PRTime *aCacheReadEndTime); \
  nsresult GetRedirectStartTime(PRTime *aRedirectStartTime); \
  nsresult GetRedirectEndTime(PRTime *aRedirectEndTime); \
  nsresult GetReportResourceTiming(bool *aReportResourceTiming); \
  nsresult SetReportResourceTiming(bool aReportResourceTiming); \
  nsresult GetServerTiming(nsIArray **aServerTiming); \
  nsresult GetNativeServerTiming(nsTArray<nsCOMPtr<nsIServerTiming>> & _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITIMEDCHANNEL(_to) \
  NS_IMETHOD GetTimingEnabled(bool *aTimingEnabled) override { return _to GetTimingEnabled(aTimingEnabled); } \
  NS_IMETHOD SetTimingEnabled(bool aTimingEnabled) override { return _to SetTimingEnabled(aTimingEnabled); } \
  NS_IMETHOD GetRedirectCount(uint8_t *aRedirectCount) override { return _to GetRedirectCount(aRedirectCount); } \
  NS_IMETHOD SetRedirectCount(uint8_t aRedirectCount) override { return _to SetRedirectCount(aRedirectCount); } \
  NS_IMETHOD GetInternalRedirectCount(uint8_t *aInternalRedirectCount) override { return _to GetInternalRedirectCount(aInternalRedirectCount); } \
  NS_IMETHOD SetInternalRedirectCount(uint8_t aInternalRedirectCount) override { return _to SetInternalRedirectCount(aInternalRedirectCount); } \
  NS_IMETHOD GetChannelCreation(mozilla::TimeStamp * aChannelCreation) override { return _to GetChannelCreation(aChannelCreation); } \
  NS_IMETHOD SetChannelCreation(mozilla::TimeStamp aChannelCreation) override { return _to SetChannelCreation(aChannelCreation); } \
  NS_IMETHOD GetAsyncOpen(mozilla::TimeStamp * aAsyncOpen) override { return _to GetAsyncOpen(aAsyncOpen); } \
  NS_IMETHOD SetAsyncOpen(mozilla::TimeStamp aAsyncOpen) override { return _to SetAsyncOpen(aAsyncOpen); } \
  NS_IMETHOD GetLaunchServiceWorkerStart(mozilla::TimeStamp * aLaunchServiceWorkerStart) override { return _to GetLaunchServiceWorkerStart(aLaunchServiceWorkerStart); } \
  NS_IMETHOD SetLaunchServiceWorkerStart(mozilla::TimeStamp aLaunchServiceWorkerStart) override { return _to SetLaunchServiceWorkerStart(aLaunchServiceWorkerStart); } \
  NS_IMETHOD GetLaunchServiceWorkerEnd(mozilla::TimeStamp * aLaunchServiceWorkerEnd) override { return _to GetLaunchServiceWorkerEnd(aLaunchServiceWorkerEnd); } \
  NS_IMETHOD SetLaunchServiceWorkerEnd(mozilla::TimeStamp aLaunchServiceWorkerEnd) override { return _to SetLaunchServiceWorkerEnd(aLaunchServiceWorkerEnd); } \
  NS_IMETHOD GetDispatchFetchEventStart(mozilla::TimeStamp * aDispatchFetchEventStart) override { return _to GetDispatchFetchEventStart(aDispatchFetchEventStart); } \
  NS_IMETHOD SetDispatchFetchEventStart(mozilla::TimeStamp aDispatchFetchEventStart) override { return _to SetDispatchFetchEventStart(aDispatchFetchEventStart); } \
  NS_IMETHOD GetDispatchFetchEventEnd(mozilla::TimeStamp * aDispatchFetchEventEnd) override { return _to GetDispatchFetchEventEnd(aDispatchFetchEventEnd); } \
  NS_IMETHOD SetDispatchFetchEventEnd(mozilla::TimeStamp aDispatchFetchEventEnd) override { return _to SetDispatchFetchEventEnd(aDispatchFetchEventEnd); } \
  NS_IMETHOD GetHandleFetchEventStart(mozilla::TimeStamp * aHandleFetchEventStart) override { return _to GetHandleFetchEventStart(aHandleFetchEventStart); } \
  NS_IMETHOD SetHandleFetchEventStart(mozilla::TimeStamp aHandleFetchEventStart) override { return _to SetHandleFetchEventStart(aHandleFetchEventStart); } \
  NS_IMETHOD GetHandleFetchEventEnd(mozilla::TimeStamp * aHandleFetchEventEnd) override { return _to GetHandleFetchEventEnd(aHandleFetchEventEnd); } \
  NS_IMETHOD SetHandleFetchEventEnd(mozilla::TimeStamp aHandleFetchEventEnd) override { return _to SetHandleFetchEventEnd(aHandleFetchEventEnd); } \
  NS_IMETHOD GetDomainLookupStart(mozilla::TimeStamp * aDomainLookupStart) override { return _to GetDomainLookupStart(aDomainLookupStart); } \
  NS_IMETHOD GetDomainLookupEnd(mozilla::TimeStamp * aDomainLookupEnd) override { return _to GetDomainLookupEnd(aDomainLookupEnd); } \
  NS_IMETHOD GetConnectStart(mozilla::TimeStamp * aConnectStart) override { return _to GetConnectStart(aConnectStart); } \
  NS_IMETHOD GetTcpConnectEnd(mozilla::TimeStamp * aTcpConnectEnd) override { return _to GetTcpConnectEnd(aTcpConnectEnd); } \
  NS_IMETHOD GetSecureConnectionStart(mozilla::TimeStamp * aSecureConnectionStart) override { return _to GetSecureConnectionStart(aSecureConnectionStart); } \
  NS_IMETHOD GetConnectEnd(mozilla::TimeStamp * aConnectEnd) override { return _to GetConnectEnd(aConnectEnd); } \
  NS_IMETHOD GetRequestStart(mozilla::TimeStamp * aRequestStart) override { return _to GetRequestStart(aRequestStart); } \
  NS_IMETHOD GetResponseStart(mozilla::TimeStamp * aResponseStart) override { return _to GetResponseStart(aResponseStart); } \
  NS_IMETHOD GetResponseEnd(mozilla::TimeStamp * aResponseEnd) override { return _to GetResponseEnd(aResponseEnd); } \
  NS_IMETHOD GetRedirectStart(mozilla::TimeStamp * aRedirectStart) override { return _to GetRedirectStart(aRedirectStart); } \
  NS_IMETHOD SetRedirectStart(mozilla::TimeStamp aRedirectStart) override { return _to SetRedirectStart(aRedirectStart); } \
  NS_IMETHOD GetRedirectEnd(mozilla::TimeStamp * aRedirectEnd) override { return _to GetRedirectEnd(aRedirectEnd); } \
  NS_IMETHOD SetRedirectEnd(mozilla::TimeStamp aRedirectEnd) override { return _to SetRedirectEnd(aRedirectEnd); } \
  NS_IMETHOD GetInitiatorType(nsAString& aInitiatorType) override { return _to GetInitiatorType(aInitiatorType); } \
  NS_IMETHOD SetInitiatorType(const nsAString& aInitiatorType) override { return _to SetInitiatorType(aInitiatorType); } \
  NS_IMETHOD GetAllRedirectsSameOrigin(bool *aAllRedirectsSameOrigin) override { return _to GetAllRedirectsSameOrigin(aAllRedirectsSameOrigin); } \
  NS_IMETHOD SetAllRedirectsSameOrigin(bool aAllRedirectsSameOrigin) override { return _to SetAllRedirectsSameOrigin(aAllRedirectsSameOrigin); } \
  NS_IMETHOD GetAllRedirectsPassTimingAllowCheck(bool *aAllRedirectsPassTimingAllowCheck) override { return _to GetAllRedirectsPassTimingAllowCheck(aAllRedirectsPassTimingAllowCheck); } \
  NS_IMETHOD SetAllRedirectsPassTimingAllowCheck(bool aAllRedirectsPassTimingAllowCheck) override { return _to SetAllRedirectsPassTimingAllowCheck(aAllRedirectsPassTimingAllowCheck); } \
  NS_IMETHOD TimingAllowCheck(nsIPrincipal *origin, bool *_retval) override { return _to TimingAllowCheck(origin, _retval); } \
  NS_IMETHOD GetCacheReadStart(mozilla::TimeStamp * aCacheReadStart) override { return _to GetCacheReadStart(aCacheReadStart); } \
  NS_IMETHOD GetCacheReadEnd(mozilla::TimeStamp * aCacheReadEnd) override { return _to GetCacheReadEnd(aCacheReadEnd); } \
  NS_IMETHOD GetChannelCreationTime(PRTime *aChannelCreationTime) override { return _to GetChannelCreationTime(aChannelCreationTime); } \
  NS_IMETHOD GetAsyncOpenTime(PRTime *aAsyncOpenTime) override { return _to GetAsyncOpenTime(aAsyncOpenTime); } \
  NS_IMETHOD GetLaunchServiceWorkerStartTime(PRTime *aLaunchServiceWorkerStartTime) override { return _to GetLaunchServiceWorkerStartTime(aLaunchServiceWorkerStartTime); } \
  NS_IMETHOD GetLaunchServiceWorkerEndTime(PRTime *aLaunchServiceWorkerEndTime) override { return _to GetLaunchServiceWorkerEndTime(aLaunchServiceWorkerEndTime); } \
  NS_IMETHOD GetDispatchFetchEventStartTime(PRTime *aDispatchFetchEventStartTime) override { return _to GetDispatchFetchEventStartTime(aDispatchFetchEventStartTime); } \
  NS_IMETHOD GetDispatchFetchEventEndTime(PRTime *aDispatchFetchEventEndTime) override { return _to GetDispatchFetchEventEndTime(aDispatchFetchEventEndTime); } \
  NS_IMETHOD GetHandleFetchEventStartTime(PRTime *aHandleFetchEventStartTime) override { return _to GetHandleFetchEventStartTime(aHandleFetchEventStartTime); } \
  NS_IMETHOD GetHandleFetchEventEndTime(PRTime *aHandleFetchEventEndTime) override { return _to GetHandleFetchEventEndTime(aHandleFetchEventEndTime); } \
  NS_IMETHOD GetDomainLookupStartTime(PRTime *aDomainLookupStartTime) override { return _to GetDomainLookupStartTime(aDomainLookupStartTime); } \
  NS_IMETHOD GetDomainLookupEndTime(PRTime *aDomainLookupEndTime) override { return _to GetDomainLookupEndTime(aDomainLookupEndTime); } \
  NS_IMETHOD GetConnectStartTime(PRTime *aConnectStartTime) override { return _to GetConnectStartTime(aConnectStartTime); } \
  NS_IMETHOD GetTcpConnectEndTime(PRTime *aTcpConnectEndTime) override { return _to GetTcpConnectEndTime(aTcpConnectEndTime); } \
  NS_IMETHOD GetSecureConnectionStartTime(PRTime *aSecureConnectionStartTime) override { return _to GetSecureConnectionStartTime(aSecureConnectionStartTime); } \
  NS_IMETHOD GetConnectEndTime(PRTime *aConnectEndTime) override { return _to GetConnectEndTime(aConnectEndTime); } \
  NS_IMETHOD GetRequestStartTime(PRTime *aRequestStartTime) override { return _to GetRequestStartTime(aRequestStartTime); } \
  NS_IMETHOD GetResponseStartTime(PRTime *aResponseStartTime) override { return _to GetResponseStartTime(aResponseStartTime); } \
  NS_IMETHOD GetResponseEndTime(PRTime *aResponseEndTime) override { return _to GetResponseEndTime(aResponseEndTime); } \
  NS_IMETHOD GetCacheReadStartTime(PRTime *aCacheReadStartTime) override { return _to GetCacheReadStartTime(aCacheReadStartTime); } \
  NS_IMETHOD GetCacheReadEndTime(PRTime *aCacheReadEndTime) override { return _to GetCacheReadEndTime(aCacheReadEndTime); } \
  NS_IMETHOD GetRedirectStartTime(PRTime *aRedirectStartTime) override { return _to GetRedirectStartTime(aRedirectStartTime); } \
  NS_IMETHOD GetRedirectEndTime(PRTime *aRedirectEndTime) override { return _to GetRedirectEndTime(aRedirectEndTime); } \
  NS_IMETHOD GetReportResourceTiming(bool *aReportResourceTiming) override { return _to GetReportResourceTiming(aReportResourceTiming); } \
  NS_IMETHOD SetReportResourceTiming(bool aReportResourceTiming) override { return _to SetReportResourceTiming(aReportResourceTiming); } \
  NS_IMETHOD GetServerTiming(nsIArray **aServerTiming) override { return _to GetServerTiming(aServerTiming); } \
  NS_IMETHOD GetNativeServerTiming(nsTArray<nsCOMPtr<nsIServerTiming>> & _retval) override { return _to GetNativeServerTiming(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITIMEDCHANNEL(_to) \
  NS_IMETHOD GetTimingEnabled(bool *aTimingEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTimingEnabled(aTimingEnabled); } \
  NS_IMETHOD SetTimingEnabled(bool aTimingEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTimingEnabled(aTimingEnabled); } \
  NS_IMETHOD GetRedirectCount(uint8_t *aRedirectCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRedirectCount(aRedirectCount); } \
  NS_IMETHOD SetRedirectCount(uint8_t aRedirectCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRedirectCount(aRedirectCount); } \
  NS_IMETHOD GetInternalRedirectCount(uint8_t *aInternalRedirectCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInternalRedirectCount(aInternalRedirectCount); } \
  NS_IMETHOD SetInternalRedirectCount(uint8_t aInternalRedirectCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInternalRedirectCount(aInternalRedirectCount); } \
  NS_IMETHOD GetChannelCreation(mozilla::TimeStamp * aChannelCreation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChannelCreation(aChannelCreation); } \
  NS_IMETHOD SetChannelCreation(mozilla::TimeStamp aChannelCreation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetChannelCreation(aChannelCreation); } \
  NS_IMETHOD GetAsyncOpen(mozilla::TimeStamp * aAsyncOpen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsyncOpen(aAsyncOpen); } \
  NS_IMETHOD SetAsyncOpen(mozilla::TimeStamp aAsyncOpen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsyncOpen(aAsyncOpen); } \
  NS_IMETHOD GetLaunchServiceWorkerStart(mozilla::TimeStamp * aLaunchServiceWorkerStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLaunchServiceWorkerStart(aLaunchServiceWorkerStart); } \
  NS_IMETHOD SetLaunchServiceWorkerStart(mozilla::TimeStamp aLaunchServiceWorkerStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLaunchServiceWorkerStart(aLaunchServiceWorkerStart); } \
  NS_IMETHOD GetLaunchServiceWorkerEnd(mozilla::TimeStamp * aLaunchServiceWorkerEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLaunchServiceWorkerEnd(aLaunchServiceWorkerEnd); } \
  NS_IMETHOD SetLaunchServiceWorkerEnd(mozilla::TimeStamp aLaunchServiceWorkerEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLaunchServiceWorkerEnd(aLaunchServiceWorkerEnd); } \
  NS_IMETHOD GetDispatchFetchEventStart(mozilla::TimeStamp * aDispatchFetchEventStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDispatchFetchEventStart(aDispatchFetchEventStart); } \
  NS_IMETHOD SetDispatchFetchEventStart(mozilla::TimeStamp aDispatchFetchEventStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDispatchFetchEventStart(aDispatchFetchEventStart); } \
  NS_IMETHOD GetDispatchFetchEventEnd(mozilla::TimeStamp * aDispatchFetchEventEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDispatchFetchEventEnd(aDispatchFetchEventEnd); } \
  NS_IMETHOD SetDispatchFetchEventEnd(mozilla::TimeStamp aDispatchFetchEventEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDispatchFetchEventEnd(aDispatchFetchEventEnd); } \
  NS_IMETHOD GetHandleFetchEventStart(mozilla::TimeStamp * aHandleFetchEventStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHandleFetchEventStart(aHandleFetchEventStart); } \
  NS_IMETHOD SetHandleFetchEventStart(mozilla::TimeStamp aHandleFetchEventStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHandleFetchEventStart(aHandleFetchEventStart); } \
  NS_IMETHOD GetHandleFetchEventEnd(mozilla::TimeStamp * aHandleFetchEventEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHandleFetchEventEnd(aHandleFetchEventEnd); } \
  NS_IMETHOD SetHandleFetchEventEnd(mozilla::TimeStamp aHandleFetchEventEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHandleFetchEventEnd(aHandleFetchEventEnd); } \
  NS_IMETHOD GetDomainLookupStart(mozilla::TimeStamp * aDomainLookupStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDomainLookupStart(aDomainLookupStart); } \
  NS_IMETHOD GetDomainLookupEnd(mozilla::TimeStamp * aDomainLookupEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDomainLookupEnd(aDomainLookupEnd); } \
  NS_IMETHOD GetConnectStart(mozilla::TimeStamp * aConnectStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConnectStart(aConnectStart); } \
  NS_IMETHOD GetTcpConnectEnd(mozilla::TimeStamp * aTcpConnectEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTcpConnectEnd(aTcpConnectEnd); } \
  NS_IMETHOD GetSecureConnectionStart(mozilla::TimeStamp * aSecureConnectionStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecureConnectionStart(aSecureConnectionStart); } \
  NS_IMETHOD GetConnectEnd(mozilla::TimeStamp * aConnectEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConnectEnd(aConnectEnd); } \
  NS_IMETHOD GetRequestStart(mozilla::TimeStamp * aRequestStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestStart(aRequestStart); } \
  NS_IMETHOD GetResponseStart(mozilla::TimeStamp * aResponseStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResponseStart(aResponseStart); } \
  NS_IMETHOD GetResponseEnd(mozilla::TimeStamp * aResponseEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResponseEnd(aResponseEnd); } \
  NS_IMETHOD GetRedirectStart(mozilla::TimeStamp * aRedirectStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRedirectStart(aRedirectStart); } \
  NS_IMETHOD SetRedirectStart(mozilla::TimeStamp aRedirectStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRedirectStart(aRedirectStart); } \
  NS_IMETHOD GetRedirectEnd(mozilla::TimeStamp * aRedirectEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRedirectEnd(aRedirectEnd); } \
  NS_IMETHOD SetRedirectEnd(mozilla::TimeStamp aRedirectEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRedirectEnd(aRedirectEnd); } \
  NS_IMETHOD GetInitiatorType(nsAString& aInitiatorType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInitiatorType(aInitiatorType); } \
  NS_IMETHOD SetInitiatorType(const nsAString& aInitiatorType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInitiatorType(aInitiatorType); } \
  NS_IMETHOD GetAllRedirectsSameOrigin(bool *aAllRedirectsSameOrigin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllRedirectsSameOrigin(aAllRedirectsSameOrigin); } \
  NS_IMETHOD SetAllRedirectsSameOrigin(bool aAllRedirectsSameOrigin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllRedirectsSameOrigin(aAllRedirectsSameOrigin); } \
  NS_IMETHOD GetAllRedirectsPassTimingAllowCheck(bool *aAllRedirectsPassTimingAllowCheck) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllRedirectsPassTimingAllowCheck(aAllRedirectsPassTimingAllowCheck); } \
  NS_IMETHOD SetAllRedirectsPassTimingAllowCheck(bool aAllRedirectsPassTimingAllowCheck) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllRedirectsPassTimingAllowCheck(aAllRedirectsPassTimingAllowCheck); } \
  NS_IMETHOD TimingAllowCheck(nsIPrincipal *origin, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TimingAllowCheck(origin, _retval); } \
  NS_IMETHOD GetCacheReadStart(mozilla::TimeStamp * aCacheReadStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheReadStart(aCacheReadStart); } \
  NS_IMETHOD GetCacheReadEnd(mozilla::TimeStamp * aCacheReadEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheReadEnd(aCacheReadEnd); } \
  NS_IMETHOD GetChannelCreationTime(PRTime *aChannelCreationTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChannelCreationTime(aChannelCreationTime); } \
  NS_IMETHOD GetAsyncOpenTime(PRTime *aAsyncOpenTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsyncOpenTime(aAsyncOpenTime); } \
  NS_IMETHOD GetLaunchServiceWorkerStartTime(PRTime *aLaunchServiceWorkerStartTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLaunchServiceWorkerStartTime(aLaunchServiceWorkerStartTime); } \
  NS_IMETHOD GetLaunchServiceWorkerEndTime(PRTime *aLaunchServiceWorkerEndTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLaunchServiceWorkerEndTime(aLaunchServiceWorkerEndTime); } \
  NS_IMETHOD GetDispatchFetchEventStartTime(PRTime *aDispatchFetchEventStartTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDispatchFetchEventStartTime(aDispatchFetchEventStartTime); } \
  NS_IMETHOD GetDispatchFetchEventEndTime(PRTime *aDispatchFetchEventEndTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDispatchFetchEventEndTime(aDispatchFetchEventEndTime); } \
  NS_IMETHOD GetHandleFetchEventStartTime(PRTime *aHandleFetchEventStartTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHandleFetchEventStartTime(aHandleFetchEventStartTime); } \
  NS_IMETHOD GetHandleFetchEventEndTime(PRTime *aHandleFetchEventEndTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHandleFetchEventEndTime(aHandleFetchEventEndTime); } \
  NS_IMETHOD GetDomainLookupStartTime(PRTime *aDomainLookupStartTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDomainLookupStartTime(aDomainLookupStartTime); } \
  NS_IMETHOD GetDomainLookupEndTime(PRTime *aDomainLookupEndTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDomainLookupEndTime(aDomainLookupEndTime); } \
  NS_IMETHOD GetConnectStartTime(PRTime *aConnectStartTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConnectStartTime(aConnectStartTime); } \
  NS_IMETHOD GetTcpConnectEndTime(PRTime *aTcpConnectEndTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTcpConnectEndTime(aTcpConnectEndTime); } \
  NS_IMETHOD GetSecureConnectionStartTime(PRTime *aSecureConnectionStartTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecureConnectionStartTime(aSecureConnectionStartTime); } \
  NS_IMETHOD GetConnectEndTime(PRTime *aConnectEndTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConnectEndTime(aConnectEndTime); } \
  NS_IMETHOD GetRequestStartTime(PRTime *aRequestStartTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestStartTime(aRequestStartTime); } \
  NS_IMETHOD GetResponseStartTime(PRTime *aResponseStartTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResponseStartTime(aResponseStartTime); } \
  NS_IMETHOD GetResponseEndTime(PRTime *aResponseEndTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResponseEndTime(aResponseEndTime); } \
  NS_IMETHOD GetCacheReadStartTime(PRTime *aCacheReadStartTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheReadStartTime(aCacheReadStartTime); } \
  NS_IMETHOD GetCacheReadEndTime(PRTime *aCacheReadEndTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheReadEndTime(aCacheReadEndTime); } \
  NS_IMETHOD GetRedirectStartTime(PRTime *aRedirectStartTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRedirectStartTime(aRedirectStartTime); } \
  NS_IMETHOD GetRedirectEndTime(PRTime *aRedirectEndTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRedirectEndTime(aRedirectEndTime); } \
  NS_IMETHOD GetReportResourceTiming(bool *aReportResourceTiming) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReportResourceTiming(aReportResourceTiming); } \
  NS_IMETHOD SetReportResourceTiming(bool aReportResourceTiming) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetReportResourceTiming(aReportResourceTiming); } \
  NS_IMETHOD GetServerTiming(nsIArray **aServerTiming) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetServerTiming(aServerTiming); } \
  NS_IMETHOD GetNativeServerTiming(nsTArray<nsCOMPtr<nsIServerTiming>> & _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNativeServerTiming(_retval); } 


#endif /* __gen_nsITimedChannel_h__ */
