/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIExternalHelperAppService.idl
 */

#ifndef __gen_nsIExternalHelperAppService_h__
#define __gen_nsIExternalHelperAppService_h__


#ifndef __gen_nsICancelable_h__
#include "nsICancelable.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIRequest; /* forward declaration */

class nsIStreamListener; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIMIMEInfo; /* forward declaration */

class nsIWebProgressListener2; /* forward declaration */

class nsIInterfaceRequestor; /* forward declaration */

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIExternalHelperAppService */
#define NS_IEXTERNALHELPERAPPSERVICE_IID_STR "1e4f3ae1-b737-431f-a95d-31fa8da70199"

#define NS_IEXTERNALHELPERAPPSERVICE_IID \
  {0x1e4f3ae1, 0xb737, 0x431f, \
    { 0xa9, 0x5d, 0x31, 0xfa, 0x8d, 0xa7, 0x01, 0x99 }}

class NS_NO_VTABLE nsIExternalHelperAppService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEXTERNALHELPERAPPSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIExternalHelperAppService;

  /* nsIStreamListener doContent (in ACString aMimeContentType, in nsIRequest aRequest, in nsIInterfaceRequestor aContentContext, in boolean aForceSave, [optional] in nsIInterfaceRequestor aWindowContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DoContent(const nsACString& aMimeContentType, nsIRequest *aRequest, nsIInterfaceRequestor *aContentContext, bool aForceSave, nsIInterfaceRequestor *aWindowContext, nsIStreamListener **_retval) = 0;

  /* nsIStreamListener createListener (in ACString aMimeContentType, in nsIRequest aRequest, in BrowsingContext aContentContext, in boolean aForceSave, [optional] in nsIInterfaceRequestor aWindowContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateListener(const nsACString& aMimeContentType, nsIRequest *aRequest, mozilla::dom::BrowsingContext *aContentContext, bool aForceSave, nsIInterfaceRequestor *aWindowContext, nsIStreamListener **_retval) = 0;

  /* boolean applyDecodingForExtension (in AUTF8String aExtension, in ACString aEncodingType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ApplyDecodingForExtension(const nsACString& aExtension, const nsACString& aEncodingType, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIExternalHelperAppService, NS_IEXTERNALHELPERAPPSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEXTERNALHELPERAPPSERVICE \
  NS_IMETHOD DoContent(const nsACString& aMimeContentType, nsIRequest *aRequest, nsIInterfaceRequestor *aContentContext, bool aForceSave, nsIInterfaceRequestor *aWindowContext, nsIStreamListener **_retval) override; \
  NS_IMETHOD CreateListener(const nsACString& aMimeContentType, nsIRequest *aRequest, mozilla::dom::BrowsingContext *aContentContext, bool aForceSave, nsIInterfaceRequestor *aWindowContext, nsIStreamListener **_retval) override; \
  NS_IMETHOD ApplyDecodingForExtension(const nsACString& aExtension, const nsACString& aEncodingType, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEXTERNALHELPERAPPSERVICE \
  nsresult DoContent(const nsACString& aMimeContentType, nsIRequest *aRequest, nsIInterfaceRequestor *aContentContext, bool aForceSave, nsIInterfaceRequestor *aWindowContext, nsIStreamListener **_retval); \
  nsresult CreateListener(const nsACString& aMimeContentType, nsIRequest *aRequest, mozilla::dom::BrowsingContext *aContentContext, bool aForceSave, nsIInterfaceRequestor *aWindowContext, nsIStreamListener **_retval); \
  nsresult ApplyDecodingForExtension(const nsACString& aExtension, const nsACString& aEncodingType, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEXTERNALHELPERAPPSERVICE(_to) \
  NS_IMETHOD DoContent(const nsACString& aMimeContentType, nsIRequest *aRequest, nsIInterfaceRequestor *aContentContext, bool aForceSave, nsIInterfaceRequestor *aWindowContext, nsIStreamListener **_retval) override { return _to DoContent(aMimeContentType, aRequest, aContentContext, aForceSave, aWindowContext, _retval); } \
  NS_IMETHOD CreateListener(const nsACString& aMimeContentType, nsIRequest *aRequest, mozilla::dom::BrowsingContext *aContentContext, bool aForceSave, nsIInterfaceRequestor *aWindowContext, nsIStreamListener **_retval) override { return _to CreateListener(aMimeContentType, aRequest, aContentContext, aForceSave, aWindowContext, _retval); } \
  NS_IMETHOD ApplyDecodingForExtension(const nsACString& aExtension, const nsACString& aEncodingType, bool *_retval) override { return _to ApplyDecodingForExtension(aExtension, aEncodingType, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEXTERNALHELPERAPPSERVICE(_to) \
  NS_IMETHOD DoContent(const nsACString& aMimeContentType, nsIRequest *aRequest, nsIInterfaceRequestor *aContentContext, bool aForceSave, nsIInterfaceRequestor *aWindowContext, nsIStreamListener **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoContent(aMimeContentType, aRequest, aContentContext, aForceSave, aWindowContext, _retval); } \
  NS_IMETHOD CreateListener(const nsACString& aMimeContentType, nsIRequest *aRequest, mozilla::dom::BrowsingContext *aContentContext, bool aForceSave, nsIInterfaceRequestor *aWindowContext, nsIStreamListener **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateListener(aMimeContentType, aRequest, aContentContext, aForceSave, aWindowContext, _retval); } \
  NS_IMETHOD ApplyDecodingForExtension(const nsACString& aExtension, const nsACString& aEncodingType, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ApplyDecodingForExtension(aExtension, aEncodingType, _retval); } 


/* starting interface:    nsPIExternalAppLauncher */
#define NS_PIEXTERNALAPPLAUNCHER_IID_STR "6613e2e7-feab-4e3a-bb1f-b03200d544ec"

