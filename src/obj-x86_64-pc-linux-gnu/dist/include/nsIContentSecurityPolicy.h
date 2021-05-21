/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/security/nsIContentSecurityPolicy.idl
 */

#ifndef __gen_nsIContentSecurityPolicy_h__
#define __gen_nsIContentSecurityPolicy_h__


#ifndef __gen_nsISerializable_h__
#include "nsISerializable.h"
#endif

#ifndef __gen_nsIContentPolicy_h__
#include "nsIContentPolicy.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIEventTarget; /* forward declaration */

class nsILoadInfo; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsICSPEventListener; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

class nsCSPPolicy;

/* starting interface:    nsIContentSecurityPolicy */
#define NS_ICONTENTSECURITYPOLICY_IID_STR "1d632008-6c97-48ae-a51c-16e2daa0f4f6"

#define NS_ICONTENTSECURITYPOLICY_IID \
  {0x1d632008, 0x6c97, 0x48ae, \
    { 0xa5, 0x1c, 0x16, 0xe2, 0xda, 0xa0, 0xf4, 0xf6 }}

class nsIContentSecurityPolicy : public nsISerializable {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTSECURITYPOLICY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentSecurityPolicy;

  enum CSPDirective : uint8_t {
    NO_DIRECTIVE = 0,
    DEFAULT_SRC_DIRECTIVE = 1,
    SCRIPT_SRC_DIRECTIVE = 2,
    OBJECT_SRC_DIRECTIVE = 3,
    STYLE_SRC_DIRECTIVE = 4,
    IMG_SRC_DIRECTIVE = 5,
    MEDIA_SRC_DIRECTIVE = 6,
    FRAME_SRC_DIRECTIVE = 7,
    FONT_SRC_DIRECTIVE = 8,
    CONNECT_SRC_DIRECTIVE = 9,
    REPORT_URI_DIRECTIVE = 10,
    FRAME_ANCESTORS_DIRECTIVE = 11,
    REFLECTED_XSS_DIRECTIVE = 12,
    BASE_URI_DIRECTIVE = 13,
    FORM_ACTION_DIRECTIVE = 14,
    WEB_MANIFEST_SRC_DIRECTIVE = 15,
    UPGRADE_IF_INSECURE_DIRECTIVE = 16,
    CHILD_SRC_DIRECTIVE = 17,
    BLOCK_ALL_MIXED_CONTENT = 18,
    SANDBOX_DIRECTIVE = 19,
    WORKER_SRC_DIRECTIVE = 20,
    NAVIGATE_TO_DIRECTIVE = 21,
  };

  /* [binaryname(GetPolicyString)] AString getPolicy (in unsigned long index); */
  NS_IMETHOD GetPolicyString(uint32_t index, nsAString& _retval) = 0;

  /* [noscript,nostdcall,notxpcom] CSPPolicyPtr GetPolicy (in unsigned long index); */
  virtual const nsCSPPolicy * GetPolicy(uint32_t index) = 0;

  /* readonly attribute unsigned long policyCount; */
  NS_IMETHOD GetPolicyCount(uint32_t *aPolicyCount) = 0;

  /* readonly attribute bool upgradeInsecureRequests; */
  NS_IMETHOD GetUpgradeInsecureRequests(bool *aUpgradeInsecureRequests) = 0;

  /* readonly attribute bool blockAllMixedContent; */
  NS_IMETHOD GetBlockAllMixedContent(bool *aBlockAllMixedContent) = 0;

  /* readonly attribute bool enforcesFrameAncestors; */
  NS_IMETHOD GetEnforcesFrameAncestors(bool *aEnforcesFrameAncestors) = 0;

  /* void appendPolicy (in AString policyString, in boolean reportOnly, in boolean deliveredViaMetaTag); */
  NS_IMETHOD AppendPolicy(const nsAString& policyString, bool reportOnly, bool deliveredViaMetaTag) = 0;

