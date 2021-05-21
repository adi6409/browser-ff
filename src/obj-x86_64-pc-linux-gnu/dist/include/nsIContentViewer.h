/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIContentViewer.idl
 */

#ifndef __gen_nsIContentViewer_h__
#define __gen_nsIContentViewer_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIDocShell; /* forward declaration */

class nsISHEntry; /* forward declaration */

class nsIPrintSettings; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

class nsINode; /* webidl Node */

#include "nsTArray.h"
#include "nsRect.h"
class nsIWidget;
class nsPresContext;
class nsView;
class nsDOMNavigationTiming;
namespace mozilla {
class Encoding;
class PresShell;
namespace dom {
class WindowGlobalChild;
} // namespace dom
} // namespace mozilla

/* starting interface:    nsIContentViewer */
#define NS_ICONTENTVIEWER_IID_STR "2da17016-7851-4a45-a7a8-00b360e01595"

#define NS_ICONTENTVIEWER_IID \
  {0x2da17016, 0x7851, 0x4a45, \
    { 0xa7, 0xa8, 0x00, 0xb3, 0x60, 0xe0, 0x15, 0x95 }}

class nsIContentViewer : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTVIEWER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentViewer;

  /* [noscript] void init (in nsIWidgetPtr aParentWidget, [const] in nsIntRectRef aBounds, in WindowGlobalChildPtr aWindowActor); */
  NS_IMETHOD Init(nsIWidget * aParentWidget, const nsIntRect & aBounds, mozilla::dom::WindowGlobalChild * aWindowActor) = 0;

  /* attribute nsIDocShell container; */
  NS_IMETHOD GetContainer(nsIDocShell **aContainer) = 0;
  NS_IMETHOD SetContainer(nsIDocShell *aContainer) = 0;

  /* [noscript,nostdcall,notxpcom] void loadStart (in Document aDoc); */
  virtual void LoadStart(mozilla::dom::Document *aDoc) = 0;

  /* [can_run_script] void loadComplete (in nsresult aStatus); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD LoadComplete(nsresult aStatus) = 0;

  /* [nostdcall,notxpcom] readonly attribute boolean loadCompleted; */
  virtual bool GetLoadCompleted() = 0;

  /* [nostdcall,notxpcom] readonly attribute boolean isStopped; */
  virtual bool GetIsStopped() = 0;

  enum PermitUnloadAction : uint8_t {
    ePrompt = 0,
    eDontPromptAndDontUnload = 1,
    eDontPromptAndUnload = 2,
  };

  enum PermitUnloadResult : uint8_t {
    eAllowNavigation = 0,
    eRequestBlockNavigation = 1,
  };

     nsresult PermitUnload(bool* canUnload) {
      return PermitUnload(ePrompt, canUnload);
    }
    /* boolean permitUnload ([optional] in nsIContentViewer_PermitUnloadAction aAction); */
  NS_IMETHOD PermitUnload(nsIContentViewer::PermitUnloadAction aAction, bool *_retval) = 0;

  /* readonly attribute boolean inPermitUnload; */
  NS_IMETHOD GetInPermitUnload(bool *aInPermitUnload) = 0;

  /* [noscript,nostdcall,notxpcom] nsIContentViewer_PermitUnloadResult dispatchBeforeUnload (); */
  virtual nsIContentViewer::PermitUnloadResult DispatchBeforeUnload(void) = 0;

  /* readonly attribute boolean beforeUnloadFiring; */
  NS_IMETHOD GetBeforeUnloadFiring(bool *aBeforeUnloadFiring) = 0;

  /* void pageHide (in boolean isUnload); */
  NS_IMETHOD PageHide(bool isUnload) = 0;

  /* void close (in nsISHEntry historyEntry); */
  NS_IMETHOD Close(nsISHEntry *historyEntry) = 0;

  /* void destroy (); */
  NS_IMETHOD Destroy(void) = 0;

  /* void stop (); */
  NS_IMETHOD Stop(void) = 0;

  /* readonly attribute Document DOMDocument; */
  NS_IMETHOD GetDOMDocument(mozilla::dom::Document **aDOMDocument) = 0;

  /* [noscript,nostdcall,notxpcom] Document getDocument (); */
  virtual mozilla::dom::Document * GetDocument(void) = 0;

  /* [noscript,nostdcall] void setDocument (in Document aDocument); */
  virtual nsresult SetDocument(mozilla::dom::Document *aDocument) = 0;

  /* [noscript] void getBounds (in nsIntRectRef aBounds); */
  NS_IMETHOD GetBounds(nsIntRect & aBounds) = 0;

  /* [noscript] void setBounds ([const] in nsIntRectRef aBounds); */
  NS_IMETHOD SetBounds(const nsIntRect & aBounds) = 0;

  enum {
    eDelayResize = 1U
  };

  /* [noscript] void setBoundsWithFlags ([const] in nsIntRectRef aBounds, in unsigned long aFlags); */
  NS_IMETHOD SetBoundsWithFlags(const nsIntRect & aBounds, uint32_t aFlags) = 0;

  /* [nostdcall,notxpcom] attribute nsIContentViewer previousViewer; */
  virtual nsIContentViewer * GetPreviousViewer() = 0;
  virtual void SetPreviousViewer(nsIContentViewer *aPreviousViewer) = 0;

  /* void move (in long aX, in long aY); */
  NS_IMETHOD Move(int32_t aX, int32_t aY) = 0;

  /* void show (); */
  NS_IMETHOD Show(void) = 0;

  /* void hide (); */
  NS_IMETHOD Hide(void) = 0;

  /* attribute boolean sticky; */
  NS_IMETHOD GetSticky(bool *aSticky) = 0;
  NS_IMETHOD SetSticky(bool aSticky) = 0;

  /* void open (in nsISupports aState, in nsISHEntry aSHEntry); */
  NS_IMETHOD Open(nsISupports *aState, nsISHEntry *aSHEntry) = 0;

  /* void clearHistoryEntry (); */
  NS_IMETHOD ClearHistoryEntry(void) = 0;

  /* void setPageModeForTesting (in boolean aPageMode, in nsIPrintSettings aPrintSettings); */
  NS_IMETHOD SetPageModeForTesting(bool aPageMode, nsIPrintSettings *aPrintSettings) = 0;

  /* readonly attribute nsISHEntry historyEntry; */
  NS_IMETHOD GetHistoryEntry(nsISHEntry **aHistoryEntry) = 0;

  /* readonly attribute boolean isTabModalPromptAllowed; */
  NS_IMETHOD GetIsTabModalPromptAllowed(bool *aIsTabModalPromptAllowed) = 0;

  /* attribute boolean isHidden; */
  NS_IMETHOD GetIsHidden(bool *aIsHidden) = 0;
  NS_IMETHOD SetIsHidden(bool aIsHidden) = 0;

  /* [nostdcall,notxpcom] readonly attribute PresShellPtr presShell; */
  virtual mozilla::PresShell * GetPresShell() = 0;

  /* [nostdcall,notxpcom] readonly attribute nsPresContextPtr presContext; */
  virtual nsPresContext * GetPresContext() = 0;

  /* [noscript] void setDocumentInternal (in Document aDocument, in boolean aForceReuseInnerWindow); */
  NS_IMETHOD SetDocumentInternal(mozilla::dom::Document *aDocument, bool aForceReuseInnerWindow) = 0;

  /* [noscript,nostdcall,notxpcom] nsViewPtr findContainerView (); */
  virtual nsView * FindContainerView(void) = 0;

  /* [noscript,nostdcall,notxpcom] void setNavigationTiming (in nsDOMNavigationTimingPtr aTiming); */
  virtual void SetNavigationTiming(nsDOMNavigationTiming * aTiming) = 0;

  /* readonly attribute float deviceFullZoomForTest; */
  NS_IMETHOD GetDeviceFullZoomForTest(float *aDeviceFullZoomForTest) = 0;

  /* attribute boolean authorStyleDisabled; */
  NS_IMETHOD GetAuthorStyleDisabled(bool *aAuthorStyleDisabled) = 0;
  NS_IMETHOD SetAuthorStyleDisabled(bool aAuthorStyleDisabled) = 0;

  /* attribute ACString hintCharacterSet; */
  NS_IMETHOD GetHintCharacterSet(nsACString& aHintCharacterSet) = 0;
  NS_IMETHOD SetHintCharacterSet(const nsACString& aHintCharacterSet) = 0;

  /* attribute int32_t hintCharacterSetSource; */
  NS_IMETHOD GetHintCharacterSetSource(int32_t *aHintCharacterSetSource) = 0;
  NS_IMETHOD SetHintCharacterSetSource(int32_t aHintCharacterSetSource) = 0;

  /* void getContentSize (out long width, out long height); */
  NS_IMETHOD GetContentSize(int32_t *width, int32_t *height) = 0;

  /* void getContentSizeConstrained (in long maxWidth, in long maxHeight, out long width, out long height); */
  NS_IMETHOD GetContentSizeConstrained(int32_t maxWidth, int32_t maxHeight, int32_t *width, int32_t *height) = 0;

  /* void pausePainting (); */
  NS_IMETHOD PausePainting(void) = 0;

  /* void resumePainting (); */
  NS_IMETHOD ResumePainting(void) = 0;

  /* [noscript,notxpcom] Encoding getHintCharset (); */
  NS_IMETHOD_(const mozilla::Encoding *) GetHintCharset(void) = 0;

  /* [noscript,notxpcom] void setHintCharset (in Encoding aEncoding); */
  NS_IMETHOD_(void) SetHintCharset(const mozilla::Encoding * aEncoding) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentViewer, NS_ICONTENTVIEWER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTVIEWER \
  NS_IMETHOD Init(nsIWidget * aParentWidget, const nsIntRect & aBounds, mozilla::dom::WindowGlobalChild * aWindowActor) override; \
  NS_IMETHOD GetContainer(nsIDocShell **aContainer) override; \
  NS_IMETHOD SetContainer(nsIDocShell *aContainer) override; \
  virtual void LoadStart(mozilla::dom::Document *aDoc) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD LoadComplete(nsresult aStatus) override; \
  virtual bool GetLoadCompleted() override; \
  virtual bool GetIsStopped() override; \
  NS_IMETHOD PermitUnload(nsIContentViewer::PermitUnloadAction aAction, bool *_retval) override; \
  NS_IMETHOD GetInPermitUnload(bool *aInPermitUnload) override; \
  virtual nsIContentViewer::PermitUnloadResult DispatchBeforeUnload(void) override; \
  NS_IMETHOD GetBeforeUnloadFiring(bool *aBeforeUnloadFiring) override; \
  NS_IMETHOD PageHide(bool isUnload) override; \
  NS_IMETHOD Close(nsISHEntry *historyEntry) override; \
  NS_IMETHOD Destroy(void) override; \
  NS_IMETHOD Stop(void) override; \
  NS_IMETHOD GetDOMDocument(mozilla::dom::Document **aDOMDocument) override; \
  virtual mozilla::dom::Document * GetDocument(void) override; \
  virtual nsresult SetDocument(mozilla::dom::Document *aDocument) override; \
  NS_IMETHOD GetBounds(nsIntRect & aBounds) override; \
  NS_IMETHOD SetBounds(const nsIntRect & aBounds) override; \
  NS_IMETHOD SetBoundsWithFlags(const nsIntRect & aBounds, uint32_t aFlags) override; \
  virtual nsIContentViewer * GetPreviousViewer() override; \
  virtual void SetPreviousViewer(nsIContentViewer *aPreviousViewer) override; \
  NS_IMETHOD Move(int32_t aX, int32_t aY) override; \
  NS_IMETHOD Show(void) override; \
  NS_IMETHOD Hide(void) override; \
  NS_IMETHOD GetSticky(bool *aSticky) override; \
  NS_IMETHOD SetSticky(bool aSticky) override; \
  NS_IMETHOD Open(nsISupports *aState, nsISHEntry *aSHEntry) override; \
  NS_IMETHOD ClearHistoryEntry(void) override; \
  NS_IMETHOD SetPageModeForTesting(bool aPageMode, nsIPrintSettings *aPrintSettings) override; \
  NS_IMETHOD GetHistoryEntry(nsISHEntry **aHistoryEntry) override; \
  NS_IMETHOD GetIsTabModalPromptAllowed(bool *aIsTabModalPromptAllowed) override; \
  NS_IMETHOD GetIsHidden(bool *aIsHidden) override; \
  NS_IMETHOD SetIsHidden(bool aIsHidden) override; \
  virtual mozilla::PresShell * GetPresShell() override; \
  virtual nsPresContext * GetPresContext() override; \
  NS_IMETHOD SetDocumentInternal(mozilla::dom::Document *aDocument, bool aForceReuseInnerWindow) override; \
  virtual nsView * FindContainerView(void) override; \
  virtual void SetNavigationTiming(nsDOMNavigationTiming * aTiming) override; \
  NS_IMETHOD GetDeviceFullZoomForTest(float *aDeviceFullZoomForTest) override; \
  NS_IMETHOD GetAuthorStyleDisabled(bool *aAuthorStyleDisabled) override; \
  NS_IMETHOD SetAuthorStyleDisabled(bool aAuthorStyleDisabled) override; \
  NS_IMETHOD GetHintCharacterSet(nsACString& aHintCharacterSet) override; \
  NS_IMETHOD SetHintCharacterSet(const nsACString& aHintCharacterSet) override; \
  NS_IMETHOD GetHintCharacterSetSource(int32_t *aHintCharacterSetSource) override; \
  NS_IMETHOD SetHintCharacterSetSource(int32_t aHintCharacterSetSource) override; \
  NS_IMETHOD GetContentSize(int32_t *width, int32_t *height) override; \
  NS_IMETHOD GetContentSizeConstrained(int32_t maxWidth, int32_t maxHeight, int32_t *width, int32_t *height) override; \
  NS_IMETHOD PausePainting(void) override; \
  NS_IMETHOD ResumePainting(void) override; \
  NS_IMETHOD_(const mozilla::Encoding *) GetHintCharset(void) override; \
  NS_IMETHOD_(void) SetHintCharset(const mozilla::Encoding * aEncoding) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTVIEWER \
  nsresult Init(nsIWidget * aParentWidget, const nsIntRect & aBounds, mozilla::dom::WindowGlobalChild * aWindowActor); \
  nsresult GetContainer(nsIDocShell **aContainer); \
  nsresult SetContainer(nsIDocShell *aContainer); \
  void LoadStart(mozilla::dom::Document *aDoc); \
  MOZ_CAN_RUN_SCRIPT nsresult LoadComplete(nsresult aStatus); \
  bool GetLoadCompleted(); \
  bool GetIsStopped(); \
  nsresult PermitUnload(nsIContentViewer::PermitUnloadAction aAction, bool *_retval); \
  nsresult GetInPermitUnload(bool *aInPermitUnload); \
  nsIContentViewer::PermitUnloadResult DispatchBeforeUnload(void); \
  nsresult GetBeforeUnloadFiring(bool *aBeforeUnloadFiring); \
  nsresult PageHide(bool isUnload); \
  nsresult Close(nsISHEntry *historyEntry); \
  nsresult Destroy(void); \
  nsresult Stop(void); \
  nsresult GetDOMDocument(mozilla::dom::Document **aDOMDocument); \
  mozilla::dom::Document * GetDocument(void); \
  nsresult SetDocument(mozilla::dom::Document *aDocument); \
  nsresult GetBounds(nsIntRect & aBounds); \
  nsresult SetBounds(const nsIntRect & aBounds); \
  nsresult SetBoundsWithFlags(const nsIntRect & aBounds, uint32_t aFlags); \
  nsIContentViewer * GetPreviousViewer(); \
  void SetPreviousViewer(nsIContentViewer *aPreviousViewer); \
  nsresult Move(int32_t aX, int32_t aY); \
  nsresult Show(void); \
  nsresult Hide(void); \
  nsresult GetSticky(bool *aSticky); \
  nsresult SetSticky(bool aSticky); \
  nsresult Open(nsISupports *aState, nsISHEntry *aSHEntry); \
  nsresult ClearHistoryEntry(void); \
  nsresult SetPageModeForTesting(bool aPageMode, nsIPrintSettings *aPrintSettings); \
  nsresult GetHistoryEntry(nsISHEntry **aHistoryEntry); \
  nsresult GetIsTabModalPromptAllowed(bool *aIsTabModalPromptAllowed); \
  nsresult GetIsHidden(bool *aIsHidden); \
  nsresult SetIsHidden(bool aIsHidden); \
  mozilla::PresShell * GetPresShell(); \
  nsPresContext * GetPresContext(); \
  nsresult SetDocumentInternal(mozilla::dom::Document *aDocument, bool aForceReuseInnerWindow); \
  nsView * FindContainerView(void); \
  void SetNavigationTiming(nsDOMNavigationTiming * aTiming); \
  nsresult GetDeviceFullZoomForTest(float *aDeviceFullZoomForTest); \
  nsresult GetAuthorStyleDisabled(bool *aAuthorStyleDisabled); \
  nsresult SetAuthorStyleDisabled(bool aAuthorStyleDisabled); \
  nsresult GetHintCharacterSet(nsACString& aHintCharacterSet); \
  nsresult SetHintCharacterSet(const nsACString& aHintCharacterSet); \
  nsresult GetHintCharacterSetSource(int32_t *aHintCharacterSetSource); \
  nsresult SetHintCharacterSetSource(int32_t aHintCharacterSetSource); \
  nsresult GetContentSize(int32_t *width, int32_t *height); \
  nsresult GetContentSizeConstrained(int32_t maxWidth, int32_t maxHeight, int32_t *width, int32_t *height); \
  nsresult PausePainting(void); \
  nsresult ResumePainting(void); \
  nsresult_(const mozilla::Encoding *) GetHintCharset(void); \
  nsresult_(void) SetHintCharset(const mozilla::Encoding * aEncoding); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTVIEWER(_to) \
  NS_IMETHOD Init(nsIWidget * aParentWidget, const nsIntRect & aBounds, mozilla::dom::WindowGlobalChild * aWindowActor) override { return _to Init(aParentWidget, aBounds, aWindowActor); } \
  NS_IMETHOD GetContainer(nsIDocShell **aContainer) override { return _to GetContainer(aContainer); } \
  NS_IMETHOD SetContainer(nsIDocShell *aContainer) override { return _to SetContainer(aContainer); } \
  virtual void LoadStart(mozilla::dom::Document *aDoc) override { return _to LoadStart(aDoc); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD LoadComplete(nsresult aStatus) override { return _to LoadComplete(aStatus); } \
  virtual bool GetLoadCompleted() override { return _to GetLoadCompleted(); } \
  virtual bool GetIsStopped() override { return _to GetIsStopped(); } \
  NS_IMETHOD PermitUnload(nsIContentViewer::PermitUnloadAction aAction, bool *_retval) override { return _to PermitUnload(aAction, _retval); } \
  NS_IMETHOD GetInPermitUnload(bool *aInPermitUnload) override { return _to GetInPermitUnload(aInPermitUnload); } \
  virtual nsIContentViewer::PermitUnloadResult DispatchBeforeUnload(void) override { return _to DispatchBeforeUnload(); } \
  NS_IMETHOD GetBeforeUnloadFiring(bool *aBeforeUnloadFiring) override { return _to GetBeforeUnloadFiring(aBeforeUnloadFiring); } \
  NS_IMETHOD PageHide(bool isUnload) override { return _to PageHide(isUnload); } \
  NS_IMETHOD Close(nsISHEntry *historyEntry) override { return _to Close(historyEntry); } \
  NS_IMETHOD Destroy(void) override { return _to Destroy(); } \
  NS_IMETHOD Stop(void) override { return _to Stop(); } \
  NS_IMETHOD GetDOMDocument(mozilla::dom::Document **aDOMDocument) override { return _to GetDOMDocument(aDOMDocument); } \
  virtual mozilla::dom::Document * GetDocument(void) override { return _to GetDocument(); } \
  virtual nsresult SetDocument(mozilla::dom::Document *aDocument) override { return _to SetDocument(aDocument); } \
  NS_IMETHOD GetBounds(nsIntRect & aBounds) override { return _to GetBounds(aBounds); } \
  NS_IMETHOD SetBounds(const nsIntRect & aBounds) override { return _to SetBounds(aBounds); } \
  NS_IMETHOD SetBoundsWithFlags(const nsIntRect & aBounds, uint32_t aFlags) override { return _to SetBoundsWithFlags(aBounds, aFlags); } \
  virtual nsIContentViewer * GetPreviousViewer() override { return _to GetPreviousViewer(); } \
  virtual void SetPreviousViewer(nsIContentViewer *aPreviousViewer) override { return _to SetPreviousViewer(aPreviousViewer); } \
  NS_IMETHOD Move(int32_t aX, int32_t aY) override { return _to Move(aX, aY); } \
  NS_IMETHOD Show(void) override { return _to Show(); } \
  NS_IMETHOD Hide(void) override { return _to Hide(); } \
  NS_IMETHOD GetSticky(bool *aSticky) override { return _to GetSticky(aSticky); } \
  NS_IMETHOD SetSticky(bool aSticky) override { return _to SetSticky(aSticky); } \
  NS_IMETHOD Open(nsISupports *aState, nsISHEntry *aSHEntry) override { return _to Open(aState, aSHEntry); } \
  NS_IMETHOD ClearHistoryEntry(void) override { return _to ClearHistoryEntry(); } \
  NS_IMETHOD SetPageModeForTesting(bool aPageMode, nsIPrintSettings *aPrintSettings) override { return _to SetPageModeForTesting(aPageMode, aPrintSettings); } \
  NS_IMETHOD GetHistoryEntry(nsISHEntry **aHistoryEntry) override { return _to GetHistoryEntry(aHistoryEntry); } \
  NS_IMETHOD GetIsTabModalPromptAllowed(bool *aIsTabModalPromptAllowed) override { return _to GetIsTabModalPromptAllowed(aIsTabModalPromptAllowed); } \
  NS_IMETHOD GetIsHidden(bool *aIsHidden) override { return _to GetIsHidden(aIsHidden); } \
  NS_IMETHOD SetIsHidden(bool aIsHidden) override { return _to SetIsHidden(aIsHidden); } \
  virtual mozilla::PresShell * GetPresShell() override { return _to GetPresShell(); } \
  virtual nsPresContext * GetPresContext() override { return _to GetPresContext(); } \
  NS_IMETHOD SetDocumentInternal(mozilla::dom::Document *aDocument, bool aForceReuseInnerWindow) override { return _to SetDocumentInternal(aDocument, aForceReuseInnerWindow); } \
  virtual nsView * FindContainerView(void) override { return _to FindContainerView(); } \
  virtual void SetNavigationTiming(nsDOMNavigationTiming * aTiming) override { return _to SetNavigationTiming(aTiming); } \
  NS_IMETHOD GetDeviceFullZoomForTest(float *aDeviceFullZoomForTest) override { return _to GetDeviceFullZoomForTest(aDeviceFullZoomForTest); } \
  NS_IMETHOD GetAuthorStyleDisabled(bool *aAuthorStyleDisabled) override { return _to GetAuthorStyleDisabled(aAuthorStyleDisabled); } \
  NS_IMETHOD SetAuthorStyleDisabled(bool aAuthorStyleDisabled) override { return _to SetAuthorStyleDisabled(aAuthorStyleDisabled); } \
  NS_IMETHOD GetHintCharacterSet(nsACString& aHintCharacterSet) override { return _to GetHintCharacterSet(aHintCharacterSet); } \
  NS_IMETHOD SetHintCharacterSet(const nsACString& aHintCharacterSet) override { return _to SetHintCharacterSet(aHintCharacterSet); } \
  NS_IMETHOD GetHintCharacterSetSource(int32_t *aHintCharacterSetSource) override { return _to GetHintCharacterSetSource(aHintCharacterSetSource); } \
  NS_IMETHOD SetHintCharacterSetSource(int32_t aHintCharacterSetSource) override { return _to SetHintCharacterSetSource(aHintCharacterSetSource); } \
  NS_IMETHOD GetContentSize(int32_t *width, int32_t *height) override { return _to GetContentSize(width, height); } \
  NS_IMETHOD GetContentSizeConstrained(int32_t maxWidth, int32_t maxHeight, int32_t *width, int32_t *height) override { return _to GetContentSizeConstrained(maxWidth, maxHeight, width, height); } \
  NS_IMETHOD PausePainting(void) override { return _to PausePainting(); } \
  NS_IMETHOD ResumePainting(void) override { return _to ResumePainting(); } \
  NS_IMETHOD_(const mozilla::Encoding *) GetHintCharset(void) override { return _to GetHintCharset(); } \
  NS_IMETHOD_(void) SetHintCharset(const mozilla::Encoding * aEncoding) override { return _to SetHintCharset(aEncoding); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTVIEWER(_to) \
  NS_IMETHOD Init(nsIWidget * aParentWidget, const nsIntRect & aBounds, mozilla::dom::WindowGlobalChild * aWindowActor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aParentWidget, aBounds, aWindowActor); } \
  NS_IMETHOD GetContainer(nsIDocShell **aContainer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContainer(aContainer); } \
  NS_IMETHOD SetContainer(nsIDocShell *aContainer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContainer(aContainer); } \
  virtual void LoadStart(mozilla::dom::Document *aDoc) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD LoadComplete(nsresult aStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadComplete(aStatus); } \
  virtual bool GetLoadCompleted() override; \
  virtual bool GetIsStopped() override; \
  NS_IMETHOD PermitUnload(nsIContentViewer::PermitUnloadAction aAction, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PermitUnload(aAction, _retval); } \
  NS_IMETHOD GetInPermitUnload(bool *aInPermitUnload) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInPermitUnload(aInPermitUnload); } \
  virtual nsIContentViewer::PermitUnloadResult DispatchBeforeUnload(void) override; \
  NS_IMETHOD GetBeforeUnloadFiring(bool *aBeforeUnloadFiring) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBeforeUnloadFiring(aBeforeUnloadFiring); } \
  NS_IMETHOD PageHide(bool isUnload) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PageHide(isUnload); } \
  NS_IMETHOD Close(nsISHEntry *historyEntry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(historyEntry); } \
  NS_IMETHOD Destroy(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Destroy(); } \
  NS_IMETHOD Stop(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Stop(); } \
  NS_IMETHOD GetDOMDocument(mozilla::dom::Document **aDOMDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDOMDocument(aDOMDocument); } \
  virtual mozilla::dom::Document * GetDocument(void) override; \
  virtual nsresult SetDocument(mozilla::dom::Document *aDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDocument(aDocument); } \
  NS_IMETHOD GetBounds(nsIntRect & aBounds) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBounds(aBounds); } \
  NS_IMETHOD SetBounds(const nsIntRect & aBounds) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBounds(aBounds); } \
  NS_IMETHOD SetBoundsWithFlags(const nsIntRect & aBounds, uint32_t aFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBoundsWithFlags(aBounds, aFlags); } \
  virtual nsIContentViewer * GetPreviousViewer() override; \
  virtual void SetPreviousViewer(nsIContentViewer *aPreviousViewer) override; \
  NS_IMETHOD Move(int32_t aX, int32_t aY) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Move(aX, aY); } \
  NS_IMETHOD Show(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Show(); } \
  NS_IMETHOD Hide(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Hide(); } \
  NS_IMETHOD GetSticky(bool *aSticky) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSticky(aSticky); } \
  NS_IMETHOD SetSticky(bool aSticky) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSticky(aSticky); } \
  NS_IMETHOD Open(nsISupports *aState, nsISHEntry *aSHEntry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Open(aState, aSHEntry); } \
  NS_IMETHOD ClearHistoryEntry(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearHistoryEntry(); } \
  NS_IMETHOD SetPageModeForTesting(bool aPageMode, nsIPrintSettings *aPrintSettings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPageModeForTesting(aPageMode, aPrintSettings); } \
  NS_IMETHOD GetHistoryEntry(nsISHEntry **aHistoryEntry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHistoryEntry(aHistoryEntry); } \
  NS_IMETHOD GetIsTabModalPromptAllowed(bool *aIsTabModalPromptAllowed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsTabModalPromptAllowed(aIsTabModalPromptAllowed); } \
  NS_IMETHOD GetIsHidden(bool *aIsHidden) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsHidden(aIsHidden); } \
  NS_IMETHOD SetIsHidden(bool aIsHidden) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsHidden(aIsHidden); } \
  virtual mozilla::PresShell * GetPresShell() override; \
  virtual nsPresContext * GetPresContext() override; \
  NS_IMETHOD SetDocumentInternal(mozilla::dom::Document *aDocument, bool aForceReuseInnerWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDocumentInternal(aDocument, aForceReuseInnerWindow); } \
  virtual nsView * FindContainerView(void) override; \
  virtual void SetNavigationTiming(nsDOMNavigationTiming * aTiming) override; \
  NS_IMETHOD GetDeviceFullZoomForTest(float *aDeviceFullZoomForTest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDeviceFullZoomForTest(aDeviceFullZoomForTest); } \
  NS_IMETHOD GetAuthorStyleDisabled(bool *aAuthorStyleDisabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAuthorStyleDisabled(aAuthorStyleDisabled); } \
  NS_IMETHOD SetAuthorStyleDisabled(bool aAuthorStyleDisabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAuthorStyleDisabled(aAuthorStyleDisabled); } \
  NS_IMETHOD GetHintCharacterSet(nsACString& aHintCharacterSet) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHintCharacterSet(aHintCharacterSet); } \
  NS_IMETHOD SetHintCharacterSet(const nsACString& aHintCharacterSet) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHintCharacterSet(aHintCharacterSet); } \
  NS_IMETHOD GetHintCharacterSetSource(int32_t *aHintCharacterSetSource) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHintCharacterSetSource(aHintCharacterSetSource); } \
  NS_IMETHOD SetHintCharacterSetSource(int32_t aHintCharacterSetSource) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHintCharacterSetSource(aHintCharacterSetSource); } \
  NS_IMETHOD GetContentSize(int32_t *width, int32_t *height) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentSize(width, height); } \
  NS_IMETHOD GetContentSizeConstrained(int32_t maxWidth, int32_t maxHeight, int32_t *width, int32_t *height) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentSizeConstrained(maxWidth, maxHeight, width, height); } \
  NS_IMETHOD PausePainting(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PausePainting(); } \
  NS_IMETHOD ResumePainting(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResumePainting(); } \
  NS_IMETHOD_(const mozilla::Encoding *) GetHintCharset(void) override; \
  NS_IMETHOD_(void) SetHintCharset(const mozilla::Encoding * aEncoding) override; 

namespace mozilla {
namespace dom {
using XPCOMPermitUnloadAction = nsIContentViewer::PermitUnloadAction;
using PermitUnloadResult = nsIContentViewer::PermitUnloadResult;
} // namespace dom
} // namespace mozilla

#endif /* __gen_nsIContentViewer_h__ */
