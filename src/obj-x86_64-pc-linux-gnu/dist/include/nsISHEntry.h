/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/shistory/nsISHEntry.idl
 */

#ifndef __gen_nsISHEntry_h__
#define __gen_nsISHEntry_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIContentSecurityPolicy; /* forward declaration */

class nsIMutableArray; /* forward declaration */

class nsILayoutHistoryState; /* forward declaration */

class nsIContentViewer; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsIDocShellTreeItem; /* forward declaration */

class nsIStructuredCloneContainer; /* forward declaration */

class nsIBFCacheEntry; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsISHistory; /* forward declaration */

class nsIReferrerInfo; /* forward declaration */

#include "nsRect.h"
class nsDocShellEditorData;
namespace mozilla {
namespace dom {
class SHEntrySharedParentState;
}
}
class nsSHEntryShared;
class nsDocShellLoadState;
struct EntriesAndBrowsingContextData;
namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsISHEntry */
#define NS_ISHENTRY_IID_STR "0dad26b8-a259-42c7-93f1-2fa7fc076e45"

#define NS_ISHENTRY_IID \
  {0x0dad26b8, 0xa259, 0x42c7, \
    { 0x93, 0xf1, 0x2f, 0xa7, 0xfc, 0x07, 0x6e, 0x45 }}