  /* boolean getAllowsInline (in nsIContentSecurityPolicy_CSPDirective aDirective, in AString aNonce, in boolean aParserCreated, in Element aTriggeringElement, in nsICSPEventListener aCSPEventListener, in AString aContentOfPseudoScript, in unsigned long aLineNumber, in unsigned long aColumnNumber); */
  NS_IMETHOD GetAllowsInline(nsIContentSecurityPolicy::CSPDirective aDirective, const nsAString& aNonce, bool aParserCreated, mozilla::dom::Element *aTriggeringElement, nsICSPEventListener *aCSPEventListener, const nsAString& aContentOfPseudoScript, uint32_t aLineNumber, uint32_t aColumnNumber, bool *_retval) = 0;

  /* boolean getAllowsNavigateTo (in nsIURI aURI, in boolean aIsFormSubmission, in boolean aWasRedirected, in boolean aEnforceWhitelist); */
  NS_IMETHOD GetAllowsNavigateTo(nsIURI *aURI, bool aIsFormSubmission, bool aWasRedirected, bool aEnforceWhitelist, bool *_retval) = 0;

  /* boolean getAllowsEval (out boolean shouldReportViolations); */
  NS_IMETHOD GetAllowsEval(bool *shouldReportViolations, bool *_retval) = 0;

  /* uint32_t getCSPSandboxFlags (); */
  NS_IMETHOD GetCSPSandboxFlags(uint32_t *_retval) = 0;

  /* void logViolationDetails (in unsigned short violationType, in Element triggeringElement, in nsICSPEventListener aCSPEventListener, in AString sourceFile, in AString scriptSample, in int32_t lineNum, in int32_t columnNum, [optional] in AString nonce, [optional] in AString content); */
  NS_IMETHOD LogViolationDetails(uint16_t violationType, mozilla::dom::Element *triggeringElement, nsICSPEventListener *aCSPEventListener, const nsAString& sourceFile, const nsAString& scriptSample, int32_t lineNum, int32_t columnNum, const nsAString& nonce, const nsAString& content) = 0;

  enum {
    VIOLATION_TYPE_INLINE_SCRIPT = 1U,
    VIOLATION_TYPE_EVAL = 2U,
    VIOLATION_TYPE_INLINE_STYLE = 3U,
    VIOLATION_TYPE_NONCE_SCRIPT = 4U,
    VIOLATION_TYPE_NONCE_STYLE = 5U,
    VIOLATION_TYPE_HASH_SCRIPT = 6U,
    VIOLATION_TYPE_HASH_STYLE = 7U,
    VIOLATION_TYPE_REQUIRE_SRI_FOR_STYLE = 8U,
    VIOLATION_TYPE_REQUIRE_SRI_FOR_SCRIPT = 9U
  };

  /* [must_use] void setRequestContextWithDocument (in Document aDocument); */
  [[nodiscard]] NS_IMETHOD SetRequestContextWithDocument(mozilla::dom::Document *aDocument) = 0;

  /* [must_use] void setRequestContextWithPrincipal (in nsIPrincipal aRequestPrincipal, in nsIURI aSelfURI, in AString aReferrer, in unsigned long long aInnerWindowId); */
  [[nodiscard]] NS_IMETHOD SetRequestContextWithPrincipal(nsIPrincipal *aRequestPrincipal, nsIURI *aSelfURI, const nsAString& aReferrer, uint64_t aInnerWindowId) = 0;

  /* [noscript,nostdcall,notxpcom] readonly attribute nsIPrincipal requestPrincipal; */
  virtual nsIPrincipal * GetRequestPrincipal() = 0;

  /* [noscript,nostdcall,notxpcom] readonly attribute nsIURI selfURI; */
  virtual nsIURI * GetSelfURI() = 0;

  /* [noscript] readonly attribute AString referrer; */
  NS_IMETHOD GetReferrer(nsAString& aReferrer) = 0;

  /* [noscript,nostdcall,notxpcom] readonly attribute unsigned long long innerWindowID; */
  virtual uint64_t GetInnerWindowID() = 0;

  /* [noscript,nostdcall,notxpcom] attribute boolean skipAllowInlineStyleCheck; */
  virtual bool GetSkipAllowInlineStyleCheck() = 0;
  virtual void SetSkipAllowInlineStyleCheck(bool aSkipAllowInlineStyleCheck) = 0;

