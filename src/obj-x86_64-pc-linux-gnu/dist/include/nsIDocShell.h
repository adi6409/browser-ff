/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIDocShell.idl
 */

#ifndef __gen_nsIDocShell_h__
#define __gen_nsIDocShell_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#ifndef __gen_nsIDocShellTreeItem_h__
#include "nsIDocShellTreeItem.h"
#endif

#ifndef __gen_nsIRequest_h__
#include "nsIRequest.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "js/TypeDecls.h"
#include "mozilla/Maybe.h"
#include "mozilla/NotNull.h"
#include "mozilla/UniquePtr.h"
#include "nsCOMPtr.h"
#include "nsIURI.h"
class nsCommandManager;
class nsPresContext;
class nsDocShellLoadState;
namespace mozilla {
class Encoding;
class HTMLEditor;
class PresShell;
namespace dom {
class BrowsingContext;
class ClientSource;
} // namespace dom
}
class nsIURI; /* forward declaration */

class nsIChannel; /* forward declaration */

class nsIContentViewer; /* forward declaration */

class nsIContentSecurityPolicy; /* forward declaration */

class nsIDocShellLoadInfo; /* forward declaration */

class nsIEditor; /* forward declaration */

class nsIEditingSession; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsIRequest; /* forward declaration */

class nsISHEntry; /* forward declaration */

class nsILayoutHistoryState; /* forward declaration */

class nsISecureBrowserUI; /* forward declaration */

class nsIScriptGlobalObject; /* forward declaration */

class nsIStructuredCloneContainer; /* forward declaration */

class nsIDOMStorage; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIWebBrowserPrint; /* forward declaration */

class nsIPrivacyTransitionObserver; /* forward declaration */

class nsIReflowObserver; /* forward declaration */

class nsIScrollObserver; /* forward declaration */

class nsIRemoteTab; /* forward declaration */

class nsIBrowserChild; /* forward declaration */

class nsICommandParams; /* forward declaration */

class nsILoadURIDelegate; /* forward declaration */

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class ContentFrameMessageManager; /* webidl ContentFrameMessageManager */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class EventTarget; /* webidl EventTarget */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDocShell */
#define NS_IDOCSHELL_IID_STR "049234fe-da10-478b-bc5d-bc6f9a1ba63d"

#define NS_IDOCSHELL_IID \
  {0x049234fe, 0xda10, 0x478b, \
    { 0xbc, 0x5d, 0xbc, 0x6f, 0x9a, 0x1b, 0xa6, 0x3d }}

class nsIDocShell : public nsIDocShellTreeItem {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOCSHELL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDocShell;

  /* void setCancelContentJSEpoch (in long aEpoch); */
  NS_IMETHOD SetCancelContentJSEpoch(int32_t aEpoch) = 0;

  /* [noscript] void loadURI (in nsDocShellLoadStatePtr aLoadState, in boolean aSetNavigating); */
  NS_IMETHOD LoadURI(nsDocShellLoadState* aLoadState, bool aSetNavigating) = 0;

  /* [implicit_jscontext] void addState (in jsval aData, in AString aTitle, in AString aURL, in boolean aReplace); */
  NS_IMETHOD AddState(JS::HandleValue aData, const nsAString& aTitle, const nsAString& aURL, bool aReplace, JSContext* cx) = 0;

  /* [nostdcall] void updateURLAndHistory (in Document aDocument, in nsIURI aNewURI, in nsIStructuredCloneContainer aData, in AString aTitle, in boolean aReplace, in nsIURI aCurrentURI, in boolean aEqualURIs); */
  virtual nsresult UpdateURLAndHistory(mozilla::dom::Document *aDocument, nsIURI *aNewURI, nsIStructuredCloneContainer *aData, const nsAString& aTitle, bool aReplace, nsIURI *aCurrentURI, bool aEqualURIs) = 0;

  /* void prepareForNewContentModel (); */
  NS_IMETHOD PrepareForNewContentModel(void) = 0;

  /* void setCurrentURI (in nsIURI aURI); */
  NS_IMETHOD SetCurrentURI(nsIURI *aURI) = 0;

  /* [noscript] void firePageHideNotification (in boolean isUnload); */
  NS_IMETHOD FirePageHideNotification(bool isUnload) = 0;

  /* [nostdcall,notxpcom] readonly attribute nsPresContext presContext; */
  virtual nsPresContext * GetPresContext() = 0;

  /* [nostdcall,notxpcom] readonly attribute PresShell presShell; */
  virtual mozilla::PresShell * GetPresShell() = 0;

  /* [nostdcall,notxpcom] readonly attribute PresShell eldestPresShell; */
  virtual mozilla::PresShell * GetEldestPresShell() = 0;