class NS_NO_VTABLE nsISHEntry : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISHENTRY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISHEntry;

  /* [infallible] attribute nsIURI URI; */
  NS_IMETHOD GetURI(nsIURI **aURI) = 0;
   inline already_AddRefed<nsIURI> GetURI()
  {
    nsIURI* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetURI(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIURI>(result);
  }
  NS_IMETHOD SetURI(nsIURI *aURI) = 0;

  /* [infallible] attribute nsIURI originalURI; */
  NS_IMETHOD GetOriginalURI(nsIURI **aOriginalURI) = 0;
   inline already_AddRefed<nsIURI> GetOriginalURI()
  {
    nsIURI* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetOriginalURI(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIURI>(result);
  }
  NS_IMETHOD SetOriginalURI(nsIURI *aOriginalURI) = 0;

  /* [infallible] attribute nsIURI resultPrincipalURI; */
  NS_IMETHOD GetResultPrincipalURI(nsIURI **aResultPrincipalURI) = 0;
   inline already_AddRefed<nsIURI> GetResultPrincipalURI()
  {
    nsIURI* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetResultPrincipalURI(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIURI>(result);
  }
  NS_IMETHOD SetResultPrincipalURI(nsIURI *aResultPrincipalURI) = 0;

  /* [infallible] attribute boolean loadReplace; */
  NS_IMETHOD GetLoadReplace(bool *aLoadReplace) = 0;
  inline bool  GetLoadReplace()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetLoadReplace(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetLoadReplace(bool aLoadReplace) = 0;

  /* attribute AString title; */
  NS_IMETHOD GetTitle(nsAString& aTitle) = 0;
  NS_IMETHOD SetTitle(const nsAString& aTitle) = 0;

  /* attribute AString name; */
  NS_IMETHOD GetName(nsAString& aName) = 0;
  NS_IMETHOD SetName(const nsAString& aName) = 0;

  /* [infallible] attribute boolean isSubFrame; */
  NS_IMETHOD GetIsSubFrame(bool *aIsSubFrame) = 0;
  inline bool  GetIsSubFrame()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsSubFrame(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetIsSubFrame(bool aIsSubFrame) = 0;

  /* [infallible] attribute boolean hasUserInteraction; */
  NS_IMETHOD GetHasUserInteraction(bool *aHasUserInteraction) = 0;
  inline bool  GetHasUserInteraction()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetHasUserInteraction(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetHasUserInteraction(bool aHasUserInteraction) = 0;

  /* [infallible] attribute nsIReferrerInfo referrerInfo; */
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) = 0;
   inline already_AddRefed<nsIReferrerInfo> GetReferrerInfo()
  {
    nsIReferrerInfo* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetReferrerInfo(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIReferrerInfo>(result);
  }
  NS_IMETHOD SetReferrerInfo(nsIReferrerInfo *aReferrerInfo) = 0;

  /* [infallible] attribute nsIContentViewer contentViewer; */
  NS_IMETHOD GetContentViewer(nsIContentViewer **aContentViewer) = 0;
   inline already_AddRefed<nsIContentViewer> GetContentViewer()
  {
    nsIContentViewer* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetContentViewer(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIContentViewer>(result);
  }
  NS_IMETHOD SetContentViewer(nsIContentViewer *aContentViewer) = 0;

  /* [infallible] attribute boolean sticky; */
  NS_IMETHOD GetSticky(bool *aSticky) = 0;
  inline bool  GetSticky()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetSticky(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetSticky(bool aSticky) = 0;

  /* [infallible] attribute nsISupports windowState; */
  NS_IMETHOD GetWindowState(nsISupports **aWindowState) = 0;
   inline already_AddRefed<nsISupports> GetWindowState()
  {
    nsISupports* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetWindowState(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsISupports>(result);
  }
  NS_IMETHOD SetWindowState(nsISupports *aWindowState) = 0;

  /* [infallible] attribute nsIMutableArray refreshURIList; */
  NS_IMETHOD GetRefreshURIList(nsIMutableArray **aRefreshURIList) = 0;
   inline already_AddRefed<nsIMutableArray> GetRefreshURIList()
  {
    nsIMutableArray* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetRefreshURIList(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIMutableArray>(result);
  }
  NS_IMETHOD SetRefreshURIList(nsIMutableArray *aRefreshURIList) = 0;

  /* [infallible] attribute nsIInputStream postData; */
  NS_IMETHOD GetPostData(nsIInputStream **aPostData) = 0;
   inline already_AddRefed<nsIInputStream> GetPostData()
  {
    nsIInputStream* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetPostData(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIInputStream>(result);
  }
  NS_IMETHOD SetPostData(nsIInputStream *aPostData) = 0;

  /* [infallible] attribute nsILayoutHistoryState layoutHistoryState; */
  NS_IMETHOD GetLayoutHistoryState(nsILayoutHistoryState **aLayoutHistoryState) = 0;
   inline already_AddRefed<nsILayoutHistoryState> GetLayoutHistoryState()
  {
    nsILayoutHistoryState* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetLayoutHistoryState(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsILayoutHistoryState>(result);
  }
  NS_IMETHOD SetLayoutHistoryState(nsILayoutHistoryState *aLayoutHistoryState) = 0;

  /* [infallible] attribute nsISHEntry parent; */
  NS_IMETHOD GetParent(nsISHEntry **aParent) = 0;
   inline already_AddRefed<nsISHEntry> GetParent()
  {
    nsISHEntry* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetParent(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsISHEntry>(result);
  }
  NS_IMETHOD SetParent(nsISHEntry *aParent) = 0;

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

  /* [infallible] attribute unsigned long ID; */
  NS_IMETHOD GetID(uint32_t *aID) = 0;
  inline uint32_t  GetID()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetID(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetID(uint32_t aID) = 0;

  /* [infallible] attribute unsigned long cacheKey; */
  NS_IMETHOD GetCacheKey(uint32_t *aCacheKey) = 0;
  inline uint32_t  GetCacheKey()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetCacheKey(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetCacheKey(uint32_t aCacheKey) = 0;

  /* [infallible] attribute boolean saveLayoutStateFlag; */
  NS_IMETHOD GetSaveLayoutStateFlag(bool *aSaveLayoutStateFlag) = 0;
  inline bool  GetSaveLayoutStateFlag()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetSaveLayoutStateFlag(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetSaveLayoutStateFlag(bool aSaveLayoutStateFlag) = 0;

  /* attribute ACString contentType; */
  NS_IMETHOD GetContentType(nsACString& aContentType) = 0;
  NS_IMETHOD SetContentType(const nsACString& aContentType) = 0;

  /* [infallible] attribute boolean URIWasModified; */
  NS_IMETHOD GetURIWasModified(bool *aURIWasModified) = 0;
  inline bool  GetURIWasModified()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetURIWasModified(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetURIWasModified(bool aURIWasModified) = 0;

  /* [infallible] attribute nsIPrincipal triggeringPrincipal; */
  NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) = 0;
   inline already_AddRefed<nsIPrincipal> GetTriggeringPrincipal()
  {
    nsIPrincipal* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetTriggeringPrincipal(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIPrincipal>(result);
  }
  NS_IMETHOD SetTriggeringPrincipal(nsIPrincipal *aTriggeringPrincipal) = 0;

  /* [infallible] attribute nsIPrincipal principalToInherit; */
  NS_IMETHOD GetPrincipalToInherit(nsIPrincipal **aPrincipalToInherit) = 0;
   inline already_AddRefed<nsIPrincipal> GetPrincipalToInherit()
  {
    nsIPrincipal* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetPrincipalToInherit(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIPrincipal>(result);
  }
  NS_IMETHOD SetPrincipalToInherit(nsIPrincipal *aPrincipalToInherit) = 0;

  /* [infallible] attribute nsIPrincipal partitionedPrincipalToInherit; */
  NS_IMETHOD GetPartitionedPrincipalToInherit(nsIPrincipal **aPartitionedPrincipalToInherit) = 0;
   inline already_AddRefed<nsIPrincipal> GetPartitionedPrincipalToInherit()
  {
    nsIPrincipal* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetPartitionedPrincipalToInherit(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIPrincipal>(result);
  }
  NS_IMETHOD SetPartitionedPrincipalToInherit(nsIPrincipal *aPartitionedPrincipalToInherit) = 0;

  /* [infallible] attribute nsIContentSecurityPolicy csp; */
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) = 0;
   inline already_AddRefed<nsIContentSecurityPolicy> GetCsp()
  {
    nsIContentSecurityPolicy* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetCsp(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIContentSecurityPolicy>(result);
  }
  NS_IMETHOD SetCsp(nsIContentSecurityPolicy *aCsp) = 0;

  /* [infallible] attribute nsIStructuredCloneContainer stateData; */
  NS_IMETHOD GetStateData(nsIStructuredCloneContainer **aStateData) = 0;
   inline already_AddRefed<nsIStructuredCloneContainer> GetStateData()
  {
    nsIStructuredCloneContainer* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetStateData(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIStructuredCloneContainer>(result);
  }
  NS_IMETHOD SetStateData(nsIStructuredCloneContainer *aStateData) = 0;

  /* attribute nsIDRef docshellID; */
  NS_IMETHOD GetDocshellID(nsID & aDocshellID) = 0;
  NS_IMETHOD SetDocshellID(const nsID & aDocshellID) = 0;

  /* [infallible] readonly attribute boolean isSrcdocEntry; */
  NS_IMETHOD GetIsSrcdocEntry(bool *aIsSrcdocEntry) = 0;
  inline bool  GetIsSrcdocEntry()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsSrcdocEntry(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* attribute AString srcdocData; */
  NS_IMETHOD GetSrcdocData(nsAString& aSrcdocData) = 0;
  NS_IMETHOD SetSrcdocData(const nsAString& aSrcdocData) = 0;

  /* [infallible] attribute nsIURI baseURI; */
  NS_IMETHOD GetBaseURI(nsIURI **aBaseURI) = 0;
   inline already_AddRefed<nsIURI> GetBaseURI()
  {
    nsIURI* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetBaseURI(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIURI>(result);
  }
  NS_IMETHOD SetBaseURI(nsIURI *aBaseURI) = 0;

  /* [infallible] attribute boolean scrollRestorationIsManual; */
  NS_IMETHOD GetScrollRestorationIsManual(bool *aScrollRestorationIsManual) = 0;
  inline bool  GetScrollRestorationIsManual()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetScrollRestorationIsManual(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetScrollRestorationIsManual(bool aScrollRestorationIsManual) = 0;

  /* [infallible] readonly attribute boolean loadedInThisProcess; */
  NS_IMETHOD GetLoadedInThisProcess(bool *aLoadedInThisProcess) = 0;
  inline bool  GetLoadedInThisProcess()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetLoadedInThisProcess(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible,noscript] attribute nsISHistory shistory; */
  NS_IMETHOD GetShistory(nsISHistory **aShistory) = 0;
   inline already_AddRefed<nsISHistory> GetShistory()
  {
    nsISHistory* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetShistory(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsISHistory>(result);
  }
  NS_IMETHOD SetShistory(nsISHistory *aShistory) = 0;

  /* [infallible,noscript] attribute unsigned long lastTouched; */
  NS_IMETHOD GetLastTouched(uint32_t *aLastTouched) = 0;
  inline uint32_t  GetLastTouched()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetLastTouched(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetLastTouched(uint32_t aLastTouched) = 0;

  /* [infallible] readonly attribute long childCount; */
  NS_IMETHOD GetChildCount(int32_t *aChildCount) = 0;
  inline int32_t  GetChildCount()
  {
    int32_t result;
    mozilla::DebugOnly<nsresult> rv = GetChildCount(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] attribute boolean persist; */
  NS_IMETHOD GetPersist(bool *aPersist) = 0;
  inline bool  GetPersist()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetPersist(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetPersist(bool aPersist) = 0;

  /* void setScrollPosition (in long x, in long y); */
  NS_IMETHOD SetScrollPosition(int32_t x, int32_t y) = 0;

  /* void getScrollPosition (out long x, out long y); */
  NS_IMETHOD GetScrollPosition(int32_t *x, int32_t *y) = 0;

  /* [noscript,notxpcom] void getViewerBounds (in nsIntRect bounds); */
  NS_IMETHOD_(void) GetViewerBounds(nsIntRect & bounds) = 0;

  /* [noscript,notxpcom] void setViewerBounds ([const] in nsIntRect bounds); */
  NS_IMETHOD_(void) SetViewerBounds(const nsIntRect & bounds) = 0;

  /* [noscript,notxpcom] void addChildShell (in nsIDocShellTreeItem shell); */
  NS_IMETHOD_(void) AddChildShell(nsIDocShellTreeItem *shell) = 0;

  /* [noscript] nsIDocShellTreeItem childShellAt (in long index); */
  NS_IMETHOD ChildShellAt(int32_t index, nsIDocShellTreeItem **_retval) = 0;

  /* [noscript,notxpcom] void clearChildShells (); */
  NS_IMETHOD_(void) ClearChildShells(void) = 0;

  /* [noscript,notxpcom] void syncPresentationState (); */
  NS_IMETHOD_(void) SyncPresentationState(void) = 0;

  /* nsILayoutHistoryState initLayoutHistoryState (); */
  NS_IMETHOD InitLayoutHistoryState(nsILayoutHistoryState **_retval) = 0;

  /* [noscript] void create (in nsIURI URI, in AString title, in nsIInputStream inputStream, in unsigned long cacheKey, in ACString contentType, in nsIPrincipal triggeringPrincipal, in nsIPrincipal principalToInherit, in nsIPrincipal partitionedPrincipalToInherit, in nsIContentSecurityPolicy aCsp, in nsIDRef docshellID, in boolean dynamicCreation, in nsIURI originalURI, in nsIURI resultPrincipalURI, in bool loadReplace, in nsIReferrerInfo referrerInfo, in AString srcdoc, in bool srcdocEntry, in nsIURI baseURI, in bool saveLayoutState, in bool expired); */
  NS_IMETHOD Create(nsIURI *URI, const nsAString& title, nsIInputStream *inputStream, uint32_t cacheKey, const nsACString& contentType, nsIPrincipal *triggeringPrincipal, nsIPrincipal *principalToInherit, nsIPrincipal *partitionedPrincipalToInherit, nsIContentSecurityPolicy *aCsp, const nsID & docshellID, bool dynamicCreation, nsIURI *originalURI, nsIURI *resultPrincipalURI, bool loadReplace, nsIReferrerInfo *referrerInfo, const nsAString& srcdoc, bool srcdocEntry, nsIURI *baseURI, bool saveLayoutState, bool expired) = 0;

  /* nsISHEntry clone (); */
  NS_IMETHOD Clone(nsISHEntry **_retval) = 0;

  /* [noscript,notxpcom] nsDocShellEditorDataPtr forgetEditorData (); */
  NS_IMETHOD_(nsDocShellEditorData *) ForgetEditorData(void) = 0;

  /* [noscript,notxpcom] void setEditorData (in nsDocShellEditorDataPtr aData); */
  NS_IMETHOD_(void) SetEditorData(nsDocShellEditorData * aData) = 0;

  /* [noscript,notxpcom] boolean hasDetachedEditor (); */
  NS_IMETHOD_(bool) HasDetachedEditor(void) = 0;

  /* [noscript,notxpcom] boolean isDynamicallyAdded (); */
  NS_IMETHOD_(bool) IsDynamicallyAdded(void) = 0;

  /* boolean hasDynamicallyAddedChild (); */
  NS_IMETHOD HasDynamicallyAddedChild(bool *_retval) = 0;

  /* [noscript,notxpcom] boolean hasBFCacheEntry (in nsIBFCacheEntry aEntry); */
  NS_IMETHOD_(bool) HasBFCacheEntry(nsIBFCacheEntry *aEntry) = 0;

  /* void adoptBFCacheEntry (in nsISHEntry aEntry); */
  NS_IMETHOD AdoptBFCacheEntry(nsISHEntry *aEntry) = 0;

  /* void abandonBFCacheEntry (); */
  NS_IMETHOD AbandonBFCacheEntry(void) = 0;

  /* boolean sharesDocumentWith (in nsISHEntry aEntry); */
  NS_IMETHOD SharesDocumentWith(nsISHEntry *aEntry, bool *_retval) = 0;

  /* void setLoadTypeAsHistory (); */
  NS_IMETHOD SetLoadTypeAsHistory(void) = 0;

  /* void AddChild (in nsISHEntry aChild, in long aOffset, [optional, default (false)] in bool aUseRemoteSubframes); */
  NS_IMETHOD AddChild(nsISHEntry *aChild, int32_t aOffset, bool aUseRemoteSubframes = false) = 0;

  /* [noscript] void RemoveChild (in nsISHEntry aChild); */
  NS_IMETHOD RemoveChild(nsISHEntry *aChild) = 0;

  /* nsISHEntry GetChildAt (in long aIndex); */
  NS_IMETHOD GetChildAt(int32_t aIndex, nsISHEntry **_retval) = 0;

  /* [notxpcom] void GetChildSHEntryIfHasNoDynamicallyAddedChild (in long aChildOffset, out nsISHEntry aChild); */
  NS_IMETHOD_(void) GetChildSHEntryIfHasNoDynamicallyAddedChild(int32_t aChildOffset, nsISHEntry **aChild) = 0;

  /* [noscript] void ReplaceChild (in nsISHEntry aNewChild); */
  NS_IMETHOD ReplaceChild(nsISHEntry *aNewChild) = 0;

  /* [notxpcom] void ClearEntry (); */
  NS_IMETHOD_(void) ClearEntry(void) = 0;

  /* [noscript] nsDocShellLoadStatePtr CreateLoadInfo (); */
  NS_IMETHOD CreateLoadInfo(nsDocShellLoadState * * _retval) = 0;

  /* [infallible] readonly attribute unsigned long long bfcacheID; */
  NS_IMETHOD GetBfcacheID(uint64_t *aBfcacheID) = 0;
  inline uint64_t  GetBfcacheID()
  {
    uint64_t result;
    mozilla::DebugOnly<nsresult> rv = GetBfcacheID(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [notxpcom] void SyncTreesForSubframeNavigation (in nsISHEntry aEntry, in BrowsingContext aTopBC, in BrowsingContext aIgnoreBC); */
  NS_IMETHOD_(void) SyncTreesForSubframeNavigation(nsISHEntry *aEntry, mozilla::dom::BrowsingContext *aTopBC, mozilla::dom::BrowsingContext *aIgnoreBC) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISHEntry, NS_ISHENTRY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISHENTRY \
  using nsISHEntry::GetURI; \
  NS_IMETHOD GetURI(nsIURI **aURI) override; \
  NS_IMETHOD SetURI(nsIURI *aURI) override; \
  using nsISHEntry::GetOriginalURI; \
  NS_IMETHOD GetOriginalURI(nsIURI **aOriginalURI) override; \
  NS_IMETHOD SetOriginalURI(nsIURI *aOriginalURI) override; \
  using nsISHEntry::GetResultPrincipalURI; \
  NS_IMETHOD GetResultPrincipalURI(nsIURI **aResultPrincipalURI) override; \
  NS_IMETHOD SetResultPrincipalURI(nsIURI *aResultPrincipalURI) override; \
  using nsISHEntry::GetLoadReplace; \
  NS_IMETHOD GetLoadReplace(bool *aLoadReplace) override; \
  NS_IMETHOD SetLoadReplace(bool aLoadReplace) override; \
  NS_IMETHOD GetTitle(nsAString& aTitle) override; \
  NS_IMETHOD SetTitle(const nsAString& aTitle) override; \
  NS_IMETHOD GetName(nsAString& aName) override; \
  NS_IMETHOD SetName(const nsAString& aName) override; \
  using nsISHEntry::GetIsSubFrame; \
  NS_IMETHOD GetIsSubFrame(bool *aIsSubFrame) override; \
  NS_IMETHOD SetIsSubFrame(bool aIsSubFrame) override; \
  using nsISHEntry::GetHasUserInteraction; \
  NS_IMETHOD GetHasUserInteraction(bool *aHasUserInteraction) override; \
  NS_IMETHOD SetHasUserInteraction(bool aHasUserInteraction) override; \
  using nsISHEntry::GetReferrerInfo; \
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override; \
  NS_IMETHOD SetReferrerInfo(nsIReferrerInfo *aReferrerInfo) override; \
  using nsISHEntry::GetContentViewer; \
  NS_IMETHOD GetContentViewer(nsIContentViewer **aContentViewer) override; \
  NS_IMETHOD SetContentViewer(nsIContentViewer *aContentViewer) override; \
  using nsISHEntry::GetSticky; \
  NS_IMETHOD GetSticky(bool *aSticky) override; \
  NS_IMETHOD SetSticky(bool aSticky) override; \
  using nsISHEntry::GetWindowState; \
  NS_IMETHOD GetWindowState(nsISupports **aWindowState) override; \
  NS_IMETHOD SetWindowState(nsISupports *aWindowState) override; \
  using nsISHEntry::GetRefreshURIList; \
  NS_IMETHOD GetRefreshURIList(nsIMutableArray **aRefreshURIList) override; \
  NS_IMETHOD SetRefreshURIList(nsIMutableArray *aRefreshURIList) override; \
  using nsISHEntry::GetPostData; \
  NS_IMETHOD GetPostData(nsIInputStream **aPostData) override; \
  NS_IMETHOD SetPostData(nsIInputStream *aPostData) override; \
  using nsISHEntry::GetLayoutHistoryState; \
  NS_IMETHOD GetLayoutHistoryState(nsILayoutHistoryState **aLayoutHistoryState) override; \
  NS_IMETHOD SetLayoutHistoryState(nsILayoutHistoryState *aLayoutHistoryState) override; \
  using nsISHEntry::GetParent; \
  NS_IMETHOD GetParent(nsISHEntry **aParent) override; \
  NS_IMETHOD SetParent(nsISHEntry *aParent) override; \
  using nsISHEntry::GetLoadType; \
  NS_IMETHOD GetLoadType(uint32_t *aLoadType) override; \
  NS_IMETHOD SetLoadType(uint32_t aLoadType) override; \
  using nsISHEntry::GetID; \
  NS_IMETHOD GetID(uint32_t *aID) override; \
  NS_IMETHOD SetID(uint32_t aID) override; \
  using nsISHEntry::GetCacheKey; \
  NS_IMETHOD GetCacheKey(uint32_t *aCacheKey) override; \
  NS_IMETHOD SetCacheKey(uint32_t aCacheKey) override; \
  using nsISHEntry::GetSaveLayoutStateFlag; \
  NS_IMETHOD GetSaveLayoutStateFlag(bool *aSaveLayoutStateFlag) override; \
  NS_IMETHOD SetSaveLayoutStateFlag(bool aSaveLayoutStateFlag) override; \
  NS_IMETHOD GetContentType(nsACString& aContentType) override; \
  NS_IMETHOD SetContentType(const nsACString& aContentType) override; \
  using nsISHEntry::GetURIWasModified; \
  NS_IMETHOD GetURIWasModified(bool *aURIWasModified) override; \
  NS_IMETHOD SetURIWasModified(bool aURIWasModified) override; \
  using nsISHEntry::GetTriggeringPrincipal; \
  NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) override; \
  NS_IMETHOD SetTriggeringPrincipal(nsIPrincipal *aTriggeringPrincipal) override; \
  using nsISHEntry::GetPrincipalToInherit; \
  NS_IMETHOD GetPrincipalToInherit(nsIPrincipal **aPrincipalToInherit) override; \
  NS_IMETHOD SetPrincipalToInherit(nsIPrincipal *aPrincipalToInherit) override; \
  using nsISHEntry::GetPartitionedPrincipalToInherit; \
  NS_IMETHOD GetPartitionedPrincipalToInherit(nsIPrincipal **aPartitionedPrincipalToInherit) override; \
  NS_IMETHOD SetPartitionedPrincipalToInherit(nsIPrincipal *aPartitionedPrincipalToInherit) override; \
  using nsISHEntry::GetCsp; \
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) override; \
  NS_IMETHOD SetCsp(nsIContentSecurityPolicy *aCsp) override; \
  using nsISHEntry::GetStateData; \
  NS_IMETHOD GetStateData(nsIStructuredCloneContainer **aStateData) override; \
  NS_IMETHOD SetStateData(nsIStructuredCloneContainer *aStateData) override; \
  NS_IMETHOD GetDocshellID(nsID & aDocshellID) override; \
  NS_IMETHOD SetDocshellID(const nsID & aDocshellID) override; \
  using nsISHEntry::GetIsSrcdocEntry; \
  NS_IMETHOD GetIsSrcdocEntry(bool *aIsSrcdocEntry) override; \
  NS_IMETHOD GetSrcdocData(nsAString& aSrcdocData) override; \
  NS_IMETHOD SetSrcdocData(const nsAString& aSrcdocData) override; \
  using nsISHEntry::GetBaseURI; \
  NS_IMETHOD GetBaseURI(nsIURI **aBaseURI) override; \
  NS_IMETHOD SetBaseURI(nsIURI *aBaseURI) override; \
  using nsISHEntry::GetScrollRestorationIsManual; \
  NS_IMETHOD GetScrollRestorationIsManual(bool *aScrollRestorationIsManual) override; \
  NS_IMETHOD SetScrollRestorationIsManual(bool aScrollRestorationIsManual) override; \
  using nsISHEntry::GetLoadedInThisProcess; \
  NS_IMETHOD GetLoadedInThisProcess(bool *aLoadedInThisProcess) override; \
  using nsISHEntry::GetShistory; \
  NS_IMETHOD GetShistory(nsISHistory **aShistory) override; \
  NS_IMETHOD SetShistory(nsISHistory *aShistory) override; \
  using nsISHEntry::GetLastTouched; \
  NS_IMETHOD GetLastTouched(uint32_t *aLastTouched) override; \
  NS_IMETHOD SetLastTouched(uint32_t aLastTouched) override; \
  using nsISHEntry::GetChildCount; \
  NS_IMETHOD GetChildCount(int32_t *aChildCount) override; \
  using nsISHEntry::GetPersist; \
  NS_IMETHOD GetPersist(bool *aPersist) override; \
  NS_IMETHOD SetPersist(bool aPersist) override; \
  NS_IMETHOD SetScrollPosition(int32_t x, int32_t y) override; \
  NS_IMETHOD GetScrollPosition(int32_t *x, int32_t *y) override; \
  NS_IMETHOD_(void) GetViewerBounds(nsIntRect & bounds) override; \
  NS_IMETHOD_(void) SetViewerBounds(const nsIntRect & bounds) override; \
  NS_IMETHOD_(void) AddChildShell(nsIDocShellTreeItem *shell) override; \
  NS_IMETHOD ChildShellAt(int32_t index, nsIDocShellTreeItem **_retval) override; \
  NS_IMETHOD_(void) ClearChildShells(void) override; \
  NS_IMETHOD_(void) SyncPresentationState(void) override; \
  NS_IMETHOD InitLayoutHistoryState(nsILayoutHistoryState **_retval) override; \
  NS_IMETHOD Create(nsIURI *URI, const nsAString& title, nsIInputStream *inputStream, uint32_t cacheKey, const nsACString& contentType, nsIPrincipal *triggeringPrincipal, nsIPrincipal *principalToInherit, nsIPrincipal *partitionedPrincipalToInherit, nsIContentSecurityPolicy *aCsp, const nsID & docshellID, bool dynamicCreation, nsIURI *originalURI, nsIURI *resultPrincipalURI, bool loadReplace, nsIReferrerInfo *referrerInfo, const nsAString& srcdoc, bool srcdocEntry, nsIURI *baseURI, bool saveLayoutState, bool expired) override; \
  NS_IMETHOD Clone(nsISHEntry **_retval) override; \
  NS_IMETHOD_(nsDocShellEditorData *) ForgetEditorData(void) override; \
  NS_IMETHOD_(void) SetEditorData(nsDocShellEditorData * aData) override; \
  NS_IMETHOD_(bool) HasDetachedEditor(void) override; \
  NS_IMETHOD_(bool) IsDynamicallyAdded(void) override; \
  NS_IMETHOD HasDynamicallyAddedChild(bool *_retval) override; \
  NS_IMETHOD_(bool) HasBFCacheEntry(nsIBFCacheEntry *aEntry) override; \
  NS_IMETHOD AdoptBFCacheEntry(nsISHEntry *aEntry) override; \
  NS_IMETHOD AbandonBFCacheEntry(void) override; \
  NS_IMETHOD SharesDocumentWith(nsISHEntry *aEntry, bool *_retval) override; \
  NS_IMETHOD SetLoadTypeAsHistory(void) override; \
  NS_IMETHOD AddChild(nsISHEntry *aChild, int32_t aOffset, bool aUseRemoteSubframes = false) override; \
  NS_IMETHOD RemoveChild(nsISHEntry *aChild) override; \
  NS_IMETHOD GetChildAt(int32_t aIndex, nsISHEntry **_retval) override; \
  NS_IMETHOD_(void) GetChildSHEntryIfHasNoDynamicallyAddedChild(int32_t aChildOffset, nsISHEntry **aChild) override; \
  NS_IMETHOD ReplaceChild(nsISHEntry *aNewChild) override; \
  NS_IMETHOD_(void) ClearEntry(void) override; \
  NS_IMETHOD CreateLoadInfo(nsDocShellLoadState * * _retval) override; \
  using nsISHEntry::GetBfcacheID; \
  NS_IMETHOD GetBfcacheID(uint64_t *aBfcacheID) override; \
  NS_IMETHOD_(void) SyncTreesForSubframeNavigation(nsISHEntry *aEntry, mozilla::dom::BrowsingContext *aTopBC, mozilla::dom::BrowsingContext *aIgnoreBC) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISHENTRY \
  using nsISHEntry::GetURI; \
  nsresult GetURI(nsIURI **aURI); \
  nsresult SetURI(nsIURI *aURI); \
  using nsISHEntry::GetOriginalURI; \
  nsresult GetOriginalURI(nsIURI **aOriginalURI); \
  nsresult SetOriginalURI(nsIURI *aOriginalURI); \
  using nsISHEntry::GetResultPrincipalURI; \
  nsresult GetResultPrincipalURI(nsIURI **aResultPrincipalURI); \
  nsresult SetResultPrincipalURI(nsIURI *aResultPrincipalURI); \
  using nsISHEntry::GetLoadReplace; \
  nsresult GetLoadReplace(bool *aLoadReplace); \
  nsresult SetLoadReplace(bool aLoadReplace); \
  nsresult GetTitle(nsAString& aTitle); \
  nsresult SetTitle(const nsAString& aTitle); \
  nsresult GetName(nsAString& aName); \
  nsresult SetName(const nsAString& aName); \
  using nsISHEntry::GetIsSubFrame; \
  nsresult GetIsSubFrame(bool *aIsSubFrame); \
  nsresult SetIsSubFrame(bool aIsSubFrame); \
  using nsISHEntry::GetHasUserInteraction; \
  nsresult GetHasUserInteraction(bool *aHasUserInteraction); \
  nsresult SetHasUserInteraction(bool aHasUserInteraction); \
  using nsISHEntry::GetReferrerInfo; \
  nsresult GetReferrerInfo(nsIReferrerInfo **aReferrerInfo); \
  nsresult SetReferrerInfo(nsIReferrerInfo *aReferrerInfo); \
  using nsISHEntry::GetContentViewer; \
  nsresult GetContentViewer(nsIContentViewer **aContentViewer); \
  nsresult SetContentViewer(nsIContentViewer *aContentViewer); \
  using nsISHEntry::GetSticky; \
  nsresult GetSticky(bool *aSticky); \
  nsresult SetSticky(bool aSticky); \
  using nsISHEntry::GetWindowState; \
  nsresult GetWindowState(nsISupports **aWindowState); \
  nsresult SetWindowState(nsISupports *aWindowState); \
  using nsISHEntry::GetRefreshURIList; \
  nsresult GetRefreshURIList(nsIMutableArray **aRefreshURIList); \
  nsresult SetRefreshURIList(nsIMutableArray *aRefreshURIList); \
  using nsISHEntry::GetPostData; \
  nsresult GetPostData(nsIInputStream **aPostData); \
  nsresult SetPostData(nsIInputStream *aPostData); \
  using nsISHEntry::GetLayoutHistoryState; \
  nsresult GetLayoutHistoryState(nsILayoutHistoryState **aLayoutHistoryState); \
  nsresult SetLayoutHistoryState(nsILayoutHistoryState *aLayoutHistoryState); \
  using nsISHEntry::GetParent; \
  nsresult GetParent(nsISHEntry **aParent); \
  nsresult SetParent(nsISHEntry *aParent); \
  using nsISHEntry::GetLoadType; \
  nsresult GetLoadType(uint32_t *aLoadType); \
  nsresult SetLoadType(uint32_t aLoadType); \
  using nsISHEntry::GetID; \
  nsresult GetID(uint32_t *aID); \
  nsresult SetID(uint32_t aID); \
  using nsISHEntry::GetCacheKey; \
  nsresult GetCacheKey(uint32_t *aCacheKey); \
  nsresult SetCacheKey(uint32_t aCacheKey); \
  using nsISHEntry::GetSaveLayoutStateFlag; \
  nsresult GetSaveLayoutStateFlag(bool *aSaveLayoutStateFlag); \
  nsresult SetSaveLayoutStateFlag(bool aSaveLayoutStateFlag); \
  nsresult GetContentType(nsACString& aContentType); \
  nsresult SetContentType(const nsACString& aContentType); \
  using nsISHEntry::GetURIWasModified; \
  nsresult GetURIWasModified(bool *aURIWasModified); \
  nsresult SetURIWasModified(bool aURIWasModified); \
  using nsISHEntry::GetTriggeringPrincipal; \
  nsresult GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal); \
  nsresult SetTriggeringPrincipal(nsIPrincipal *aTriggeringPrincipal); \
  using nsISHEntry::GetPrincipalToInherit; \
  nsresult GetPrincipalToInherit(nsIPrincipal **aPrincipalToInherit); \
  nsresult SetPrincipalToInherit(nsIPrincipal *aPrincipalToInherit); \
  using nsISHEntry::GetPartitionedPrincipalToInherit; \
  nsresult GetPartitionedPrincipalToInherit(nsIPrincipal **aPartitionedPrincipalToInherit); \
  nsresult SetPartitionedPrincipalToInherit(nsIPrincipal *aPartitionedPrincipalToInherit); \
  using nsISHEntry::GetCsp; \
  nsresult GetCsp(nsIContentSecurityPolicy **aCsp); \
  nsresult SetCsp(nsIContentSecurityPolicy *aCsp); \
  using nsISHEntry::GetStateData; \
  nsresult GetStateData(nsIStructuredCloneContainer **aStateData); \
  nsresult SetStateData(nsIStructuredCloneContainer *aStateData); \
  nsresult GetDocshellID(nsID & aDocshellID); \
  nsresult SetDocshellID(const nsID & aDocshellID); \
  using nsISHEntry::GetIsSrcdocEntry; \
  nsresult GetIsSrcdocEntry(bool *aIsSrcdocEntry); \
  nsresult GetSrcdocData(nsAString& aSrcdocData); \
  nsresult SetSrcdocData(const nsAString& aSrcdocData); \
  using nsISHEntry::GetBaseURI; \
  nsresult GetBaseURI(nsIURI **aBaseURI); \
  nsresult SetBaseURI(nsIURI *aBaseURI); \
  using nsISHEntry::GetScrollRestorationIsManual; \
  nsresult GetScrollRestorationIsManual(bool *aScrollRestorationIsManual); \
  nsresult SetScrollRestorationIsManual(bool aScrollRestorationIsManual); \
  using nsISHEntry::GetLoadedInThisProcess; \
  nsresult GetLoadedInThisProcess(bool *aLoadedInThisProcess); \
  using nsISHEntry::GetShistory; \
  nsresult GetShistory(nsISHistory **aShistory); \
  nsresult SetShistory(nsISHistory *aShistory); \
  using nsISHEntry::GetLastTouched; \
  nsresult GetLastTouched(uint32_t *aLastTouched); \
  nsresult SetLastTouched(uint32_t aLastTouched); \
  using nsISHEntry::GetChildCount; \
  nsresult GetChildCount(int32_t *aChildCount); \
  using nsISHEntry::GetPersist; \
  nsresult GetPersist(bool *aPersist); \
  nsresult SetPersist(bool aPersist); \
  nsresult SetScrollPosition(int32_t x, int32_t y); \
  nsresult GetScrollPosition(int32_t *x, int32_t *y); \
  nsresult_(void) GetViewerBounds(nsIntRect & bounds); \
  nsresult_(void) SetViewerBounds(const nsIntRect & bounds); \
  nsresult_(void) AddChildShell(nsIDocShellTreeItem *shell); \
  nsresult ChildShellAt(int32_t index, nsIDocShellTreeItem **_retval); \
  nsresult_(void) ClearChildShells(void); \
  nsresult_(void) SyncPresentationState(void); \
  nsresult InitLayoutHistoryState(nsILayoutHistoryState **_retval); \
  nsresult Create(nsIURI *URI, const nsAString& title, nsIInputStream *inputStream, uint32_t cacheKey, const nsACString& contentType, nsIPrincipal *triggeringPrincipal, nsIPrincipal *principalToInherit, nsIPrincipal *partitionedPrincipalToInherit, nsIContentSecurityPolicy *aCsp, const nsID & docshellID, bool dynamicCreation, nsIURI *originalURI, nsIURI *resultPrincipalURI, bool loadReplace, nsIReferrerInfo *referrerInfo, const nsAString& srcdoc, bool srcdocEntry, nsIURI *baseURI, bool saveLayoutState, bool expired); \
  nsresult Clone(nsISHEntry **_retval); \
  nsresult_(nsDocShellEditorData *) ForgetEditorData(void); \
  nsresult_(void) SetEditorData(nsDocShellEditorData * aData); \
  nsresult_(bool) HasDetachedEditor(void); \
  nsresult_(bool) IsDynamicallyAdded(void); \
  nsresult HasDynamicallyAddedChild(bool *_retval); \
  nsresult_(bool) HasBFCacheEntry(nsIBFCacheEntry *aEntry); \
  nsresult AdoptBFCacheEntry(nsISHEntry *aEntry); \
  nsresult AbandonBFCacheEntry(void); \
  nsresult SharesDocumentWith(nsISHEntry *aEntry, bool *_retval); \
  nsresult SetLoadTypeAsHistory(void); \
  nsresult AddChild(nsISHEntry *aChild, int32_t aOffset, bool aUseRemoteSubframes = false); \
  nsresult RemoveChild(nsISHEntry *aChild); \
  nsresult GetChildAt(int32_t aIndex, nsISHEntry **_retval); \
  nsresult_(void) GetChildSHEntryIfHasNoDynamicallyAddedChild(int32_t aChildOffset, nsISHEntry **aChild); \
  nsresult ReplaceChild(nsISHEntry *aNewChild); \
  nsresult_(void) ClearEntry(void); \
  nsresult CreateLoadInfo(nsDocShellLoadState * * _retval); \
  using nsISHEntry::GetBfcacheID; \
  nsresult GetBfcacheID(uint64_t *aBfcacheID); \
  nsresult_(void) SyncTreesForSubframeNavigation(nsISHEntry *aEntry, mozilla::dom::BrowsingContext *aTopBC, mozilla::dom::BrowsingContext *aIgnoreBC); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISHENTRY(_to) \
  using nsISHEntry::GetURI; \
  NS_IMETHOD GetURI(nsIURI **aURI) override { return _to GetURI(aURI); } \
  NS_IMETHOD SetURI(nsIURI *aURI) override { return _to SetURI(aURI); } \
  using nsISHEntry::GetOriginalURI; \
  NS_IMETHOD GetOriginalURI(nsIURI **aOriginalURI) override { return _to GetOriginalURI(aOriginalURI); } \
  NS_IMETHOD SetOriginalURI(nsIURI *aOriginalURI) override { return _to SetOriginalURI(aOriginalURI); } \
  using nsISHEntry::GetResultPrincipalURI; \
  NS_IMETHOD GetResultPrincipalURI(nsIURI **aResultPrincipalURI) override { return _to GetResultPrincipalURI(aResultPrincipalURI); } \
  NS_IMETHOD SetResultPrincipalURI(nsIURI *aResultPrincipalURI) override { return _to SetResultPrincipalURI(aResultPrincipalURI); } \
  using nsISHEntry::GetLoadReplace; \
  NS_IMETHOD GetLoadReplace(bool *aLoadReplace) override { return _to GetLoadReplace(aLoadReplace); } \
  NS_IMETHOD SetLoadReplace(bool aLoadReplace) override { return _to SetLoadReplace(aLoadReplace); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return _to GetTitle(aTitle); } \
  NS_IMETHOD SetTitle(const nsAString& aTitle) override { return _to SetTitle(aTitle); } \
  NS_IMETHOD GetName(nsAString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD SetName(const nsAString& aName) override { return _to SetName(aName); } \
  using nsISHEntry::GetIsSubFrame; \
  NS_IMETHOD GetIsSubFrame(bool *aIsSubFrame) override { return _to GetIsSubFrame(aIsSubFrame); } \
  NS_IMETHOD SetIsSubFrame(bool aIsSubFrame) override { return _to SetIsSubFrame(aIsSubFrame); } \
  using nsISHEntry::GetHasUserInteraction; \
  NS_IMETHOD GetHasUserInteraction(bool *aHasUserInteraction) override { return _to GetHasUserInteraction(aHasUserInteraction); } \
  NS_IMETHOD SetHasUserInteraction(bool aHasUserInteraction) override { return _to SetHasUserInteraction(aHasUserInteraction); } \
  using nsISHEntry::GetReferrerInfo; \
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override { return _to GetReferrerInfo(aReferrerInfo); } \
  NS_IMETHOD SetReferrerInfo(nsIReferrerInfo *aReferrerInfo) override { return _to SetReferrerInfo(aReferrerInfo); } \
  using nsISHEntry::GetContentViewer; \
  NS_IMETHOD GetContentViewer(nsIContentViewer **aContentViewer) override { return _to GetContentViewer(aContentViewer); } \
  NS_IMETHOD SetContentViewer(nsIContentViewer *aContentViewer) override { return _to SetContentViewer(aContentViewer); } \
  using nsISHEntry::GetSticky; \
  NS_IMETHOD GetSticky(bool *aSticky) override { return _to GetSticky(aSticky); } \
  NS_IMETHOD SetSticky(bool aSticky) override { return _to SetSticky(aSticky); } \
  using nsISHEntry::GetWindowState; \
  NS_IMETHOD GetWindowState(nsISupports **aWindowState) override { return _to GetWindowState(aWindowState); } \
  NS_IMETHOD SetWindowState(nsISupports *aWindowState) override { return _to SetWindowState(aWindowState); } \
  using nsISHEntry::GetRefreshURIList; \
  NS_IMETHOD GetRefreshURIList(nsIMutableArray **aRefreshURIList) override { return _to GetRefreshURIList(aRefreshURIList); } \
  NS_IMETHOD SetRefreshURIList(nsIMutableArray *aRefreshURIList) override { return _to SetRefreshURIList(aRefreshURIList); } \
  using nsISHEntry::GetPostData; \
  NS_IMETHOD GetPostData(nsIInputStream **aPostData) override { return _to GetPostData(aPostData); } \
  NS_IMETHOD SetPostData(nsIInputStream *aPostData) override { return _to SetPostData(aPostData); } \
  using nsISHEntry::GetLayoutHistoryState; \
  NS_IMETHOD GetLayoutHistoryState(nsILayoutHistoryState **aLayoutHistoryState) override { return _to GetLayoutHistoryState(aLayoutHistoryState); } \
  NS_IMETHOD SetLayoutHistoryState(nsILayoutHistoryState *aLayoutHistoryState) override { return _to SetLayoutHistoryState(aLayoutHistoryState); } \
  using nsISHEntry::GetParent; \
  NS_IMETHOD GetParent(nsISHEntry **aParent) override { return _to GetParent(aParent); } \
  NS_IMETHOD SetParent(nsISHEntry *aParent) override { return _to SetParent(aParent); } \
  using nsISHEntry::GetLoadType; \
  NS_IMETHOD GetLoadType(uint32_t *aLoadType) override { return _to GetLoadType(aLoadType); } \
  NS_IMETHOD SetLoadType(uint32_t aLoadType) override { return _to SetLoadType(aLoadType); } \
  using nsISHEntry::GetID; \
  NS_IMETHOD GetID(uint32_t *aID) override { return _to GetID(aID); } \
  NS_IMETHOD SetID(uint32_t aID) override { return _to SetID(aID); } \
  using nsISHEntry::GetCacheKey; \
  NS_IMETHOD GetCacheKey(uint32_t *aCacheKey) override { return _to GetCacheKey(aCacheKey); } \
  NS_IMETHOD SetCacheKey(uint32_t aCacheKey) override { return _to SetCacheKey(aCacheKey); } \
  using nsISHEntry::GetSaveLayoutStateFlag; \
  NS_IMETHOD GetSaveLayoutStateFlag(bool *aSaveLayoutStateFlag) override { return _to GetSaveLayoutStateFlag(aSaveLayoutStateFlag); } \
  NS_IMETHOD SetSaveLayoutStateFlag(bool aSaveLayoutStateFlag) override { return _to SetSaveLayoutStateFlag(aSaveLayoutStateFlag); } \
  NS_IMETHOD GetContentType(nsACString& aContentType) override { return _to GetContentType(aContentType); } \
  NS_IMETHOD SetContentType(const nsACString& aContentType) override { return _to SetContentType(aContentType); } \
  using nsISHEntry::GetURIWasModified; \
  NS_IMETHOD GetURIWasModified(bool *aURIWasModified) override { return _to GetURIWasModified(aURIWasModified); } \
  NS_IMETHOD SetURIWasModified(bool aURIWasModified) override { return _to SetURIWasModified(aURIWasModified); } \
  using nsISHEntry::GetTriggeringPrincipal; \
  NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) override { return _to GetTriggeringPrincipal(aTriggeringPrincipal); } \
  NS_IMETHOD SetTriggeringPrincipal(nsIPrincipal *aTriggeringPrincipal) override { return _to SetTriggeringPrincipal(aTriggeringPrincipal); } \
  using nsISHEntry::GetPrincipalToInherit; \
  NS_IMETHOD GetPrincipalToInherit(nsIPrincipal **aPrincipalToInherit) override { return _to GetPrincipalToInherit(aPrincipalToInherit); } \
  NS_IMETHOD SetPrincipalToInherit(nsIPrincipal *aPrincipalToInherit) override { return _to SetPrincipalToInherit(aPrincipalToInherit); } \
  using nsISHEntry::GetPartitionedPrincipalToInherit; \
  NS_IMETHOD GetPartitionedPrincipalToInherit(nsIPrincipal **aPartitionedPrincipalToInherit) override { return _to GetPartitionedPrincipalToInherit(aPartitionedPrincipalToInherit); } \
  NS_IMETHOD SetPartitionedPrincipalToInherit(nsIPrincipal *aPartitionedPrincipalToInherit) override { return _to SetPartitionedPrincipalToInherit(aPartitionedPrincipalToInherit); } \
  using nsISHEntry::GetCsp; \
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) override { return _to GetCsp(aCsp); } \
  NS_IMETHOD SetCsp(nsIContentSecurityPolicy *aCsp) override { return _to SetCsp(aCsp); } \
  using nsISHEntry::GetStateData; \
  NS_IMETHOD GetStateData(nsIStructuredCloneContainer **aStateData) override { return _to GetStateData(aStateData); } \
  NS_IMETHOD SetStateData(nsIStructuredCloneContainer *aStateData) override { return _to SetStateData(aStateData); } \
  NS_IMETHOD GetDocshellID(nsID & aDocshellID) override { return _to GetDocshellID(aDocshellID); } \
  NS_IMETHOD SetDocshellID(const nsID & aDocshellID) override { return _to SetDocshellID(aDocshellID); } \
  using nsISHEntry::GetIsSrcdocEntry; \
  NS_IMETHOD GetIsSrcdocEntry(bool *aIsSrcdocEntry) override { return _to GetIsSrcdocEntry(aIsSrcdocEntry); } \
  NS_IMETHOD GetSrcdocData(nsAString& aSrcdocData) override { return _to GetSrcdocData(aSrcdocData); } \
  NS_IMETHOD SetSrcdocData(const nsAString& aSrcdocData) override { return _to SetSrcdocData(aSrcdocData); } \
  using nsISHEntry::GetBaseURI; \
  NS_IMETHOD GetBaseURI(nsIURI **aBaseURI) override { return _to GetBaseURI(aBaseURI); } \
  NS_IMETHOD SetBaseURI(nsIURI *aBaseURI) override { return _to SetBaseURI(aBaseURI); } \
  using nsISHEntry::GetScrollRestorationIsManual; \
  NS_IMETHOD GetScrollRestorationIsManual(bool *aScrollRestorationIsManual) override { return _to GetScrollRestorationIsManual(aScrollRestorationIsManual); } \
  NS_IMETHOD SetScrollRestorationIsManual(bool aScrollRestorationIsManual) override { return _to SetScrollRestorationIsManual(aScrollRestorationIsManual); } \
  using nsISHEntry::GetLoadedInThisProcess; \
  NS_IMETHOD GetLoadedInThisProcess(bool *aLoadedInThisProcess) override { return _to GetLoadedInThisProcess(aLoadedInThisProcess); } \
  using nsISHEntry::GetShistory; \
  NS_IMETHOD GetShistory(nsISHistory **aShistory) override { return _to GetShistory(aShistory); } \
  NS_IMETHOD SetShistory(nsISHistory *aShistory) override { return _to SetShistory(aShistory); } \
  using nsISHEntry::GetLastTouched; \
  NS_IMETHOD GetLastTouched(uint32_t *aLastTouched) override { return _to GetLastTouched(aLastTouched); } \
  NS_IMETHOD SetLastTouched(uint32_t aLastTouched) override { return _to SetLastTouched(aLastTouched); } \
  using nsISHEntry::GetChildCount; \
  NS_IMETHOD GetChildCount(int32_t *aChildCount) override { return _to GetChildCount(aChildCount); } \
  using nsISHEntry::GetPersist; \
  NS_IMETHOD GetPersist(bool *aPersist) override { return _to GetPersist(aPersist); } \
  NS_IMETHOD SetPersist(bool aPersist) override { return _to SetPersist(aPersist); } \
  NS_IMETHOD SetScrollPosition(int32_t x, int32_t y) override { return _to SetScrollPosition(x, y); } \
  NS_IMETHOD GetScrollPosition(int32_t *x, int32_t *y) override { return _to GetScrollPosition(x, y); } \
  NS_IMETHOD_(void) GetViewerBounds(nsIntRect & bounds) override { return _to GetViewerBounds(bounds); } \
  NS_IMETHOD_(void) SetViewerBounds(const nsIntRect & bounds) override { return _to SetViewerBounds(bounds); } \
  NS_IMETHOD_(void) AddChildShell(nsIDocShellTreeItem *shell) override { return _to AddChildShell(shell); } \
  NS_IMETHOD ChildShellAt(int32_t index, nsIDocShellTreeItem **_retval) override { return _to ChildShellAt(index, _retval); } \
  NS_IMETHOD_(void) ClearChildShells(void) override { return _to ClearChildShells(); } \
  NS_IMETHOD_(void) SyncPresentationState(void) override { return _to SyncPresentationState(); } \
  NS_IMETHOD InitLayoutHistoryState(nsILayoutHistoryState **_retval) override { return _to InitLayoutHistoryState(_retval); } \
  NS_IMETHOD Create(nsIURI *URI, const nsAString& title, nsIInputStream *inputStream, uint32_t cacheKey, const nsACString& contentType, nsIPrincipal *triggeringPrincipal, nsIPrincipal *principalToInherit, nsIPrincipal *partitionedPrincipalToInherit, nsIContentSecurityPolicy *aCsp, const nsID & docshellID, bool dynamicCreation, nsIURI *originalURI, nsIURI *resultPrincipalURI, bool loadReplace, nsIReferrerInfo *referrerInfo, const nsAString& srcdoc, bool srcdocEntry, nsIURI *baseURI, bool saveLayoutState, bool expired) override { return _to Create(URI, title, inputStream, cacheKey, contentType, triggeringPrincipal, principalToInherit, partitionedPrincipalToInherit, aCsp, docshellID, dynamicCreation, originalURI, resultPrincipalURI, loadReplace, referrerInfo, srcdoc, srcdocEntry, baseURI, saveLayoutState, expired); } \
  NS_IMETHOD Clone(nsISHEntry **_retval) override { return _to Clone(_retval); } \
  NS_IMETHOD_(nsDocShellEditorData *) ForgetEditorData(void) override { return _to ForgetEditorData(); } \
  NS_IMETHOD_(void) SetEditorData(nsDocShellEditorData * aData) override { return _to SetEditorData(aData); } \
  NS_IMETHOD_(bool) HasDetachedEditor(void) override { return _to HasDetachedEditor(); } \
  NS_IMETHOD_(bool) IsDynamicallyAdded(void) override { return _to IsDynamicallyAdded(); } \
  NS_IMETHOD HasDynamicallyAddedChild(bool *_retval) override { return _to HasDynamicallyAddedChild(_retval); } \
  NS_IMETHOD_(bool) HasBFCacheEntry(nsIBFCacheEntry *aEntry) override { return _to HasBFCacheEntry(aEntry); } \
  NS_IMETHOD AdoptBFCacheEntry(nsISHEntry *aEntry) override { return _to AdoptBFCacheEntry(aEntry); } \
  NS_IMETHOD AbandonBFCacheEntry(void) override { return _to AbandonBFCacheEntry(); } \
  NS_IMETHOD SharesDocumentWith(nsISHEntry *aEntry, bool *_retval) override { return _to SharesDocumentWith(aEntry, _retval); } \
  NS_IMETHOD SetLoadTypeAsHistory(void) override { return _to SetLoadTypeAsHistory(); } \
  NS_IMETHOD AddChild(nsISHEntry *aChild, int32_t aOffset, bool aUseRemoteSubframes = false) override { return _to AddChild(aChild, aOffset, aUseRemoteSubframes); } \
  NS_IMETHOD RemoveChild(nsISHEntry *aChild) override { return _to RemoveChild(aChild); } \
  NS_IMETHOD GetChildAt(int32_t aIndex, nsISHEntry **_retval) override { return _to GetChildAt(aIndex, _retval); } \
  NS_IMETHOD_(void) GetChildSHEntryIfHasNoDynamicallyAddedChild(int32_t aChildOffset, nsISHEntry **aChild) override { return _to GetChildSHEntryIfHasNoDynamicallyAddedChild(aChildOffset, aChild); } \
  NS_IMETHOD ReplaceChild(nsISHEntry *aNewChild) override { return _to ReplaceChild(aNewChild); } \
  NS_IMETHOD_(void) ClearEntry(void) override { return _to ClearEntry(); } \
  NS_IMETHOD CreateLoadInfo(nsDocShellLoadState * * _retval) override { return _to CreateLoadInfo(_retval); } \
  using nsISHEntry::GetBfcacheID; \
  NS_IMETHOD GetBfcacheID(uint64_t *aBfcacheID) override { return _to GetBfcacheID(aBfcacheID); } \
  NS_IMETHOD_(void) SyncTreesForSubframeNavigation(nsISHEntry *aEntry, mozilla::dom::BrowsingContext *aTopBC, mozilla::dom::BrowsingContext *aIgnoreBC) override { return _to SyncTreesForSubframeNavigation(aEntry, aTopBC, aIgnoreBC); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISHENTRY(_to) \
  NS_IMETHOD GetURI(nsIURI **aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURI(aURI); } \
  NS_IMETHOD SetURI(nsIURI *aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetURI(aURI); } \
  NS_IMETHOD GetOriginalURI(nsIURI **aOriginalURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginalURI(aOriginalURI); } \
  NS_IMETHOD SetOriginalURI(nsIURI *aOriginalURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOriginalURI(aOriginalURI); } \
  NS_IMETHOD GetResultPrincipalURI(nsIURI **aResultPrincipalURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResultPrincipalURI(aResultPrincipalURI); } \
  NS_IMETHOD SetResultPrincipalURI(nsIURI *aResultPrincipalURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetResultPrincipalURI(aResultPrincipalURI); } \
  NS_IMETHOD GetLoadReplace(bool *aLoadReplace) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadReplace(aLoadReplace); } \
  NS_IMETHOD SetLoadReplace(bool aLoadReplace) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLoadReplace(aLoadReplace); } \
  NS_IMETHOD GetTitle(nsAString& aTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTitle(aTitle); } \
  NS_IMETHOD SetTitle(const nsAString& aTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTitle(aTitle); } \
  NS_IMETHOD GetName(nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD SetName(const nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetName(aName); } \
  NS_IMETHOD GetIsSubFrame(bool *aIsSubFrame) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsSubFrame(aIsSubFrame); } \
  NS_IMETHOD SetIsSubFrame(bool aIsSubFrame) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsSubFrame(aIsSubFrame); } \
  NS_IMETHOD GetHasUserInteraction(bool *aHasUserInteraction) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasUserInteraction(aHasUserInteraction); } \
  NS_IMETHOD SetHasUserInteraction(bool aHasUserInteraction) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHasUserInteraction(aHasUserInteraction); } \
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReferrerInfo(aReferrerInfo); } \
  NS_IMETHOD SetReferrerInfo(nsIReferrerInfo *aReferrerInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetReferrerInfo(aReferrerInfo); } \
  NS_IMETHOD GetContentViewer(nsIContentViewer **aContentViewer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentViewer(aContentViewer); } \
  NS_IMETHOD SetContentViewer(nsIContentViewer *aContentViewer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContentViewer(aContentViewer); } \
  NS_IMETHOD GetSticky(bool *aSticky) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSticky(aSticky); } \
  NS_IMETHOD SetSticky(bool aSticky) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSticky(aSticky); } \
  NS_IMETHOD GetWindowState(nsISupports **aWindowState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWindowState(aWindowState); } \
  NS_IMETHOD SetWindowState(nsISupports *aWindowState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetWindowState(aWindowState); } \
  NS_IMETHOD GetRefreshURIList(nsIMutableArray **aRefreshURIList) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRefreshURIList(aRefreshURIList); } \
  NS_IMETHOD SetRefreshURIList(nsIMutableArray *aRefreshURIList) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRefreshURIList(aRefreshURIList); } \
  NS_IMETHOD GetPostData(nsIInputStream **aPostData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPostData(aPostData); } \
  NS_IMETHOD SetPostData(nsIInputStream *aPostData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPostData(aPostData); } \
  NS_IMETHOD GetLayoutHistoryState(nsILayoutHistoryState **aLayoutHistoryState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLayoutHistoryState(aLayoutHistoryState); } \
  NS_IMETHOD SetLayoutHistoryState(nsILayoutHistoryState *aLayoutHistoryState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLayoutHistoryState(aLayoutHistoryState); } \
  NS_IMETHOD GetParent(nsISHEntry **aParent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParent(aParent); } \
  NS_IMETHOD SetParent(nsISHEntry *aParent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetParent(aParent); } \
  NS_IMETHOD GetLoadType(uint32_t *aLoadType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadType(aLoadType); } \
  NS_IMETHOD SetLoadType(uint32_t aLoadType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLoadType(aLoadType); } \
  NS_IMETHOD GetID(uint32_t *aID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetID(aID); } \
  NS_IMETHOD SetID(uint32_t aID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetID(aID); } \
  NS_IMETHOD GetCacheKey(uint32_t *aCacheKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheKey(aCacheKey); } \
  NS_IMETHOD SetCacheKey(uint32_t aCacheKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCacheKey(aCacheKey); } \
  NS_IMETHOD GetSaveLayoutStateFlag(bool *aSaveLayoutStateFlag) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSaveLayoutStateFlag(aSaveLayoutStateFlag); } \
  NS_IMETHOD SetSaveLayoutStateFlag(bool aSaveLayoutStateFlag) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSaveLayoutStateFlag(aSaveLayoutStateFlag); } \
  NS_IMETHOD GetContentType(nsACString& aContentType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentType(aContentType); } \
  NS_IMETHOD SetContentType(const nsACString& aContentType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContentType(aContentType); } \
  NS_IMETHOD GetURIWasModified(bool *aURIWasModified) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURIWasModified(aURIWasModified); } \
  NS_IMETHOD SetURIWasModified(bool aURIWasModified) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetURIWasModified(aURIWasModified); } \
  NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTriggeringPrincipal(aTriggeringPrincipal); } \
  NS_IMETHOD SetTriggeringPrincipal(nsIPrincipal *aTriggeringPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTriggeringPrincipal(aTriggeringPrincipal); } \
  NS_IMETHOD GetPrincipalToInherit(nsIPrincipal **aPrincipalToInherit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrincipalToInherit(aPrincipalToInherit); } \
  NS_IMETHOD SetPrincipalToInherit(nsIPrincipal *aPrincipalToInherit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrincipalToInherit(aPrincipalToInherit); } \
  NS_IMETHOD GetPartitionedPrincipalToInherit(nsIPrincipal **aPartitionedPrincipalToInherit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPartitionedPrincipalToInherit(aPartitionedPrincipalToInherit); } \
  NS_IMETHOD SetPartitionedPrincipalToInherit(nsIPrincipal *aPartitionedPrincipalToInherit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPartitionedPrincipalToInherit(aPartitionedPrincipalToInherit); } \
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCsp(aCsp); } \
  NS_IMETHOD SetCsp(nsIContentSecurityPolicy *aCsp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCsp(aCsp); } \
  NS_IMETHOD GetStateData(nsIStructuredCloneContainer **aStateData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStateData(aStateData); } \
  NS_IMETHOD SetStateData(nsIStructuredCloneContainer *aStateData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetStateData(aStateData); } \
  NS_IMETHOD GetDocshellID(nsID & aDocshellID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocshellID(aDocshellID); } \
  NS_IMETHOD SetDocshellID(const nsID & aDocshellID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDocshellID(aDocshellID); } \
  NS_IMETHOD GetIsSrcdocEntry(bool *aIsSrcdocEntry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsSrcdocEntry(aIsSrcdocEntry); } \
  NS_IMETHOD GetSrcdocData(nsAString& aSrcdocData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSrcdocData(aSrcdocData); } \
  NS_IMETHOD SetSrcdocData(const nsAString& aSrcdocData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSrcdocData(aSrcdocData); } \
  NS_IMETHOD GetBaseURI(nsIURI **aBaseURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBaseURI(aBaseURI); } \
  NS_IMETHOD SetBaseURI(nsIURI *aBaseURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBaseURI(aBaseURI); } \
  NS_IMETHOD GetScrollRestorationIsManual(bool *aScrollRestorationIsManual) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScrollRestorationIsManual(aScrollRestorationIsManual); } \
  NS_IMETHOD SetScrollRestorationIsManual(bool aScrollRestorationIsManual) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetScrollRestorationIsManual(aScrollRestorationIsManual); } \
  NS_IMETHOD GetLoadedInThisProcess(bool *aLoadedInThisProcess) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadedInThisProcess(aLoadedInThisProcess); } \
  NS_IMETHOD GetShistory(nsISHistory **aShistory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetShistory(aShistory); } \
  NS_IMETHOD SetShistory(nsISHistory *aShistory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetShistory(aShistory); } \
  NS_IMETHOD GetLastTouched(uint32_t *aLastTouched) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastTouched(aLastTouched); } \
  NS_IMETHOD SetLastTouched(uint32_t aLastTouched) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLastTouched(aLastTouched); } \
  NS_IMETHOD GetChildCount(int32_t *aChildCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChildCount(aChildCount); } \
  NS_IMETHOD GetPersist(bool *aPersist) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPersist(aPersist); } \
  NS_IMETHOD SetPersist(bool aPersist) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPersist(aPersist); } \
  NS_IMETHOD SetScrollPosition(int32_t x, int32_t y) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetScrollPosition(x, y); } \
  NS_IMETHOD GetScrollPosition(int32_t *x, int32_t *y) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScrollPosition(x, y); } \
  NS_IMETHOD_(void) GetViewerBounds(nsIntRect & bounds) override; \
  NS_IMETHOD_(void) SetViewerBounds(const nsIntRect & bounds) override; \
  NS_IMETHOD_(void) AddChildShell(nsIDocShellTreeItem *shell) override; \
  NS_IMETHOD ChildShellAt(int32_t index, nsIDocShellTreeItem **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ChildShellAt(index, _retval); } \
  NS_IMETHOD_(void) ClearChildShells(void) override; \
  NS_IMETHOD_(void) SyncPresentationState(void) override; \
  NS_IMETHOD InitLayoutHistoryState(nsILayoutHistoryState **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitLayoutHistoryState(_retval); } \
  NS_IMETHOD Create(nsIURI *URI, const nsAString& title, nsIInputStream *inputStream, uint32_t cacheKey, const nsACString& contentType, nsIPrincipal *triggeringPrincipal, nsIPrincipal *principalToInherit, nsIPrincipal *partitionedPrincipalToInherit, nsIContentSecurityPolicy *aCsp, const nsID & docshellID, bool dynamicCreation, nsIURI *originalURI, nsIURI *resultPrincipalURI, bool loadReplace, nsIReferrerInfo *referrerInfo, const nsAString& srcdoc, bool srcdocEntry, nsIURI *baseURI, bool saveLayoutState, bool expired) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Create(URI, title, inputStream, cacheKey, contentType, triggeringPrincipal, principalToInherit, partitionedPrincipalToInherit, aCsp, docshellID, dynamicCreation, originalURI, resultPrincipalURI, loadReplace, referrerInfo, srcdoc, srcdocEntry, baseURI, saveLayoutState, expired); } \
  NS_IMETHOD Clone(nsISHEntry **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clone(_retval); } \
  NS_IMETHOD_(nsDocShellEditorData *) ForgetEditorData(void) override; \
  NS_IMETHOD_(void) SetEditorData(nsDocShellEditorData * aData) override; \
  NS_IMETHOD_(bool) HasDetachedEditor(void) override; \
  NS_IMETHOD_(bool) IsDynamicallyAdded(void) override; \
  NS_IMETHOD HasDynamicallyAddedChild(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasDynamicallyAddedChild(_retval); } \
  NS_IMETHOD_(bool) HasBFCacheEntry(nsIBFCacheEntry *aEntry) override; \
  NS_IMETHOD AdoptBFCacheEntry(nsISHEntry *aEntry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AdoptBFCacheEntry(aEntry); } \
  NS_IMETHOD AbandonBFCacheEntry(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AbandonBFCacheEntry(); } \
  NS_IMETHOD SharesDocumentWith(nsISHEntry *aEntry, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SharesDocumentWith(aEntry, _retval); } \
  NS_IMETHOD SetLoadTypeAsHistory(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLoadTypeAsHistory(); } \
  NS_IMETHOD AddChild(nsISHEntry *aChild, int32_t aOffset, bool aUseRemoteSubframes = false) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddChild(aChild, aOffset, aUseRemoteSubframes); } \
  NS_IMETHOD RemoveChild(nsISHEntry *aChild) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveChild(aChild); } \
  NS_IMETHOD GetChildAt(int32_t aIndex, nsISHEntry **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChildAt(aIndex, _retval); } \
  NS_IMETHOD_(void) GetChildSHEntryIfHasNoDynamicallyAddedChild(int32_t aChildOffset, nsISHEntry **aChild) override; \
  NS_IMETHOD ReplaceChild(nsISHEntry *aNewChild) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReplaceChild(aNewChild); } \
  NS_IMETHOD_(void) ClearEntry(void) override; \
  NS_IMETHOD CreateLoadInfo(nsDocShellLoadState * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateLoadInfo(_retval); } \
  NS_IMETHOD GetBfcacheID(uint64_t *aBfcacheID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBfcacheID(aBfcacheID); } \
  NS_IMETHOD_(void) SyncTreesForSubframeNavigation(nsISHEntry *aEntry, mozilla::dom::BrowsingContext *aTopBC, mozilla::dom::BrowsingContext *aIgnoreBC) override; 


#endif /* __gen_nsISHEntry_h__ */