  /* [noscript] void ensureEventTarget (in nsIEventTarget aEventTarget); */
  NS_IMETHOD EnsureEventTarget(nsIEventTarget *aEventTarget) = 0;

  /* boolean permitsAncestry (in nsILoadInfo aLoadInfo); */
  NS_IMETHOD PermitsAncestry(nsILoadInfo *aLoadInfo, bool *_retval) = 0;

  /* boolean permits (in Element aTriggeringElement, in nsICSPEventListener aCSPEventListener, in nsIURI aURI, in nsIContentSecurityPolicy_CSPDirective aDir, in boolean aSpecific); */
  NS_IMETHOD Permits(mozilla::dom::Element *aTriggeringElement, nsICSPEventListener *aCSPEventListener, nsIURI *aURI, nsIContentSecurityPolicy::CSPDirective aDir, bool aSpecific, bool *_retval) = 0;

  /* short shouldLoad (in nsContentPolicyType aContentType, in nsICSPEventListener aCSPEventListener, in nsIURI aContentLocation, in nsIURI aOriginalURIIfRedirect, in bool aSendViolationReports, in AString aNonce, in boolean aParserCreated); */
  NS_IMETHOD ShouldLoad(nsContentPolicyType aContentType, nsICSPEventListener *aCSPEventListener, nsIURI *aContentLocation, nsIURI *aOriginalURIIfRedirect, bool aSendViolationReports, const nsAString& aNonce, bool aParserCreated, int16_t *_retval) = 0;