  /* [infallible] readonly attribute nsIContentViewer contentViewer; */
  NS_IMETHOD GetContentViewer(nsIContentViewer **aContentViewer) = 0;
   inline already_AddRefed<nsIContentViewer> GetContentViewer()
  {
    nsIContentViewer* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetContentViewer(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIContentViewer>(result);
  }

  /* [infallible] readonly attribute unsigned long long outerWindowID; */
  NS_IMETHOD GetOuterWindowID(uint64_t *aOuterWindowID) = 0;
  inline uint64_t  GetOuterWindowID()
  {
    uint64_t result;
    mozilla::DebugOnly<nsresult> rv = GetOuterWindowID(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* attribute EventTarget chromeEventHandler; */
  NS_IMETHOD GetChromeEventHandler(mozilla::dom::EventTarget **aChromeEventHandler) = 0;
  NS_IMETHOD SetChromeEventHandler(mozilla::dom::EventTarget *aChromeEventHandler) = 0;

  /* attribute AString customUserAgent; */
  NS_IMETHOD GetCustomUserAgent(nsAString& aCustomUserAgent) = 0;
  NS_IMETHOD SetCustomUserAgent(const nsAString& aCustomUserAgent) = 0;

  /* attribute boolean cssErrorReportingEnabled; */
  NS_IMETHOD GetCssErrorReportingEnabled(bool *aCssErrorReportingEnabled) = 0;
  NS_IMETHOD SetCssErrorReportingEnabled(bool aCssErrorReportingEnabled) = 0;

  /* attribute boolean allowPlugins; */
  NS_IMETHOD GetAllowPlugins(bool *aAllowPlugins) = 0;
  NS_IMETHOD SetAllowPlugins(bool aAllowPlugins) = 0;

  /* attribute boolean allowJavascript; */
  NS_IMETHOD GetAllowJavascript(bool *aAllowJavascript) = 0;
  NS_IMETHOD SetAllowJavascript(bool aAllowJavascript) = 0;

  /* attribute boolean allowMetaRedirects; */
  NS_IMETHOD GetAllowMetaRedirects(bool *aAllowMetaRedirects) = 0;
  NS_IMETHOD SetAllowMetaRedirects(bool aAllowMetaRedirects) = 0;

  /* attribute boolean allowSubframes; */
  NS_IMETHOD GetAllowSubframes(bool *aAllowSubframes) = 0;
  NS_IMETHOD SetAllowSubframes(bool aAllowSubframes) = 0;

  /* attribute boolean allowImages; */
  NS_IMETHOD GetAllowImages(bool *aAllowImages) = 0;
  NS_IMETHOD SetAllowImages(bool aAllowImages) = 0;

  /* [infallible] attribute boolean allowMedia; */
  NS_IMETHOD GetAllowMedia(bool *aAllowMedia) = 0;
  inline bool  GetAllowMedia()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetAllowMedia(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetAllowMedia(bool aAllowMedia) = 0;

  /* attribute boolean allowDNSPrefetch; */
  NS_IMETHOD GetAllowDNSPrefetch(bool *aAllowDNSPrefetch) = 0;
  NS_IMETHOD SetAllowDNSPrefetch(bool aAllowDNSPrefetch) = 0;

  /* attribute boolean allowWindowControl; */
  NS_IMETHOD GetAllowWindowControl(bool *aAllowWindowControl) = 0;
  NS_IMETHOD SetAllowWindowControl(bool aAllowWindowControl) = 0;

  /* [infallible] attribute boolean allowContentRetargeting; */
  NS_IMETHOD GetAllowContentRetargeting(bool *aAllowContentRetargeting) = 0;
  inline bool  GetAllowContentRetargeting()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetAllowContentRetargeting(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetAllowContentRetargeting(bool aAllowContentRetargeting) = 0;

  /* [infallible] attribute boolean allowContentRetargetingOnChildren; */
  NS_IMETHOD GetAllowContentRetargetingOnChildren(bool *aAllowContentRetargetingOnChildren) = 0;
  inline bool  GetAllowContentRetargetingOnChildren()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetAllowContentRetargetingOnChildren(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetAllowContentRetargetingOnChildren(bool aAllowContentRetargetingOnChildren) = 0;

  enum DocShellEnumeratorDirection : uint8_t {
    ENUMERATE_FORWARDS = 0,
    ENUMERATE_BACKWARDS = 1,
  };

  /* Array<nsIDocShell> getAllDocShellsInSubtree (in long aItemType, in nsIDocShell_DocShellEnumeratorDirection aDirection); */
  NS_IMETHOD GetAllDocShellsInSubtree(int32_t aItemType, nsIDocShell::DocShellEnumeratorDirection aDirection, nsTArray<RefPtr<nsIDocShell>>& _retval) = 0;

  enum AppType : uint8_t {
    APP_TYPE_UNKNOWN = 0,
    APP_TYPE_MAIL = 1,
    APP_TYPE_EDITOR = 2,
  };

  /* [infallible] attribute nsIDocShell_AppType appType; */
  NS_IMETHOD GetAppType(nsIDocShell::AppType *aAppType) = 0;
  inline nsIDocShell::AppType  GetAppType()
  {
    nsIDocShell::AppType result;
    mozilla::DebugOnly<nsresult> rv = GetAppType(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetAppType(nsIDocShell::AppType aAppType) = 0;

  /* attribute boolean allowAuth; */
  NS_IMETHOD GetAllowAuth(bool *aAllowAuth) = 0;
  NS_IMETHOD SetAllowAuth(bool aAllowAuth) = 0;

  /* attribute float zoom; */
  NS_IMETHOD GetZoom(float *aZoom) = 0;
  NS_IMETHOD SetZoom(float aZoom) = 0;

  /* bool tabToTreeOwner (in boolean forward, in boolean forDocumentNavigation); */
  NS_IMETHOD TabToTreeOwner(bool forward, bool forDocumentNavigation, bool *_retval) = 0;

  enum BusyFlags : uint8_t {
    BUSY_FLAGS_NONE = 0,
    BUSY_FLAGS_BUSY = 1,
    BUSY_FLAGS_BEFORE_PAGE_LOAD = 2,
    BUSY_FLAGS_PAGE_LOADING = 4,
  };

  /* [infallible] readonly attribute nsIDocShell_BusyFlags busyFlags; */
  NS_IMETHOD GetBusyFlags(nsIDocShell::BusyFlags *aBusyFlags) = 0;
  inline nsIDocShell::BusyFlags  GetBusyFlags()
  {
    nsIDocShell::BusyFlags result;
    mozilla::DebugOnly<nsresult> rv = GetBusyFlags(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  enum LoadCommand : uint8_t {
    LOAD_CMD_NORMAL = 1,
    LOAD_CMD_RELOAD = 2,
    LOAD_CMD_HISTORY = 4,
    LOAD_CMD_PUSHSTATE = 8,
  };

  /* [infallible] attribute unsigned long loadType; */
  NS_IMETHOD GetLoadType(uint32_t *aLoadType) = 0;
  inline uint32_t  GetLoadType()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetLoadType(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetLoadType(uint32_t aLoadType) = 0;

  /* attribute nsLoadFlags defaultLoadFlags; */
  NS_IMETHOD GetDefaultLoadFlags(nsLoadFlags *aDefaultLoadFlags) = 0;
  NS_IMETHOD SetDefaultLoadFlags(nsLoadFlags aDefaultLoadFlags) = 0;

  /* boolean isBeingDestroyed (); */
  NS_IMETHOD IsBeingDestroyed(bool *_retval) = 0;

  /* readonly attribute boolean isExecutingOnLoadHandler; */
  NS_IMETHOD GetIsExecutingOnLoadHandler(bool *aIsExecutingOnLoadHandler) = 0;

  /* attribute nsILayoutHistoryState layoutHistoryState; */
  NS_IMETHOD GetLayoutHistoryState(nsILayoutHistoryState **aLayoutHistoryState) = 0;
  NS_IMETHOD SetLayoutHistoryState(nsILayoutHistoryState *aLayoutHistoryState) = 0;

  /* readonly attribute nsILoadURIDelegate loadURIDelegate; */
  NS_IMETHOD GetLoadURIDelegate(nsILoadURIDelegate **aLoadURIDelegate) = 0;

  /* void suspendRefreshURIs (); */
  NS_IMETHOD SuspendRefreshURIs(void) = 0;

  /* void resumeRefreshURIs (); */
  NS_IMETHOD ResumeRefreshURIs(void) = 0;

  /* void beginRestore (in nsIContentViewer viewer, in boolean top); */
  NS_IMETHOD BeginRestore(nsIContentViewer *viewer, bool top) = 0;

  /* void finishRestore (); */
  NS_IMETHOD FinishRestore(void) = 0;

  /* void clearCachedUserAgent (); */
  NS_IMETHOD ClearCachedUserAgent(void) = 0;

  /* void clearCachedPlatform (); */
  NS_IMETHOD ClearCachedPlatform(void) = 0;

  /* readonly attribute boolean restoringDocument; */
  NS_IMETHOD GetRestoringDocument(bool *aRestoringDocument) = 0;

  /* attribute boolean useErrorPages; */
  NS_IMETHOD GetUseErrorPages(bool *aUseErrorPages) = 0;
  NS_IMETHOD SetUseErrorPages(bool aUseErrorPages) = 0;

  /* boolean displayLoadError (in nsresult aError, in nsIURI aURI, in wstring aURL, [optional] in nsIChannel aFailedChannel); */
  NS_IMETHOD DisplayLoadError(nsresult aError, nsIURI *aURI, const char16_t * aURL, nsIChannel *aFailedChannel, bool *_retval) = 0;

  /* readonly attribute nsIChannel failedChannel; */
  NS_IMETHOD GetFailedChannel(nsIChannel **aFailedChannel) = 0;

  /* readonly attribute long previousEntryIndex; */
  NS_IMETHOD GetPreviousEntryIndex(int32_t *aPreviousEntryIndex) = 0;

  /* readonly attribute long loadedEntryIndex; */
  NS_IMETHOD GetLoadedEntryIndex(int32_t *aLoadedEntryIndex) = 0;

  /* void historyPurged (in long numEntries); */
  NS_IMETHOD HistoryPurged(int32_t numEntries) = 0;

  /* readonly attribute nsIChannel currentDocumentChannel; */
  NS_IMETHOD GetCurrentDocumentChannel(nsIChannel **aCurrentDocumentChannel) = 0;

  /* [nostdcall,notxpcom] attribute long childOffset; */
  virtual int32_t GetChildOffset() = 0;
  virtual void SetChildOffset(int32_t aChildOffset) = 0;

  /* [infallible] readonly attribute boolean isInUnload; */
  NS_IMETHOD GetIsInUnload(bool *aIsInUnload) = 0;
  inline bool  GetIsInUnload()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsInUnload(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [noscript,notxpcom] void DetachEditorFromWindow (); */
  NS_IMETHOD_(void) DetachEditorFromWindow(void) = 0;

  /* attribute boolean isOffScreenBrowser; */
  NS_IMETHOD GetIsOffScreenBrowser(bool *aIsOffScreenBrowser) = 0;
  NS_IMETHOD SetIsOffScreenBrowser(bool aIsOffScreenBrowser) = 0;

  /* void exitPrintPreview (); */
  NS_IMETHOD ExitPrintPreview(void) = 0;

  /* [infallible] readonly attribute boolean canExecuteScripts; */
  NS_IMETHOD GetCanExecuteScripts(bool *aCanExecuteScripts) = 0;
  inline bool  GetCanExecuteScripts()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetCanExecuteScripts(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* readonly attribute nsIDRef historyID; */
  NS_IMETHOD GetHistoryID(nsID & aHistoryID) = 0;

  /* [noscript,notxpcom] nsIDRef HistoryID (); */
  NS_IMETHOD_(const nsID &) HistoryID(void) = 0;

  /* attribute boolean isAppTab; */
  NS_IMETHOD GetIsAppTab(bool *aIsAppTab) = 0;
  NS_IMETHOD SetIsAppTab(bool aIsAppTab) = 0;

  /* void createAboutBlankContentViewer (in nsIPrincipal aPrincipal, in nsIPrincipal aPartitionedPrincipal, [optional] in nsIContentSecurityPolicy aCSP); */
  NS_IMETHOD CreateAboutBlankContentViewer(nsIPrincipal *aPrincipal, nsIPrincipal *aPartitionedPrincipal, nsIContentSecurityPolicy *aCSP) = 0;

  /* attribute ACString charset; */
  NS_IMETHOD GetCharset(nsACString& aCharset) = 0;
  NS_IMETHOD SetCharset(const nsACString& aCharset) = 0;

  /* void gatherCharsetMenuTelemetry (); */
  NS_IMETHOD GatherCharsetMenuTelemetry(void) = 0;

  /* [noscript,nostdcall,notxpcom] void setParentCharset (in Encoding parentCharset, in int32_t parentCharsetSource, in nsIPrincipal parentCharsetPrincipal); */
  virtual void SetParentCharset(const mozilla::Encoding* & parentCharset, int32_t parentCharsetSource, nsIPrincipal *parentCharsetPrincipal) = 0;

  /* [noscript,nostdcall,notxpcom] void getParentCharset (out Encoding parentCharset, out int32_t parentCharsetSource, out nsIPrincipal parentCharsetPrincipal); */
  virtual void GetParentCharset(const mozilla::Encoding* & parentCharset, int32_t *parentCharsetSource, nsIPrincipal **parentCharsetPrincipal) = 0;

  /* [infallible] attribute boolean recordProfileTimelineMarkers; */
  NS_IMETHOD GetRecordProfileTimelineMarkers(bool *aRecordProfileTimelineMarkers) = 0;
  inline bool  GetRecordProfileTimelineMarkers()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetRecordProfileTimelineMarkers(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetRecordProfileTimelineMarkers(bool aRecordProfileTimelineMarkers) = 0;

  /* DOMHighResTimeStamp now (); */
  NS_IMETHOD Now(DOMHighResTimeStamp *_retval) = 0;

  /* [implicit_jscontext] jsval popProfileTimelineMarkers (); */
  NS_IMETHOD PopProfileTimelineMarkers(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* void addWeakPrivacyTransitionObserver (in nsIPrivacyTransitionObserver obs); */
  NS_IMETHOD AddWeakPrivacyTransitionObserver(nsIPrivacyTransitionObserver *obs) = 0;

  /* void addWeakReflowObserver (in nsIReflowObserver obs); */
  NS_IMETHOD AddWeakReflowObserver(nsIReflowObserver *obs) = 0;

  /* void removeWeakReflowObserver (in nsIReflowObserver obs); */
  NS_IMETHOD RemoveWeakReflowObserver(nsIReflowObserver *obs) = 0;

  /* [noscript] void notifyReflowObservers (in bool interruptible, in DOMHighResTimeStamp start, in DOMHighResTimeStamp end); */
  NS_IMETHOD NotifyReflowObservers(bool interruptible, DOMHighResTimeStamp start, DOMHighResTimeStamp end) = 0;

  /* [noscript] void addWeakScrollObserver (in nsIScrollObserver obs); */
  NS_IMETHOD AddWeakScrollObserver(nsIScrollObserver *obs) = 0;

  /* [noscript] void removeWeakScrollObserver (in nsIScrollObserver obs); */
  NS_IMETHOD RemoveWeakScrollObserver(nsIScrollObserver *obs) = 0;

  /* [noscript] void notifyScrollObservers (); */
  NS_IMETHOD NotifyScrollObservers(void) = 0;

  /* [infallible] readonly attribute boolean isTopLevelContentDocShell; */
  NS_IMETHOD GetIsTopLevelContentDocShell(bool *aIsTopLevelContentDocShell) = 0;
  inline bool  GetIsTopLevelContentDocShell()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsTopLevelContentDocShell(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* nsIDocShell getSameTypeInProcessParentIgnoreBrowserBoundaries (); */
  NS_IMETHOD GetSameTypeInProcessParentIgnoreBrowserBoundaries(nsIDocShell **_retval) = 0;

  /* readonly attribute bool asyncPanZoomEnabled; */
  NS_IMETHOD GetAsyncPanZoomEnabled(bool *aAsyncPanZoomEnabled) = 0;

  /* attribute nsIChannel mixedContentChannel; */
  NS_IMETHOD GetMixedContentChannel(nsIChannel **aMixedContentChannel) = 0;
  NS_IMETHOD SetMixedContentChannel(nsIChannel *aMixedContentChannel) = 0;

  /* [noscript,notxpcom] bool pluginsAllowedInCurrentDoc (); */
  NS_IMETHOD_(bool) PluginsAllowedInCurrentDoc(void) = 0;

  /* [infallible,noscript] attribute boolean affectPrivateSessionLifetime; */
  NS_IMETHOD GetAffectPrivateSessionLifetime(bool *aAffectPrivateSessionLifetime) = 0;
  inline bool  GetAffectPrivateSessionLifetime()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetAffectPrivateSessionLifetime(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetAffectPrivateSessionLifetime(bool aAffectPrivateSessionLifetime) = 0;

  /* [infallible] readonly attribute boolean mayEnableCharacterEncodingMenu; */
  NS_IMETHOD GetMayEnableCharacterEncodingMenu(bool *aMayEnableCharacterEncodingMenu) = 0;
  inline bool  GetMayEnableCharacterEncodingMenu()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetMayEnableCharacterEncodingMenu(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean charsetAutodetected; */
  NS_IMETHOD GetCharsetAutodetected(bool *aCharsetAutodetected) = 0;
  inline bool  GetCharsetAutodetected()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetCharsetAutodetected(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* attribute nsIEditor editor; */
  NS_IMETHOD GetEditor(nsIEditor **aEditor) = 0;
  NS_IMETHOD SetEditor(nsIEditor *aEditor) = 0;

  /* readonly attribute boolean editable; */
  NS_IMETHOD GetEditable(bool *aEditable) = 0;

  /* readonly attribute boolean hasEditingSession; */
  NS_IMETHOD GetHasEditingSession(bool *aHasEditingSession) = 0;

  /* void makeEditable (in boolean inWaitForUriLoad); */
  NS_IMETHOD MakeEditable(bool inWaitForUriLoad) = 0;

  /* boolean getCurrentSHEntry (out nsISHEntry aEntry); */
  NS_IMETHOD GetCurrentSHEntry(nsISHEntry **aEntry, bool *_retval) = 0;

  /* boolean isCommandEnabled (in string command); */
  NS_IMETHOD IsCommandEnabled(const char * command, bool *_retval) = 0;

  /* [can_run_script] void doCommand (in string command); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * command) = 0;

  /* [can_run_script] void doCommandWithParams (in string command, in nsICommandParams aParams); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandWithParams(const char * command, nsICommandParams *aParams) = 0;

  /* [noscript,notxpcom] bool IsInvisible (); */
  NS_IMETHOD_(bool) IsInvisible(void) = 0;

  /* [noscript,notxpcom] void SetInvisible (in bool aIsInvisibleDocshell); */
  NS_IMETHOD_(void) SetInvisible(bool aIsInvisibleDocshell) = 0;

  /* [noscript,nostdcall,notxpcom] nsIScriptGlobalObject GetScriptGlobalObject (); */
  virtual nsIScriptGlobalObject * GetScriptGlobalObject(void) = 0;

  /* [noscript,nostdcall,notxpcom] Document getExtantDocument (); */
  virtual mozilla::dom::Document * GetExtantDocument(void) = 0;

  /* [infallible] attribute boolean deviceSizeIsPageSize; */
  NS_IMETHOD GetDeviceSizeIsPageSize(bool *aDeviceSizeIsPageSize) = 0;
  inline bool  GetDeviceSizeIsPageSize()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetDeviceSizeIsPageSize(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetDeviceSizeIsPageSize(bool aDeviceSizeIsPageSize) = 0;

  /* [noscript,nostdcall,notxpcom] void notifyJSRunToCompletionStart (in string aReason, in AString functionName, in AString fileName, in unsigned long lineNumber, in jsval asyncStack, in string asyncCause); */
  virtual void NotifyJSRunToCompletionStart(const char * aReason, const nsAString& functionName, const nsAString& fileName, uint32_t lineNumber, JS::HandleValue asyncStack, const char * asyncCause) = 0;

  /* [noscript,nostdcall,notxpcom] void notifyJSRunToCompletionStop (); */
  virtual void NotifyJSRunToCompletionStop(void) = 0;

  /* [infallible] readonly attribute boolean hasLoadedNonBlankURI; */
  NS_IMETHOD GetHasLoadedNonBlankURI(bool *aHasLoadedNonBlankURI) = 0;
  inline bool  GetHasLoadedNonBlankURI()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetHasLoadedNonBlankURI(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* attribute boolean windowDraggingAllowed; */
  NS_IMETHOD GetWindowDraggingAllowed(bool *aWindowDraggingAllowed) = 0;
  NS_IMETHOD SetWindowDraggingAllowed(bool aWindowDraggingAllowed) = 0;

  /* attribute boolean currentScrollRestorationIsManual; */
  NS_IMETHOD GetCurrentScrollRestorationIsManual(bool *aCurrentScrollRestorationIsManual) = 0;
  NS_IMETHOD SetCurrentScrollRestorationIsManual(bool aCurrentScrollRestorationIsManual) = 0;

  /* [implicit_jscontext] jsval getOriginAttributes (); */
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] void setOriginAttributes (in jsval aAttrs); */
  NS_IMETHOD SetOriginAttributes(JS::HandleValue aAttrs, JSContext* cx) = 0;

  /* readonly attribute nsIEditingSession editingSession; */
  NS_IMETHOD GetEditingSession(nsIEditingSession **aEditingSession) = 0;

  /* [binaryname(ScriptableBrowserChild)] readonly attribute nsIBrowserChild browserChild; */
  NS_IMETHOD GetScriptableBrowserChild(nsIBrowserChild **aBrowserChild) = 0;

  /* [noscript,nostdcall,notxpcom] BrowserChildRef GetBrowserChild (); */
  virtual already_AddRefed<nsIBrowserChild> GetBrowserChild(void) = 0;

  /* [noscript,nostdcall,notxpcom] nsCommandManager GetCommandManager (); */
  virtual nsCommandManager * GetCommandManager(void) = 0;

  enum MetaViewportOverride : uint8_t {
    META_VIEWPORT_OVERRIDE_DISABLED = 0,
    META_VIEWPORT_OVERRIDE_ENABLED = 1,
    META_VIEWPORT_OVERRIDE_NONE = 2,
  };

  /* [infallible] attribute nsIDocShell_MetaViewportOverride metaViewportOverride; */
  NS_IMETHOD GetMetaViewportOverride(nsIDocShell::MetaViewportOverride *aMetaViewportOverride) = 0;
  inline nsIDocShell::MetaViewportOverride  GetMetaViewportOverride()
  {
    nsIDocShell::MetaViewportOverride result;
    mozilla::DebugOnly<nsresult> rv = GetMetaViewportOverride(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetMetaViewportOverride(nsIDocShell::MetaViewportOverride aMetaViewportOverride) = 0;

  /* attribute boolean useTrackingProtection; */
  NS_IMETHOD GetUseTrackingProtection(bool *aUseTrackingProtection) = 0;
  NS_IMETHOD SetUseTrackingProtection(bool aUseTrackingProtection) = 0;

  /* [noscript] void dispatchLocationChangeEvent (); */
  NS_IMETHOD DispatchLocationChangeEvent(void) = 0;

  /* [noscript] void startDelayedAutoplayMediaComponents (); */
  NS_IMETHOD StartDelayedAutoplayMediaComponents(void) = 0;

  /* [noscript,nostdcall,notxpcom] UniqueClientSource TakeInitialClientSource (); */
  virtual mozilla::UniquePtr<mozilla::dom::ClientSource> TakeInitialClientSource(void) = 0;

  /* void setColorMatrix (in Array<float> aMatrix); */
  NS_IMETHOD SetColorMatrix(const nsTArray<float >& aMatrix) = 0;

  /* readonly attribute bool isForceReloading; */
  NS_IMETHOD GetIsForceReloading(bool *aIsForceReloading) = 0;

  /* Array<float> getColorMatrix (); */
  NS_IMETHOD GetColorMatrix(nsTArray<float >& _retval) = 0;

   /**
   * These methods call nsDocShell::GetHTMLEditorInternal() and
   * nsDocShell::SetHTMLEditorInternal() with static_cast.
   */
  mozilla::HTMLEditor* GetHTMLEditor();
  nsresult SetHTMLEditor(mozilla::HTMLEditor* aHTMLEditor);
  /* [infallible] readonly attribute ContentFrameMessageManager messageManager; */
  NS_IMETHOD GetMessageManager(mozilla::dom::ContentFrameMessageManager **aMessageManager) = 0;
   inline already_AddRefed<mozilla::dom::ContentFrameMessageManager> GetMessageManager()
  {
    mozilla::dom::ContentFrameMessageManager* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetMessageManager(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<mozilla::dom::ContentFrameMessageManager>(result);
  }

  /* Promise getHasTrackingContentBlocked (); */
  NS_IMETHOD GetHasTrackingContentBlocked(::mozilla::dom::Promise * * _retval) = 0;

  /* [nostdcall,notxpcom] readonly attribute boolean isAttemptingToNavigate; */
  virtual bool GetIsAttemptingToNavigate() = 0;

  /* [infallible] readonly attribute boolean isNavigating; */
  NS_IMETHOD GetIsNavigating(bool *aIsNavigating) = 0;
  inline bool  GetIsNavigating()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsNavigating(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* void synchronizeLayoutHistoryState (); */
  NS_IMETHOD SynchronizeLayoutHistoryState(void) = 0;

  /* void persistLayoutHistoryState (); */
  NS_IMETHOD PersistLayoutHistoryState(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDocShell, NS_IDOCSHELL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOCSHELL \
  NS_IMETHOD SetCancelContentJSEpoch(int32_t aEpoch) override; \
  NS_IMETHOD LoadURI(nsDocShellLoadState* aLoadState, bool aSetNavigating) override; \
  NS_IMETHOD AddState(JS::HandleValue aData, const nsAString& aTitle, const nsAString& aURL, bool aReplace, JSContext* cx) override; \
  virtual nsresult UpdateURLAndHistory(mozilla::dom::Document *aDocument, nsIURI *aNewURI, nsIStructuredCloneContainer *aData, const nsAString& aTitle, bool aReplace, nsIURI *aCurrentURI, bool aEqualURIs) override; \
  NS_IMETHOD PrepareForNewContentModel(void) override; \
  NS_IMETHOD SetCurrentURI(nsIURI *aURI) override; \
  NS_IMETHOD FirePageHideNotification(bool isUnload) override; \
  virtual nsPresContext * GetPresContext() override; \
  virtual mozilla::PresShell * GetPresShell() override; \
  virtual mozilla::PresShell * GetEldestPresShell() override; \
  using nsIDocShell::GetContentViewer; \
  NS_IMETHOD GetContentViewer(nsIContentViewer **aContentViewer) override; \
  using nsIDocShell::GetOuterWindowID; \
  NS_IMETHOD GetOuterWindowID(uint64_t *aOuterWindowID) override; \
  NS_IMETHOD GetChromeEventHandler(mozilla::dom::EventTarget **aChromeEventHandler) override; \
  NS_IMETHOD SetChromeEventHandler(mozilla::dom::EventTarget *aChromeEventHandler) override; \
  NS_IMETHOD GetCustomUserAgent(nsAString& aCustomUserAgent) override; \
  NS_IMETHOD SetCustomUserAgent(const nsAString& aCustomUserAgent) override; \
  NS_IMETHOD GetCssErrorReportingEnabled(bool *aCssErrorReportingEnabled) override; \
  NS_IMETHOD SetCssErrorReportingEnabled(bool aCssErrorReportingEnabled) override; \
  NS_IMETHOD GetAllowPlugins(bool *aAllowPlugins) override; \
  NS_IMETHOD SetAllowPlugins(bool aAllowPlugins) override; \
  NS_IMETHOD GetAllowJavascript(bool *aAllowJavascript) override; \
  NS_IMETHOD SetAllowJavascript(bool aAllowJavascript) override; \
  NS_IMETHOD GetAllowMetaRedirects(bool *aAllowMetaRedirects) override; \
  NS_IMETHOD SetAllowMetaRedirects(bool aAllowMetaRedirects) override; \
  NS_IMETHOD GetAllowSubframes(bool *aAllowSubframes) override; \
  NS_IMETHOD SetAllowSubframes(bool aAllowSubframes) override; \
  NS_IMETHOD GetAllowImages(bool *aAllowImages) override; \
  NS_IMETHOD SetAllowImages(bool aAllowImages) override; \
  using nsIDocShell::GetAllowMedia; \
  NS_IMETHOD GetAllowMedia(bool *aAllowMedia) override; \
  NS_IMETHOD SetAllowMedia(bool aAllowMedia) override; \
  NS_IMETHOD GetAllowDNSPrefetch(bool *aAllowDNSPrefetch) override; \
  NS_IMETHOD SetAllowDNSPrefetch(bool aAllowDNSPrefetch) override; \
  NS_IMETHOD GetAllowWindowControl(bool *aAllowWindowControl) override; \
  NS_IMETHOD SetAllowWindowControl(bool aAllowWindowControl) override; \
  using nsIDocShell::GetAllowContentRetargeting; \
  NS_IMETHOD GetAllowContentRetargeting(bool *aAllowContentRetargeting) override; \
  NS_IMETHOD SetAllowContentRetargeting(bool aAllowContentRetargeting) override; \
  using nsIDocShell::GetAllowContentRetargetingOnChildren; \
  NS_IMETHOD GetAllowContentRetargetingOnChildren(bool *aAllowContentRetargetingOnChildren) override; \
  NS_IMETHOD SetAllowContentRetargetingOnChildren(bool aAllowContentRetargetingOnChildren) override; \
  NS_IMETHOD GetAllDocShellsInSubtree(int32_t aItemType, nsIDocShell::DocShellEnumeratorDirection aDirection, nsTArray<RefPtr<nsIDocShell>>& _retval) override; \
  using nsIDocShell::GetAppType; \
  NS_IMETHOD GetAppType(nsIDocShell::AppType *aAppType) override; \
  NS_IMETHOD SetAppType(nsIDocShell::AppType aAppType) override; \
  NS_IMETHOD GetAllowAuth(bool *aAllowAuth) override; \
  NS_IMETHOD SetAllowAuth(bool aAllowAuth) override; \
  NS_IMETHOD GetZoom(float *aZoom) override; \
  NS_IMETHOD SetZoom(float aZoom) override; \
  NS_IMETHOD TabToTreeOwner(bool forward, bool forDocumentNavigation, bool *_retval) override; \
  using nsIDocShell::GetBusyFlags; \
  NS_IMETHOD GetBusyFlags(nsIDocShell::BusyFlags *aBusyFlags) override; \
  using nsIDocShell::GetLoadType; \
  NS_IMETHOD GetLoadType(uint32_t *aLoadType) override; \
  NS_IMETHOD SetLoadType(uint32_t aLoadType) override; \
  NS_IMETHOD GetDefaultLoadFlags(nsLoadFlags *aDefaultLoadFlags) override; \
  NS_IMETHOD SetDefaultLoadFlags(nsLoadFlags aDefaultLoadFlags) override; \
  NS_IMETHOD IsBeingDestroyed(bool *_retval) override; \
  NS_IMETHOD GetIsExecutingOnLoadHandler(bool *aIsExecutingOnLoadHandler) override; \
  NS_IMETHOD GetLayoutHistoryState(nsILayoutHistoryState **aLayoutHistoryState) override; \
  NS_IMETHOD SetLayoutHistoryState(nsILayoutHistoryState *aLayoutHistoryState) override; \
  NS_IMETHOD GetLoadURIDelegate(nsILoadURIDelegate **aLoadURIDelegate) override; \
  NS_IMETHOD SuspendRefreshURIs(void) override; \
  NS_IMETHOD ResumeRefreshURIs(void) override; \
  NS_IMETHOD BeginRestore(nsIContentViewer *viewer, bool top) override; \
  NS_IMETHOD FinishRestore(void) override; \
  NS_IMETHOD ClearCachedUserAgent(void) override; \
  NS_IMETHOD ClearCachedPlatform(void) override; \
  NS_IMETHOD GetRestoringDocument(bool *aRestoringDocument) override; \
  NS_IMETHOD GetUseErrorPages(bool *aUseErrorPages) override; \
  NS_IMETHOD SetUseErrorPages(bool aUseErrorPages) override; \
  NS_IMETHOD DisplayLoadError(nsresult aError, nsIURI *aURI, const char16_t * aURL, nsIChannel *aFailedChannel, bool *_retval) override; \
  NS_IMETHOD GetFailedChannel(nsIChannel **aFailedChannel) override; \
  NS_IMETHOD GetPreviousEntryIndex(int32_t *aPreviousEntryIndex) override; \
  NS_IMETHOD GetLoadedEntryIndex(int32_t *aLoadedEntryIndex) override; \
  NS_IMETHOD HistoryPurged(int32_t numEntries) override; \
  NS_IMETHOD GetCurrentDocumentChannel(nsIChannel **aCurrentDocumentChannel) override; \
  virtual int32_t GetChildOffset() override; \
  virtual void SetChildOffset(int32_t aChildOffset) override; \
  using nsIDocShell::GetIsInUnload; \
  NS_IMETHOD GetIsInUnload(bool *aIsInUnload) override; \
  NS_IMETHOD_(void) DetachEditorFromWindow(void) override; \
  NS_IMETHOD GetIsOffScreenBrowser(bool *aIsOffScreenBrowser) override; \
  NS_IMETHOD SetIsOffScreenBrowser(bool aIsOffScreenBrowser) override; \
  NS_IMETHOD ExitPrintPreview(void) override; \
  using nsIDocShell::GetCanExecuteScripts; \
  NS_IMETHOD GetCanExecuteScripts(bool *aCanExecuteScripts) override; \
  NS_IMETHOD GetHistoryID(nsID & aHistoryID) override; \
  NS_IMETHOD_(const nsID &) HistoryID(void) override; \
  NS_IMETHOD GetIsAppTab(bool *aIsAppTab) override; \
  NS_IMETHOD SetIsAppTab(bool aIsAppTab) override; \
  NS_IMETHOD CreateAboutBlankContentViewer(nsIPrincipal *aPrincipal, nsIPrincipal *aPartitionedPrincipal, nsIContentSecurityPolicy *aCSP) override; \
  NS_IMETHOD GetCharset(nsACString& aCharset) override; \
  NS_IMETHOD SetCharset(const nsACString& aCharset) override; \
  NS_IMETHOD GatherCharsetMenuTelemetry(void) override; \
  virtual void SetParentCharset(const mozilla::Encoding* & parentCharset, int32_t parentCharsetSource, nsIPrincipal *parentCharsetPrincipal) override; \
  virtual void GetParentCharset(const mozilla::Encoding* & parentCharset, int32_t *parentCharsetSource, nsIPrincipal **parentCharsetPrincipal) override; \
  using nsIDocShell::GetRecordProfileTimelineMarkers; \
  NS_IMETHOD GetRecordProfileTimelineMarkers(bool *aRecordProfileTimelineMarkers) override; \
  NS_IMETHOD SetRecordProfileTimelineMarkers(bool aRecordProfileTimelineMarkers) override; \
  NS_IMETHOD Now(DOMHighResTimeStamp *_retval) override; \
  NS_IMETHOD PopProfileTimelineMarkers(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD AddWeakPrivacyTransitionObserver(nsIPrivacyTransitionObserver *obs) override; \
  NS_IMETHOD AddWeakReflowObserver(nsIReflowObserver *obs) override; \
  NS_IMETHOD RemoveWeakReflowObserver(nsIReflowObserver *obs) override; \
  NS_IMETHOD NotifyReflowObservers(bool interruptible, DOMHighResTimeStamp start, DOMHighResTimeStamp end) override; \
  NS_IMETHOD AddWeakScrollObserver(nsIScrollObserver *obs) override; \
  NS_IMETHOD RemoveWeakScrollObserver(nsIScrollObserver *obs) override; \
  NS_IMETHOD NotifyScrollObservers(void) override; \
  using nsIDocShell::GetIsTopLevelContentDocShell; \
  NS_IMETHOD GetIsTopLevelContentDocShell(bool *aIsTopLevelContentDocShell) override; \
  NS_IMETHOD GetSameTypeInProcessParentIgnoreBrowserBoundaries(nsIDocShell **_retval) override; \
  NS_IMETHOD GetAsyncPanZoomEnabled(bool *aAsyncPanZoomEnabled) override; \
  NS_IMETHOD GetMixedContentChannel(nsIChannel **aMixedContentChannel) override; \
  NS_IMETHOD SetMixedContentChannel(nsIChannel *aMixedContentChannel) override; \
  NS_IMETHOD_(bool) PluginsAllowedInCurrentDoc(void) override; \
  using nsIDocShell::GetAffectPrivateSessionLifetime; \
  NS_IMETHOD GetAffectPrivateSessionLifetime(bool *aAffectPrivateSessionLifetime) override; \
  NS_IMETHOD SetAffectPrivateSessionLifetime(bool aAffectPrivateSessionLifetime) override; \
  using nsIDocShell::GetMayEnableCharacterEncodingMenu; \
  NS_IMETHOD GetMayEnableCharacterEncodingMenu(bool *aMayEnableCharacterEncodingMenu) override; \
  using nsIDocShell::GetCharsetAutodetected; \
  NS_IMETHOD GetCharsetAutodetected(bool *aCharsetAutodetected) override; \
  NS_IMETHOD GetEditor(nsIEditor **aEditor) override; \
  NS_IMETHOD SetEditor(nsIEditor *aEditor) override; \
  NS_IMETHOD GetEditable(bool *aEditable) override; \
  NS_IMETHOD GetHasEditingSession(bool *aHasEditingSession) override; \
  NS_IMETHOD MakeEditable(bool inWaitForUriLoad) override; \
  NS_IMETHOD GetCurrentSHEntry(nsISHEntry **aEntry, bool *_retval) override; \
  NS_IMETHOD IsCommandEnabled(const char * command, bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * command) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandWithParams(const char * command, nsICommandParams *aParams) override; \
  NS_IMETHOD_(bool) IsInvisible(void) override; \
  NS_IMETHOD_(void) SetInvisible(bool aIsInvisibleDocshell) override; \
  virtual nsIScriptGlobalObject * GetScriptGlobalObject(void) override; \
  virtual mozilla::dom::Document * GetExtantDocument(void) override; \
  using nsIDocShell::GetDeviceSizeIsPageSize; \
  NS_IMETHOD GetDeviceSizeIsPageSize(bool *aDeviceSizeIsPageSize) override; \
  NS_IMETHOD SetDeviceSizeIsPageSize(bool aDeviceSizeIsPageSize) override; \
  virtual void NotifyJSRunToCompletionStart(const char * aReason, const nsAString& functionName, const nsAString& fileName, uint32_t lineNumber, JS::HandleValue asyncStack, const char * asyncCause) override; \
  virtual void NotifyJSRunToCompletionStop(void) override; \
  using nsIDocShell::GetHasLoadedNonBlankURI; \
  NS_IMETHOD GetHasLoadedNonBlankURI(bool *aHasLoadedNonBlankURI) override; \
  NS_IMETHOD GetWindowDraggingAllowed(bool *aWindowDraggingAllowed) override; \
  NS_IMETHOD SetWindowDraggingAllowed(bool aWindowDraggingAllowed) override; \
  NS_IMETHOD GetCurrentScrollRestorationIsManual(bool *aCurrentScrollRestorationIsManual) override; \
  NS_IMETHOD SetCurrentScrollRestorationIsManual(bool aCurrentScrollRestorationIsManual) override; \
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD SetOriginAttributes(JS::HandleValue aAttrs, JSContext* cx) override; \
  NS_IMETHOD GetEditingSession(nsIEditingSession **aEditingSession) override; \
  NS_IMETHOD GetScriptableBrowserChild(nsIBrowserChild **aBrowserChild) override; \
  virtual already_AddRefed<nsIBrowserChild> GetBrowserChild(void) override; \
  virtual nsCommandManager * GetCommandManager(void) override; \
  using nsIDocShell::GetMetaViewportOverride; \
  NS_IMETHOD GetMetaViewportOverride(nsIDocShell::MetaViewportOverride *aMetaViewportOverride) override; \
  NS_IMETHOD SetMetaViewportOverride(nsIDocShell::MetaViewportOverride aMetaViewportOverride) override; \
  NS_IMETHOD GetUseTrackingProtection(bool *aUseTrackingProtection) override; \
  NS_IMETHOD SetUseTrackingProtection(bool aUseTrackingProtection) override; \
  NS_IMETHOD DispatchLocationChangeEvent(void) override; \
  NS_IMETHOD StartDelayedAutoplayMediaComponents(void) override; \
  virtual mozilla::UniquePtr<mozilla::dom::ClientSource> TakeInitialClientSource(void) override; \
  NS_IMETHOD SetColorMatrix(const nsTArray<float >& aMatrix) override; \
  NS_IMETHOD GetIsForceReloading(bool *aIsForceReloading) override; \
  NS_IMETHOD GetColorMatrix(nsTArray<float >& _retval) override; \
  using nsIDocShell::GetMessageManager; \
  NS_IMETHOD GetMessageManager(mozilla::dom::ContentFrameMessageManager **aMessageManager) override; \
  NS_IMETHOD GetHasTrackingContentBlocked(::mozilla::dom::Promise * * _retval) override; \
  virtual bool GetIsAttemptingToNavigate() override; \
  using nsIDocShell::GetIsNavigating; \
  NS_IMETHOD GetIsNavigating(bool *aIsNavigating) override; \
  NS_IMETHOD SynchronizeLayoutHistoryState(void) override; \
  NS_IMETHOD PersistLayoutHistoryState(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOCSHELL \
  nsresult SetCancelContentJSEpoch(int32_t aEpoch); \
  nsresult LoadURI(nsDocShellLoadState* aLoadState, bool aSetNavigating); \
  nsresult AddState(JS::HandleValue aData, const nsAString& aTitle, const nsAString& aURL, bool aReplace, JSContext* cx); \
  nsresult UpdateURLAndHistory(mozilla::dom::Document *aDocument, nsIURI *aNewURI, nsIStructuredCloneContainer *aData, const nsAString& aTitle, bool aReplace, nsIURI *aCurrentURI, bool aEqualURIs); \
  nsresult PrepareForNewContentModel(void); \
  nsresult SetCurrentURI(nsIURI *aURI); \
  nsresult FirePageHideNotification(bool isUnload); \
  nsPresContext * GetPresContext(); \
  mozilla::PresShell * GetPresShell(); \
  mozilla::PresShell * GetEldestPresShell(); \
  using nsIDocShell::GetContentViewer; \
  nsresult GetContentViewer(nsIContentViewer **aContentViewer); \
  using nsIDocShell::GetOuterWindowID; \
  nsresult GetOuterWindowID(uint64_t *aOuterWindowID); \
  nsresult GetChromeEventHandler(mozilla::dom::EventTarget **aChromeEventHandler); \
  nsresult SetChromeEventHandler(mozilla::dom::EventTarget *aChromeEventHandler); \
  nsresult GetCustomUserAgent(nsAString& aCustomUserAgent); \
  nsresult SetCustomUserAgent(const nsAString& aCustomUserAgent); \
  nsresult GetCssErrorReportingEnabled(bool *aCssErrorReportingEnabled); \
  nsresult SetCssErrorReportingEnabled(bool aCssErrorReportingEnabled); \
  nsresult GetAllowPlugins(bool *aAllowPlugins); \
  nsresult SetAllowPlugins(bool aAllowPlugins); \
  nsresult GetAllowJavascript(bool *aAllowJavascript); \
  nsresult SetAllowJavascript(bool aAllowJavascript); \
  nsresult GetAllowMetaRedirects(bool *aAllowMetaRedirects); \
  nsresult SetAllowMetaRedirects(bool aAllowMetaRedirects); \
  nsresult GetAllowSubframes(bool *aAllowSubframes); \
  nsresult SetAllowSubframes(bool aAllowSubframes); \
  nsresult GetAllowImages(bool *aAllowImages); \
  nsresult SetAllowImages(bool aAllowImages); \
  using nsIDocShell::GetAllowMedia; \
  nsresult GetAllowMedia(bool *aAllowMedia); \
  nsresult SetAllowMedia(bool aAllowMedia); \
  nsresult GetAllowDNSPrefetch(bool *aAllowDNSPrefetch); \
  nsresult SetAllowDNSPrefetch(bool aAllowDNSPrefetch); \
  nsresult GetAllowWindowControl(bool *aAllowWindowControl); \
  nsresult SetAllowWindowControl(bool aAllowWindowControl); \
  using nsIDocShell::GetAllowContentRetargeting; \
  nsresult GetAllowContentRetargeting(bool *aAllowContentRetargeting); \
  nsresult SetAllowContentRetargeting(bool aAllowContentRetargeting); \
  using nsIDocShell::GetAllowContentRetargetingOnChildren; \
  nsresult GetAllowContentRetargetingOnChildren(bool *aAllowContentRetargetingOnChildren); \
  nsresult SetAllowContentRetargetingOnChildren(bool aAllowContentRetargetingOnChildren); \
  nsresult GetAllDocShellsInSubtree(int32_t aItemType, nsIDocShell::DocShellEnumeratorDirection aDirection, nsTArray<RefPtr<nsIDocShell>>& _retval); \
  using nsIDocShell::GetAppType; \
  nsresult GetAppType(nsIDocShell::AppType *aAppType); \
  nsresult SetAppType(nsIDocShell::AppType aAppType); \
  nsresult GetAllowAuth(bool *aAllowAuth); \
  nsresult SetAllowAuth(bool aAllowAuth); \
  nsresult GetZoom(float *aZoom); \
  nsresult SetZoom(float aZoom); \
  nsresult TabToTreeOwner(bool forward, bool forDocumentNavigation, bool *_retval); \
  using nsIDocShell::GetBusyFlags; \
  nsresult GetBusyFlags(nsIDocShell::BusyFlags *aBusyFlags); \
  using nsIDocShell::GetLoadType; \
  nsresult GetLoadType(uint32_t *aLoadType); \
  nsresult SetLoadType(uint32_t aLoadType); \
  nsresult GetDefaultLoadFlags(nsLoadFlags *aDefaultLoadFlags); \
  nsresult SetDefaultLoadFlags(nsLoadFlags aDefaultLoadFlags); \
  nsresult IsBeingDestroyed(bool *_retval); \
  nsresult GetIsExecutingOnLoadHandler(bool *aIsExecutingOnLoadHandler); \
  nsresult GetLayoutHistoryState(nsILayoutHistoryState **aLayoutHistoryState); \
  nsresult SetLayoutHistoryState(nsILayoutHistoryState *aLayoutHistoryState); \
  nsresult GetLoadURIDelegate(nsILoadURIDelegate **aLoadURIDelegate); \
  nsresult SuspendRefreshURIs(void); \
  nsresult ResumeRefreshURIs(void); \
  nsresult BeginRestore(nsIContentViewer *viewer, bool top); \
  nsresult FinishRestore(void); \
  nsresult ClearCachedUserAgent(void); \
  nsresult ClearCachedPlatform(void); \
  nsresult GetRestoringDocument(bool *aRestoringDocument); \
  nsresult GetUseErrorPages(bool *aUseErrorPages); \
  nsresult SetUseErrorPages(bool aUseErrorPages); \
  nsresult DisplayLoadError(nsresult aError, nsIURI *aURI, const char16_t * aURL, nsIChannel *aFailedChannel, bool *_retval); \
  nsresult GetFailedChannel(nsIChannel **aFailedChannel); \
  nsresult GetPreviousEntryIndex(int32_t *aPreviousEntryIndex); \
  nsresult GetLoadedEntryIndex(int32_t *aLoadedEntryIndex); \
  nsresult HistoryPurged(int32_t numEntries); \
  nsresult GetCurrentDocumentChannel(nsIChannel **aCurrentDocumentChannel); \
  int32_t GetChildOffset(); \
  void SetChildOffset(int32_t aChildOffset); \
  using nsIDocShell::GetIsInUnload; \
  nsresult GetIsInUnload(bool *aIsInUnload); \
  nsresult_(void) DetachEditorFromWindow(void); \
  nsresult GetIsOffScreenBrowser(bool *aIsOffScreenBrowser); \
  nsresult SetIsOffScreenBrowser(bool aIsOffScreenBrowser); \
  nsresult ExitPrintPreview(void); \
  using nsIDocShell::GetCanExecuteScripts; \
  nsresult GetCanExecuteScripts(bool *aCanExecuteScripts); \
  nsresult GetHistoryID(nsID & aHistoryID); \
  nsresult_(const nsID &) HistoryID(void); \
  nsresult GetIsAppTab(bool *aIsAppTab); \
  nsresult SetIsAppTab(bool aIsAppTab); \
  nsresult CreateAboutBlankContentViewer(nsIPrincipal *aPrincipal, nsIPrincipal *aPartitionedPrincipal, nsIContentSecurityPolicy *aCSP); \
  nsresult GetCharset(nsACString& aCharset); \
  nsresult SetCharset(const nsACString& aCharset); \
  nsresult GatherCharsetMenuTelemetry(void); \
  void SetParentCharset(const mozilla::Encoding* & parentCharset, int32_t parentCharsetSource, nsIPrincipal *parentCharsetPrincipal); \
  void GetParentCharset(const mozilla::Encoding* & parentCharset, int32_t *parentCharsetSource, nsIPrincipal **parentCharsetPrincipal); \
  using nsIDocShell::GetRecordProfileTimelineMarkers; \
  nsresult GetRecordProfileTimelineMarkers(bool *aRecordProfileTimelineMarkers); \
  nsresult SetRecordProfileTimelineMarkers(bool aRecordProfileTimelineMarkers); \
  nsresult Now(DOMHighResTimeStamp *_retval); \
  nsresult PopProfileTimelineMarkers(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult AddWeakPrivacyTransitionObserver(nsIPrivacyTransitionObserver *obs); \
  nsresult AddWeakReflowObserver(nsIReflowObserver *obs); \
  nsresult RemoveWeakReflowObserver(nsIReflowObserver *obs); \
  nsresult NotifyReflowObservers(bool interruptible, DOMHighResTimeStamp start, DOMHighResTimeStamp end); \
  nsresult AddWeakScrollObserver(nsIScrollObserver *obs); \
  nsresult RemoveWeakScrollObserver(nsIScrollObserver *obs); \
  nsresult NotifyScrollObservers(void); \
  using nsIDocShell::GetIsTopLevelContentDocShell; \
  nsresult GetIsTopLevelContentDocShell(bool *aIsTopLevelContentDocShell); \
  nsresult GetSameTypeInProcessParentIgnoreBrowserBoundaries(nsIDocShell **_retval); \
  nsresult GetAsyncPanZoomEnabled(bool *aAsyncPanZoomEnabled); \
  nsresult GetMixedContentChannel(nsIChannel **aMixedContentChannel); \
  nsresult SetMixedContentChannel(nsIChannel *aMixedContentChannel); \
  nsresult_(bool) PluginsAllowedInCurrentDoc(void); \
  using nsIDocShell::GetAffectPrivateSessionLifetime; \
  nsresult GetAffectPrivateSessionLifetime(bool *aAffectPrivateSessionLifetime); \
  nsresult SetAffectPrivateSessionLifetime(bool aAffectPrivateSessionLifetime); \
  using nsIDocShell::GetMayEnableCharacterEncodingMenu; \
  nsresult GetMayEnableCharacterEncodingMenu(bool *aMayEnableCharacterEncodingMenu); \
  using nsIDocShell::GetCharsetAutodetected; \
  nsresult GetCharsetAutodetected(bool *aCharsetAutodetected); \
  nsresult GetEditor(nsIEditor **aEditor); \
  nsresult SetEditor(nsIEditor *aEditor); \
  nsresult GetEditable(bool *aEditable); \
  nsresult GetHasEditingSession(bool *aHasEditingSession); \
  nsresult MakeEditable(bool inWaitForUriLoad); \
  nsresult GetCurrentSHEntry(nsISHEntry **aEntry, bool *_retval); \
  nsresult IsCommandEnabled(const char * command, bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult DoCommand(const char * command); \
  MOZ_CAN_RUN_SCRIPT nsresult DoCommandWithParams(const char * command, nsICommandParams *aParams); \
  nsresult_(bool) IsInvisible(void); \
  nsresult_(void) SetInvisible(bool aIsInvisibleDocshell); \
  nsIScriptGlobalObject * GetScriptGlobalObject(void); \
  mozilla::dom::Document * GetExtantDocument(void); \
  using nsIDocShell::GetDeviceSizeIsPageSize; \
  nsresult GetDeviceSizeIsPageSize(bool *aDeviceSizeIsPageSize); \
  nsresult SetDeviceSizeIsPageSize(bool aDeviceSizeIsPageSize); \
  void NotifyJSRunToCompletionStart(const char * aReason, const nsAString& functionName, const nsAString& fileName, uint32_t lineNumber, JS::HandleValue asyncStack, const char * asyncCause); \
  void NotifyJSRunToCompletionStop(void); \
  using nsIDocShell::GetHasLoadedNonBlankURI; \
  nsresult GetHasLoadedNonBlankURI(bool *aHasLoadedNonBlankURI); \
  nsresult GetWindowDraggingAllowed(bool *aWindowDraggingAllowed); \
  nsresult SetWindowDraggingAllowed(bool aWindowDraggingAllowed); \
  nsresult GetCurrentScrollRestorationIsManual(bool *aCurrentScrollRestorationIsManual); \
  nsresult SetCurrentScrollRestorationIsManual(bool aCurrentScrollRestorationIsManual); \
  nsresult GetOriginAttributes(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult SetOriginAttributes(JS::HandleValue aAttrs, JSContext* cx); \
  nsresult GetEditingSession(nsIEditingSession **aEditingSession); \
  nsresult GetScriptableBrowserChild(nsIBrowserChild **aBrowserChild); \
  already_AddRefed<nsIBrowserChild> GetBrowserChild(void); \
  nsCommandManager * GetCommandManager(void); \
  using nsIDocShell::GetMetaViewportOverride; \
  nsresult GetMetaViewportOverride(nsIDocShell::MetaViewportOverride *aMetaViewportOverride); \
  nsresult SetMetaViewportOverride(nsIDocShell::MetaViewportOverride aMetaViewportOverride); \
  nsresult GetUseTrackingProtection(bool *aUseTrackingProtection); \
  nsresult SetUseTrackingProtection(bool aUseTrackingProtection); \
  nsresult DispatchLocationChangeEvent(void); \
  nsresult StartDelayedAutoplayMediaComponents(void); \
  mozilla::UniquePtr<mozilla::dom::ClientSource> TakeInitialClientSource(void); \
  nsresult SetColorMatrix(const nsTArray<float >& aMatrix); \
  nsresult GetIsForceReloading(bool *aIsForceReloading); \
  nsresult GetColorMatrix(nsTArray<float >& _retval); \
  using nsIDocShell::GetMessageManager; \
  nsresult GetMessageManager(mozilla::dom::ContentFrameMessageManager **aMessageManager); \
  nsresult GetHasTrackingContentBlocked(::mozilla::dom::Promise * * _retval); \
  bool GetIsAttemptingToNavigate(); \
  using nsIDocShell::GetIsNavigating; \
  nsresult GetIsNavigating(bool *aIsNavigating); \
  nsresult SynchronizeLayoutHistoryState(void); \
  nsresult PersistLayoutHistoryState(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOCSHELL(_to) \
  NS_IMETHOD SetCancelContentJSEpoch(int32_t aEpoch) override { return _to SetCancelContentJSEpoch(aEpoch); } \
  NS_IMETHOD LoadURI(nsDocShellLoadState* aLoadState, bool aSetNavigating) override { return _to LoadURI(aLoadState, aSetNavigating); } \
  NS_IMETHOD AddState(JS::HandleValue aData, const nsAString& aTitle, const nsAString& aURL, bool aReplace, JSContext* cx) override { return _to AddState(aData, aTitle, aURL, aReplace, cx); } \
  virtual nsresult UpdateURLAndHistory(mozilla::dom::Document *aDocument, nsIURI *aNewURI, nsIStructuredCloneContainer *aData, const nsAString& aTitle, bool aReplace, nsIURI *aCurrentURI, bool aEqualURIs) override { return _to UpdateURLAndHistory(aDocument, aNewURI, aData, aTitle, aReplace, aCurrentURI, aEqualURIs); } \
  NS_IMETHOD PrepareForNewContentModel(void) override { return _to PrepareForNewContentModel(); } \
  NS_IMETHOD SetCurrentURI(nsIURI *aURI) override { return _to SetCurrentURI(aURI); } \
  NS_IMETHOD FirePageHideNotification(bool isUnload) override { return _to FirePageHideNotification(isUnload); } \
  virtual nsPresContext * GetPresContext() override { return _to GetPresContext(); } \
  virtual mozilla::PresShell * GetPresShell() override { return _to GetPresShell(); } \
  virtual mozilla::PresShell * GetEldestPresShell() override { return _to GetEldestPresShell(); } \
  using nsIDocShell::GetContentViewer; \
  NS_IMETHOD GetContentViewer(nsIContentViewer **aContentViewer) override { return _to GetContentViewer(aContentViewer); } \
  using nsIDocShell::GetOuterWindowID; \
  NS_IMETHOD GetOuterWindowID(uint64_t *aOuterWindowID) override { return _to GetOuterWindowID(aOuterWindowID); } \
  NS_IMETHOD GetChromeEventHandler(mozilla::dom::EventTarget **aChromeEventHandler) override { return _to GetChromeEventHandler(aChromeEventHandler); } \
  NS_IMETHOD SetChromeEventHandler(mozilla::dom::EventTarget *aChromeEventHandler) override { return _to SetChromeEventHandler(aChromeEventHandler); } \
  NS_IMETHOD GetCustomUserAgent(nsAString& aCustomUserAgent) override { return _to GetCustomUserAgent(aCustomUserAgent); } \
  NS_IMETHOD SetCustomUserAgent(const nsAString& aCustomUserAgent) override { return _to SetCustomUserAgent(aCustomUserAgent); } \
  NS_IMETHOD GetCssErrorReportingEnabled(bool *aCssErrorReportingEnabled) override { return _to GetCssErrorReportingEnabled(aCssErrorReportingEnabled); } \
  NS_IMETHOD SetCssErrorReportingEnabled(bool aCssErrorReportingEnabled) override { return _to SetCssErrorReportingEnabled(aCssErrorReportingEnabled); } \
  NS_IMETHOD GetAllowPlugins(bool *aAllowPlugins) override { return _to GetAllowPlugins(aAllowPlugins); } \
  NS_IMETHOD SetAllowPlugins(bool aAllowPlugins) override { return _to SetAllowPlugins(aAllowPlugins); } \
  NS_IMETHOD GetAllowJavascript(bool *aAllowJavascript) override { return _to GetAllowJavascript(aAllowJavascript); } \
  NS_IMETHOD SetAllowJavascript(bool aAllowJavascript) override { return _to SetAllowJavascript(aAllowJavascript); } \
  NS_IMETHOD GetAllowMetaRedirects(bool *aAllowMetaRedirects) override { return _to GetAllowMetaRedirects(aAllowMetaRedirects); } \
  NS_IMETHOD SetAllowMetaRedirects(bool aAllowMetaRedirects) override { return _to SetAllowMetaRedirects(aAllowMetaRedirects); } \
  NS_IMETHOD GetAllowSubframes(bool *aAllowSubframes) override { return _to GetAllowSubframes(aAllowSubframes); } \
  NS_IMETHOD SetAllowSubframes(bool aAllowSubframes) override { return _to SetAllowSubframes(aAllowSubframes); } \
  NS_IMETHOD GetAllowImages(bool *aAllowImages) override { return _to GetAllowImages(aAllowImages); } \
  NS_IMETHOD SetAllowImages(bool aAllowImages) override { return _to SetAllowImages(aAllowImages); } \
  using nsIDocShell::GetAllowMedia; \
  NS_IMETHOD GetAllowMedia(bool *aAllowMedia) override { return _to GetAllowMedia(aAllowMedia); } \
  NS_IMETHOD SetAllowMedia(bool aAllowMedia) override { return _to SetAllowMedia(aAllowMedia); } \
  NS_IMETHOD GetAllowDNSPrefetch(bool *aAllowDNSPrefetch) override { return _to GetAllowDNSPrefetch(aAllowDNSPrefetch); } \
  NS_IMETHOD SetAllowDNSPrefetch(bool aAllowDNSPrefetch) override { return _to SetAllowDNSPrefetch(aAllowDNSPrefetch); } \
  NS_IMETHOD GetAllowWindowControl(bool *aAllowWindowControl) override { return _to GetAllowWindowControl(aAllowWindowControl); } \
  NS_IMETHOD SetAllowWindowControl(bool aAllowWindowControl) override { return _to SetAllowWindowControl(aAllowWindowControl); } \
  using nsIDocShell::GetAllowContentRetargeting; \
  NS_IMETHOD GetAllowContentRetargeting(bool *aAllowContentRetargeting) override { return _to GetAllowContentRetargeting(aAllowContentRetargeting); } \
  NS_IMETHOD SetAllowContentRetargeting(bool aAllowContentRetargeting) override { return _to SetAllowContentRetargeting(aAllowContentRetargeting); } \
  using nsIDocShell::GetAllowContentRetargetingOnChildren; \
  NS_IMETHOD GetAllowContentRetargetingOnChildren(bool *aAllowContentRetargetingOnChildren) override { return _to GetAllowContentRetargetingOnChildren(aAllowContentRetargetingOnChildren); } \
  NS_IMETHOD SetAllowContentRetargetingOnChildren(bool aAllowContentRetargetingOnChildren) override { return _to SetAllowContentRetargetingOnChildren(aAllowContentRetargetingOnChildren); } \
  NS_IMETHOD GetAllDocShellsInSubtree(int32_t aItemType, nsIDocShell::DocShellEnumeratorDirection aDirection, nsTArray<RefPtr<nsIDocShell>>& _retval) override { return _to GetAllDocShellsInSubtree(aItemType, aDirection, _retval); } \
  using nsIDocShell::GetAppType; \
  NS_IMETHOD GetAppType(nsIDocShell::AppType *aAppType) override { return _to GetAppType(aAppType); } \
  NS_IMETHOD SetAppType(nsIDocShell::AppType aAppType) override { return _to SetAppType(aAppType); } \
  NS_IMETHOD GetAllowAuth(bool *aAllowAuth) override { return _to GetAllowAuth(aAllowAuth); } \
  NS_IMETHOD SetAllowAuth(bool aAllowAuth) override { return _to SetAllowAuth(aAllowAuth); } \
  NS_IMETHOD GetZoom(float *aZoom) override { return _to GetZoom(aZoom); } \
  NS_IMETHOD SetZoom(float aZoom) override { return _to SetZoom(aZoom); } \
  NS_IMETHOD TabToTreeOwner(bool forward, bool forDocumentNavigation, bool *_retval) override { return _to TabToTreeOwner(forward, forDocumentNavigation, _retval); } \
  using nsIDocShell::GetBusyFlags; \
  NS_IMETHOD GetBusyFlags(nsIDocShell::BusyFlags *aBusyFlags) override { return _to GetBusyFlags(aBusyFlags); } \
  using nsIDocShell::GetLoadType; \
  NS_IMETHOD GetLoadType(uint32_t *aLoadType) override { return _to GetLoadType(aLoadType); } \
  NS_IMETHOD SetLoadType(uint32_t aLoadType) override { return _to SetLoadType(aLoadType); } \
  NS_IMETHOD GetDefaultLoadFlags(nsLoadFlags *aDefaultLoadFlags) override { return _to GetDefaultLoadFlags(aDefaultLoadFlags); } \
  NS_IMETHOD SetDefaultLoadFlags(nsLoadFlags aDefaultLoadFlags) override { return _to SetDefaultLoadFlags(aDefaultLoadFlags); } \
  NS_IMETHOD IsBeingDestroyed(bool *_retval) override { return _to IsBeingDestroyed(_retval); } \
  NS_IMETHOD GetIsExecutingOnLoadHandler(bool *aIsExecutingOnLoadHandler) override { return _to GetIsExecutingOnLoadHandler(aIsExecutingOnLoadHandler); } \
  NS_IMETHOD GetLayoutHistoryState(nsILayoutHistoryState **aLayoutHistoryState) override { return _to GetLayoutHistoryState(aLayoutHistoryState); } \
  NS_IMETHOD SetLayoutHistoryState(nsILayoutHistoryState *aLayoutHistoryState) override { return _to SetLayoutHistoryState(aLayoutHistoryState); } \
  NS_IMETHOD GetLoadURIDelegate(nsILoadURIDelegate **aLoadURIDelegate) override { return _to GetLoadURIDelegate(aLoadURIDelegate); } \
  NS_IMETHOD SuspendRefreshURIs(void) override { return _to SuspendRefreshURIs(); } \
  NS_IMETHOD ResumeRefreshURIs(void) override { return _to ResumeRefreshURIs(); } \
  NS_IMETHOD BeginRestore(nsIContentViewer *viewer, bool top) override { return _to BeginRestore(viewer, top); } \
  NS_IMETHOD FinishRestore(void) override { return _to FinishRestore(); } \
  NS_IMETHOD ClearCachedUserAgent(void) override { return _to ClearCachedUserAgent(); } \
  NS_IMETHOD ClearCachedPlatform(void) override { return _to ClearCachedPlatform(); } \
  NS_IMETHOD GetRestoringDocument(bool *aRestoringDocument) override { return _to GetRestoringDocument(aRestoringDocument); } \
  NS_IMETHOD GetUseErrorPages(bool *aUseErrorPages) override { return _to GetUseErrorPages(aUseErrorPages); } \
  NS_IMETHOD SetUseErrorPages(bool aUseErrorPages) override { return _to SetUseErrorPages(aUseErrorPages); } \
  NS_IMETHOD DisplayLoadError(nsresult aError, nsIURI *aURI, const char16_t * aURL, nsIChannel *aFailedChannel, bool *_retval) override { return _to DisplayLoadError(aError, aURI, aURL, aFailedChannel, _retval); } \
  NS_IMETHOD GetFailedChannel(nsIChannel **aFailedChannel) override { return _to GetFailedChannel(aFailedChannel); } \
  NS_IMETHOD GetPreviousEntryIndex(int32_t *aPreviousEntryIndex) override { return _to GetPreviousEntryIndex(aPreviousEntryIndex); } \
  NS_IMETHOD GetLoadedEntryIndex(int32_t *aLoadedEntryIndex) override { return _to GetLoadedEntryIndex(aLoadedEntryIndex); } \
  NS_IMETHOD HistoryPurged(int32_t numEntries) override { return _to HistoryPurged(numEntries); } \
  NS_IMETHOD GetCurrentDocumentChannel(nsIChannel **aCurrentDocumentChannel) override { return _to GetCurrentDocumentChannel(aCurrentDocumentChannel); } \
  virtual int32_t GetChildOffset() override { return _to GetChildOffset(); } \
  virtual void SetChildOffset(int32_t aChildOffset) override { return _to SetChildOffset(aChildOffset); } \
  using nsIDocShell::GetIsInUnload; \
  NS_IMETHOD GetIsInUnload(bool *aIsInUnload) override { return _to GetIsInUnload(aIsInUnload); } \
  NS_IMETHOD_(void) DetachEditorFromWindow(void) override { return _to DetachEditorFromWindow(); } \
  NS_IMETHOD GetIsOffScreenBrowser(bool *aIsOffScreenBrowser) override { return _to GetIsOffScreenBrowser(aIsOffScreenBrowser); } \
  NS_IMETHOD SetIsOffScreenBrowser(bool aIsOffScreenBrowser) override { return _to SetIsOffScreenBrowser(aIsOffScreenBrowser); } \
  NS_IMETHOD ExitPrintPreview(void) override { return _to ExitPrintPreview(); } \
  using nsIDocShell::GetCanExecuteScripts; \
  NS_IMETHOD GetCanExecuteScripts(bool *aCanExecuteScripts) override { return _to GetCanExecuteScripts(aCanExecuteScripts); } \
  NS_IMETHOD GetHistoryID(nsID & aHistoryID) override { return _to GetHistoryID(aHistoryID); } \
  NS_IMETHOD_(const nsID &) HistoryID(void) override { return _to HistoryID(); } \
  NS_IMETHOD GetIsAppTab(bool *aIsAppTab) override { return _to GetIsAppTab(aIsAppTab); } \
  NS_IMETHOD SetIsAppTab(bool aIsAppTab) override { return _to SetIsAppTab(aIsAppTab); } \
  NS_IMETHOD CreateAboutBlankContentViewer(nsIPrincipal *aPrincipal, nsIPrincipal *aPartitionedPrincipal, nsIContentSecurityPolicy *aCSP) override { return _to CreateAboutBlankContentViewer(aPrincipal, aPartitionedPrincipal, aCSP); } \
  NS_IMETHOD GetCharset(nsACString& aCharset) override { return _to GetCharset(aCharset); } \
  NS_IMETHOD SetCharset(const nsACString& aCharset) override { return _to SetCharset(aCharset); } \
  NS_IMETHOD GatherCharsetMenuTelemetry(void) override { return _to GatherCharsetMenuTelemetry(); } \
  virtual void SetParentCharset(const mozilla::Encoding* & parentCharset, int32_t parentCharsetSource, nsIPrincipal *parentCharsetPrincipal) override { return _to SetParentCharset(parentCharset, parentCharsetSource, parentCharsetPrincipal); } \
  virtual void GetParentCharset(const mozilla::Encoding* & parentCharset, int32_t *parentCharsetSource, nsIPrincipal **parentCharsetPrincipal) override { return _to GetParentCharset(parentCharset, parentCharsetSource, parentCharsetPrincipal); } \
  using nsIDocShell::GetRecordProfileTimelineMarkers; \
  NS_IMETHOD GetRecordProfileTimelineMarkers(bool *aRecordProfileTimelineMarkers) override { return _to GetRecordProfileTimelineMarkers(aRecordProfileTimelineMarkers); } \
  NS_IMETHOD SetRecordProfileTimelineMarkers(bool aRecordProfileTimelineMarkers) override { return _to SetRecordProfileTimelineMarkers(aRecordProfileTimelineMarkers); } \
  NS_IMETHOD Now(DOMHighResTimeStamp *_retval) override { return _to Now(_retval); } \
  NS_IMETHOD PopProfileTimelineMarkers(JSContext* cx, JS::MutableHandleValue _retval) override { return _to PopProfileTimelineMarkers(cx, _retval); } \
  NS_IMETHOD AddWeakPrivacyTransitionObserver(nsIPrivacyTransitionObserver *obs) override { return _to AddWeakPrivacyTransitionObserver(obs); } \
  NS_IMETHOD AddWeakReflowObserver(nsIReflowObserver *obs) override { return _to AddWeakReflowObserver(obs); } \
  NS_IMETHOD RemoveWeakReflowObserver(nsIReflowObserver *obs) override { return _to RemoveWeakReflowObserver(obs); } \
  NS_IMETHOD NotifyReflowObservers(bool interruptible, DOMHighResTimeStamp start, DOMHighResTimeStamp end) override { return _to NotifyReflowObservers(interruptible, start, end); } \
  NS_IMETHOD AddWeakScrollObserver(nsIScrollObserver *obs) override { return _to AddWeakScrollObserver(obs); } \
  NS_IMETHOD RemoveWeakScrollObserver(nsIScrollObserver *obs) override { return _to RemoveWeakScrollObserver(obs); } \
  NS_IMETHOD NotifyScrollObservers(void) override { return _to NotifyScrollObservers(); } \
  using nsIDocShell::GetIsTopLevelContentDocShell; \
  NS_IMETHOD GetIsTopLevelContentDocShell(bool *aIsTopLevelContentDocShell) override { return _to GetIsTopLevelContentDocShell(aIsTopLevelContentDocShell); } \
  NS_IMETHOD GetSameTypeInProcessParentIgnoreBrowserBoundaries(nsIDocShell **_retval) override { return _to GetSameTypeInProcessParentIgnoreBrowserBoundaries(_retval); } \
  NS_IMETHOD GetAsyncPanZoomEnabled(bool *aAsyncPanZoomEnabled) override { return _to GetAsyncPanZoomEnabled(aAsyncPanZoomEnabled); } \
  NS_IMETHOD GetMixedContentChannel(nsIChannel **aMixedContentChannel) override { return _to GetMixedContentChannel(aMixedContentChannel); } \
  NS_IMETHOD SetMixedContentChannel(nsIChannel *aMixedContentChannel) override { return _to SetMixedContentChannel(aMixedContentChannel); } \
  NS_IMETHOD_(bool) PluginsAllowedInCurrentDoc(void) override { return _to PluginsAllowedInCurrentDoc(); } \
  using nsIDocShell::GetAffectPrivateSessionLifetime; \
  NS_IMETHOD GetAffectPrivateSessionLifetime(bool *aAffectPrivateSessionLifetime) override { return _to GetAffectPrivateSessionLifetime(aAffectPrivateSessionLifetime); } \
  NS_IMETHOD SetAffectPrivateSessionLifetime(bool aAffectPrivateSessionLifetime) override { return _to SetAffectPrivateSessionLifetime(aAffectPrivateSessionLifetime); } \
  using nsIDocShell::GetMayEnableCharacterEncodingMenu; \
  NS_IMETHOD GetMayEnableCharacterEncodingMenu(bool *aMayEnableCharacterEncodingMenu) override { return _to GetMayEnableCharacterEncodingMenu(aMayEnableCharacterEncodingMenu); } \
  using nsIDocShell::GetCharsetAutodetected; \
  NS_IMETHOD GetCharsetAutodetected(bool *aCharsetAutodetected) override { return _to GetCharsetAutodetected(aCharsetAutodetected); } \
  NS_IMETHOD GetEditor(nsIEditor **aEditor) override { return _to GetEditor(aEditor); } \
  NS_IMETHOD SetEditor(nsIEditor *aEditor) override { return _to SetEditor(aEditor); } \
  NS_IMETHOD GetEditable(bool *aEditable) override { return _to GetEditable(aEditable); } \
  NS_IMETHOD GetHasEditingSession(bool *aHasEditingSession) override { return _to GetHasEditingSession(aHasEditingSession); } \
  NS_IMETHOD MakeEditable(bool inWaitForUriLoad) override { return _to MakeEditable(inWaitForUriLoad); } \
  NS_IMETHOD GetCurrentSHEntry(nsISHEntry **aEntry, bool *_retval) override { return _to GetCurrentSHEntry(aEntry, _retval); } \
  NS_IMETHOD IsCommandEnabled(const char * command, bool *_retval) override { return _to IsCommandEnabled(command, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * command) override { return _to DoCommand(command); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandWithParams(const char * command, nsICommandParams *aParams) override { return _to DoCommandWithParams(command, aParams); } \
  NS_IMETHOD_(bool) IsInvisible(void) override { return _to IsInvisible(); } \
  NS_IMETHOD_(void) SetInvisible(bool aIsInvisibleDocshell) override { return _to SetInvisible(aIsInvisibleDocshell); } \
  virtual nsIScriptGlobalObject * GetScriptGlobalObject(void) override { return _to GetScriptGlobalObject(); } \
  virtual mozilla::dom::Document * GetExtantDocument(void) override { return _to GetExtantDocument(); } \
  using nsIDocShell::GetDeviceSizeIsPageSize; \
  NS_IMETHOD GetDeviceSizeIsPageSize(bool *aDeviceSizeIsPageSize) override { return _to GetDeviceSizeIsPageSize(aDeviceSizeIsPageSize); } \
  NS_IMETHOD SetDeviceSizeIsPageSize(bool aDeviceSizeIsPageSize) override { return _to SetDeviceSizeIsPageSize(aDeviceSizeIsPageSize); } \
  virtual void NotifyJSRunToCompletionStart(const char * aReason, const nsAString& functionName, const nsAString& fileName, uint32_t lineNumber, JS::HandleValue asyncStack, const char * asyncCause) override { return _to NotifyJSRunToCompletionStart(aReason, functionName, fileName, lineNumber, asyncStack, asyncCause); } \
  virtual void NotifyJSRunToCompletionStop(void) override { return _to NotifyJSRunToCompletionStop(); } \
  using nsIDocShell::GetHasLoadedNonBlankURI; \
  NS_IMETHOD GetHasLoadedNonBlankURI(bool *aHasLoadedNonBlankURI) override { return _to GetHasLoadedNonBlankURI(aHasLoadedNonBlankURI); } \
  NS_IMETHOD GetWindowDraggingAllowed(bool *aWindowDraggingAllowed) override { return _to GetWindowDraggingAllowed(aWindowDraggingAllowed); } \
  NS_IMETHOD SetWindowDraggingAllowed(bool aWindowDraggingAllowed) override { return _to SetWindowDraggingAllowed(aWindowDraggingAllowed); } \
  NS_IMETHOD GetCurrentScrollRestorationIsManual(bool *aCurrentScrollRestorationIsManual) override { return _to GetCurrentScrollRestorationIsManual(aCurrentScrollRestorationIsManual); } \
  NS_IMETHOD SetCurrentScrollRestorationIsManual(bool aCurrentScrollRestorationIsManual) override { return _to SetCurrentScrollRestorationIsManual(aCurrentScrollRestorationIsManual); } \
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetOriginAttributes(cx, _retval); } \
  NS_IMETHOD SetOriginAttributes(JS::HandleValue aAttrs, JSContext* cx) override { return _to SetOriginAttributes(aAttrs, cx); } \
  NS_IMETHOD GetEditingSession(nsIEditingSession **aEditingSession) override { return _to GetEditingSession(aEditingSession); } \
  NS_IMETHOD GetScriptableBrowserChild(nsIBrowserChild **aBrowserChild) override { return _to GetScriptableBrowserChild(aBrowserChild); } \
  virtual already_AddRefed<nsIBrowserChild> GetBrowserChild(void) override { return _to GetBrowserChild(); } \
  virtual nsCommandManager * GetCommandManager(void) override { return _to GetCommandManager(); } \
  using nsIDocShell::GetMetaViewportOverride; \
  NS_IMETHOD GetMetaViewportOverride(nsIDocShell::MetaViewportOverride *aMetaViewportOverride) override { return _to GetMetaViewportOverride(aMetaViewportOverride); } \
  NS_IMETHOD SetMetaViewportOverride(nsIDocShell::MetaViewportOverride aMetaViewportOverride) override { return _to SetMetaViewportOverride(aMetaViewportOverride); } \
  NS_IMETHOD GetUseTrackingProtection(bool *aUseTrackingProtection) override { return _to GetUseTrackingProtection(aUseTrackingProtection); } \
  NS_IMETHOD SetUseTrackingProtection(bool aUseTrackingProtection) override { return _to SetUseTrackingProtection(aUseTrackingProtection); } \
  NS_IMETHOD DispatchLocationChangeEvent(void) override { return _to DispatchLocationChangeEvent(); } \
  NS_IMETHOD StartDelayedAutoplayMediaComponents(void) override { return _to StartDelayedAutoplayMediaComponents(); } \
  virtual mozilla::UniquePtr<mozilla::dom::ClientSource> TakeInitialClientSource(void) override { return _to TakeInitialClientSource(); } \
  NS_IMETHOD SetColorMatrix(const nsTArray<float >& aMatrix) override { return _to SetColorMatrix(aMatrix); } \
  NS_IMETHOD GetIsForceReloading(bool *aIsForceReloading) override { return _to GetIsForceReloading(aIsForceReloading); } \
  NS_IMETHOD GetColorMatrix(nsTArray<float >& _retval) override { return _to GetColorMatrix(_retval); } \
  using nsIDocShell::GetMessageManager; \
  NS_IMETHOD GetMessageManager(mozilla::dom::ContentFrameMessageManager **aMessageManager) override { return _to GetMessageManager(aMessageManager); } \
  NS_IMETHOD GetHasTrackingContentBlocked(::mozilla::dom::Promise * * _retval) override { return _to GetHasTrackingContentBlocked(_retval); } \
  virtual bool GetIsAttemptingToNavigate() override { return _to GetIsAttemptingToNavigate(); } \
  using nsIDocShell::GetIsNavigating; \
  NS_IMETHOD GetIsNavigating(bool *aIsNavigating) override { return _to GetIsNavigating(aIsNavigating); } \
  NS_IMETHOD SynchronizeLayoutHistoryState(void) override { return _to SynchronizeLayoutHistoryState(); } \
  NS_IMETHOD PersistLayoutHistoryState(void) override { return _to PersistLayoutHistoryState(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOCSHELL(_to) \
  NS_IMETHOD SetCancelContentJSEpoch(int32_t aEpoch) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCancelContentJSEpoch(aEpoch); } \
  NS_IMETHOD LoadURI(nsDocShellLoadState* aLoadState, bool aSetNavigating) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadURI(aLoadState, aSetNavigating); } \
  NS_IMETHOD AddState(JS::HandleValue aData, const nsAString& aTitle, const nsAString& aURL, bool aReplace, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddState(aData, aTitle, aURL, aReplace, cx); } \
  virtual nsresult UpdateURLAndHistory(mozilla::dom::Document *aDocument, nsIURI *aNewURI, nsIStructuredCloneContainer *aData, const nsAString& aTitle, bool aReplace, nsIURI *aCurrentURI, bool aEqualURIs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateURLAndHistory(aDocument, aNewURI, aData, aTitle, aReplace, aCurrentURI, aEqualURIs); } \
  NS_IMETHOD PrepareForNewContentModel(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PrepareForNewContentModel(); } \
  NS_IMETHOD SetCurrentURI(nsIURI *aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCurrentURI(aURI); } \
  NS_IMETHOD FirePageHideNotification(bool isUnload) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FirePageHideNotification(isUnload); } \
  virtual nsPresContext * GetPresContext() override; \
  virtual mozilla::PresShell * GetPresShell() override; \
  virtual mozilla::PresShell * GetEldestPresShell() override; \
  NS_IMETHOD GetContentViewer(nsIContentViewer **aContentViewer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentViewer(aContentViewer); } \
  NS_IMETHOD GetOuterWindowID(uint64_t *aOuterWindowID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOuterWindowID(aOuterWindowID); } \
  NS_IMETHOD GetChromeEventHandler(mozilla::dom::EventTarget **aChromeEventHandler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChromeEventHandler(aChromeEventHandler); } \
  NS_IMETHOD SetChromeEventHandler(mozilla::dom::EventTarget *aChromeEventHandler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetChromeEventHandler(aChromeEventHandler); } \
  NS_IMETHOD GetCustomUserAgent(nsAString& aCustomUserAgent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCustomUserAgent(aCustomUserAgent); } \
  NS_IMETHOD SetCustomUserAgent(const nsAString& aCustomUserAgent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCustomUserAgent(aCustomUserAgent); } \
  NS_IMETHOD GetCssErrorReportingEnabled(bool *aCssErrorReportingEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCssErrorReportingEnabled(aCssErrorReportingEnabled); } \
  NS_IMETHOD SetCssErrorReportingEnabled(bool aCssErrorReportingEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCssErrorReportingEnabled(aCssErrorReportingEnabled); } \
  NS_IMETHOD GetAllowPlugins(bool *aAllowPlugins) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowPlugins(aAllowPlugins); } \
  NS_IMETHOD SetAllowPlugins(bool aAllowPlugins) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowPlugins(aAllowPlugins); } \
  NS_IMETHOD GetAllowJavascript(bool *aAllowJavascript) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowJavascript(aAllowJavascript); } \
  NS_IMETHOD SetAllowJavascript(bool aAllowJavascript) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowJavascript(aAllowJavascript); } \
  NS_IMETHOD GetAllowMetaRedirects(bool *aAllowMetaRedirects) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowMetaRedirects(aAllowMetaRedirects); } \
  NS_IMETHOD SetAllowMetaRedirects(bool aAllowMetaRedirects) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowMetaRedirects(aAllowMetaRedirects); } \
  NS_IMETHOD GetAllowSubframes(bool *aAllowSubframes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowSubframes(aAllowSubframes); } \
  NS_IMETHOD SetAllowSubframes(bool aAllowSubframes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowSubframes(aAllowSubframes); } \
  NS_IMETHOD GetAllowImages(bool *aAllowImages) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowImages(aAllowImages); } \
  NS_IMETHOD SetAllowImages(bool aAllowImages) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowImages(aAllowImages); } \
  NS_IMETHOD GetAllowMedia(bool *aAllowMedia) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowMedia(aAllowMedia); } \
  NS_IMETHOD SetAllowMedia(bool aAllowMedia) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowMedia(aAllowMedia); } \
  NS_IMETHOD GetAllowDNSPrefetch(bool *aAllowDNSPrefetch) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowDNSPrefetch(aAllowDNSPrefetch); } \
  NS_IMETHOD SetAllowDNSPrefetch(bool aAllowDNSPrefetch) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowDNSPrefetch(aAllowDNSPrefetch); } \
  NS_IMETHOD GetAllowWindowControl(bool *aAllowWindowControl) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowWindowControl(aAllowWindowControl); } \
  NS_IMETHOD SetAllowWindowControl(bool aAllowWindowControl) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowWindowControl(aAllowWindowControl); } \
  NS_IMETHOD GetAllowContentRetargeting(bool *aAllowContentRetargeting) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowContentRetargeting(aAllowContentRetargeting); } \
  NS_IMETHOD SetAllowContentRetargeting(bool aAllowContentRetargeting) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowContentRetargeting(aAllowContentRetargeting); } \
  NS_IMETHOD GetAllowContentRetargetingOnChildren(bool *aAllowContentRetargetingOnChildren) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowContentRetargetingOnChildren(aAllowContentRetargetingOnChildren); } \
  NS_IMETHOD SetAllowContentRetargetingOnChildren(bool aAllowContentRetargetingOnChildren) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowContentRetargetingOnChildren(aAllowContentRetargetingOnChildren); } \
  NS_IMETHOD GetAllDocShellsInSubtree(int32_t aItemType, nsIDocShell::DocShellEnumeratorDirection aDirection, nsTArray<RefPtr<nsIDocShell>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllDocShellsInSubtree(aItemType, aDirection, _retval); } \
  NS_IMETHOD GetAppType(nsIDocShell::AppType *aAppType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAppType(aAppType); } \
  NS_IMETHOD SetAppType(nsIDocShell::AppType aAppType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAppType(aAppType); } \
  NS_IMETHOD GetAllowAuth(bool *aAllowAuth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowAuth(aAllowAuth); } \
  NS_IMETHOD SetAllowAuth(bool aAllowAuth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowAuth(aAllowAuth); } \
  NS_IMETHOD GetZoom(float *aZoom) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetZoom(aZoom); } \
  NS_IMETHOD SetZoom(float aZoom) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetZoom(aZoom); } \
  NS_IMETHOD TabToTreeOwner(bool forward, bool forDocumentNavigation, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TabToTreeOwner(forward, forDocumentNavigation, _retval); } \
  NS_IMETHOD GetBusyFlags(nsIDocShell::BusyFlags *aBusyFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBusyFlags(aBusyFlags); } \
  NS_IMETHOD GetLoadType(uint32_t *aLoadType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadType(aLoadType); } \
  NS_IMETHOD SetLoadType(uint32_t aLoadType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLoadType(aLoadType); } \
  NS_IMETHOD GetDefaultLoadFlags(nsLoadFlags *aDefaultLoadFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultLoadFlags(aDefaultLoadFlags); } \
  NS_IMETHOD SetDefaultLoadFlags(nsLoadFlags aDefaultLoadFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDefaultLoadFlags(aDefaultLoadFlags); } \
  NS_IMETHOD IsBeingDestroyed(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsBeingDestroyed(_retval); } \
  NS_IMETHOD GetIsExecutingOnLoadHandler(bool *aIsExecutingOnLoadHandler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsExecutingOnLoadHandler(aIsExecutingOnLoadHandler); } \
  NS_IMETHOD GetLayoutHistoryState(nsILayoutHistoryState **aLayoutHistoryState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLayoutHistoryState(aLayoutHistoryState); } \
  NS_IMETHOD SetLayoutHistoryState(nsILayoutHistoryState *aLayoutHistoryState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLayoutHistoryState(aLayoutHistoryState); } \
  NS_IMETHOD GetLoadURIDelegate(nsILoadURIDelegate **aLoadURIDelegate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadURIDelegate(aLoadURIDelegate); } \
  NS_IMETHOD SuspendRefreshURIs(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SuspendRefreshURIs(); } \
  NS_IMETHOD ResumeRefreshURIs(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResumeRefreshURIs(); } \
  NS_IMETHOD BeginRestore(nsIContentViewer *viewer, bool top) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BeginRestore(viewer, top); } \
  NS_IMETHOD FinishRestore(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FinishRestore(); } \
  NS_IMETHOD ClearCachedUserAgent(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearCachedUserAgent(); } \
  NS_IMETHOD ClearCachedPlatform(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearCachedPlatform(); } \
  NS_IMETHOD GetRestoringDocument(bool *aRestoringDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRestoringDocument(aRestoringDocument); } \
  NS_IMETHOD GetUseErrorPages(bool *aUseErrorPages) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUseErrorPages(aUseErrorPages); } \
  NS_IMETHOD SetUseErrorPages(bool aUseErrorPages) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUseErrorPages(aUseErrorPages); } \
  NS_IMETHOD DisplayLoadError(nsresult aError, nsIURI *aURI, const char16_t * aURL, nsIChannel *aFailedChannel, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DisplayLoadError(aError, aURI, aURL, aFailedChannel, _retval); } \
  NS_IMETHOD GetFailedChannel(nsIChannel **aFailedChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFailedChannel(aFailedChannel); } \
  NS_IMETHOD GetPreviousEntryIndex(int32_t *aPreviousEntryIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPreviousEntryIndex(aPreviousEntryIndex); } \
  NS_IMETHOD GetLoadedEntryIndex(int32_t *aLoadedEntryIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadedEntryIndex(aLoadedEntryIndex); } \
  NS_IMETHOD HistoryPurged(int32_t numEntries) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HistoryPurged(numEntries); } \
  NS_IMETHOD GetCurrentDocumentChannel(nsIChannel **aCurrentDocumentChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentDocumentChannel(aCurrentDocumentChannel); } \
  virtual int32_t GetChildOffset() override; \
  virtual void SetChildOffset(int32_t aChildOffset) override; \
  NS_IMETHOD GetIsInUnload(bool *aIsInUnload) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsInUnload(aIsInUnload); } \
  NS_IMETHOD_(void) DetachEditorFromWindow(void) override; \
  NS_IMETHOD GetIsOffScreenBrowser(bool *aIsOffScreenBrowser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsOffScreenBrowser(aIsOffScreenBrowser); } \
  NS_IMETHOD SetIsOffScreenBrowser(bool aIsOffScreenBrowser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsOffScreenBrowser(aIsOffScreenBrowser); } \
  NS_IMETHOD ExitPrintPreview(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExitPrintPreview(); } \
  NS_IMETHOD GetCanExecuteScripts(bool *aCanExecuteScripts) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanExecuteScripts(aCanExecuteScripts); } \
  NS_IMETHOD GetHistoryID(nsID & aHistoryID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHistoryID(aHistoryID); } \
  NS_IMETHOD_(const nsID &) HistoryID(void) override; \
  NS_IMETHOD GetIsAppTab(bool *aIsAppTab) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsAppTab(aIsAppTab); } \
  NS_IMETHOD SetIsAppTab(bool aIsAppTab) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsAppTab(aIsAppTab); } \
  NS_IMETHOD CreateAboutBlankContentViewer(nsIPrincipal *aPrincipal, nsIPrincipal *aPartitionedPrincipal, nsIContentSecurityPolicy *aCSP) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateAboutBlankContentViewer(aPrincipal, aPartitionedPrincipal, aCSP); } \
  NS_IMETHOD GetCharset(nsACString& aCharset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCharset(aCharset); } \
  NS_IMETHOD SetCharset(const nsACString& aCharset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCharset(aCharset); } \
  NS_IMETHOD GatherCharsetMenuTelemetry(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GatherCharsetMenuTelemetry(); } \
  virtual void SetParentCharset(const mozilla::Encoding* & parentCharset, int32_t parentCharsetSource, nsIPrincipal *parentCharsetPrincipal) override; \
  virtual void GetParentCharset(const mozilla::Encoding* & parentCharset, int32_t *parentCharsetSource, nsIPrincipal **parentCharsetPrincipal) override; \
  NS_IMETHOD GetRecordProfileTimelineMarkers(bool *aRecordProfileTimelineMarkers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRecordProfileTimelineMarkers(aRecordProfileTimelineMarkers); } \
  NS_IMETHOD SetRecordProfileTimelineMarkers(bool aRecordProfileTimelineMarkers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRecordProfileTimelineMarkers(aRecordProfileTimelineMarkers); } \
  NS_IMETHOD Now(DOMHighResTimeStamp *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Now(_retval); } \
  NS_IMETHOD PopProfileTimelineMarkers(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PopProfileTimelineMarkers(cx, _retval); } \
  NS_IMETHOD AddWeakPrivacyTransitionObserver(nsIPrivacyTransitionObserver *obs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddWeakPrivacyTransitionObserver(obs); } \
  NS_IMETHOD AddWeakReflowObserver(nsIReflowObserver *obs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddWeakReflowObserver(obs); } \
  NS_IMETHOD RemoveWeakReflowObserver(nsIReflowObserver *obs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveWeakReflowObserver(obs); } \
  NS_IMETHOD NotifyReflowObservers(bool interruptible, DOMHighResTimeStamp start, DOMHighResTimeStamp end) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyReflowObservers(interruptible, start, end); } \
  NS_IMETHOD AddWeakScrollObserver(nsIScrollObserver *obs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddWeakScrollObserver(obs); } \
  NS_IMETHOD RemoveWeakScrollObserver(nsIScrollObserver *obs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveWeakScrollObserver(obs); } \
  NS_IMETHOD NotifyScrollObservers(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyScrollObservers(); } \
  NS_IMETHOD GetIsTopLevelContentDocShell(bool *aIsTopLevelContentDocShell) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsTopLevelContentDocShell(aIsTopLevelContentDocShell); } \
  NS_IMETHOD GetSameTypeInProcessParentIgnoreBrowserBoundaries(nsIDocShell **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSameTypeInProcessParentIgnoreBrowserBoundaries(_retval); } \
  NS_IMETHOD GetAsyncPanZoomEnabled(bool *aAsyncPanZoomEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsyncPanZoomEnabled(aAsyncPanZoomEnabled); } \
  NS_IMETHOD GetMixedContentChannel(nsIChannel **aMixedContentChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMixedContentChannel(aMixedContentChannel); } \
  NS_IMETHOD SetMixedContentChannel(nsIChannel *aMixedContentChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMixedContentChannel(aMixedContentChannel); } \
  NS_IMETHOD_(bool) PluginsAllowedInCurrentDoc(void) override; \
  NS_IMETHOD GetAffectPrivateSessionLifetime(bool *aAffectPrivateSessionLifetime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAffectPrivateSessionLifetime(aAffectPrivateSessionLifetime); } \
  NS_IMETHOD SetAffectPrivateSessionLifetime(bool aAffectPrivateSessionLifetime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAffectPrivateSessionLifetime(aAffectPrivateSessionLifetime); } \
  NS_IMETHOD GetMayEnableCharacterEncodingMenu(bool *aMayEnableCharacterEncodingMenu) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMayEnableCharacterEncodingMenu(aMayEnableCharacterEncodingMenu); } \
  NS_IMETHOD GetCharsetAutodetected(bool *aCharsetAutodetected) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCharsetAutodetected(aCharsetAutodetected); } \
  NS_IMETHOD GetEditor(nsIEditor **aEditor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEditor(aEditor); } \
  NS_IMETHOD SetEditor(nsIEditor *aEditor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEditor(aEditor); } \
  NS_IMETHOD GetEditable(bool *aEditable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEditable(aEditable); } \
  NS_IMETHOD GetHasEditingSession(bool *aHasEditingSession) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasEditingSession(aHasEditingSession); } \
  NS_IMETHOD MakeEditable(bool inWaitForUriLoad) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MakeEditable(inWaitForUriLoad); } \
  NS_IMETHOD GetCurrentSHEntry(nsISHEntry **aEntry, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentSHEntry(aEntry, _retval); } \
  NS_IMETHOD IsCommandEnabled(const char * command, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCommandEnabled(command, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommand(const char * command) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoCommand(command); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoCommandWithParams(const char * command, nsICommandParams *aParams) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoCommandWithParams(command, aParams); } \
  NS_IMETHOD_(bool) IsInvisible(void) override; \
  NS_IMETHOD_(void) SetInvisible(bool aIsInvisibleDocshell) override; \
  virtual nsIScriptGlobalObject * GetScriptGlobalObject(void) override; \
  virtual mozilla::dom::Document * GetExtantDocument(void) override; \
  NS_IMETHOD GetDeviceSizeIsPageSize(bool *aDeviceSizeIsPageSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDeviceSizeIsPageSize(aDeviceSizeIsPageSize); } \
  NS_IMETHOD SetDeviceSizeIsPageSize(bool aDeviceSizeIsPageSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDeviceSizeIsPageSize(aDeviceSizeIsPageSize); } \
  virtual void NotifyJSRunToCompletionStart(const char * aReason, const nsAString& functionName, const nsAString& fileName, uint32_t lineNumber, JS::HandleValue asyncStack, const char * asyncCause) override; \
  virtual void NotifyJSRunToCompletionStop(void) override; \
  NS_IMETHOD GetHasLoadedNonBlankURI(bool *aHasLoadedNonBlankURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasLoadedNonBlankURI(aHasLoadedNonBlankURI); } \
  NS_IMETHOD GetWindowDraggingAllowed(bool *aWindowDraggingAllowed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWindowDraggingAllowed(aWindowDraggingAllowed); } \
  NS_IMETHOD SetWindowDraggingAllowed(bool aWindowDraggingAllowed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetWindowDraggingAllowed(aWindowDraggingAllowed); } \
  NS_IMETHOD GetCurrentScrollRestorationIsManual(bool *aCurrentScrollRestorationIsManual) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentScrollRestorationIsManual(aCurrentScrollRestorationIsManual); } \
  NS_IMETHOD SetCurrentScrollRestorationIsManual(bool aCurrentScrollRestorationIsManual) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCurrentScrollRestorationIsManual(aCurrentScrollRestorationIsManual); } \
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginAttributes(cx, _retval); } \
  NS_IMETHOD SetOriginAttributes(JS::HandleValue aAttrs, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOriginAttributes(aAttrs, cx); } \
  NS_IMETHOD GetEditingSession(nsIEditingSession **aEditingSession) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEditingSession(aEditingSession); } \
  NS_IMETHOD GetScriptableBrowserChild(nsIBrowserChild **aBrowserChild) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScriptableBrowserChild(aBrowserChild); } \
  virtual already_AddRefed<nsIBrowserChild> GetBrowserChild(void) override; \
  virtual nsCommandManager * GetCommandManager(void) override; \
  NS_IMETHOD GetMetaViewportOverride(nsIDocShell::MetaViewportOverride *aMetaViewportOverride) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMetaViewportOverride(aMetaViewportOverride); } \
  NS_IMETHOD SetMetaViewportOverride(nsIDocShell::MetaViewportOverride aMetaViewportOverride) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMetaViewportOverride(aMetaViewportOverride); } \
  NS_IMETHOD GetUseTrackingProtection(bool *aUseTrackingProtection) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUseTrackingProtection(aUseTrackingProtection); } \
  NS_IMETHOD SetUseTrackingProtection(bool aUseTrackingProtection) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUseTrackingProtection(aUseTrackingProtection); } \
  NS_IMETHOD DispatchLocationChangeEvent(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DispatchLocationChangeEvent(); } \
  NS_IMETHOD StartDelayedAutoplayMediaComponents(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartDelayedAutoplayMediaComponents(); } \
  virtual mozilla::UniquePtr<mozilla::dom::ClientSource> TakeInitialClientSource(void) override; \
  NS_IMETHOD SetColorMatrix(const nsTArray<float >& aMatrix) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetColorMatrix(aMatrix); } \
  NS_IMETHOD GetIsForceReloading(bool *aIsForceReloading) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsForceReloading(aIsForceReloading); } \
  NS_IMETHOD GetColorMatrix(nsTArray<float >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetColorMatrix(_retval); } \
  NS_IMETHOD GetMessageManager(mozilla::dom::ContentFrameMessageManager **aMessageManager) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMessageManager(aMessageManager); } \
  NS_IMETHOD GetHasTrackingContentBlocked(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasTrackingContentBlocked(_retval); } \
  virtual bool GetIsAttemptingToNavigate() override; \
  NS_IMETHOD GetIsNavigating(bool *aIsNavigating) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsNavigating(aIsNavigating); } \
  NS_IMETHOD SynchronizeLayoutHistoryState(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SynchronizeLayoutHistoryState(); } \
  NS_IMETHOD PersistLayoutHistoryState(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PersistLayoutHistoryState(); } 


#endif /* __gen_nsIDocShell_h__ */