#define NS_PIEXTERNALAPPLAUNCHER_IID \
  {0x6613e2e7, 0xfeab, 0x4e3a, \
    { 0xbb, 0x1f, 0xb0, 0x32, 0x00, 0xd5, 0x44, 0xec }}

class NS_NO_VTABLE nsPIExternalAppLauncher : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_PIEXTERNALAPPLAUNCHER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsPIExternalAppLauncher;

  /* void deleteTemporaryFileOnExit (in nsIFile aTemporaryFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTemporaryFileOnExit(nsIFile *aTemporaryFile) = 0;

  /* void deleteTemporaryPrivateFileWhenPossible (in nsIFile aTemporaryFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteTemporaryPrivateFileWhenPossible(nsIFile *aTemporaryFile) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsPIExternalAppLauncher, NS_PIEXTERNALAPPLAUNCHER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSPIEXTERNALAPPLAUNCHER \
  NS_IMETHOD DeleteTemporaryFileOnExit(nsIFile *aTemporaryFile) override; \
  NS_IMETHOD DeleteTemporaryPrivateFileWhenPossible(nsIFile *aTemporaryFile) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSPIEXTERNALAPPLAUNCHER \
  nsresult DeleteTemporaryFileOnExit(nsIFile *aTemporaryFile); \
  nsresult DeleteTemporaryPrivateFileWhenPossible(nsIFile *aTemporaryFile); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSPIEXTERNALAPPLAUNCHER(_to) \
  NS_IMETHOD DeleteTemporaryFileOnExit(nsIFile *aTemporaryFile) override { return _to DeleteTemporaryFileOnExit(aTemporaryFile); } \
  NS_IMETHOD DeleteTemporaryPrivateFileWhenPossible(nsIFile *aTemporaryFile) override { return _to DeleteTemporaryPrivateFileWhenPossible(aTemporaryFile); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSPIEXTERNALAPPLAUNCHER(_to) \
  NS_IMETHOD DeleteTemporaryFileOnExit(nsIFile *aTemporaryFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteTemporaryFileOnExit(aTemporaryFile); } \
  NS_IMETHOD DeleteTemporaryPrivateFileWhenPossible(nsIFile *aTemporaryFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteTemporaryPrivateFileWhenPossible(aTemporaryFile); } 


/* starting interface:    nsIHelperAppLauncher */
#define NS_IHELPERAPPLAUNCHER_IID_STR "acf2a516-7d7f-4771-8b22-6c4a559c088e"