 // nsIObserver topic to fire when the policy encounters a violation.
#define CSP_VIOLATION_TOPIC "csp-on-violate-policy"
  /* AString toJSON (); */
  NS_IMETHOD ToJSON(nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentSecurityPolicy, NS_ICONTENTSECURITYPOLICY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTSECURITYPOLICY \
  NS_IMETHOD GetPolicyString(uint32_t index, nsAString& _retval) override; \
  virtual const nsCSPPolicy * GetPolicy(uint32_t index) override; \
  NS_IMETHOD GetPolicyCount(uint32_t *aPolicyCount) override; \
  NS_IMETHOD GetUpgradeInsecureRequests(bool *aUpgradeInsecureRequests) override; \
  NS_IMETHOD GetBlockAllMixedContent(bool *aBlockAllMixedContent) override; \
  NS_IMETHOD GetEnforcesFrameAncestors(bool *aEnforcesFrameAncestors) override; \
  NS_IMETHOD AppendPolicy(const nsAString& policyString, bool reportOnly, bool deliveredViaMetaTag) override; \
  NS_IMETHOD GetAllowsInline(nsIContentSecurityPolicy::CSPDirective aDirective, const nsAString& aNonce, bool aParserCreated, mozilla::dom::Element *aTriggeringElement, nsICSPEventListener *aCSPEventListener, const nsAString& aContentOfPseudoScript, uint32_t aLineNumber, uint32_t aColumnNumber, bool *_retval) override; \
  NS_IMETHOD GetAllowsNavigateTo(nsIURI *aURI, bool aIsFormSubmission, bool aWasRedirected, bool aEnforceWhitelist, bool *_retval) override; \
  NS_IMETHOD GetAllowsEval(bool *shouldReportViolations, bool *_retval) override; \
  NS_IMETHOD GetCSPSandboxFlags(uint32_t *_retval) override; \
  NS_IMETHOD LogViolationDetails(uint16_t violationType, mozilla::dom::Element *triggeringElement, nsICSPEventListener *aCSPEventListener, const nsAString& sourceFile, const nsAString& scriptSample, int32_t lineNum, int32_t columnNum, const nsAString& nonce, const nsAString& content) override; \
  [[nodiscard]] NS_IMETHOD SetRequestContextWithDocument(mozilla::dom::Document *aDocument) override; \
  [[nodiscard]] NS_IMETHOD SetRequestContextWithPrincipal(nsIPrincipal *aRequestPrincipal, nsIURI *aSelfURI, const nsAString& aReferrer, uint64_t aInnerWindowId) override; \
  virtual nsIPrincipal * GetRequestPrincipal() override; \
  virtual nsIURI * GetSelfURI() override; \
  NS_IMETHOD GetReferrer(nsAString& aReferrer) override; \
  virtual uint64_t GetInnerWindowID() override; \
  virtual bool GetSkipAllowInlineStyleCheck() override; \
  virtual void SetSkipAllowInlineStyleCheck(bool aSkipAllowInlineStyleCheck) override; \
  NS_IMETHOD EnsureEventTarget(nsIEventTarget *aEventTarget) override; \
  NS_IMETHOD PermitsAncestry(nsILoadInfo *aLoadInfo, bool *_retval) override; \
  NS_IMETHOD Permits(mozilla::dom::Element *aTriggeringElement, nsICSPEventListener *aCSPEventListener, nsIURI *aURI, nsIContentSecurityPolicy::CSPDirective aDir, bool aSpecific, bool *_retval) override; \
  NS_IMETHOD ShouldLoad(nsContentPolicyType aContentType, nsICSPEventListener *aCSPEventListener, nsIURI *aContentLocation, nsIURI *aOriginalURIIfRedirect, bool aSendViolationReports, const nsAString& aNonce, bool aParserCreated, int16_t *_retval) override; \
  NS_IMETHOD ToJSON(nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTSECURITYPOLICY \
  nsresult GetPolicyString(uint32_t index, nsAString& _retval); \
  const nsCSPPolicy * GetPolicy(uint32_t index); \
  nsresult GetPolicyCount(uint32_t *aPolicyCount); \
  nsresult GetUpgradeInsecureRequests(bool *aUpgradeInsecureRequests); \
  nsresult GetBlockAllMixedContent(bool *aBlockAllMixedContent); \
  nsresult GetEnforcesFrameAncestors(bool *aEnforcesFrameAncestors); \
  nsresult AppendPolicy(const nsAString& policyString, bool reportOnly, bool deliveredViaMetaTag); \
  nsresult GetAllowsInline(nsIContentSecurityPolicy::CSPDirective aDirective, const nsAString& aNonce, bool aParserCreated, mozilla::dom::Element *aTriggeringElement, nsICSPEventListener *aCSPEventListener, const nsAString& aContentOfPseudoScript, uint32_t aLineNumber, uint32_t aColumnNumber, bool *_retval); \
  nsresult GetAllowsNavigateTo(nsIURI *aURI, bool aIsFormSubmission, bool aWasRedirected, bool aEnforceWhitelist, bool *_retval); \
  nsresult GetAllowsEval(bool *shouldReportViolations, bool *_retval); \
  nsresult GetCSPSandboxFlags(uint32_t *_retval); \
  nsresult LogViolationDetails(uint16_t violationType, mozilla::dom::Element *triggeringElement, nsICSPEventListener *aCSPEventListener, const nsAString& sourceFile, const nsAString& scriptSample, int32_t lineNum, int32_t columnNum, const nsAString& nonce, const nsAString& content); \
  [[nodiscard]] nsresult SetRequestContextWithDocument(mozilla::dom::Document *aDocument); \
  [[nodiscard]] nsresult SetRequestContextWithPrincipal(nsIPrincipal *aRequestPrincipal, nsIURI *aSelfURI, const nsAString& aReferrer, uint64_t aInnerWindowId); \
  nsIPrincipal * GetRequestPrincipal(); \
  nsIURI * GetSelfURI(); \
  nsresult GetReferrer(nsAString& aReferrer); \
  uint64_t GetInnerWindowID(); \
  bool GetSkipAllowInlineStyleCheck(); \
  void SetSkipAllowInlineStyleCheck(bool aSkipAllowInlineStyleCheck); \
  nsresult EnsureEventTarget(nsIEventTarget *aEventTarget); \
  nsresult PermitsAncestry(nsILoadInfo *aLoadInfo, bool *_retval); \
  nsresult Permits(mozilla::dom::Element *aTriggeringElement, nsICSPEventListener *aCSPEventListener, nsIURI *aURI, nsIContentSecurityPolicy::CSPDirective aDir, bool aSpecific, bool *_retval); \
  nsresult ShouldLoad(nsContentPolicyType aContentType, nsICSPEventListener *aCSPEventListener, nsIURI *aContentLocation, nsIURI *aOriginalURIIfRedirect, bool aSendViolationReports, const nsAString& aNonce, bool aParserCreated, int16_t *_retval); \
  nsresult ToJSON(nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTSECURITYPOLICY(_to) \
  NS_IMETHOD GetPolicyString(uint32_t index, nsAString& _retval) override { return _to GetPolicyString(index, _retval); } \
  virtual const nsCSPPolicy * GetPolicy(uint32_t index) override { return _to GetPolicy(index); } \
  NS_IMETHOD GetPolicyCount(uint32_t *aPolicyCount) override { return _to GetPolicyCount(aPolicyCount); } \
  NS_IMETHOD GetUpgradeInsecureRequests(bool *aUpgradeInsecureRequests) override { return _to GetUpgradeInsecureRequests(aUpgradeInsecureRequests); } \
  NS_IMETHOD GetBlockAllMixedContent(bool *aBlockAllMixedContent) override { return _to GetBlockAllMixedContent(aBlockAllMixedContent); } \
  NS_IMETHOD GetEnforcesFrameAncestors(bool *aEnforcesFrameAncestors) override { return _to GetEnforcesFrameAncestors(aEnforcesFrameAncestors); } \
  NS_IMETHOD AppendPolicy(const nsAString& policyString, bool reportOnly, bool deliveredViaMetaTag) override { return _to AppendPolicy(policyString, reportOnly, deliveredViaMetaTag); } \
  NS_IMETHOD GetAllowsInline(nsIContentSecurityPolicy::CSPDirective aDirective, const nsAString& aNonce, bool aParserCreated, mozilla::dom::Element *aTriggeringElement, nsICSPEventListener *aCSPEventListener, const nsAString& aContentOfPseudoScript, uint32_t aLineNumber, uint32_t aColumnNumber, bool *_retval) override { return _to GetAllowsInline(aDirective, aNonce, aParserCreated, aTriggeringElement, aCSPEventListener, aContentOfPseudoScript, aLineNumber, aColumnNumber, _retval); } \
  NS_IMETHOD GetAllowsNavigateTo(nsIURI *aURI, bool aIsFormSubmission, bool aWasRedirected, bool aEnforceWhitelist, bool *_retval) override { return _to GetAllowsNavigateTo(aURI, aIsFormSubmission, aWasRedirected, aEnforceWhitelist, _retval); } \
  NS_IMETHOD GetAllowsEval(bool *shouldReportViolations, bool *_retval) override { return _to GetAllowsEval(shouldReportViolations, _retval); } \
  NS_IMETHOD GetCSPSandboxFlags(uint32_t *_retval) override { return _to GetCSPSandboxFlags(_retval); } \
  NS_IMETHOD LogViolationDetails(uint16_t violationType, mozilla::dom::Element *triggeringElement, nsICSPEventListener *aCSPEventListener, const nsAString& sourceFile, const nsAString& scriptSample, int32_t lineNum, int32_t columnNum, const nsAString& nonce, const nsAString& content) override { return _to LogViolationDetails(violationType, triggeringElement, aCSPEventListener, sourceFile, scriptSample, lineNum, columnNum, nonce, content); } \
  [[nodiscard]] NS_IMETHOD SetRequestContextWithDocument(mozilla::dom::Document *aDocument) override { return _to SetRequestContextWithDocument(aDocument); } \
  [[nodiscard]] NS_IMETHOD SetRequestContextWithPrincipal(nsIPrincipal *aRequestPrincipal, nsIURI *aSelfURI, const nsAString& aReferrer, uint64_t aInnerWindowId) override { return _to SetRequestContextWithPrincipal(aRequestPrincipal, aSelfURI, aReferrer, aInnerWindowId); } \
  virtual nsIPrincipal * GetRequestPrincipal() override { return _to GetRequestPrincipal(); } \
  virtual nsIURI * GetSelfURI() override { return _to GetSelfURI(); } \
  NS_IMETHOD GetReferrer(nsAString& aReferrer) override { return _to GetReferrer(aReferrer); } \
  virtual uint64_t GetInnerWindowID() override { return _to GetInnerWindowID(); } \
  virtual bool GetSkipAllowInlineStyleCheck() override { return _to GetSkipAllowInlineStyleCheck(); } \
  virtual void SetSkipAllowInlineStyleCheck(bool aSkipAllowInlineStyleCheck) override { return _to SetSkipAllowInlineStyleCheck(aSkipAllowInlineStyleCheck); } \
  NS_IMETHOD EnsureEventTarget(nsIEventTarget *aEventTarget) override { return _to EnsureEventTarget(aEventTarget); } \
  NS_IMETHOD PermitsAncestry(nsILoadInfo *aLoadInfo, bool *_retval) override { return _to PermitsAncestry(aLoadInfo, _retval); } \
  NS_IMETHOD Permits(mozilla::dom::Element *aTriggeringElement, nsICSPEventListener *aCSPEventListener, nsIURI *aURI, nsIContentSecurityPolicy::CSPDirective aDir, bool aSpecific, bool *_retval) override { return _to Permits(aTriggeringElement, aCSPEventListener, aURI, aDir, aSpecific, _retval); } \
  NS_IMETHOD ShouldLoad(nsContentPolicyType aContentType, nsICSPEventListener *aCSPEventListener, nsIURI *aContentLocation, nsIURI *aOriginalURIIfRedirect, bool aSendViolationReports, const nsAString& aNonce, bool aParserCreated, int16_t *_retval) override { return _to ShouldLoad(aContentType, aCSPEventListener, aContentLocation, aOriginalURIIfRedirect, aSendViolationReports, aNonce, aParserCreated, _retval); } \
  NS_IMETHOD ToJSON(nsAString& _retval) override { return _to ToJSON(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTSECURITYPOLICY(_to) \
  NS_IMETHOD GetPolicyString(uint32_t index, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPolicyString(index, _retval); } \
  virtual const nsCSPPolicy * GetPolicy(uint32_t index) override; \
  NS_IMETHOD GetPolicyCount(uint32_t *aPolicyCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPolicyCount(aPolicyCount); } \
  NS_IMETHOD GetUpgradeInsecureRequests(bool *aUpgradeInsecureRequests) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUpgradeInsecureRequests(aUpgradeInsecureRequests); } \
  NS_IMETHOD GetBlockAllMixedContent(bool *aBlockAllMixedContent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBlockAllMixedContent(aBlockAllMixedContent); } \
  NS_IMETHOD GetEnforcesFrameAncestors(bool *aEnforcesFrameAncestors) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEnforcesFrameAncestors(aEnforcesFrameAncestors); } \
  NS_IMETHOD AppendPolicy(const nsAString& policyString, bool reportOnly, bool deliveredViaMetaTag) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendPolicy(policyString, reportOnly, deliveredViaMetaTag); } \
  NS_IMETHOD GetAllowsInline(nsIContentSecurityPolicy::CSPDirective aDirective, const nsAString& aNonce, bool aParserCreated, mozilla::dom::Element *aTriggeringElement, nsICSPEventListener *aCSPEventListener, const nsAString& aContentOfPseudoScript, uint32_t aLineNumber, uint32_t aColumnNumber, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowsInline(aDirective, aNonce, aParserCreated, aTriggeringElement, aCSPEventListener, aContentOfPseudoScript, aLineNumber, aColumnNumber, _retval); } \
  NS_IMETHOD GetAllowsNavigateTo(nsIURI *aURI, bool aIsFormSubmission, bool aWasRedirected, bool aEnforceWhitelist, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowsNavigateTo(aURI, aIsFormSubmission, aWasRedirected, aEnforceWhitelist, _retval); } \
  NS_IMETHOD GetAllowsEval(bool *shouldReportViolations, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowsEval(shouldReportViolations, _retval); } \
  NS_IMETHOD GetCSPSandboxFlags(uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCSPSandboxFlags(_retval); } \
  NS_IMETHOD LogViolationDetails(uint16_t violationType, mozilla::dom::Element *triggeringElement, nsICSPEventListener *aCSPEventListener, const nsAString& sourceFile, const nsAString& scriptSample, int32_t lineNum, int32_t columnNum, const nsAString& nonce, const nsAString& content) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LogViolationDetails(violationType, triggeringElement, aCSPEventListener, sourceFile, scriptSample, lineNum, columnNum, nonce, content); } \
  [[nodiscard]] NS_IMETHOD SetRequestContextWithDocument(mozilla::dom::Document *aDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRequestContextWithDocument(aDocument); } \
  [[nodiscard]] NS_IMETHOD SetRequestContextWithPrincipal(nsIPrincipal *aRequestPrincipal, nsIURI *aSelfURI, const nsAString& aReferrer, uint64_t aInnerWindowId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRequestContextWithPrincipal(aRequestPrincipal, aSelfURI, aReferrer, aInnerWindowId); } \
  virtual nsIPrincipal * GetRequestPrincipal() override; \
  virtual nsIURI * GetSelfURI() override; \
  NS_IMETHOD GetReferrer(nsAString& aReferrer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReferrer(aReferrer); } \
  virtual uint64_t GetInnerWindowID() override; \
  virtual bool GetSkipAllowInlineStyleCheck() override; \
  virtual void SetSkipAllowInlineStyleCheck(bool aSkipAllowInlineStyleCheck) override; \
  NS_IMETHOD EnsureEventTarget(nsIEventTarget *aEventTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnsureEventTarget(aEventTarget); } \
  NS_IMETHOD PermitsAncestry(nsILoadInfo *aLoadInfo, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PermitsAncestry(aLoadInfo, _retval); } \
  NS_IMETHOD Permits(mozilla::dom::Element *aTriggeringElement, nsICSPEventListener *aCSPEventListener, nsIURI *aURI, nsIContentSecurityPolicy::CSPDirective aDir, bool aSpecific, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Permits(aTriggeringElement, aCSPEventListener, aURI, aDir, aSpecific, _retval); } \
  NS_IMETHOD ShouldLoad(nsContentPolicyType aContentType, nsICSPEventListener *aCSPEventListener, nsIURI *aContentLocation, nsIURI *aOriginalURIIfRedirect, bool aSendViolationReports, const nsAString& aNonce, bool aParserCreated, int16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShouldLoad(aContentType, aCSPEventListener, aContentLocation, aOriginalURIIfRedirect, aSendViolationReports, aNonce, aParserCreated, _retval); } \
  NS_IMETHOD ToJSON(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ToJSON(_retval); } 

typedef nsIContentSecurityPolicy::CSPDirective  CSPDirective;


/* starting interface:    nsICSPEventListener */
#define NS_ICSPEVENTLISTENER_IID_STR "c3163b14-3a8f-46dd-a4af-bd04680364cd"

#define NS_ICSPEVENTLISTENER_IID \
  {0xc3163b14, 0x3a8f, 0x46dd, \
    { 0xa4, 0xaf, 0xbd, 0x04, 0x68, 0x03, 0x64, 0xcd }}

class NS_NO_VTABLE nsICSPEventListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICSPEVENTLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICSPEventListener;

  /* void onCSPViolationEvent (in AString aJSON); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnCSPViolationEvent(const nsAString& aJSON) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICSPEventListener, NS_ICSPEVENTLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICSPEVENTLISTENER \
  NS_IMETHOD OnCSPViolationEvent(const nsAString& aJSON) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICSPEVENTLISTENER \
  nsresult OnCSPViolationEvent(const nsAString& aJSON); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICSPEVENTLISTENER(_to) \
  NS_IMETHOD OnCSPViolationEvent(const nsAString& aJSON) override { return _to OnCSPViolationEvent(aJSON); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICSPEVENTLISTENER(_to) \
  NS_IMETHOD OnCSPViolationEvent(const nsAString& aJSON) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnCSPViolationEvent(aJSON); } 


#endif /* __gen_nsIContentSecurityPolicy_h__ */
