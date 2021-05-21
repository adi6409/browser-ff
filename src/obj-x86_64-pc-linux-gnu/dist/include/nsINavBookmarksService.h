/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/nsINavBookmarksService.idl
 */

#ifndef __gen_nsINavBookmarksService_h__
#define __gen_nsINavBookmarksService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */

class nsIURI; /* forward declaration */

class nsITransaction; /* forward declaration */

class nsINavHistoryBatchCallback; /* forward declaration */


/* starting interface:    nsINavBookmarkObserver */
#define NS_INAVBOOKMARKOBSERVER_IID_STR "4d00c221-2c4a-47ab-a617-abb324110492"

#define NS_INAVBOOKMARKOBSERVER_IID \
  {0x4d00c221, 0x2c4a, 0x47ab, \
    { 0xa6, 0x17, 0xab, 0xb3, 0x24, 0x11, 0x04, 0x92 }}

class NS_NO_VTABLE nsINavBookmarkObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INAVBOOKMARKOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINavBookmarkObserver;

  /* readonly attribute boolean skipTags; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSkipTags(bool *aSkipTags) = 0;

  /* void onBeginUpdateBatch (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnBeginUpdateBatch(void) = 0;

  /* void onEndUpdateBatch (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnEndUpdateBatch(void) = 0;

  /* void onItemChanged (in long long aItemId, in ACString aProperty, in boolean aIsAnnotationProperty, in AUTF8String aNewValue, in PRTime aLastModified, in unsigned short aItemType, in long long aParentId, in ACString aGuid, in ACString aParentGuid, in AUTF8String aOldValue, in unsigned short aSource); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnItemChanged(int64_t aItemId, const nsACString& aProperty, bool aIsAnnotationProperty, const nsACString& aNewValue, PRTime aLastModified, uint16_t aItemType, int64_t aParentId, const nsACString& aGuid, const nsACString& aParentGuid, const nsACString& aOldValue, uint16_t aSource) = 0;

  /* void onItemMoved (in long long aItemId, in long long aOldParentId, in long aOldIndex, in long long aNewParentId, in long aNewIndex, in unsigned short aItemType, in ACString aGuid, in ACString aOldParentGuid, in ACString aNewParentGuid, in unsigned short aSource, in AUTF8String aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnItemMoved(int64_t aItemId, int64_t aOldParentId, int32_t aOldIndex, int64_t aNewParentId, int32_t aNewIndex, uint16_t aItemType, const nsACString& aGuid, const nsACString& aOldParentGuid, const nsACString& aNewParentGuid, uint16_t aSource, const nsACString& aURI) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINavBookmarkObserver, NS_INAVBOOKMARKOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINAVBOOKMARKOBSERVER \
  NS_IMETHOD GetSkipTags(bool *aSkipTags) override; \
  NS_IMETHOD OnBeginUpdateBatch(void) override; \
  NS_IMETHOD OnEndUpdateBatch(void) override; \
  NS_IMETHOD OnItemChanged(int64_t aItemId, const nsACString& aProperty, bool aIsAnnotationProperty, const nsACString& aNewValue, PRTime aLastModified, uint16_t aItemType, int64_t aParentId, const nsACString& aGuid, const nsACString& aParentGuid, const nsACString& aOldValue, uint16_t aSource) override; \
  NS_IMETHOD OnItemMoved(int64_t aItemId, int64_t aOldParentId, int32_t aOldIndex, int64_t aNewParentId, int32_t aNewIndex, uint16_t aItemType, const nsACString& aGuid, const nsACString& aOldParentGuid, const nsACString& aNewParentGuid, uint16_t aSource, const nsACString& aURI) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINAVBOOKMARKOBSERVER \
  nsresult GetSkipTags(bool *aSkipTags); \
  nsresult OnBeginUpdateBatch(void); \
  nsresult OnEndUpdateBatch(void); \
  nsresult OnItemChanged(int64_t aItemId, const nsACString& aProperty, bool aIsAnnotationProperty, const nsACString& aNewValue, PRTime aLastModified, uint16_t aItemType, int64_t aParentId, const nsACString& aGuid, const nsACString& aParentGuid, const nsACString& aOldValue, uint16_t aSource); \
  nsresult OnItemMoved(int64_t aItemId, int64_t aOldParentId, int32_t aOldIndex, int64_t aNewParentId, int32_t aNewIndex, uint16_t aItemType, const nsACString& aGuid, const nsACString& aOldParentGuid, const nsACString& aNewParentGuid, uint16_t aSource, const nsACString& aURI); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINAVBOOKMARKOBSERVER(_to) \
  NS_IMETHOD GetSkipTags(bool *aSkipTags) override { return _to GetSkipTags(aSkipTags); } \
  NS_IMETHOD OnBeginUpdateBatch(void) override { return _to OnBeginUpdateBatch(); } \
  NS_IMETHOD OnEndUpdateBatch(void) override { return _to OnEndUpdateBatch(); } \
  NS_IMETHOD OnItemChanged(int64_t aItemId, const nsACString& aProperty, bool aIsAnnotationProperty, const nsACString& aNewValue, PRTime aLastModified, uint16_t aItemType, int64_t aParentId, const nsACString& aGuid, const nsACString& aParentGuid, const nsACString& aOldValue, uint16_t aSource) override { return _to OnItemChanged(aItemId, aProperty, aIsAnnotationProperty, aNewValue, aLastModified, aItemType, aParentId, aGuid, aParentGuid, aOldValue, aSource); } \
  NS_IMETHOD OnItemMoved(int64_t aItemId, int64_t aOldParentId, int32_t aOldIndex, int64_t aNewParentId, int32_t aNewIndex, uint16_t aItemType, const nsACString& aGuid, const nsACString& aOldParentGuid, const nsACString& aNewParentGuid, uint16_t aSource, const nsACString& aURI) override { return _to OnItemMoved(aItemId, aOldParentId, aOldIndex, aNewParentId, aNewIndex, aItemType, aGuid, aOldParentGuid, aNewParentGuid, aSource, aURI); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINAVBOOKMARKOBSERVER(_to) \
  NS_IMETHOD GetSkipTags(bool *aSkipTags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSkipTags(aSkipTags); } \
  NS_IMETHOD OnBeginUpdateBatch(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnBeginUpdateBatch(); } \
  NS_IMETHOD OnEndUpdateBatch(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnEndUpdateBatch(); } \
  NS_IMETHOD OnItemChanged(int64_t aItemId, const nsACString& aProperty, bool aIsAnnotationProperty, const nsACString& aNewValue, PRTime aLastModified, uint16_t aItemType, int64_t aParentId, const nsACString& aGuid, const nsACString& aParentGuid, const nsACString& aOldValue, uint16_t aSource) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnItemChanged(aItemId, aProperty, aIsAnnotationProperty, aNewValue, aLastModified, aItemType, aParentId, aGuid, aParentGuid, aOldValue, aSource); } \
  NS_IMETHOD OnItemMoved(int64_t aItemId, int64_t aOldParentId, int32_t aOldIndex, int64_t aNewParentId, int32_t aNewIndex, uint16_t aItemType, const nsACString& aGuid, const nsACString& aOldParentGuid, const nsACString& aNewParentGuid, uint16_t aSource, const nsACString& aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnItemMoved(aItemId, aOldParentId, aOldIndex, aNewParentId, aNewIndex, aItemType, aGuid, aOldParentGuid, aNewParentGuid, aSource, aURI); } 


/* starting interface:    nsINavBookmarksService */
#define NS_INAVBOOKMARKSSERVICE_IID_STR "24533891-afa6-4663-b72d-3143d03f1b04"