#define NS_IHELPERAPPLAUNCHER_IID \
  {0xacf2a516, 0x7d7f, 0x4771, \
    { 0x8b, 0x22, 0x6c, 0x4a, 0x55, 0x9c, 0x08, 0x8e }}

class NS_NO_VTABLE nsIHelperAppLauncher : public nsICancelable {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHELPERAPPLAUNCHER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHelperAppLauncher;

  /* readonly attribute nsIMIMEInfo MIMEInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMIMEInfo(nsIMIMEInfo **aMIMEInfo) = 0;

  /* readonly attribute nsIURI source; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSource(nsIURI **aSource) = 0;

  /* readonly attribute AString suggestedFileName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSuggestedFileName(nsAString& aSuggestedFileName) = 0;

  /* void promptForSaveDestination (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PromptForSaveDestination(void) = 0;

  /* void launchWithApplication (in boolean aHandleInternally); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LaunchWithApplication(bool aHandleInternally) = 0;

  /* void saveDestinationAvailable (in nsIFile aFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SaveDestinationAvailable(nsIFile *aFile) = 0;

  /* void setWebProgressListener (in nsIWebProgressListener2 aWebProgressListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetWebProgressListener(nsIWebProgressListener2 *aWebProgressListener) = 0;

  /* readonly attribute nsIFile targetFile; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTargetFile(nsIFile **aTargetFile) = 0;

  /* readonly attribute boolean targetFileIsExecutable; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTargetFileIsExecutable(bool *aTargetFileIsExecutable) = 0;

  /* readonly attribute PRTime timeDownloadStarted; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTimeDownloadStarted(PRTime *aTimeDownloadStarted) = 0;

  /* readonly attribute int64_t contentLength; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContentLength(int64_t *aContentLength) = 0;

  /* readonly attribute uint64_t browsingContextId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBrowsingContextId(uint64_t *aBrowsingContextId) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHelperAppLauncher, NS_IHELPERAPPLAUNCHER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHELPERAPPLAUNCHER \
  NS_IMETHOD GetMIMEInfo(nsIMIMEInfo **aMIMEInfo) override; \
  NS_IMETHOD GetSource(nsIURI **aSource) override; \
  NS_IMETHOD GetSuggestedFileName(nsAString& aSuggestedFileName) override; \
  NS_IMETHOD PromptForSaveDestination(void) override; \
  NS_IMETHOD LaunchWithApplication(bool aHandleInternally) override; \
  NS_IMETHOD SaveDestinationAvailable(nsIFile *aFile) override; \
  NS_IMETHOD SetWebProgressListener(nsIWebProgressListener2 *aWebProgressListener) override; \
  NS_IMETHOD GetTargetFile(nsIFile **aTargetFile) override; \
  NS_IMETHOD GetTargetFileIsExecutable(bool *aTargetFileIsExecutable) override; \
  NS_IMETHOD GetTimeDownloadStarted(PRTime *aTimeDownloadStarted) override; \
  NS_IMETHOD GetContentLength(int64_t *aContentLength) override; \
  NS_IMETHOD GetBrowsingContextId(uint64_t *aBrowsingContextId) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHELPERAPPLAUNCHER \
  nsresult GetMIMEInfo(nsIMIMEInfo **aMIMEInfo); \
  nsresult GetSource(nsIURI **aSource); \
  nsresult GetSuggestedFileName(nsAString& aSuggestedFileName); \
  nsresult PromptForSaveDestination(void); \
  nsresult LaunchWithApplication(bool aHandleInternally); \
  nsresult SaveDestinationAvailable(nsIFile *aFile); \
  nsresult SetWebProgressListener(nsIWebProgressListener2 *aWebProgressListener); \
  nsresult GetTargetFile(nsIFile **aTargetFile); \
  nsresult GetTargetFileIsExecutable(bool *aTargetFileIsExecutable); \
  nsresult GetTimeDownloadStarted(PRTime *aTimeDownloadStarted); \
  nsresult GetContentLength(int64_t *aContentLength); \
  nsresult GetBrowsingContextId(uint64_t *aBrowsingContextId); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHELPERAPPLAUNCHER(_to) \
  NS_IMETHOD GetMIMEInfo(nsIMIMEInfo **aMIMEInfo) override { return _to GetMIMEInfo(aMIMEInfo); } \
  NS_IMETHOD GetSource(nsIURI **aSource) override { return _to GetSource(aSource); } \
  NS_IMETHOD GetSuggestedFileName(nsAString& aSuggestedFileName) override { return _to GetSuggestedFileName(aSuggestedFileName); } \
  NS_IMETHOD PromptForSaveDestination(void) override { return _to PromptForSaveDestination(); } \
  NS_IMETHOD LaunchWithApplication(bool aHandleInternally) override { return _to LaunchWithApplication(aHandleInternally); } \
  NS_IMETHOD SaveDestinationAvailable(nsIFile *aFile) override { return _to SaveDestinationAvailable(aFile); } \
  NS_IMETHOD SetWebProgressListener(nsIWebProgressListener2 *aWebProgressListener) override { return _to SetWebProgressListener(aWebProgressListener); } \
  NS_IMETHOD GetTargetFile(nsIFile **aTargetFile) override { return _to GetTargetFile(aTargetFile); } \
  NS_IMETHOD GetTargetFileIsExecutable(bool *aTargetFileIsExecutable) override { return _to GetTargetFileIsExecutable(aTargetFileIsExecutable); } \
  NS_IMETHOD GetTimeDownloadStarted(PRTime *aTimeDownloadStarted) override { return _to GetTimeDownloadStarted(aTimeDownloadStarted); } \
  NS_IMETHOD GetContentLength(int64_t *aContentLength) override { return _to GetContentLength(aContentLength); } \
  NS_IMETHOD GetBrowsingContextId(uint64_t *aBrowsingContextId) override { return _to GetBrowsingContextId(aBrowsingContextId); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHELPERAPPLAUNCHER(_to) \
  NS_IMETHOD GetMIMEInfo(nsIMIMEInfo **aMIMEInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMIMEInfo(aMIMEInfo); } \
  NS_IMETHOD GetSource(nsIURI **aSource) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSource(aSource); } \
  NS_IMETHOD GetSuggestedFileName(nsAString& aSuggestedFileName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSuggestedFileName(aSuggestedFileName); } \
  NS_IMETHOD PromptForSaveDestination(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PromptForSaveDestination(); } \
  NS_IMETHOD LaunchWithApplication(bool aHandleInternally) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LaunchWithApplication(aHandleInternally); } \
  NS_IMETHOD SaveDestinationAvailable(nsIFile *aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SaveDestinationAvailable(aFile); } \
  NS_IMETHOD SetWebProgressListener(nsIWebProgressListener2 *aWebProgressListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetWebProgressListener(aWebProgressListener); } \
  NS_IMETHOD GetTargetFile(nsIFile **aTargetFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTargetFile(aTargetFile); } \
  NS_IMETHOD GetTargetFileIsExecutable(bool *aTargetFileIsExecutable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTargetFileIsExecutable(aTargetFileIsExecutable); } \
  NS_IMETHOD GetTimeDownloadStarted(PRTime *aTimeDownloadStarted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTimeDownloadStarted(aTimeDownloadStarted); } \
  NS_IMETHOD GetContentLength(int64_t *aContentLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentLength(aContentLength); } \
  NS_IMETHOD GetBrowsingContextId(uint64_t *aBrowsingContextId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBrowsingContextId(aBrowsingContextId); } 


#endif /* __gen_nsIExternalHelperAppService_h__ */
