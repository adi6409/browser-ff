/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpChannel.idl
 */

#ifndef __gen_nsIHttpChannel_h__
#define __gen_nsIHttpChannel_h__


#ifndef __gen_nsIChannel_h__
#include "nsIChannel.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIHttpHeaderVisitor; /* forward declaration */

class nsIReferrerInfo; /* forward declaration */

#include "GeckoProfiler.h"

/* starting interface:    nsIHttpChannel */
#define NS_IHTTPCHANNEL_IID_STR "c5a4a073-4539-49c7-a3f2-cec3f0619c6c"

#define NS_IHTTPCHANNEL_IID \
  {0xc5a4a073, 0x4539, 0x49c7, \
    { 0xa3, 0xf2, 0xce, 0xc3, 0xf0, 0x61, 0x9c, 0x6c }}

class nsIHttpChannel : public nsIIdentChannel {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHttpChannel;

  /* [must_use] attribute ACString requestMethod; */
  [[nodiscard]] NS_IMETHOD GetRequestMethod(nsACString& aRequestMethod) = 0;
  [[nodiscard]] NS_IMETHOD SetRequestMethod(const nsACString& aRequestMethod) = 0;

  /* [infallible,must_use] attribute nsIReferrerInfo referrerInfo; */
  [[nodiscard]] NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) = 0;
  [[nodiscard]]  inline already_AddRefed<nsIReferrerInfo> GetReferrerInfo()
  {
    nsIReferrerInfo* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetReferrerInfo(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIReferrerInfo>(result);
  }
  [[nodiscard]] NS_IMETHOD SetReferrerInfo(nsIReferrerInfo *aReferrerInfo) = 0;

  /* [must_use,noscript] void setReferrerInfoWithoutClone (in nsIReferrerInfo aReferrerInfo); */
  [[nodiscard]] NS_IMETHOD SetReferrerInfoWithoutClone(nsIReferrerInfo *aReferrerInfo) = 0;

  /* [must_use] readonly attribute ACString protocolVersion; */
  [[nodiscard]] NS_IMETHOD GetProtocolVersion(nsACString& aProtocolVersion) = 0;

  /* [must_use] readonly attribute uint64_t transferSize; */
  [[nodiscard]] NS_IMETHOD GetTransferSize(uint64_t *aTransferSize) = 0;

  /* [must_use] readonly attribute uint64_t requestSize; */
  [[nodiscard]] NS_IMETHOD GetRequestSize(uint64_t *aRequestSize) = 0;

  /* [must_use] readonly attribute uint64_t decodedBodySize; */
  [[nodiscard]] NS_IMETHOD GetDecodedBodySize(uint64_t *aDecodedBodySize) = 0;

  /* [must_use] readonly attribute uint64_t encodedBodySize; */
  [[nodiscard]] NS_IMETHOD GetEncodedBodySize(uint64_t *aEncodedBodySize) = 0;

  /* [must_use] ACString getRequestHeader (in ACString aHeader); */
  [[nodiscard]] NS_IMETHOD GetRequestHeader(const nsACString& aHeader, nsACString& _retval) = 0;

  /* [must_use] void setRequestHeader (in ACString aHeader, in ACString aValue, in boolean aMerge); */
  [[nodiscard]] NS_IMETHOD SetRequestHeader(const nsACString& aHeader, const nsACString& aValue, bool aMerge) = 0;

  /* [must_use] void setNewReferrerInfo (in ACString aUrl, in nsIReferrerInfo_ReferrerPolicyIDL aPolicy, in boolean aSendReferrer); */
  [[nodiscard]] NS_IMETHOD SetNewReferrerInfo(const nsACString& aUrl, nsIReferrerInfo::ReferrerPolicyIDL aPolicy, bool aSendReferrer) = 0;

  /* [must_use] void setEmptyRequestHeader (in ACString aHeader); */
  [[nodiscard]] NS_IMETHOD SetEmptyRequestHeader(const nsACString& aHeader) = 0;

  /* [must_use] void visitRequestHeaders (in nsIHttpHeaderVisitor aVisitor); */
  [[nodiscard]] NS_IMETHOD VisitRequestHeaders(nsIHttpHeaderVisitor *aVisitor) = 0;

  /* [must_use] void visitNonDefaultRequestHeaders (in nsIHttpHeaderVisitor aVisitor); */
  [[nodiscard]] NS_IMETHOD VisitNonDefaultRequestHeaders(nsIHttpHeaderVisitor *aVisitor) = 0;

  /* [must_use] bool ShouldStripRequestBodyHeader (in ACString aMethod); */
  [[nodiscard]] NS_IMETHOD ShouldStripRequestBodyHeader(const nsACString& aMethod, bool *_retval) = 0;

  /* [must_use] attribute boolean allowPipelining; */
  [[nodiscard]] NS_IMETHOD GetAllowPipelining(bool *aAllowPipelining) = 0;
  [[nodiscard]] NS_IMETHOD SetAllowPipelining(bool aAllowPipelining) = 0;

  /* [must_use] attribute boolean allowSTS; */
  [[nodiscard]] NS_IMETHOD GetAllowSTS(bool *aAllowSTS) = 0;
  [[nodiscard]] NS_IMETHOD SetAllowSTS(bool aAllowSTS) = 0;

  /* [must_use] attribute unsigned long redirectionLimit; */
  [[nodiscard]] NS_IMETHOD GetRedirectionLimit(uint32_t *aRedirectionLimit) = 0;
  [[nodiscard]] NS_IMETHOD SetRedirectionLimit(uint32_t aRedirectionLimit) = 0;

  /* [must_use] readonly attribute unsigned long responseStatus; */
  [[nodiscard]] NS_IMETHOD GetResponseStatus(uint32_t *aResponseStatus) = 0;

  /* [must_use] readonly attribute ACString responseStatusText; */
  [[nodiscard]] NS_IMETHOD GetResponseStatusText(nsACString& aResponseStatusText) = 0;

  /* [must_use] readonly attribute boolean requestSucceeded; */
  [[nodiscard]] NS_IMETHOD GetRequestSucceeded(bool *aRequestSucceeded) = 0;

  /* [must_use] attribute boolean isMainDocumentChannel; */
  [[nodiscard]] NS_IMETHOD GetIsMainDocumentChannel(bool *aIsMainDocumentChannel) = 0;
  [[nodiscard]] NS_IMETHOD SetIsMainDocumentChannel(bool aIsMainDocumentChannel) = 0;

  /* [must_use] ACString getResponseHeader (in ACString header); */
  [[nodiscard]] NS_IMETHOD GetResponseHeader(const nsACString& header, nsACString& _retval) = 0;

  /* [must_use] void setResponseHeader (in ACString header, in ACString value, in boolean merge); */
  [[nodiscard]] NS_IMETHOD SetResponseHeader(const nsACString& header, const nsACString& value, bool merge) = 0;

  /* [must_use] void visitResponseHeaders (in nsIHttpHeaderVisitor aVisitor); */
  [[nodiscard]] NS_IMETHOD VisitResponseHeaders(nsIHttpHeaderVisitor *aVisitor) = 0;

  /* [must_use] void getOriginalResponseHeader (in ACString aHeader, in nsIHttpHeaderVisitor aVisitor); */
  [[nodiscard]] NS_IMETHOD GetOriginalResponseHeader(const nsACString& aHeader, nsIHttpHeaderVisitor *aVisitor) = 0;

  /* [must_use] void visitOriginalResponseHeaders (in nsIHttpHeaderVisitor aVisitor); */
  [[nodiscard]] NS_IMETHOD VisitOriginalResponseHeaders(nsIHttpHeaderVisitor *aVisitor) = 0;

  /* [must_use] boolean isNoStoreResponse (); */
  [[nodiscard]] NS_IMETHOD IsNoStoreResponse(bool *_retval) = 0;

  /* [must_use] boolean isNoCacheResponse (); */
  [[nodiscard]] NS_IMETHOD IsNoCacheResponse(bool *_retval) = 0;

  /* [must_use] boolean isPrivateResponse (); */
  [[nodiscard]] NS_IMETHOD IsPrivateResponse(bool *_retval) = 0;

  /* [must_use] void redirectTo (in nsIURI aTargetURI); */
  [[nodiscard]] NS_IMETHOD RedirectTo(nsIURI *aTargetURI) = 0;

  /* [must_use] void upgradeToSecure (); */
  [[nodiscard]] NS_IMETHOD UpgradeToSecure(void) = 0;

  /* [must_use,noscript] attribute uint64_t requestContextID; */
  [[nodiscard]] NS_IMETHOD GetRequestContextID(uint64_t *aRequestContextID) = 0;
  [[nodiscard]] NS_IMETHOD SetRequestContextID(uint64_t aRequestContextID) = 0;

  /* [must_use] attribute uint64_t topLevelContentWindowId; */
  [[nodiscard]] NS_IMETHOD GetTopLevelContentWindowId(uint64_t *aTopLevelContentWindowId) = 0;
  [[nodiscard]] NS_IMETHOD SetTopLevelContentWindowId(uint64_t aTopLevelContentWindowId) = 0;

  enum FlashPluginState : uint8_t {
    FlashPluginUnknown = 0,
    FlashPluginAllowed = 1,
    FlashPluginDenied = 2,
    FlashPluginDeniedInSubdocuments = 3,
    FlashPluginLastValue = 3,
  };

  /* [infallible] readonly attribute nsIHttpChannel_FlashPluginState flashPluginState; */
  NS_IMETHOD GetFlashPluginState(nsIHttpChannel::FlashPluginState *aFlashPluginState) = 0;
  inline nsIHttpChannel::FlashPluginState  GetFlashPluginState()
  {
    nsIHttpChannel::FlashPluginState result;
    mozilla::DebugOnly<nsresult> rv = GetFlashPluginState(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [must_use] attribute uint64_t topLevelOuterContentWindowId; */
  [[nodiscard]] NS_IMETHOD GetTopLevelOuterContentWindowId(uint64_t *aTopLevelOuterContentWindowId) = 0;
  [[nodiscard]] NS_IMETHOD SetTopLevelOuterContentWindowId(uint64_t aTopLevelOuterContentWindowId) = 0;

  /* void logBlockedCORSRequest (in AString aMessage, in ACString aCategory); */
  NS_IMETHOD LogBlockedCORSRequest(const nsAString& aMessage, const nsACString& aCategory) = 0;

  /* void logMimeTypeMismatch (in ACString aMessageName, in boolean aWarning, in AString aURL, in AString aContentType); */
  NS_IMETHOD LogMimeTypeMismatch(const nsACString& aMessageName, bool aWarning, const nsAString& aURL, const nsAString& aContentType) = 0;

   virtual void SetSource(mozilla::UniquePtr<mozilla::ProfileChunkedBuffer> aSource) {}
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpChannel, NS_IHTTPCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPCHANNEL \
  [[nodiscard]] NS_IMETHOD GetRequestMethod(nsACString& aRequestMethod) override; \
  [[nodiscard]] NS_IMETHOD SetRequestMethod(const nsACString& aRequestMethod) override; \
  using nsIHttpChannel::GetReferrerInfo; \
  [[nodiscard]] NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override; \
  [[nodiscard]] NS_IMETHOD SetReferrerInfo(nsIReferrerInfo *aReferrerInfo) override; \
  [[nodiscard]] NS_IMETHOD SetReferrerInfoWithoutClone(nsIReferrerInfo *aReferrerInfo) override; \
  [[nodiscard]] NS_IMETHOD GetProtocolVersion(nsACString& aProtocolVersion) override; \
  [[nodiscard]] NS_IMETHOD GetTransferSize(uint64_t *aTransferSize) override; \
  [[nodiscard]] NS_IMETHOD GetRequestSize(uint64_t *aRequestSize) override; \
  [[nodiscard]] NS_IMETHOD GetDecodedBodySize(uint64_t *aDecodedBodySize) override; \
  [[nodiscard]] NS_IMETHOD GetEncodedBodySize(uint64_t *aEncodedBodySize) override; \
  [[nodiscard]] NS_IMETHOD GetRequestHeader(const nsACString& aHeader, nsACString& _retval) override; \
  [[nodiscard]] NS_IMETHOD SetRequestHeader(const nsACString& aHeader, const nsACString& aValue, bool aMerge) override; \
  [[nodiscard]] NS_IMETHOD SetNewReferrerInfo(const nsACString& aUrl, nsIReferrerInfo::ReferrerPolicyIDL aPolicy, bool aSendReferrer) override; \
  [[nodiscard]] NS_IMETHOD SetEmptyRequestHeader(const nsACString& aHeader) override; \
  [[nodiscard]] NS_IMETHOD VisitRequestHeaders(nsIHttpHeaderVisitor *aVisitor) override; \
  [[nodiscard]] NS_IMETHOD VisitNonDefaultRequestHeaders(nsIHttpHeaderVisitor *aVisitor) override; \
  [[nodiscard]] NS_IMETHOD ShouldStripRequestBodyHeader(const nsACString& aMethod, bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD GetAllowPipelining(bool *aAllowPipelining) override; \
  [[nodiscard]] NS_IMETHOD SetAllowPipelining(bool aAllowPipelining) override; \
  [[nodiscard]] NS_IMETHOD GetAllowSTS(bool *aAllowSTS) override; \
  [[nodiscard]] NS_IMETHOD SetAllowSTS(bool aAllowSTS) override; \
  [[nodiscard]] NS_IMETHOD GetRedirectionLimit(uint32_t *aRedirectionLimit) override; \
  [[nodiscard]] NS_IMETHOD SetRedirectionLimit(uint32_t aRedirectionLimit) override; \
  [[nodiscard]] NS_IMETHOD GetResponseStatus(uint32_t *aResponseStatus) override; \
  [[nodiscard]] NS_IMETHOD GetResponseStatusText(nsACString& aResponseStatusText) override; \
  [[nodiscard]] NS_IMETHOD GetRequestSucceeded(bool *aRequestSucceeded) override; \
  [[nodiscard]] NS_IMETHOD GetIsMainDocumentChannel(bool *aIsMainDocumentChannel) override; \
  [[nodiscard]] NS_IMETHOD SetIsMainDocumentChannel(bool aIsMainDocumentChannel) override; \
  [[nodiscard]] NS_IMETHOD GetResponseHeader(const nsACString& header, nsACString& _retval) override; \
  [[nodiscard]] NS_IMETHOD SetResponseHeader(const nsACString& header, const nsACString& value, bool merge) override; \
  [[nodiscard]] NS_IMETHOD VisitResponseHeaders(nsIHttpHeaderVisitor *aVisitor) override; \
  [[nodiscard]] NS_IMETHOD GetOriginalResponseHeader(const nsACString& aHeader, nsIHttpHeaderVisitor *aVisitor) override; \
  [[nodiscard]] NS_IMETHOD VisitOriginalResponseHeaders(nsIHttpHeaderVisitor *aVisitor) override; \
  [[nodiscard]] NS_IMETHOD IsNoStoreResponse(bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD IsNoCacheResponse(bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD IsPrivateResponse(bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD RedirectTo(nsIURI *aTargetURI) override; \
  [[nodiscard]] NS_IMETHOD UpgradeToSecure(void) override; \
  [[nodiscard]] NS_IMETHOD GetRequestContextID(uint64_t *aRequestContextID) override; \
  [[nodiscard]] NS_IMETHOD SetRequestContextID(uint64_t aRequestContextID) override; \
  [[nodiscard]] NS_IMETHOD GetTopLevelContentWindowId(uint64_t *aTopLevelContentWindowId) override; \
  [[nodiscard]] NS_IMETHOD SetTopLevelContentWindowId(uint64_t aTopLevelContentWindowId) override; \
  using nsIHttpChannel::GetFlashPluginState; \
  NS_IMETHOD GetFlashPluginState(nsIHttpChannel::FlashPluginState *aFlashPluginState) override; \
  [[nodiscard]] NS_IMETHOD GetTopLevelOuterContentWindowId(uint64_t *aTopLevelOuterContentWindowId) override; \
  [[nodiscard]] NS_IMETHOD SetTopLevelOuterContentWindowId(uint64_t aTopLevelOuterContentWindowId) override; \
  NS_IMETHOD LogBlockedCORSRequest(const nsAString& aMessage, const nsACString& aCategory) override; \
  NS_IMETHOD LogMimeTypeMismatch(const nsACString& aMessageName, bool aWarning, const nsAString& aURL, const nsAString& aContentType) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPCHANNEL \
  [[nodiscard]] nsresult GetRequestMethod(nsACString& aRequestMethod); \
  [[nodiscard]] nsresult SetRequestMethod(const nsACString& aRequestMethod); \
  using nsIHttpChannel::GetReferrerInfo; \
  [[nodiscard]] nsresult GetReferrerInfo(nsIReferrerInfo **aReferrerInfo); \
  [[nodiscard]] nsresult SetReferrerInfo(nsIReferrerInfo *aReferrerInfo); \
  [[nodiscard]] nsresult SetReferrerInfoWithoutClone(nsIReferrerInfo *aReferrerInfo); \
  [[nodiscard]] nsresult GetProtocolVersion(nsACString& aProtocolVersion); \
  [[nodiscard]] nsresult GetTransferSize(uint64_t *aTransferSize); \
  [[nodiscard]] nsresult GetRequestSize(uint64_t *aRequestSize); \
  [[nodiscard]] nsresult GetDecodedBodySize(uint64_t *aDecodedBodySize); \
  [[nodiscard]] nsresult GetEncodedBodySize(uint64_t *aEncodedBodySize); \
  [[nodiscard]] nsresult GetRequestHeader(const nsACString& aHeader, nsACString& _retval); \
  [[nodiscard]] nsresult SetRequestHeader(const nsACString& aHeader, const nsACString& aValue, bool aMerge); \
  [[nodiscard]] nsresult SetNewReferrerInfo(const nsACString& aUrl, nsIReferrerInfo::ReferrerPolicyIDL aPolicy, bool aSendReferrer); \
  [[nodiscard]] nsresult SetEmptyRequestHeader(const nsACString& aHeader); \
  [[nodiscard]] nsresult VisitRequestHeaders(nsIHttpHeaderVisitor *aVisitor); \
  [[nodiscard]] nsresult VisitNonDefaultRequestHeaders(nsIHttpHeaderVisitor *aVisitor); \
  [[nodiscard]] nsresult ShouldStripRequestBodyHeader(const nsACString& aMethod, bool *_retval); \
  [[nodiscard]] nsresult GetAllowPipelining(bool *aAllowPipelining); \
  [[nodiscard]] nsresult SetAllowPipelining(bool aAllowPipelining); \
  [[nodiscard]] nsresult GetAllowSTS(bool *aAllowSTS); \
  [[nodiscard]] nsresult SetAllowSTS(bool aAllowSTS); \
  [[nodiscard]] nsresult GetRedirectionLimit(uint32_t *aRedirectionLimit); \
  [[nodiscard]] nsresult SetRedirectionLimit(uint32_t aRedirectionLimit); \
  [[nodiscard]] nsresult GetResponseStatus(uint32_t *aResponseStatus); \
  [[nodiscard]] nsresult GetResponseStatusText(nsACString& aResponseStatusText); \
  [[nodiscard]] nsresult GetRequestSucceeded(bool *aRequestSucceeded); \
  [[nodiscard]] nsresult GetIsMainDocumentChannel(bool *aIsMainDocumentChannel); \
  [[nodiscard]] nsresult SetIsMainDocumentChannel(bool aIsMainDocumentChannel); \
  [[nodiscard]] nsresult GetResponseHeader(const nsACString& header, nsACString& _retval); \
  [[nodiscard]] nsresult SetResponseHeader(const nsACString& header, const nsACString& value, bool merge); \
  [[nodiscard]] nsresult VisitResponseHeaders(nsIHttpHeaderVisitor *aVisitor); \
  [[nodiscard]] nsresult GetOriginalResponseHeader(const nsACString& aHeader, nsIHttpHeaderVisitor *aVisitor); \
  [[nodiscard]] nsresult VisitOriginalResponseHeaders(nsIHttpHeaderVisitor *aVisitor); \
  [[nodiscard]] nsresult IsNoStoreResponse(bool *_retval); \
  [[nodiscard]] nsresult IsNoCacheResponse(bool *_retval); \
  [[nodiscard]] nsresult IsPrivateResponse(bool *_retval); \
  [[nodiscard]] nsresult RedirectTo(nsIURI *aTargetURI); \
  [[nodiscard]] nsresult UpgradeToSecure(void); \
  [[nodiscard]] nsresult GetRequestContextID(uint64_t *aRequestContextID); \
  [[nodiscard]] nsresult SetRequestContextID(uint64_t aRequestContextID); \
  [[nodiscard]] nsresult GetTopLevelContentWindowId(uint64_t *aTopLevelContentWindowId); \
  [[nodiscard]] nsresult SetTopLevelContentWindowId(uint64_t aTopLevelContentWindowId); \
  using nsIHttpChannel::GetFlashPluginState; \
  nsresult GetFlashPluginState(nsIHttpChannel::FlashPluginState *aFlashPluginState); \
  [[nodiscard]] nsresult GetTopLevelOuterContentWindowId(uint64_t *aTopLevelOuterContentWindowId); \
  [[nodiscard]] nsresult SetTopLevelOuterContentWindowId(uint64_t aTopLevelOuterContentWindowId); \
  nsresult LogBlockedCORSRequest(const nsAString& aMessage, const nsACString& aCategory); \
  nsresult LogMimeTypeMismatch(const nsACString& aMessageName, bool aWarning, const nsAString& aURL, const nsAString& aContentType); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPCHANNEL(_to) \
  [[nodiscard]] NS_IMETHOD GetRequestMethod(nsACString& aRequestMethod) override { return _to GetRequestMethod(aRequestMethod); } \
  [[nodiscard]] NS_IMETHOD SetRequestMethod(const nsACString& aRequestMethod) override { return _to SetRequestMethod(aRequestMethod); } \
  using nsIHttpChannel::GetReferrerInfo; \
  [[nodiscard]] NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override { return _to GetReferrerInfo(aReferrerInfo); } \
  [[nodiscard]] NS_IMETHOD SetReferrerInfo(nsIReferrerInfo *aReferrerInfo) override { return _to SetReferrerInfo(aReferrerInfo); } \
  [[nodiscard]] NS_IMETHOD SetReferrerInfoWithoutClone(nsIReferrerInfo *aReferrerInfo) override { return _to SetReferrerInfoWithoutClone(aReferrerInfo); } \
  [[nodiscard]] NS_IMETHOD GetProtocolVersion(nsACString& aProtocolVersion) override { return _to GetProtocolVersion(aProtocolVersion); } \
  [[nodiscard]] NS_IMETHOD GetTransferSize(uint64_t *aTransferSize) override { return _to GetTransferSize(aTransferSize); } \
  [[nodiscard]] NS_IMETHOD GetRequestSize(uint64_t *aRequestSize) override { return _to GetRequestSize(aRequestSize); } \
  [[nodiscard]] NS_IMETHOD GetDecodedBodySize(uint64_t *aDecodedBodySize) override { return _to GetDecodedBodySize(aDecodedBodySize); } \
  [[nodiscard]] NS_IMETHOD GetEncodedBodySize(uint64_t *aEncodedBodySize) override { return _to GetEncodedBodySize(aEncodedBodySize); } \
  [[nodiscard]] NS_IMETHOD GetRequestHeader(const nsACString& aHeader, nsACString& _retval) override { return _to GetRequestHeader(aHeader, _retval); } \
  [[nodiscard]] NS_IMETHOD SetRequestHeader(const nsACString& aHeader, const nsACString& aValue, bool aMerge) override { return _to SetRequestHeader(aHeader, aValue, aMerge); } \
  [[nodiscard]] NS_IMETHOD SetNewReferrerInfo(const nsACString& aUrl, nsIReferrerInfo::ReferrerPolicyIDL aPolicy, bool aSendReferrer) override { return _to SetNewReferrerInfo(aUrl, aPolicy, aSendReferrer); } \
  [[nodiscard]] NS_IMETHOD SetEmptyRequestHeader(const nsACString& aHeader) override { return _to SetEmptyRequestHeader(aHeader); } \
  [[nodiscard]] NS_IMETHOD VisitRequestHeaders(nsIHttpHeaderVisitor *aVisitor) override { return _to VisitRequestHeaders(aVisitor); } \
  [[nodiscard]] NS_IMETHOD VisitNonDefaultRequestHeaders(nsIHttpHeaderVisitor *aVisitor) override { return _to VisitNonDefaultRequestHeaders(aVisitor); } \
  [[nodiscard]] NS_IMETHOD ShouldStripRequestBodyHeader(const nsACString& aMethod, bool *_retval) override { return _to ShouldStripRequestBodyHeader(aMethod, _retval); } \
  [[nodiscard]] NS_IMETHOD GetAllowPipelining(bool *aAllowPipelining) override { return _to GetAllowPipelining(aAllowPipelining); } \
  [[nodiscard]] NS_IMETHOD SetAllowPipelining(bool aAllowPipelining) override { return _to SetAllowPipelining(aAllowPipelining); } \
  [[nodiscard]] NS_IMETHOD GetAllowSTS(bool *aAllowSTS) override { return _to GetAllowSTS(aAllowSTS); } \
  [[nodiscard]] NS_IMETHOD SetAllowSTS(bool aAllowSTS) override { return _to SetAllowSTS(aAllowSTS); } \
  [[nodiscard]] NS_IMETHOD GetRedirectionLimit(uint32_t *aRedirectionLimit) override { return _to GetRedirectionLimit(aRedirectionLimit); } \
  [[nodiscard]] NS_IMETHOD SetRedirectionLimit(uint32_t aRedirectionLimit) override { return _to SetRedirectionLimit(aRedirectionLimit); } \
  [[nodiscard]] NS_IMETHOD GetResponseStatus(uint32_t *aResponseStatus) override { return _to GetResponseStatus(aResponseStatus); } \
  [[nodiscard]] NS_IMETHOD GetResponseStatusText(nsACString& aResponseStatusText) override { return _to GetResponseStatusText(aResponseStatusText); } \
  [[nodiscard]] NS_IMETHOD GetRequestSucceeded(bool *aRequestSucceeded) override { return _to GetRequestSucceeded(aRequestSucceeded); } \
  [[nodiscard]] NS_IMETHOD GetIsMainDocumentChannel(bool *aIsMainDocumentChannel) override { return _to GetIsMainDocumentChannel(aIsMainDocumentChannel); } \
  [[nodiscard]] NS_IMETHOD SetIsMainDocumentChannel(bool aIsMainDocumentChannel) override { return _to SetIsMainDocumentChannel(aIsMainDocumentChannel); } \
  [[nodiscard]] NS_IMETHOD GetResponseHeader(const nsACString& header, nsACString& _retval) override { return _to GetResponseHeader(header, _retval); } \
  [[nodiscard]] NS_IMETHOD SetResponseHeader(const nsACString& header, const nsACString& value, bool merge) override { return _to SetResponseHeader(header, value, merge); } \
  [[nodiscard]] NS_IMETHOD VisitResponseHeaders(nsIHttpHeaderVisitor *aVisitor) override { return _to VisitResponseHeaders(aVisitor); } \
  [[nodiscard]] NS_IMETHOD GetOriginalResponseHeader(const nsACString& aHeader, nsIHttpHeaderVisitor *aVisitor) override { return _to GetOriginalResponseHeader(aHeader, aVisitor); } \
  [[nodiscard]] NS_IMETHOD VisitOriginalResponseHeaders(nsIHttpHeaderVisitor *aVisitor) override { return _to VisitOriginalResponseHeaders(aVisitor); } \
  [[nodiscard]] NS_IMETHOD IsNoStoreResponse(bool *_retval) override { return _to IsNoStoreResponse(_retval); } \
  [[nodiscard]] NS_IMETHOD IsNoCacheResponse(bool *_retval) override { return _to IsNoCacheResponse(_retval); } \
  [[nodiscard]] NS_IMETHOD IsPrivateResponse(bool *_retval) override { return _to IsPrivateResponse(_retval); } \
  [[nodiscard]] NS_IMETHOD RedirectTo(nsIURI *aTargetURI) override { return _to RedirectTo(aTargetURI); } \
  [[nodiscard]] NS_IMETHOD UpgradeToSecure(void) override { return _to UpgradeToSecure(); } \
  [[nodiscard]] NS_IMETHOD GetRequestContextID(uint64_t *aRequestContextID) override { return _to GetRequestContextID(aRequestContextID); } \
  [[nodiscard]] NS_IMETHOD SetRequestContextID(uint64_t aRequestContextID) override { return _to SetRequestContextID(aRequestContextID); } \
  [[nodiscard]] NS_IMETHOD GetTopLevelContentWindowId(uint64_t *aTopLevelContentWindowId) override { return _to GetTopLevelContentWindowId(aTopLevelContentWindowId); } \
  [[nodiscard]] NS_IMETHOD SetTopLevelContentWindowId(uint64_t aTopLevelContentWindowId) override { return _to SetTopLevelContentWindowId(aTopLevelContentWindowId); } \
  using nsIHttpChannel::GetFlashPluginState; \
  NS_IMETHOD GetFlashPluginState(nsIHttpChannel::FlashPluginState *aFlashPluginState) override { return _to GetFlashPluginState(aFlashPluginState); } \
  [[nodiscard]] NS_IMETHOD GetTopLevelOuterContentWindowId(uint64_t *aTopLevelOuterContentWindowId) override { return _to GetTopLevelOuterContentWindowId(aTopLevelOuterContentWindowId); } \
  [[nodiscard]] NS_IMETHOD SetTopLevelOuterContentWindowId(uint64_t aTopLevelOuterContentWindowId) override { return _to SetTopLevelOuterContentWindowId(aTopLevelOuterContentWindowId); } \
  NS_IMETHOD LogBlockedCORSRequest(const nsAString& aMessage, const nsACString& aCategory) override { return _to LogBlockedCORSRequest(aMessage, aCategory); } \
  NS_IMETHOD LogMimeTypeMismatch(const nsACString& aMessageName, bool aWarning, const nsAString& aURL, const nsAString& aContentType) override { return _to LogMimeTypeMismatch(aMessageName, aWarning, aURL, aContentType); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPCHANNEL(_to) \
  [[nodiscard]] NS_IMETHOD GetRequestMethod(nsACString& aRequestMethod) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestMethod(aRequestMethod); } \
  [[nodiscard]] NS_IMETHOD SetRequestMethod(const nsACString& aRequestMethod) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRequestMethod(aRequestMethod); } \
  [[nodiscard]] NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReferrerInfo(aReferrerInfo); } \
  [[nodiscard]] NS_IMETHOD SetReferrerInfo(nsIReferrerInfo *aReferrerInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetReferrerInfo(aReferrerInfo); } \
  [[nodiscard]] NS_IMETHOD SetReferrerInfoWithoutClone(nsIReferrerInfo *aReferrerInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetReferrerInfoWithoutClone(aReferrerInfo); } \
  [[nodiscard]] NS_IMETHOD GetProtocolVersion(nsACString& aProtocolVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProtocolVersion(aProtocolVersion); } \
  [[nodiscard]] NS_IMETHOD GetTransferSize(uint64_t *aTransferSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTransferSize(aTransferSize); } \
  [[nodiscard]] NS_IMETHOD GetRequestSize(uint64_t *aRequestSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestSize(aRequestSize); } \
  [[nodiscard]] NS_IMETHOD GetDecodedBodySize(uint64_t *aDecodedBodySize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDecodedBodySize(aDecodedBodySize); } \
  [[nodiscard]] NS_IMETHOD GetEncodedBodySize(uint64_t *aEncodedBodySize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEncodedBodySize(aEncodedBodySize); } \
  [[nodiscard]] NS_IMETHOD GetRequestHeader(const nsACString& aHeader, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestHeader(aHeader, _retval); } \
  [[nodiscard]] NS_IMETHOD SetRequestHeader(const nsACString& aHeader, const nsACString& aValue, bool aMerge) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRequestHeader(aHeader, aValue, aMerge); } \
  [[nodiscard]] NS_IMETHOD SetNewReferrerInfo(const nsACString& aUrl, nsIReferrerInfo::ReferrerPolicyIDL aPolicy, bool aSendReferrer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNewReferrerInfo(aUrl, aPolicy, aSendReferrer); } \
  [[nodiscard]] NS_IMETHOD SetEmptyRequestHeader(const nsACString& aHeader) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEmptyRequestHeader(aHeader); } \
  [[nodiscard]] NS_IMETHOD VisitRequestHeaders(nsIHttpHeaderVisitor *aVisitor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VisitRequestHeaders(aVisitor); } \
  [[nodiscard]] NS_IMETHOD VisitNonDefaultRequestHeaders(nsIHttpHeaderVisitor *aVisitor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VisitNonDefaultRequestHeaders(aVisitor); } \
  [[nodiscard]] NS_IMETHOD ShouldStripRequestBodyHeader(const nsACString& aMethod, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShouldStripRequestBodyHeader(aMethod, _retval); } \
  [[nodiscard]] NS_IMETHOD GetAllowPipelining(bool *aAllowPipelining) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowPipelining(aAllowPipelining); } \
  [[nodiscard]] NS_IMETHOD SetAllowPipelining(bool aAllowPipelining) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowPipelining(aAllowPipelining); } \
  [[nodiscard]] NS_IMETHOD GetAllowSTS(bool *aAllowSTS) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowSTS(aAllowSTS); } \
  [[nodiscard]] NS_IMETHOD SetAllowSTS(bool aAllowSTS) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowSTS(aAllowSTS); } \
  [[nodiscard]] NS_IMETHOD GetRedirectionLimit(uint32_t *aRedirectionLimit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRedirectionLimit(aRedirectionLimit); } \
  [[nodiscard]] NS_IMETHOD SetRedirectionLimit(uint32_t aRedirectionLimit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRedirectionLimit(aRedirectionLimit); } \
  [[nodiscard]] NS_IMETHOD GetResponseStatus(uint32_t *aResponseStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResponseStatus(aResponseStatus); } \
  [[nodiscard]] NS_IMETHOD GetResponseStatusText(nsACString& aResponseStatusText) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResponseStatusText(aResponseStatusText); } \
  [[nodiscard]] NS_IMETHOD GetRequestSucceeded(bool *aRequestSucceeded) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestSucceeded(aRequestSucceeded); } \
  [[nodiscard]] NS_IMETHOD GetIsMainDocumentChannel(bool *aIsMainDocumentChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsMainDocumentChannel(aIsMainDocumentChannel); } \
  [[nodiscard]] NS_IMETHOD SetIsMainDocumentChannel(bool aIsMainDocumentChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsMainDocumentChannel(aIsMainDocumentChannel); } \
  [[nodiscard]] NS_IMETHOD GetResponseHeader(const nsACString& header, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResponseHeader(header, _retval); } \
  [[nodiscard]] NS_IMETHOD SetResponseHeader(const nsACString& header, const nsACString& value, bool merge) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetResponseHeader(header, value, merge); } \
  [[nodiscard]] NS_IMETHOD VisitResponseHeaders(nsIHttpHeaderVisitor *aVisitor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VisitResponseHeaders(aVisitor); } \
  [[nodiscard]] NS_IMETHOD GetOriginalResponseHeader(const nsACString& aHeader, nsIHttpHeaderVisitor *aVisitor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginalResponseHeader(aHeader, aVisitor); } \
  [[nodiscard]] NS_IMETHOD VisitOriginalResponseHeaders(nsIHttpHeaderVisitor *aVisitor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VisitOriginalResponseHeaders(aVisitor); } \
  [[nodiscard]] NS_IMETHOD IsNoStoreResponse(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsNoStoreResponse(_retval); } \
  [[nodiscard]] NS_IMETHOD IsNoCacheResponse(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsNoCacheResponse(_retval); } \
  [[nodiscard]] NS_IMETHOD IsPrivateResponse(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsPrivateResponse(_retval); } \
  [[nodiscard]] NS_IMETHOD RedirectTo(nsIURI *aTargetURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RedirectTo(aTargetURI); } \
  [[nodiscard]] NS_IMETHOD UpgradeToSecure(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpgradeToSecure(); } \
  [[nodiscard]] NS_IMETHOD GetRequestContextID(uint64_t *aRequestContextID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestContextID(aRequestContextID); } \
  [[nodiscard]] NS_IMETHOD SetRequestContextID(uint64_t aRequestContextID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRequestContextID(aRequestContextID); } \
  [[nodiscard]] NS_IMETHOD GetTopLevelContentWindowId(uint64_t *aTopLevelContentWindowId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTopLevelContentWindowId(aTopLevelContentWindowId); } \
  [[nodiscard]] NS_IMETHOD SetTopLevelContentWindowId(uint64_t aTopLevelContentWindowId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTopLevelContentWindowId(aTopLevelContentWindowId); } \
  NS_IMETHOD GetFlashPluginState(nsIHttpChannel::FlashPluginState *aFlashPluginState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFlashPluginState(aFlashPluginState); } \
  [[nodiscard]] NS_IMETHOD GetTopLevelOuterContentWindowId(uint64_t *aTopLevelOuterContentWindowId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTopLevelOuterContentWindowId(aTopLevelOuterContentWindowId); } \
  [[nodiscard]] NS_IMETHOD SetTopLevelOuterContentWindowId(uint64_t aTopLevelOuterContentWindowId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTopLevelOuterContentWindowId(aTopLevelOuterContentWindowId); } \
  NS_IMETHOD LogBlockedCORSRequest(const nsAString& aMessage, const nsACString& aCategory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LogBlockedCORSRequest(aMessage, aCategory); } \
  NS_IMETHOD LogMimeTypeMismatch(const nsACString& aMessageName, bool aWarning, const nsAString& aURL, const nsAString& aContentType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LogMimeTypeMismatch(aMessageName, aWarning, aURL, aContentType); } \


#endif /* __gen_nsIHttpChannel_h__ */