#define NS_INAVBOOKMARKSSERVICE_IID \
  {0x24533891, 0xafa6, 0x4663, \
    { 0xb7, 0x2d, 0x31, 0x43, 0xd0, 0x3f, 0x1b, 0x04 }}

class NS_NO_VTABLE nsINavBookmarksService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INAVBOOKMARKSSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINavBookmarksService;

  /* readonly attribute long long placesRoot; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPlacesRoot(int64_t *aPlacesRoot) = 0;

  /* readonly attribute long long bookmarksMenuFolder; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBookmarksMenuFolder(int64_t *aBookmarksMenuFolder) = 0;

  /* readonly attribute long long tagsFolder; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTagsFolder(int64_t *aTagsFolder) = 0;

  /* readonly attribute long long toolbarFolder; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetToolbarFolder(int64_t *aToolbarFolder) = 0;

  /* readonly attribute long long totalSyncChanges; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTotalSyncChanges(int64_t *aTotalSyncChanges) = 0;

  enum {
    DEFAULT_INDEX = -1,
    TYPE_BOOKMARK = 1U,
    TYPE_FOLDER = 2U,
    TYPE_SEPARATOR = 3U,
    TYPE_DYNAMIC_CONTAINER = 4U,
    SOURCE_DEFAULT = 0U,
    SOURCE_SYNC = 1U,
    SOURCE_IMPORT = 2U,
    SOURCE_SYNC_REPARENT_REMOVED_FOLDER_CHILDREN = 4U,
    SOURCE_RESTORE = 5U,
    SOURCE_RESTORE_ON_STARTUP = 6U,
    SYNC_STATUS_UNKNOWN = 0U,
    SYNC_STATUS_NEW = 1U,
    SYNC_STATUS_NORMAL = 2U
  };

  /* [can_run_script] long long insertBookmark (in long long aParentId, in nsIURI aURI, in long aIndex, in AUTF8String aTitle, [optional] in ACString aGuid, [optional] in unsigned short aSource); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertBookmark(int64_t aParentId, nsIURI *aURI, int32_t aIndex, const nsACString& aTitle, const nsACString& aGuid, uint16_t aSource, int64_t *_retval) = 0;

  /* [can_run_script] void removeItem (in long long aItemId, [optional] in unsigned short aSource); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveItem(int64_t aItemId, uint16_t aSource) = 0;

  /* [can_run_script] long long createFolder (in long long aParentFolder, in AUTF8String name, in long index, [optional] in ACString aGuid, [optional] in unsigned short aSource); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD CreateFolder(int64_t aParentFolder, const nsACString& name, int32_t index, const nsACString& aGuid, uint16_t aSource, int64_t *_retval) = 0;

  /* void setItemTitle (in long long aItemId, in AUTF8String aTitle, [optional] in unsigned short aSource); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetItemTitle(int64_t aItemId, const nsACString& aTitle, uint16_t aSource) = 0;

  /* AUTF8String getItemTitle (in long long aItemId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetItemTitle(int64_t aItemId, nsACString& _retval) = 0;

  /* void setItemLastModified (in long long aItemId, in PRTime aLastModified, [optional] in unsigned short aSource); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetItemLastModified(int64_t aItemId, PRTime aLastModified, uint16_t aSource) = 0;

  /* long long getFolderIdForItem (in long long aItemId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFolderIdForItem(int64_t aItemId, int64_t *_retval) = 0;

  /* void addObserver (in nsINavBookmarkObserver observer, [optional] in boolean ownsWeak); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddObserver(nsINavBookmarkObserver *observer, bool ownsWeak) = 0;

  /* void removeObserver (in nsINavBookmarkObserver observer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveObserver(nsINavBookmarkObserver *observer) = 0;

  /* Array<nsINavBookmarkObserver> getObservers (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetObservers(nsTArray<RefPtr<nsINavBookmarkObserver>>& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINavBookmarksService, NS_INAVBOOKMARKSSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINAVBOOKMARKSSERVICE \
  NS_IMETHOD GetPlacesRoot(int64_t *aPlacesRoot) override; \
  NS_IMETHOD GetBookmarksMenuFolder(int64_t *aBookmarksMenuFolder) override; \
  NS_IMETHOD GetTagsFolder(int64_t *aTagsFolder) override; \
  NS_IMETHOD GetToolbarFolder(int64_t *aToolbarFolder) override; \
  NS_IMETHOD GetTotalSyncChanges(int64_t *aTotalSyncChanges) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertBookmark(int64_t aParentId, nsIURI *aURI, int32_t aIndex, const nsACString& aTitle, const nsACString& aGuid, uint16_t aSource, int64_t *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveItem(int64_t aItemId, uint16_t aSource) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CreateFolder(int64_t aParentFolder, const nsACString& name, int32_t index, const nsACString& aGuid, uint16_t aSource, int64_t *_retval) override; \
  NS_IMETHOD SetItemTitle(int64_t aItemId, const nsACString& aTitle, uint16_t aSource) override; \
  NS_IMETHOD GetItemTitle(int64_t aItemId, nsACString& _retval) override; \
  NS_IMETHOD SetItemLastModified(int64_t aItemId, PRTime aLastModified, uint16_t aSource) override; \
  NS_IMETHOD GetFolderIdForItem(int64_t aItemId, int64_t *_retval) override; \
  NS_IMETHOD AddObserver(nsINavBookmarkObserver *observer, bool ownsWeak) override; \
  NS_IMETHOD RemoveObserver(nsINavBookmarkObserver *observer) override; \
  NS_IMETHOD GetObservers(nsTArray<RefPtr<nsINavBookmarkObserver>>& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINAVBOOKMARKSSERVICE \
  nsresult GetPlacesRoot(int64_t *aPlacesRoot); \
  nsresult GetBookmarksMenuFolder(int64_t *aBookmarksMenuFolder); \
  nsresult GetTagsFolder(int64_t *aTagsFolder); \
  nsresult GetToolbarFolder(int64_t *aToolbarFolder); \
  nsresult GetTotalSyncChanges(int64_t *aTotalSyncChanges); \
  MOZ_CAN_RUN_SCRIPT nsresult InsertBookmark(int64_t aParentId, nsIURI *aURI, int32_t aIndex, const nsACString& aTitle, const nsACString& aGuid, uint16_t aSource, int64_t *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult RemoveItem(int64_t aItemId, uint16_t aSource); \
  MOZ_CAN_RUN_SCRIPT nsresult CreateFolder(int64_t aParentFolder, const nsACString& name, int32_t index, const nsACString& aGuid, uint16_t aSource, int64_t *_retval); \
  nsresult SetItemTitle(int64_t aItemId, const nsACString& aTitle, uint16_t aSource); \
  nsresult GetItemTitle(int64_t aItemId, nsACString& _retval); \
  nsresult SetItemLastModified(int64_t aItemId, PRTime aLastModified, uint16_t aSource); \
  nsresult GetFolderIdForItem(int64_t aItemId, int64_t *_retval); \
  nsresult AddObserver(nsINavBookmarkObserver *observer, bool ownsWeak); \
  nsresult RemoveObserver(nsINavBookmarkObserver *observer); \
  nsresult GetObservers(nsTArray<RefPtr<nsINavBookmarkObserver>>& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINAVBOOKMARKSSERVICE(_to) \
  NS_IMETHOD GetPlacesRoot(int64_t *aPlacesRoot) override { return _to GetPlacesRoot(aPlacesRoot); } \
  NS_IMETHOD GetBookmarksMenuFolder(int64_t *aBookmarksMenuFolder) override { return _to GetBookmarksMenuFolder(aBookmarksMenuFolder); } \
  NS_IMETHOD GetTagsFolder(int64_t *aTagsFolder) override { return _to GetTagsFolder(aTagsFolder); } \
  NS_IMETHOD GetToolbarFolder(int64_t *aToolbarFolder) override { return _to GetToolbarFolder(aToolbarFolder); } \
  NS_IMETHOD GetTotalSyncChanges(int64_t *aTotalSyncChanges) override { return _to GetTotalSyncChanges(aTotalSyncChanges); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertBookmark(int64_t aParentId, nsIURI *aURI, int32_t aIndex, const nsACString& aTitle, const nsACString& aGuid, uint16_t aSource, int64_t *_retval) override { return _to InsertBookmark(aParentId, aURI, aIndex, aTitle, aGuid, aSource, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveItem(int64_t aItemId, uint16_t aSource) override { return _to RemoveItem(aItemId, aSource); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CreateFolder(int64_t aParentFolder, const nsACString& name, int32_t index, const nsACString& aGuid, uint16_t aSource, int64_t *_retval) override { return _to CreateFolder(aParentFolder, name, index, aGuid, aSource, _retval); } \
  NS_IMETHOD SetItemTitle(int64_t aItemId, const nsACString& aTitle, uint16_t aSource) override { return _to SetItemTitle(aItemId, aTitle, aSource); } \
  NS_IMETHOD GetItemTitle(int64_t aItemId, nsACString& _retval) override { return _to GetItemTitle(aItemId, _retval); } \
  NS_IMETHOD SetItemLastModified(int64_t aItemId, PRTime aLastModified, uint16_t aSource) override { return _to SetItemLastModified(aItemId, aLastModified, aSource); } \
  NS_IMETHOD GetFolderIdForItem(int64_t aItemId, int64_t *_retval) override { return _to GetFolderIdForItem(aItemId, _retval); } \
  NS_IMETHOD AddObserver(nsINavBookmarkObserver *observer, bool ownsWeak) override { return _to AddObserver(observer, ownsWeak); } \
  NS_IMETHOD RemoveObserver(nsINavBookmarkObserver *observer) override { return _to RemoveObserver(observer); } \
  NS_IMETHOD GetObservers(nsTArray<RefPtr<nsINavBookmarkObserver>>& _retval) override { return _to GetObservers(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINAVBOOKMARKSSERVICE(_to) \
  NS_IMETHOD GetPlacesRoot(int64_t *aPlacesRoot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPlacesRoot(aPlacesRoot); } \
  NS_IMETHOD GetBookmarksMenuFolder(int64_t *aBookmarksMenuFolder) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBookmarksMenuFolder(aBookmarksMenuFolder); } \
  NS_IMETHOD GetTagsFolder(int64_t *aTagsFolder) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTagsFolder(aTagsFolder); } \
  NS_IMETHOD GetToolbarFolder(int64_t *aToolbarFolder) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetToolbarFolder(aToolbarFolder); } \
  NS_IMETHOD GetTotalSyncChanges(int64_t *aTotalSyncChanges) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTotalSyncChanges(aTotalSyncChanges); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertBookmark(int64_t aParentId, nsIURI *aURI, int32_t aIndex, const nsACString& aTitle, const nsACString& aGuid, uint16_t aSource, int64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InsertBookmark(aParentId, aURI, aIndex, aTitle, aGuid, aSource, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveItem(int64_t aItemId, uint16_t aSource) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveItem(aItemId, aSource); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CreateFolder(int64_t aParentFolder, const nsACString& name, int32_t index, const nsACString& aGuid, uint16_t aSource, int64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateFolder(aParentFolder, name, index, aGuid, aSource, _retval); } \
  NS_IMETHOD SetItemTitle(int64_t aItemId, const nsACString& aTitle, uint16_t aSource) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetItemTitle(aItemId, aTitle, aSource); } \
  NS_IMETHOD GetItemTitle(int64_t aItemId, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetItemTitle(aItemId, _retval); } \
  NS_IMETHOD SetItemLastModified(int64_t aItemId, PRTime aLastModified, uint16_t aSource) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetItemLastModified(aItemId, aLastModified, aSource); } \
  NS_IMETHOD GetFolderIdForItem(int64_t aItemId, int64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFolderIdForItem(aItemId, _retval); } \
  NS_IMETHOD AddObserver(nsINavBookmarkObserver *observer, bool ownsWeak) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddObserver(observer, ownsWeak); } \
  NS_IMETHOD RemoveObserver(nsINavBookmarkObserver *observer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveObserver(observer); } \
  NS_IMETHOD GetObservers(nsTArray<RefPtr<nsINavBookmarkObserver>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetObservers(_retval); } 


#endif /* __gen_nsINavBookmarksService_h__ */
