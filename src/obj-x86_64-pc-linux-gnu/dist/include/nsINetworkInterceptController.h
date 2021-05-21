/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkInterceptController.idl
 */

#ifndef __gen_nsINetworkInterceptController_h__
#define __gen_nsINetworkInterceptController_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIContentPolicy_h__
#include "nsIContentPolicy.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsICacheInfoChannel; /* forward declaration */

class nsIChannel; /* forward declaration */

class nsIConsoleReportCollector; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsIOutputStream; /* forward declaration */

class nsIURI; /* forward declaration */

#include "nsContentUtils.h"
#include "nsIChannel.h"
#include "nsIConsoleReportCollector.h"
#include "nsILoadInfo.h"
namespace mozilla {
class TimeStamp;
namespace dom {
class ChannelInfo;
}
}

/* starting interface:    nsIInterceptedBodyCallback */
#define NS_IINTERCEPTEDBODYCALLBACK_IID_STR "51039eb6-bea0-40c7-b523-ccab56cc4fde"

#define NS_IINTERCEPTEDBODYCALLBACK_IID \
  {0x51039eb6, 0xbea0, 0x40c7, \
    { 0xb5, 0x23, 0xcc, 0xab, 0x56, 0xcc, 0x4f, 0xde }}

class NS_NO_VTABLE nsIInterceptedBodyCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINTERCEPTEDBODYCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIInterceptedBodyCallback;

  /* void bodyComplete (in nsresult aRv); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD BodyComplete(nsresult aRv) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIInterceptedBodyCallback, NS_IINTERCEPTEDBODYCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINTERCEPTEDBODYCALLBACK \
  NS_IMETHOD BodyComplete(nsresult aRv) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINTERCEPTEDBODYCALLBACK \
  nsresult BodyComplete(nsresult aRv); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINTERCEPTEDBODYCALLBACK(_to) \
  NS_IMETHOD BodyComplete(nsresult aRv) override { return _to BodyComplete(aRv); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINTERCEPTEDBODYCALLBACK(_to) \
  NS_IMETHOD BodyComplete(nsresult aRv) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BodyComplete(aRv); } 


/* starting interface:    nsIInterceptedChannel */
#define NS_IINTERCEPTEDCHANNEL_IID_STR "f4b82975-6a86-4cc4-87fe-9a1fd430c86d"

#define NS_IINTERCEPTEDCHANNEL_IID \
  {0xf4b82975, 0x6a86, 0x4cc4, \
    { 0x87, 0xfe, 0x9a, 0x1f, 0xd4, 0x30, 0xc8, 0x6d }}

class nsIInterceptedChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINTERCEPTEDCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIInterceptedChannel;

  /* void resetInterception (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ResetInterception(void) = 0;

  /* void synthesizeStatus (in uint16_t status, in ACString reason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SynthesizeStatus(uint16_t status, const nsACString& reason) = 0;

  /* void synthesizeHeader (in ACString name, in ACString value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SynthesizeHeader(const nsACString& name, const nsACString& value) = 0;

  /* void startSynthesizedResponse (in nsIInputStream body, in nsIInterceptedBodyCallback callback, in nsICacheInfoChannel channel, in ACString finalURLSpec, in bool responseRedirected); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StartSynthesizedResponse(nsIInputStream *body, nsIInterceptedBodyCallback *callback, nsICacheInfoChannel *channel, const nsACString& finalURLSpec, bool responseRedirected) = 0;

  /* void finishSynthesizedResponse (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FinishSynthesizedResponse(void) = 0;

  /* void cancelInterception (in nsresult status); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CancelInterception(nsresult status) = 0;

  /* readonly attribute nsIChannel channel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetChannel(nsIChannel **aChannel) = 0;

  /* readonly attribute nsIURI secureUpgradedChannelURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSecureUpgradedChannelURI(nsIURI **aSecureUpgradedChannelURI) = 0;

  /* [noscript] void setChannelInfo (in ChannelInfo channelInfo); */
  NS_IMETHOD SetChannelInfo(mozilla::dom::ChannelInfo * channelInfo) = 0;

  /* [noscript] readonly attribute nsContentPolicyType internalContentPolicyType; */
  NS_IMETHOD GetInternalContentPolicyType(nsContentPolicyType *aInternalContentPolicyType) = 0;

  /* [noscript] readonly attribute nsIConsoleReportCollector consoleReportCollector; */
  NS_IMETHOD GetConsoleReportCollector(nsIConsoleReportCollector **aConsoleReportCollector) = 0;

  /* [noscript] void SetLaunchServiceWorkerStart (in TimeStamp aTimeStamp); */
  NS_IMETHOD SetLaunchServiceWorkerStart(mozilla::TimeStamp aTimeStamp) = 0;

  /* [noscript] void GetLaunchServiceWorkerStart (out TimeStamp aTimeStamp); */
  NS_IMETHOD GetLaunchServiceWorkerStart(mozilla::TimeStamp * aTimeStamp) = 0;

  /* [noscript] void SetLaunchServiceWorkerEnd (in TimeStamp aTimeStamp); */
  NS_IMETHOD SetLaunchServiceWorkerEnd(mozilla::TimeStamp aTimeStamp) = 0;

  /* [noscript] void GetLaunchServiceWorkerEnd (out TimeStamp aTimeStamp); */
  NS_IMETHOD GetLaunchServiceWorkerEnd(mozilla::TimeStamp * aTimeStamp) = 0;

  /* [noscript] void SetDispatchFetchEventStart (in TimeStamp aTimeStamp); */
  NS_IMETHOD SetDispatchFetchEventStart(mozilla::TimeStamp aTimeStamp) = 0;

  /* [noscript] void SetDispatchFetchEventEnd (in TimeStamp aTimeStamp); */
  NS_IMETHOD SetDispatchFetchEventEnd(mozilla::TimeStamp aTimeStamp) = 0;

  /* [noscript] void SetHandleFetchEventStart (in TimeStamp aTimeStamp); */
  NS_IMETHOD SetHandleFetchEventStart(mozilla::TimeStamp aTimeStamp) = 0;

  /* [noscript] void SetHandleFetchEventEnd (in TimeStamp aTimeStamp); */
  NS_IMETHOD SetHandleFetchEventEnd(mozilla::TimeStamp aTimeStamp) = 0;

  /* [noscript] void SetFinishResponseStart (in TimeStamp aTimeStamp); */
  NS_IMETHOD SetFinishResponseStart(mozilla::TimeStamp aTimeStamp) = 0;

  /* [noscript] void SetFinishSynthesizedResponseEnd (in TimeStamp aTimeStamp); */
  NS_IMETHOD SetFinishSynthesizedResponseEnd(mozilla::TimeStamp aTimeStamp) = 0;

  /* [noscript] void SetChannelResetEnd (in TimeStamp aTimeStamp); */
  NS_IMETHOD SetChannelResetEnd(mozilla::TimeStamp aTimeStamp) = 0;

  /* [noscript] void SaveTimeStamps (); */
  NS_IMETHOD SaveTimeStamps(void) = 0;

     already_AddRefed<nsIConsoleReportCollector>
    GetConsoleReportCollector()
    {
      nsCOMPtr<nsIConsoleReportCollector> reporter;
      GetConsoleReportCollector(getter_AddRefs(reporter));
      return reporter.forget();
    }
    void
    GetSubresourceTimeStampKey(nsIChannel* aChannel, nsACString& aKey)
    {
      if (!nsContentUtils::IsNonSubresourceRequest(aChannel)) {
        nsCOMPtr<nsILoadInfo> loadInfo = aChannel->LoadInfo();
        switch(loadInfo->InternalContentPolicyType()) {
          case nsIContentPolicy::TYPE_SCRIPT:
          case nsIContentPolicy::TYPE_INTERNAL_SCRIPT:
          case nsIContentPolicy::TYPE_INTERNAL_SCRIPT_PRELOAD:
          case nsIContentPolicy::TYPE_INTERNAL_MODULE:
          case nsIContentPolicy::TYPE_INTERNAL_MODULE_PRELOAD:
          case nsIContentPolicy::TYPE_INTERNAL_WORKER_IMPORT_SCRIPTS: {
            aKey = "subresource-script"_ns;
            break;
          }
          case nsIContentPolicy::TYPE_IMAGE:
          case nsIContentPolicy::TYPE_INTERNAL_IMAGE:
          case nsIContentPolicy::TYPE_INTERNAL_IMAGE_PRELOAD:
          case nsIContentPolicy::TYPE_INTERNAL_IMAGE_FAVICON: {
            aKey = "subresource-image"_ns;
            break;
          }
          case nsIContentPolicy::TYPE_STYLESHEET:
          case nsIContentPolicy::TYPE_INTERNAL_STYLESHEET:
          case nsIContentPolicy::TYPE_INTERNAL_STYLESHEET_PRELOAD: {
            aKey = "subresource-stylesheet"_ns;
            break;
          }
          default: {
            aKey = "subresource-other"_ns;
            break;
          }
        }
      }
    }
  /* [noscript] void setReleaseHandle (in nsISupports aHandle); */
  NS_IMETHOD SetReleaseHandle(nsISupports *aHandle) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIInterceptedChannel, NS_IINTERCEPTEDCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINTERCEPTEDCHANNEL \
  NS_IMETHOD ResetInterception(void) override; \
  NS_IMETHOD SynthesizeStatus(uint16_t status, const nsACString& reason) override; \
  NS_IMETHOD SynthesizeHeader(const nsACString& name, const nsACString& value) override; \
  NS_IMETHOD StartSynthesizedResponse(nsIInputStream *body, nsIInterceptedBodyCallback *callback, nsICacheInfoChannel *channel, const nsACString& finalURLSpec, bool responseRedirected) override; \
  NS_IMETHOD FinishSynthesizedResponse(void) override; \
  NS_IMETHOD CancelInterception(nsresult status) override; \
  NS_IMETHOD GetChannel(nsIChannel **aChannel) override; \
  NS_IMETHOD GetSecureUpgradedChannelURI(nsIURI **aSecureUpgradedChannelURI) override; \
  NS_IMETHOD SetChannelInfo(mozilla::dom::ChannelInfo * channelInfo) override; \
  NS_IMETHOD GetInternalContentPolicyType(nsContentPolicyType *aInternalContentPolicyType) override; \
  NS_IMETHOD GetConsoleReportCollector(nsIConsoleReportCollector **aConsoleReportCollector) override; \
  NS_IMETHOD SetLaunchServiceWorkerStart(mozilla::TimeStamp aTimeStamp) override; \
  NS_IMETHOD GetLaunchServiceWorkerStart(mozilla::TimeStamp * aTimeStamp) override; \
  NS_IMETHOD SetLaunchServiceWorkerEnd(mozilla::TimeStamp aTimeStamp) override; \
  NS_IMETHOD GetLaunchServiceWorkerEnd(mozilla::TimeStamp * aTimeStamp) override; \
  NS_IMETHOD SetDispatchFetchEventStart(mozilla::TimeStamp aTimeStamp) override; \
  NS_IMETHOD SetDispatchFetchEventEnd(mozilla::TimeStamp aTimeStamp) override; \
  NS_IMETHOD SetHandleFetchEventStart(mozilla::TimeStamp aTimeStamp) override; \
  NS_IMETHOD SetHandleFetchEventEnd(mozilla::TimeStamp aTimeStamp) override; \
  NS_IMETHOD SetFinishResponseStart(mozilla::TimeStamp aTimeStamp) override; \
  NS_IMETHOD SetFinishSynthesizedResponseEnd(mozilla::TimeStamp aTimeStamp) override; \
  NS_IMETHOD SetChannelResetEnd(mozilla::TimeStamp aTimeStamp) override; \
  NS_IMETHOD SaveTimeStamps(void) override; \
  NS_IMETHOD SetReleaseHandle(nsISupports *aHandle) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINTERCEPTEDCHANNEL \
  nsresult ResetInterception(void); \
  nsresult SynthesizeStatus(uint16_t status, const nsACString& reason); \
  nsresult SynthesizeHeader(const nsACString& name, const nsACString& value); \
  nsresult StartSynthesizedResponse(nsIInputStream *body, nsIInterceptedBodyCallback *callback, nsICacheInfoChannel *channel, const nsACString& finalURLSpec, bool responseRedirected); \
  nsresult FinishSynthesizedResponse(void); \
  nsresult CancelInterception(nsresult status); \
  nsresult GetChannel(nsIChannel **aChannel); \
  nsresult GetSecureUpgradedChannelURI(nsIURI **aSecureUpgradedChannelURI); \
  nsresult SetChannelInfo(mozilla::dom::ChannelInfo * channelInfo); \
  nsresult GetInternalContentPolicyType(nsContentPolicyType *aInternalContentPolicyType); \
  nsresult GetConsoleReportCollector(nsIConsoleReportCollector **aConsoleReportCollector); \
  nsresult SetLaunchServiceWorkerStart(mozilla::TimeStamp aTimeStamp); \
  nsresult GetLaunchServiceWorkerStart(mozilla::TimeStamp * aTimeStamp); \
  nsresult SetLaunchServiceWorkerEnd(mozilla::TimeStamp aTimeStamp); \
  nsresult GetLaunchServiceWorkerEnd(mozilla::TimeStamp * aTimeStamp); \
  nsresult SetDispatchFetchEventStart(mozilla::TimeStamp aTimeStamp); \
  nsresult SetDispatchFetchEventEnd(mozilla::TimeStamp aTimeStamp); \
  nsresult SetHandleFetchEventStart(mozilla::TimeStamp aTimeStamp); \
  nsresult SetHandleFetchEventEnd(mozilla::TimeStamp aTimeStamp); \
  nsresult SetFinishResponseStart(mozilla::TimeStamp aTimeStamp); \
  nsresult SetFinishSynthesizedResponseEnd(mozilla::TimeStamp aTimeStamp); \
  nsresult SetChannelResetEnd(mozilla::TimeStamp aTimeStamp); \
  nsresult SaveTimeStamps(void); \
  nsresult SetReleaseHandle(nsISupports *aHandle); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINTERCEPTEDCHANNEL(_to) \
  NS_IMETHOD ResetInterception(void) override { return _to ResetInterception(); } \
  NS_IMETHOD SynthesizeStatus(uint16_t status, const nsACString& reason) override { return _to SynthesizeStatus(status, reason); } \
  NS_IMETHOD SynthesizeHeader(const nsACString& name, const nsACString& value) override { return _to SynthesizeHeader(name, value); } \
  NS_IMETHOD StartSynthesizedResponse(nsIInputStream *body, nsIInterceptedBodyCallback *callback, nsICacheInfoChannel *channel, const nsACString& finalURLSpec, bool responseRedirected) override { return _to StartSynthesizedResponse(body, callback, channel, finalURLSpec, responseRedirected); } \
  NS_IMETHOD FinishSynthesizedResponse(void) override { return _to FinishSynthesizedResponse(); } \
  NS_IMETHOD CancelInterception(nsresult status) override { return _to CancelInterception(status); } \
  NS_IMETHOD GetChannel(nsIChannel **aChannel) override { return _to GetChannel(aChannel); } \
  NS_IMETHOD GetSecureUpgradedChannelURI(nsIURI **aSecureUpgradedChannelURI) override { return _to GetSecureUpgradedChannelURI(aSecureUpgradedChannelURI); } \
  NS_IMETHOD SetChannelInfo(mozilla::dom::ChannelInfo * channelInfo) override { return _to SetChannelInfo(channelInfo); } \
  NS_IMETHOD GetInternalContentPolicyType(nsContentPolicyType *aInternalContentPolicyType) override { return _to GetInternalContentPolicyType(aInternalContentPolicyType); } \
  NS_IMETHOD GetConsoleReportCollector(nsIConsoleReportCollector **aConsoleReportCollector) override { return _to GetConsoleReportCollector(aConsoleReportCollector); } \
  NS_IMETHOD SetLaunchServiceWorkerStart(mozilla::TimeStamp aTimeStamp) override { return _to SetLaunchServiceWorkerStart(aTimeStamp); } \
  NS_IMETHOD GetLaunchServiceWorkerStart(mozilla::TimeStamp * aTimeStamp) override { return _to GetLaunchServiceWorkerStart(aTimeStamp); } \
  NS_IMETHOD SetLaunchServiceWorkerEnd(mozilla::TimeStamp aTimeStamp) override { return _to SetLaunchServiceWorkerEnd(aTimeStamp); } \
  NS_IMETHOD GetLaunchServiceWorkerEnd(mozilla::TimeStamp * aTimeStamp) override { return _to GetLaunchServiceWorkerEnd(aTimeStamp); } \
  NS_IMETHOD SetDispatchFetchEventStart(mozilla::TimeStamp aTimeStamp) override { return _to SetDispatchFetchEventStart(aTimeStamp); } \
  NS_IMETHOD SetDispatchFetchEventEnd(mozilla::TimeStamp aTimeStamp) override { return _to SetDispatchFetchEventEnd(aTimeStamp); } \
  NS_IMETHOD SetHandleFetchEventStart(mozilla::TimeStamp aTimeStamp) override { return _to SetHandleFetchEventStart(aTimeStamp); } \
  NS_IMETHOD SetHandleFetchEventEnd(mozilla::TimeStamp aTimeStamp) override { return _to SetHandleFetchEventEnd(aTimeStamp); } \
  NS_IMETHOD SetFinishResponseStart(mozilla::TimeStamp aTimeStamp) override { return _to SetFinishResponseStart(aTimeStamp); } \
  NS_IMETHOD SetFinishSynthesizedResponseEnd(mozilla::TimeStamp aTimeStamp) override { return _to SetFinishSynthesizedResponseEnd(aTimeStamp); } \
  NS_IMETHOD SetChannelResetEnd(mozilla::TimeStamp aTimeStamp) override { return _to SetChannelResetEnd(aTimeStamp); } \
  NS_IMETHOD SaveTimeStamps(void) override { return _to SaveTimeStamps(); } \
  NS_IMETHOD SetReleaseHandle(nsISupports *aHandle) override { return _to SetReleaseHandle(aHandle); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINTERCEPTEDCHANNEL(_to) \
  NS_IMETHOD ResetInterception(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetInterception(); } \
  NS_IMETHOD SynthesizeStatus(uint16_t status, const nsACString& reason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SynthesizeStatus(status, reason); } \
  NS_IMETHOD SynthesizeHeader(const nsACString& name, const nsACString& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SynthesizeHeader(name, value); } \
  NS_IMETHOD StartSynthesizedResponse(nsIInputStream *body, nsIInterceptedBodyCallback *callback, nsICacheInfoChannel *channel, const nsACString& finalURLSpec, bool responseRedirected) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartSynthesizedResponse(body, callback, channel, finalURLSpec, responseRedirected); } \
  NS_IMETHOD FinishSynthesizedResponse(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FinishSynthesizedResponse(); } \
  NS_IMETHOD CancelInterception(nsresult status) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CancelInterception(status); } \
  NS_IMETHOD GetChannel(nsIChannel **aChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChannel(aChannel); } \
  NS_IMETHOD GetSecureUpgradedChannelURI(nsIURI **aSecureUpgradedChannelURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecureUpgradedChannelURI(aSecureUpgradedChannelURI); } \
  NS_IMETHOD SetChannelInfo(mozilla::dom::ChannelInfo * channelInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetChannelInfo(channelInfo); } \
  NS_IMETHOD GetInternalContentPolicyType(nsContentPolicyType *aInternalContentPolicyType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInternalContentPolicyType(aInternalContentPolicyType); } \
  NS_IMETHOD GetConsoleReportCollector(nsIConsoleReportCollector **aConsoleReportCollector) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConsoleReportCollector(aConsoleReportCollector); } \
  NS_IMETHOD SetLaunchServiceWorkerStart(mozilla::TimeStamp aTimeStamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLaunchServiceWorkerStart(aTimeStamp); } \
  NS_IMETHOD GetLaunchServiceWorkerStart(mozilla::TimeStamp * aTimeStamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLaunchServiceWorkerStart(aTimeStamp); } \
  NS_IMETHOD SetLaunchServiceWorkerEnd(mozilla::TimeStamp aTimeStamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLaunchServiceWorkerEnd(aTimeStamp); } \
  NS_IMETHOD GetLaunchServiceWorkerEnd(mozilla::TimeStamp * aTimeStamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLaunchServiceWorkerEnd(aTimeStamp); } \
  NS_IMETHOD SetDispatchFetchEventStart(mozilla::TimeStamp aTimeStamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDispatchFetchEventStart(aTimeStamp); } \
  NS_IMETHOD SetDispatchFetchEventEnd(mozilla::TimeStamp aTimeStamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDispatchFetchEventEnd(aTimeStamp); } \
  NS_IMETHOD SetHandleFetchEventStart(mozilla::TimeStamp aTimeStamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHandleFetchEventStart(aTimeStamp); } \
  NS_IMETHOD SetHandleFetchEventEnd(mozilla::TimeStamp aTimeStamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHandleFetchEventEnd(aTimeStamp); } \
  NS_IMETHOD SetFinishResponseStart(mozilla::TimeStamp aTimeStamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFinishResponseStart(aTimeStamp); } \
  NS_IMETHOD SetFinishSynthesizedResponseEnd(mozilla::TimeStamp aTimeStamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFinishSynthesizedResponseEnd(aTimeStamp); } \
  NS_IMETHOD SetChannelResetEnd(mozilla::TimeStamp aTimeStamp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetChannelResetEnd(aTimeStamp); } \
  NS_IMETHOD SaveTimeStamps(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SaveTimeStamps(); } \
  NS_IMETHOD SetReleaseHandle(nsISupports *aHandle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetReleaseHandle(aHandle); } 


