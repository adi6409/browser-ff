/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/shistory/nsISHistory.idl
 */

#ifndef __gen_nsISHistory_h__
#define __gen_nsISHistory_h__


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
class nsIBFCacheEntry; /* forward declaration */

class nsISHEntry; /* forward declaration */

class nsISHistoryListener; /* forward declaration */

class nsIURI; /* forward declaration */

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla

#include "nsTArrayForwardDeclare.h"
#include "mozilla/Maybe.h"
struct EntriesAndBrowsingContextData;

/* starting interface:    nsISHistory */
#define NS_ISHISTORY_IID_STR "7b807041-e60a-4384-935f-af3061d8b815"

#define NS_ISHISTORY_IID \
  {0x7b807041, 0xe60a, 0x4384, \
    { 0x93, 0x5f, 0xaf, 0x30, 0x61, 0xd8, 0xb8, 0x15 }}

class NS_NO_VTABLE nsISHistory : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISHISTORY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISHistory;

  /* [infallible] readonly attribute long count; */
  NS_IMETHOD GetCount(int32_t *aCount) = 0;
  inline int32_t  GetCount()
  {
    int32_t result;
    mozilla::DebugOnly<nsresult> rv = GetCount(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* attribute long index; */
  NS_IMETHOD GetIndex(int32_t *aIndex) = 0;
  NS_IMETHOD SetIndex(int32_t aIndex) = 0;

  /* [infallible] readonly attribute long requestedIndex; */
  NS_IMETHOD GetRequestedIndex(int32_t *aRequestedIndex) = 0;
  inline int32_t  GetRequestedIndex()
  {
    int32_t result;
    mozilla::DebugOnly<nsresult> rv = GetRequestedIndex(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [noscript,notxpcom] void internalSetRequestedIndex (in long aRequestedIndex); */
  NS_IMETHOD_(void) InternalSetRequestedIndex(int32_t aRequestedIndex) = 0;

  /* nsISHEntry getEntryAtIndex (in long aIndex); */
  NS_IMETHOD GetEntryAtIndex(int32_t aIndex, nsISHEntry **_retval) = 0;

  /* void purgeHistory (in long aNumEntries); */
  NS_IMETHOD PurgeHistory(int32_t aNumEntries) = 0;

  /* void addSHistoryListener (in nsISHistoryListener aListener); */
  NS_IMETHOD AddSHistoryListener(nsISHistoryListener *aListener) = 0;

  /* void removeSHistoryListener (in nsISHistoryListener aListener); */
  NS_IMETHOD RemoveSHistoryListener(nsISHistoryListener *aListener) = 0;

  /* void reloadCurrentEntry (); */
  NS_IMETHOD ReloadCurrentEntry(void) = 0;

  /* [noscript] void gotoIndex (in long aIndex); */
  NS_IMETHOD GotoIndex(int32_t aIndex) = 0;

  /* [noscript,notxpcom] boolean hasUserInteractionAtIndex (in long aIndex); */
  NS_IMETHOD_(bool) HasUserInteractionAtIndex(int32_t aIndex) = 0;

  /* [noscript,notxpcom] long getIndexOfEntry (in nsISHEntry aEntry); */
  NS_IMETHOD_(int32_t) GetIndexOfEntry(nsISHEntry *aEntry) = 0;

  /* void addEntry (in nsISHEntry aEntry, in boolean aPersist); */
  NS_IMETHOD AddEntry(nsISHEntry *aEntry, bool aPersist) = 0;

  /* void updateIndex (); */
  NS_IMETHOD UpdateIndex(void) = 0;

  /* void replaceEntry (in long aIndex, in nsISHEntry aReplaceEntry); */
  NS_IMETHOD ReplaceEntry(int32_t aIndex, nsISHEntry *aReplaceEntry) = 0;

  /* boolean notifyOnHistoryReload (); */
  NS_IMETHOD NotifyOnHistoryReload(bool *_retval) = 0;

  /* void evictOutOfRangeContentViewers (in long aIndex); */
  NS_IMETHOD EvictOutOfRangeContentViewers(int32_t aIndex) = 0;

  /* void evictExpiredContentViewerForEntry (in nsIBFCacheEntry aEntry); */
  NS_IMETHOD EvictExpiredContentViewerForEntry(nsIBFCacheEntry *aEntry) = 0;

  /* void evictAllContentViewers (); */
  NS_IMETHOD EvictAllContentViewers(void) = 0;

  /* [noscript,notxpcom] void addToExpirationTracker (in nsIBFCacheEntry aEntry); */
  NS_IMETHOD_(void) AddToExpirationTracker(nsIBFCacheEntry *aEntry) = 0;

  /* [noscript,notxpcom] void removeFromExpirationTracker (in nsIBFCacheEntry aEntry); */
  NS_IMETHOD_(void) RemoveFromExpirationTracker(nsIBFCacheEntry *aEntry) = 0;

  /* [noscript,notxpcom] void RemoveDynEntries (in long aIndex, in nsISHEntry aEntry); */
  NS_IMETHOD_(void) RemoveDynEntries(int32_t aIndex, nsISHEntry *aEntry) = 0;

  /* [noscript,notxpcom] void RemoveDynEntriesForBFCacheEntry (in nsIBFCacheEntry aEntry); */
  NS_IMETHOD_(void) RemoveDynEntriesForBFCacheEntry(nsIBFCacheEntry *aEntry) = 0;

  /* [noscript,notxpcom] void RemoveEntries (in nsDocshellIDArray aIDs, in long aStartIndex); */
  NS_IMETHOD_(void) RemoveEntries(nsTArray<nsID> & aIDs, int32_t aStartIndex) = 0;

  /* [noscript,notxpcom] void RemoveFrameEntries (in nsISHEntry aEntry); */
  NS_IMETHOD_(void) RemoveFrameEntries(nsISHEntry *aEntry) = 0;

  /* [noscript] void Reload (in unsigned long aReloadFlags); */
  NS_IMETHOD Reload(uint32_t aReloadFlags) = 0;

  /* [notxpcom] void EnsureCorrectEntryAtCurrIndex (in nsISHEntry aEntry); */
  NS_IMETHOD_(void) EnsureCorrectEntryAtCurrIndex(nsISHEntry *aEntry) = 0;

  /* [notxpcom] void EvictContentViewersOrReplaceEntry (in nsISHEntry aNewSHEntry, in bool aReplace); */
  NS_IMETHOD_(void) EvictContentViewersOrReplaceEntry(nsISHEntry *aNewSHEntry, bool aReplace) = 0;

  /* nsISHEntry createEntry (); */
  NS_IMETHOD CreateEntry(nsISHEntry **_retval) = 0;

  /* [noscript] void AddToRootSessionHistory (in bool aCloneChildren, in nsISHEntry aOSHE, in BrowsingContext aRootBC, in nsISHEntry aEntry, in unsigned long aLoadType, in bool aShouldPersist, out MaybeInt32 aPreviousEntryIndex, out MaybeInt32 aLoadedEntryIndex); */
  NS_IMETHOD AddToRootSessionHistory(bool aCloneChildren, nsISHEntry *aOSHE, mozilla::dom::BrowsingContext *aRootBC, nsISHEntry *aEntry, uint32_t aLoadType, bool aShouldPersist, mozilla::Maybe<int32_t> * aPreviousEntryIndex, mozilla::Maybe<int32_t> * aLoadedEntryIndex) = 0;

  /* [noscript] void AddChildSHEntryHelper (in nsISHEntry aCloneRef, in nsISHEntry aNewEntry, in BrowsingContext aRootBC, in bool aCloneChildren); */
  NS_IMETHOD AddChildSHEntryHelper(nsISHEntry *aCloneRef, nsISHEntry *aNewEntry, mozilla::dom::BrowsingContext *aRootBC, bool aCloneChildren) = 0;

  /* [noscript,notxpcom] boolean isEmptyOrHasEntriesForSingleTopLevelPage (); */
  NS_IMETHOD_(bool) IsEmptyOrHasEntriesForSingleTopLevelPage(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISHistory, NS_ISHISTORY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISHISTORY \
  using nsISHistory::GetCount; \
  NS_IMETHOD GetCount(int32_t *aCount) override; \
  NS_IMETHOD GetIndex(int32_t *aIndex) override; \
  NS_IMETHOD SetIndex(int32_t aIndex) override; \
  using nsISHistory::GetRequestedIndex; \
  NS_IMETHOD GetRequestedIndex(int32_t *aRequestedIndex) override; \
  NS_IMETHOD_(void) InternalSetRequestedIndex(int32_t aRequestedIndex) override; \
  NS_IMETHOD GetEntryAtIndex(int32_t aIndex, nsISHEntry **_retval) override; \
  NS_IMETHOD PurgeHistory(int32_t aNumEntries) override; \
  NS_IMETHOD AddSHistoryListener(nsISHistoryListener *aListener) override; \
  NS_IMETHOD RemoveSHistoryListener(nsISHistoryListener *aListener) override; \
  NS_IMETHOD ReloadCurrentEntry(void) override; \
  NS_IMETHOD GotoIndex(int32_t aIndex) override; \
  NS_IMETHOD_(bool) HasUserInteractionAtIndex(int32_t aIndex) override; \
  NS_IMETHOD_(int32_t) GetIndexOfEntry(nsISHEntry *aEntry) override; \
  NS_IMETHOD AddEntry(nsISHEntry *aEntry, bool aPersist) override; \
  NS_IMETHOD UpdateIndex(void) override; \
  NS_IMETHOD ReplaceEntry(int32_t aIndex, nsISHEntry *aReplaceEntry) override; \
  NS_IMETHOD NotifyOnHistoryReload(bool *_retval) override; \
  NS_IMETHOD EvictOutOfRangeContentViewers(int32_t aIndex) override; \
  NS_IMETHOD EvictExpiredContentViewerForEntry(nsIBFCacheEntry *aEntry) override; \
  NS_IMETHOD EvictAllContentViewers(void) override; \
  NS_IMETHOD_(void) AddToExpirationTracker(nsIBFCacheEntry *aEntry) override; \
  NS_IMETHOD_(void) RemoveFromExpirationTracker(nsIBFCacheEntry *aEntry) override; \
  NS_IMETHOD_(void) RemoveDynEntries(int32_t aIndex, nsISHEntry *aEntry) override; \
  NS_IMETHOD_(void) RemoveDynEntriesForBFCacheEntry(nsIBFCacheEntry *aEntry) override; \
  NS_IMETHOD_(void) RemoveEntries(nsTArray<nsID> & aIDs, int32_t aStartIndex) override; \
  NS_IMETHOD_(void) RemoveFrameEntries(nsISHEntry *aEntry) override; \
  NS_IMETHOD Reload(uint32_t aReloadFlags) override; \
  NS_IMETHOD_(void) EnsureCorrectEntryAtCurrIndex(nsISHEntry *aEntry) override; \
  NS_IMETHOD_(void) EvictContentViewersOrReplaceEntry(nsISHEntry *aNewSHEntry, bool aReplace) override; \
  NS_IMETHOD CreateEntry(nsISHEntry **_retval) override; \
  NS_IMETHOD AddToRootSessionHistory(bool aCloneChildren, nsISHEntry *aOSHE, mozilla::dom::BrowsingContext *aRootBC, nsISHEntry *aEntry, uint32_t aLoadType, bool aShouldPersist, mozilla::Maybe<int32_t> * aPreviousEntryIndex, mozilla::Maybe<int32_t> * aLoadedEntryIndex) override; \
  NS_IMETHOD AddChildSHEntryHelper(nsISHEntry *aCloneRef, nsISHEntry *aNewEntry, mozilla::dom::BrowsingContext *aRootBC, bool aCloneChildren) override; \
  NS_IMETHOD_(bool) IsEmptyOrHasEntriesForSingleTopLevelPage(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISHISTORY \
  using nsISHistory::GetCount; \
  nsresult GetCount(int32_t *aCount); \
  nsresult GetIndex(int32_t *aIndex); \
  nsresult SetIndex(int32_t aIndex); \
  using nsISHistory::GetRequestedIndex; \
  nsresult GetRequestedIndex(int32_t *aRequestedIndex); \
  nsresult_(void) InternalSetRequestedIndex(int32_t aRequestedIndex); \
  nsresult GetEntryAtIndex(int32_t aIndex, nsISHEntry **_retval); \
  nsresult PurgeHistory(int32_t aNumEntries); \
  nsresult AddSHistoryListener(nsISHistoryListener *aListener); \
  nsresult RemoveSHistoryListener(nsISHistoryListener *aListener); \
  nsresult ReloadCurrentEntry(void); \
  nsresult GotoIndex(int32_t aIndex); \
  nsresult_(bool) HasUserInteractionAtIndex(int32_t aIndex); \
  nsresult_(int32_t) GetIndexOfEntry(nsISHEntry *aEntry); \
  nsresult AddEntry(nsISHEntry *aEntry, bool aPersist); \
  nsresult UpdateIndex(void); \
  nsresult ReplaceEntry(int32_t aIndex, nsISHEntry *aReplaceEntry); \
  nsresult NotifyOnHistoryReload(bool *_retval); \
  nsresult EvictOutOfRangeContentViewers(int32_t aIndex); \
  nsresult EvictExpiredContentViewerForEntry(nsIBFCacheEntry *aEntry); \
  nsresult EvictAllContentViewers(void); \
  nsresult_(void) AddToExpirationTracker(nsIBFCacheEntry *aEntry); \
  nsresult_(void) RemoveFromExpirationTracker(nsIBFCacheEntry *aEntry); \
  nsresult_(void) RemoveDynEntries(int32_t aIndex, nsISHEntry *aEntry); \
  nsresult_(void) RemoveDynEntriesForBFCacheEntry(nsIBFCacheEntry *aEntry); \
  nsresult_(void) RemoveEntries(nsTArray<nsID> & aIDs, int32_t aStartIndex); \
  nsresult_(void) RemoveFrameEntries(nsISHEntry *aEntry); \
  nsresult Reload(uint32_t aReloadFlags); \
  nsresult_(void) EnsureCorrectEntryAtCurrIndex(nsISHEntry *aEntry); \
  nsresult_(void) EvictContentViewersOrReplaceEntry(nsISHEntry *aNewSHEntry, bool aReplace); \
  nsresult CreateEntry(nsISHEntry **_retval); \
  nsresult AddToRootSessionHistory(bool aCloneChildren, nsISHEntry *aOSHE, mozilla::dom::BrowsingContext *aRootBC, nsISHEntry *aEntry, uint32_t aLoadType, bool aShouldPersist, mozilla::Maybe<int32_t> * aPreviousEntryIndex, mozilla::Maybe<int32_t> * aLoadedEntryIndex); \
  nsresult AddChildSHEntryHelper(nsISHEntry *aCloneRef, nsISHEntry *aNewEntry, mozilla::dom::BrowsingContext *aRootBC, bool aCloneChildren); \
  nsresult_(bool) IsEmptyOrHasEntriesForSingleTopLevelPage(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISHISTORY(_to) \
  using nsISHistory::GetCount; \
  NS_IMETHOD GetCount(int32_t *aCount) override { return _to GetCount(aCount); } \
  NS_IMETHOD GetIndex(int32_t *aIndex) override { return _to GetIndex(aIndex); } \
  NS_IMETHOD SetIndex(int32_t aIndex) override { return _to SetIndex(aIndex); } \
  using nsISHistory::GetRequestedIndex; \
  NS_IMETHOD GetRequestedIndex(int32_t *aRequestedIndex) override { return _to GetRequestedIndex(aRequestedIndex); } \
  NS_IMETHOD_(void) InternalSetRequestedIndex(int32_t aRequestedIndex) override { return _to InternalSetRequestedIndex(aRequestedIndex); } \
  NS_IMETHOD GetEntryAtIndex(int32_t aIndex, nsISHEntry **_retval) override { return _to GetEntryAtIndex(aIndex, _retval); } \
  NS_IMETHOD PurgeHistory(int32_t aNumEntries) override { return _to PurgeHistory(aNumEntries); } \
  NS_IMETHOD AddSHistoryListener(nsISHistoryListener *aListener) override { return _to AddSHistoryListener(aListener); } \
  NS_IMETHOD RemoveSHistoryListener(nsISHistoryListener *aListener) override { return _to RemoveSHistoryListener(aListener); } \
  NS_IMETHOD ReloadCurrentEntry(void) override { return _to ReloadCurrentEntry(); } \
  NS_IMETHOD GotoIndex(int32_t aIndex) override { return _to GotoIndex(aIndex); } \
  NS_IMETHOD_(bool) HasUserInteractionAtIndex(int32_t aIndex) override { return _to HasUserInteractionAtIndex(aIndex); } \
  NS_IMETHOD_(int32_t) GetIndexOfEntry(nsISHEntry *aEntry) override { return _to GetIndexOfEntry(aEntry); } \
  NS_IMETHOD AddEntry(nsISHEntry *aEntry, bool aPersist) override { return _to AddEntry(aEntry, aPersist); } \
  NS_IMETHOD UpdateIndex(void) override { return _to UpdateIndex(); } \
  NS_IMETHOD ReplaceEntry(int32_t aIndex, nsISHEntry *aReplaceEntry) override { return _to ReplaceEntry(aIndex, aReplaceEntry); } \
  NS_IMETHOD NotifyOnHistoryReload(bool *_retval) override { return _to NotifyOnHistoryReload(_retval); } \
  NS_IMETHOD EvictOutOfRangeContentViewers(int32_t aIndex) override { return _to EvictOutOfRangeContentViewers(aIndex); } \
  NS_IMETHOD EvictExpiredContentViewerForEntry(nsIBFCacheEntry *aEntry) override { return _to EvictExpiredContentViewerForEntry(aEntry); } \
  NS_IMETHOD EvictAllContentViewers(void) override { return _to EvictAllContentViewers(); } \
  NS_IMETHOD_(void) AddToExpirationTracker(nsIBFCacheEntry *aEntry) override { return _to AddToExpirationTracker(aEntry); } \
  NS_IMETHOD_(void) RemoveFromExpirationTracker(nsIBFCacheEntry *aEntry) override { return _to RemoveFromExpirationTracker(aEntry); } \
  NS_IMETHOD_(void) RemoveDynEntries(int32_t aIndex, nsISHEntry *aEntry) override { return _to RemoveDynEntries(aIndex, aEntry); } \
  NS_IMETHOD_(void) RemoveDynEntriesForBFCacheEntry(nsIBFCacheEntry *aEntry) override { return _to RemoveDynEntriesForBFCacheEntry(aEntry); } \
  NS_IMETHOD_(void) RemoveEntries(nsTArray<nsID> & aIDs, int32_t aStartIndex) override { return _to RemoveEntries(aIDs, aStartIndex); } \
  NS_IMETHOD_(void) RemoveFrameEntries(nsISHEntry *aEntry) override { return _to RemoveFrameEntries(aEntry); } \
  NS_IMETHOD Reload(uint32_t aReloadFlags) override { return _to Reload(aReloadFlags); } \
  NS_IMETHOD_(void) EnsureCorrectEntryAtCurrIndex(nsISHEntry *aEntry) override { return _to EnsureCorrectEntryAtCurrIndex(aEntry); } \
  NS_IMETHOD_(void) EvictContentViewersOrReplaceEntry(nsISHEntry *aNewSHEntry, bool aReplace) override { return _to EvictContentViewersOrReplaceEntry(aNewSHEntry, aReplace); } \
  NS_IMETHOD CreateEntry(nsISHEntry **_retval) override { return _to CreateEntry(_retval); } \
  NS_IMETHOD AddToRootSessionHistory(bool aCloneChildren, nsISHEntry *aOSHE, mozilla::dom::BrowsingContext *aRootBC, nsISHEntry *aEntry, uint32_t aLoadType, bool aShouldPersist, mozilla::Maybe<int32_t> * aPreviousEntryIndex, mozilla::Maybe<int32_t> * aLoadedEntryIndex) override { return _to AddToRootSessionHistory(aCloneChildren, aOSHE, aRootBC, aEntry, aLoadType, aShouldPersist, aPreviousEntryIndex, aLoadedEntryIndex); } \
  NS_IMETHOD AddChildSHEntryHelper(nsISHEntry *aCloneRef, nsISHEntry *aNewEntry, mozilla::dom::BrowsingContext *aRootBC, bool aCloneChildren) override { return _to AddChildSHEntryHelper(aCloneRef, aNewEntry, aRootBC, aCloneChildren); } \
  NS_IMETHOD_(bool) IsEmptyOrHasEntriesForSingleTopLevelPage(void) override { return _to IsEmptyOrHasEntriesForSingleTopLevelPage(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISHISTORY(_to) \
  NS_IMETHOD GetCount(int32_t *aCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCount(aCount); } \
  NS_IMETHOD GetIndex(int32_t *aIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIndex(aIndex); } \
  NS_IMETHOD SetIndex(int32_t aIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIndex(aIndex); } \
  NS_IMETHOD GetRequestedIndex(int32_t *aRequestedIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestedIndex(aRequestedIndex); } \
  NS_IMETHOD_(void) InternalSetRequestedIndex(int32_t aRequestedIndex) override; \
  NS_IMETHOD GetEntryAtIndex(int32_t aIndex, nsISHEntry **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEntryAtIndex(aIndex, _retval); } \
  NS_IMETHOD PurgeHistory(int32_t aNumEntries) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PurgeHistory(aNumEntries); } \
  NS_IMETHOD AddSHistoryListener(nsISHistoryListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddSHistoryListener(aListener); } \
  NS_IMETHOD RemoveSHistoryListener(nsISHistoryListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveSHistoryListener(aListener); } \
  NS_IMETHOD ReloadCurrentEntry(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReloadCurrentEntry(); } \
  NS_IMETHOD GotoIndex(int32_t aIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GotoIndex(aIndex); } \
  NS_IMETHOD_(bool) HasUserInteractionAtIndex(int32_t aIndex) override; \
  NS_IMETHOD_(int32_t) GetIndexOfEntry(nsISHEntry *aEntry) override; \
  NS_IMETHOD AddEntry(nsISHEntry *aEntry, bool aPersist) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddEntry(aEntry, aPersist); } \
  NS_IMETHOD UpdateIndex(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateIndex(); } \
  NS_IMETHOD ReplaceEntry(int32_t aIndex, nsISHEntry *aReplaceEntry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReplaceEntry(aIndex, aReplaceEntry); } \
  NS_IMETHOD NotifyOnHistoryReload(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyOnHistoryReload(_retval); } \
  NS_IMETHOD EvictOutOfRangeContentViewers(int32_t aIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EvictOutOfRangeContentViewers(aIndex); } \
  NS_IMETHOD EvictExpiredContentViewerForEntry(nsIBFCacheEntry *aEntry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EvictExpiredContentViewerForEntry(aEntry); } \
  NS_IMETHOD EvictAllContentViewers(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EvictAllContentViewers(); } \
  NS_IMETHOD_(void) AddToExpirationTracker(nsIBFCacheEntry *aEntry) override; \
  NS_IMETHOD_(void) RemoveFromExpirationTracker(nsIBFCacheEntry *aEntry) override; \
  NS_IMETHOD_(void) RemoveDynEntries(int32_t aIndex, nsISHEntry *aEntry) override; \
  NS_IMETHOD_(void) RemoveDynEntriesForBFCacheEntry(nsIBFCacheEntry *aEntry) override; \
  NS_IMETHOD_(void) RemoveEntries(nsTArray<nsID> & aIDs, int32_t aStartIndex) override; \
  NS_IMETHOD_(void) RemoveFrameEntries(nsISHEntry *aEntry) override; \
  NS_IMETHOD Reload(uint32_t aReloadFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reload(aReloadFlags); } \
  NS_IMETHOD_(void) EnsureCorrectEntryAtCurrIndex(nsISHEntry *aEntry) override; \
  NS_IMETHOD_(void) EvictContentViewersOrReplaceEntry(nsISHEntry *aNewSHEntry, bool aReplace) override; \
  NS_IMETHOD CreateEntry(nsISHEntry **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateEntry(_retval); } \
  NS_IMETHOD AddToRootSessionHistory(bool aCloneChildren, nsISHEntry *aOSHE, mozilla::dom::BrowsingContext *aRootBC, nsISHEntry *aEntry, uint32_t aLoadType, bool aShouldPersist, mozilla::Maybe<int32_t> * aPreviousEntryIndex, mozilla::Maybe<int32_t> * aLoadedEntryIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddToRootSessionHistory(aCloneChildren, aOSHE, aRootBC, aEntry, aLoadType, aShouldPersist, aPreviousEntryIndex, aLoadedEntryIndex); } \
  NS_IMETHOD AddChildSHEntryHelper(nsISHEntry *aCloneRef, nsISHEntry *aNewEntry, mozilla::dom::BrowsingContext *aRootBC, bool aCloneChildren) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddChildSHEntryHelper(aCloneRef, aNewEntry, aRootBC, aCloneChildren); } \
  NS_IMETHOD_(bool) IsEmptyOrHasEntriesForSingleTopLevelPage(void) override; 


#endif /* __gen_nsISHistory_h__ */