/* starting interface:    nsINetworkInterceptController */
#define NS_INETWORKINTERCEPTCONTROLLER_IID_STR "70d2b4fe-a552-48cd-8d93-1d8437a56b53"

#define NS_INETWORKINTERCEPTCONTROLLER_IID \
  {0x70d2b4fe, 0xa552, 0x48cd, \
    { 0x8d, 0x93, 0x1d, 0x84, 0x37, 0xa5, 0x6b, 0x53 }}

class NS_NO_VTABLE nsINetworkInterceptController : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INETWORKINTERCEPTCONTROLLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINetworkInterceptController;

  /* bool shouldPrepareForIntercept (in nsIURI aURI, in nsIChannel aChannel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ShouldPrepareForIntercept(nsIURI *aURI, nsIChannel *aChannel, bool *_retval) = 0;

  /* void channelIntercepted (in nsIInterceptedChannel aChannel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ChannelIntercepted(nsIInterceptedChannel *aChannel) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINetworkInterceptController, NS_INETWORKINTERCEPTCONTROLLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINETWORKINTERCEPTCONTROLLER \
  NS_IMETHOD ShouldPrepareForIntercept(nsIURI *aURI, nsIChannel *aChannel, bool *_retval) override; \
  NS_IMETHOD ChannelIntercepted(nsIInterceptedChannel *aChannel) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINETWORKINTERCEPTCONTROLLER \
  nsresult ShouldPrepareForIntercept(nsIURI *aURI, nsIChannel *aChannel, bool *_retval); \
  nsresult ChannelIntercepted(nsIInterceptedChannel *aChannel); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINETWORKINTERCEPTCONTROLLER(_to) \
  NS_IMETHOD ShouldPrepareForIntercept(nsIURI *aURI, nsIChannel *aChannel, bool *_retval) override { return _to ShouldPrepareForIntercept(aURI, aChannel, _retval); } \
  NS_IMETHOD ChannelIntercepted(nsIInterceptedChannel *aChannel) override { return _to ChannelIntercepted(aChannel); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINETWORKINTERCEPTCONTROLLER(_to) \
  NS_IMETHOD ShouldPrepareForIntercept(nsIURI *aURI, nsIChannel *aChannel, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShouldPrepareForIntercept(aURI, aChannel, _retval); } \
  NS_IMETHOD ChannelIntercepted(nsIInterceptedChannel *aChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ChannelIntercepted(aChannel); } 


#endif /* __gen_nsINetworkInterceptController_h__ */
