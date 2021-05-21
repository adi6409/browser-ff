/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/nsINavHistoryService.idl
 */

#ifndef __gen_nsINavHistoryService_h__
#define __gen_nsINavHistoryService_h__


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
class nsIArray; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIObserver; /* forward declaration */

class nsIVariant; /* forward declaration */

class nsIURI; /* forward declaration */

class mozIStorageConnection; /* forward declaration */

class mozIStorageStatementCallback; /* forward declaration */

class mozIStoragePendingStatement; /* forward declaration */

class nsIAsyncShutdownClient; /* forward declaration */

class nsINavHistoryContainerResultNode; /* forward declaration */

class nsINavHistoryQueryResultNode; /* forward declaration */

class nsINavHistoryQuery; /* forward declaration */

class nsINavHistoryQueryOptions; /* forward declaration */

class nsINavHistoryResult; /* forward declaration */

class nsINavHistoryBatchCallback; /* forward declaration */


/* starting interface:    nsINavHistoryResultNode */
#define NS_INAVHISTORYRESULTNODE_IID_STR "91d104bb-17ef-404b-9f9a-d9ed8de6824c"

#define NS_INAVHISTORYRESULTNODE_IID \
  {0x91d104bb, 0x17ef, 0x404b, \
    { 0x9f, 0x9a, 0xd9, 0xed, 0x8d, 0xe6, 0x82, 0x4c }}

class NS_NO_VTABLE nsINavHistoryResultNode : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INAVHISTORYRESULTNODE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINavHistoryResultNode;

  /* readonly attribute nsINavHistoryContainerResultNode parent; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetParent(nsINavHistoryContainerResultNode **aParent) = 0;

  /* readonly attribute nsINavHistoryResult parentResult; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetParentResult(nsINavHistoryResult **aParentResult) = 0;

  /* readonly attribute AUTF8String uri; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUri(nsACString& aUri) = 0;

  enum {
    RESULT_TYPE_URI = 0U,
    RESULT_TYPE_QUERY = 5U,
    RESULT_TYPE_FOLDER = 6U,
    RESULT_TYPE_SEPARATOR = 7U,
    RESULT_TYPE_FOLDER_SHORTCUT = 9U
  };

  /* readonly attribute unsigned long type; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetType(uint32_t *aType) = 0;

  /* readonly attribute AUTF8String title; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTitle(nsACString& aTitle) = 0;

  /* readonly attribute unsigned long accessCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAccessCount(uint32_t *aAccessCount) = 0;

  /* readonly attribute PRTime time; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTime(PRTime *aTime) = 0;

  /* readonly attribute AUTF8String icon; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIcon(nsACString& aIcon) = 0;

  /* readonly attribute long indentLevel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIndentLevel(int32_t *aIndentLevel) = 0;

  /* readonly attribute long bookmarkIndex; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBookmarkIndex(int32_t *aBookmarkIndex) = 0;

  /* readonly attribute long long itemId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetItemId(int64_t *aItemId) = 0;

  /* readonly attribute PRTime dateAdded; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDateAdded(PRTime *aDateAdded) = 0;

  /* readonly attribute PRTime lastModified; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastModified(PRTime *aLastModified) = 0;

  /* readonly attribute AString tags; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTags(nsAString& aTags) = 0;

  /* readonly attribute ACString pageGuid; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPageGuid(nsACString& aPageGuid) = 0;

  /* readonly attribute ACString bookmarkGuid; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBookmarkGuid(nsACString& aBookmarkGuid) = 0;

  /* readonly attribute long long visitId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetVisitId(int64_t *aVisitId) = 0;

  /* readonly attribute long long fromVisitId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFromVisitId(int64_t *aFromVisitId) = 0;

  /* readonly attribute unsigned long visitType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetVisitType(uint32_t *aVisitType) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINavHistoryResultNode, NS_INAVHISTORYRESULTNODE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINAVHISTORYRESULTNODE \
  NS_IMETHOD GetParent(nsINavHistoryContainerResultNode **aParent) override; \
  NS_IMETHOD GetParentResult(nsINavHistoryResult **aParentResult) override; \
  NS_IMETHOD GetUri(nsACString& aUri) override; \
  NS_IMETHOD GetType(uint32_t *aType) override; \
  NS_IMETHOD GetTitle(nsACString& aTitle) override; \
  NS_IMETHOD GetAccessCount(uint32_t *aAccessCount) override; \
  NS_IMETHOD GetTime(PRTime *aTime) override; \
  NS_IMETHOD GetIcon(nsACString& aIcon) override; \
  NS_IMETHOD GetIndentLevel(int32_t *aIndentLevel) override; \
  NS_IMETHOD GetBookmarkIndex(int32_t *aBookmarkIndex) override; \
  NS_IMETHOD GetItemId(int64_t *aItemId) override; \
  NS_IMETHOD GetDateAdded(PRTime *aDateAdded) override; \
  NS_IMETHOD GetLastModified(PRTime *aLastModified) override; \
  NS_IMETHOD GetTags(nsAString& aTags) override; \
  NS_IMETHOD GetPageGuid(nsACString& aPageGuid) override; \
  NS_IMETHOD GetBookmarkGuid(nsACString& aBookmarkGuid) override; \
  NS_IMETHOD GetVisitId(int64_t *aVisitId) override; \
  NS_IMETHOD GetFromVisitId(int64_t *aFromVisitId) override; \
  NS_IMETHOD GetVisitType(uint32_t *aVisitType) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINAVHISTORYRESULTNODE \
  nsresult GetParent(nsINavHistoryContainerResultNode **aParent); \
  nsresult GetParentResult(nsINavHistoryResult **aParentResult); \
  nsresult GetUri(nsACString& aUri); \
  nsresult GetType(uint32_t *aType); \
  nsresult GetTitle(nsACString& aTitle); \
  nsresult GetAccessCount(uint32_t *aAccessCount); \
  nsresult GetTime(PRTime *aTime); \
  nsresult GetIcon(nsACString& aIcon); \
  nsresult GetIndentLevel(int32_t *aIndentLevel); \
  nsresult GetBookmarkIndex(int32_t *aBookmarkIndex); \
  nsresult GetItemId(int64_t *aItemId); \
  nsresult GetDateAdded(PRTime *aDateAdded); \
  nsresult GetLastModified(PRTime *aLastModified); \
  nsresult GetTags(nsAString& aTags); \
  nsresult GetPageGuid(nsACString& aPageGuid); \
  nsresult GetBookmarkGuid(nsACString& aBookmarkGuid); \
  nsresult GetVisitId(int64_t *aVisitId); \
  nsresult GetFromVisitId(int64_t *aFromVisitId); \
  nsresult GetVisitType(uint32_t *aVisitType); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINAVHISTORYRESULTNODE(_to) \
  NS_IMETHOD GetParent(nsINavHistoryContainerResultNode **aParent) override { return _to GetParent(aParent); } \
  NS_IMETHOD GetParentResult(nsINavHistoryResult **aParentResult) override { return _to GetParentResult(aParentResult); } \
  NS_IMETHOD GetUri(nsACString& aUri) override { return _to GetUri(aUri); } \
  NS_IMETHOD GetType(uint32_t *aType) override { return _to GetType(aType); } \
  NS_IMETHOD GetTitle(nsACString& aTitle) override { return _to GetTitle(aTitle); } \
  NS_IMETHOD GetAccessCount(uint32_t *aAccessCount) override { return _to GetAccessCount(aAccessCount); } \
  NS_IMETHOD GetTime(PRTime *aTime) override { return _to GetTime(aTime); } \
  NS_IMETHOD GetIcon(nsACString& aIcon) override { return _to GetIcon(aIcon); } \
  NS_IMETHOD GetIndentLevel(int32_t *aIndentLevel) override { return _to GetIndentLevel(aIndentLevel); } \
  NS_IMETHOD GetBookmarkIndex(int32_t *aBookmarkIndex) override { return _to GetBookmarkIndex(aBookmarkIndex); } \
  NS_IMETHOD GetItemId(int64_t *aItemId) override { return _to GetItemId(aItemId); } \
  NS_IMETHOD GetDateAdded(PRTime *aDateAdded) override { return _to GetDateAdded(aDateAdded); } \
  NS_IMETHOD GetLastModified(PRTime *aLastModified) override { return _to GetLastModified(aLastModified); } \
  NS_IMETHOD GetTags(nsAString& aTags) override { return _to GetTags(aTags); } \
  NS_IMETHOD GetPageGuid(nsACString& aPageGuid) override { return _to GetPageGuid(aPageGuid); } \
  NS_IMETHOD GetBookmarkGuid(nsACString& aBookmarkGuid) override { return _to GetBookmarkGuid(aBookmarkGuid); } \
  NS_IMETHOD GetVisitId(int64_t *aVisitId) override { return _to GetVisitId(aVisitId); } \
  NS_IMETHOD GetFromVisitId(int64_t *aFromVisitId) override { return _to GetFromVisitId(aFromVisitId); } \
  NS_IMETHOD GetVisitType(uint32_t *aVisitType) override { return _to GetVisitType(aVisitType); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINAVHISTORYRESULTNODE(_to) \
  NS_IMETHOD GetParent(nsINavHistoryContainerResultNode **aParent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParent(aParent); } \
  NS_IMETHOD GetParentResult(nsINavHistoryResult **aParentResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParentResult(aParentResult); } \
  NS_IMETHOD GetUri(nsACString& aUri) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUri(aUri); } \
  NS_IMETHOD GetType(uint32_t *aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD GetTitle(nsACString& aTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTitle(aTitle); } \
  NS_IMETHOD GetAccessCount(uint32_t *aAccessCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAccessCount(aAccessCount); } \
  NS_IMETHOD GetTime(PRTime *aTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTime(aTime); } \
  NS_IMETHOD GetIcon(nsACString& aIcon) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIcon(aIcon); } \
  NS_IMETHOD GetIndentLevel(int32_t *aIndentLevel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIndentLevel(aIndentLevel); } \
  NS_IMETHOD GetBookmarkIndex(int32_t *aBookmarkIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBookmarkIndex(aBookmarkIndex); } \
  NS_IMETHOD GetItemId(int64_t *aItemId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetItemId(aItemId); } \
  NS_IMETHOD GetDateAdded(PRTime *aDateAdded) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDateAdded(aDateAdded); } \
  NS_IMETHOD GetLastModified(PRTime *aLastModified) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastModified(aLastModified); } \
  NS_IMETHOD GetTags(nsAString& aTags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTags(aTags); } \
  NS_IMETHOD GetPageGuid(nsACString& aPageGuid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPageGuid(aPageGuid); } \
  NS_IMETHOD GetBookmarkGuid(nsACString& aBookmarkGuid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBookmarkGuid(aBookmarkGuid); } \
  NS_IMETHOD GetVisitId(int64_t *aVisitId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVisitId(aVisitId); } \
  NS_IMETHOD GetFromVisitId(int64_t *aFromVisitId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFromVisitId(aFromVisitId); } \
  NS_IMETHOD GetVisitType(uint32_t *aVisitType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVisitType(aVisitType); } 


/* starting interface:    nsINavHistoryContainerResultNode */
#define NS_INAVHISTORYCONTAINERRESULTNODE_IID_STR "3e9cc95f-0d93-45f1-894f-908eeb9866d7"

#define NS_INAVHISTORYCONTAINERRESULTNODE_IID \
  {0x3e9cc95f, 0x0d93, 0x45f1, \
    { 0x89, 0x4f, 0x90, 0x8e, 0xeb, 0x98, 0x66, 0xd7 }}

class NS_NO_VTABLE nsINavHistoryContainerResultNode : public nsINavHistoryResultNode {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INAVHISTORYCONTAINERRESULTNODE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINavHistoryContainerResultNode;

  /* attribute boolean containerOpen; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContainerOpen(bool *aContainerOpen) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetContainerOpen(bool aContainerOpen) = 0;

  /* readonly attribute unsigned short state; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetState(uint16_t *aState) = 0;

  enum {
    STATE_CLOSED = 0U,
    STATE_LOADING = 1U,
    STATE_OPENED = 2U
  };

  /* readonly attribute boolean hasChildren; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHasChildren(bool *aHasChildren) = 0;

  /* readonly attribute unsigned long childCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetChildCount(uint32_t *aChildCount) = 0;

  /* nsINavHistoryResultNode getChild (in unsigned long aIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetChild(uint32_t aIndex, nsINavHistoryResultNode **_retval) = 0;

  /* unsigned long getChildIndex (in nsINavHistoryResultNode aNode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetChildIndex(nsINavHistoryResultNode *aNode, uint32_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINavHistoryContainerResultNode, NS_INAVHISTORYCONTAINERRESULTNODE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINAVHISTORYCONTAINERRESULTNODE \
  NS_IMETHOD GetContainerOpen(bool *aContainerOpen) override; \
  NS_IMETHOD SetContainerOpen(bool aContainerOpen) override; \
  NS_IMETHOD GetState(uint16_t *aState) override; \
  NS_IMETHOD GetHasChildren(bool *aHasChildren) override; \
  NS_IMETHOD GetChildCount(uint32_t *aChildCount) override; \
  NS_IMETHOD GetChild(uint32_t aIndex, nsINavHistoryResultNode **_retval) override; \
  NS_IMETHOD GetChildIndex(nsINavHistoryResultNode *aNode, uint32_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINAVHISTORYCONTAINERRESULTNODE \
  nsresult GetContainerOpen(bool *aContainerOpen); \
  nsresult SetContainerOpen(bool aContainerOpen); \
  nsresult GetState(uint16_t *aState); \
  nsresult GetHasChildren(bool *aHasChildren); \
  nsresult GetChildCount(uint32_t *aChildCount); \
  nsresult GetChild(uint32_t aIndex, nsINavHistoryResultNode **_retval); \
  nsresult GetChildIndex(nsINavHistoryResultNode *aNode, uint32_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINAVHISTORYCONTAINERRESULTNODE(_to) \
  NS_IMETHOD GetContainerOpen(bool *aContainerOpen) override { return _to GetContainerOpen(aContainerOpen); } \
  NS_IMETHOD SetContainerOpen(bool aContainerOpen) override { return _to SetContainerOpen(aContainerOpen); } \
  NS_IMETHOD GetState(uint16_t *aState) override { return _to GetState(aState); } \
  NS_IMETHOD GetHasChildren(bool *aHasChildren) override { return _to GetHasChildren(aHasChildren); } \
  NS_IMETHOD GetChildCount(uint32_t *aChildCount) override { return _to GetChildCount(aChildCount); } \
  NS_IMETHOD GetChild(uint32_t aIndex, nsINavHistoryResultNode **_retval) override { return _to GetChild(aIndex, _retval); } \
  NS_IMETHOD GetChildIndex(nsINavHistoryResultNode *aNode, uint32_t *_retval) override { return _to GetChildIndex(aNode, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINAVHISTORYCONTAINERRESULTNODE(_to) \
  NS_IMETHOD GetContainerOpen(bool *aContainerOpen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContainerOpen(aContainerOpen); } \
  NS_IMETHOD SetContainerOpen(bool aContainerOpen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContainerOpen(aContainerOpen); } \
  NS_IMETHOD GetState(uint16_t *aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetState(aState); } \
  NS_IMETHOD GetHasChildren(bool *aHasChildren) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasChildren(aHasChildren); } \
  NS_IMETHOD GetChildCount(uint32_t *aChildCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChildCount(aChildCount); } \
  NS_IMETHOD GetChild(uint32_t aIndex, nsINavHistoryResultNode **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChild(aIndex, _retval); } \
  NS_IMETHOD GetChildIndex(nsINavHistoryResultNode *aNode, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChildIndex(aNode, _retval); } 


/* starting interface:    nsINavHistoryQueryResultNode */
#define NS_INAVHISTORYQUERYRESULTNODE_IID_STR "62817759-4fee-44a3-b58c-3e2f5afc9d0a"

#define NS_INAVHISTORYQUERYRESULTNODE_IID \
  {0x62817759, 0x4fee, 0x44a3, \
    { 0xb5, 0x8c, 0x3e, 0x2f, 0x5a, 0xfc, 0x9d, 0x0a }}

class NS_NO_VTABLE nsINavHistoryQueryResultNode : public nsINavHistoryContainerResultNode {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INAVHISTORYQUERYRESULTNODE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINavHistoryQueryResultNode;

  /* readonly attribute nsINavHistoryQuery query; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetQuery(nsINavHistoryQuery **aQuery) = 0;

  /* readonly attribute nsINavHistoryQueryOptions queryOptions; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetQueryOptions(nsINavHistoryQueryOptions **aQueryOptions) = 0;

  /* readonly attribute long long folderItemId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFolderItemId(int64_t *aFolderItemId) = 0;

  /* readonly attribute ACString targetFolderGuid; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTargetFolderGuid(nsACString& aTargetFolderGuid) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINavHistoryQueryResultNode, NS_INAVHISTORYQUERYRESULTNODE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINAVHISTORYQUERYRESULTNODE \
  NS_IMETHOD GetQuery(nsINavHistoryQuery **aQuery) override; \
  NS_IMETHOD GetQueryOptions(nsINavHistoryQueryOptions **aQueryOptions) override; \
  NS_IMETHOD GetFolderItemId(int64_t *aFolderItemId) override; \
  NS_IMETHOD GetTargetFolderGuid(nsACString& aTargetFolderGuid) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINAVHISTORYQUERYRESULTNODE \
  nsresult GetQuery(nsINavHistoryQuery **aQuery); \
  nsresult GetQueryOptions(nsINavHistoryQueryOptions **aQueryOptions); \
  nsresult GetFolderItemId(int64_t *aFolderItemId); \
  nsresult GetTargetFolderGuid(nsACString& aTargetFolderGuid); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINAVHISTORYQUERYRESULTNODE(_to) \
  NS_IMETHOD GetQuery(nsINavHistoryQuery **aQuery) override { return _to GetQuery(aQuery); } \
  NS_IMETHOD GetQueryOptions(nsINavHistoryQueryOptions **aQueryOptions) override { return _to GetQueryOptions(aQueryOptions); } \
  NS_IMETHOD GetFolderItemId(int64_t *aFolderItemId) override { return _to GetFolderItemId(aFolderItemId); } \
  NS_IMETHOD GetTargetFolderGuid(nsACString& aTargetFolderGuid) override { return _to GetTargetFolderGuid(aTargetFolderGuid); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINAVHISTORYQUERYRESULTNODE(_to) \
  NS_IMETHOD GetQuery(nsINavHistoryQuery **aQuery) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetQuery(aQuery); } \
  NS_IMETHOD GetQueryOptions(nsINavHistoryQueryOptions **aQueryOptions) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetQueryOptions(aQueryOptions); } \
  NS_IMETHOD GetFolderItemId(int64_t *aFolderItemId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFolderItemId(aFolderItemId); } \
  NS_IMETHOD GetTargetFolderGuid(nsACString& aTargetFolderGuid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTargetFolderGuid(aTargetFolderGuid); } 


/* starting interface:    nsINavHistoryResultObserver */
#define NS_INAVHISTORYRESULTOBSERVER_IID_STR "f62d8b6b-3c4e-4a9f-a897-db605d0b7a0f"

#define NS_INAVHISTORYRESULTOBSERVER_IID \
  {0xf62d8b6b, 0x3c4e, 0x4a9f, \
    { 0xa8, 0x97, 0xdb, 0x60, 0x5d, 0x0b, 0x7a, 0x0f }}

class NS_NO_VTABLE nsINavHistoryResultObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INAVHISTORYRESULTOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINavHistoryResultObserver;

  /* readonly attribute boolean observeHistoryDetails; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetObserveHistoryDetails(bool *aObserveHistoryDetails) = 0;

  /* void nodeInserted (in nsINavHistoryContainerResultNode aParent, in nsINavHistoryResultNode aNode, in unsigned long aNewIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NodeInserted(nsINavHistoryContainerResultNode *aParent, nsINavHistoryResultNode *aNode, uint32_t aNewIndex) = 0;

  /* void nodeRemoved (in nsINavHistoryContainerResultNode aParent, in nsINavHistoryResultNode aItem, in unsigned long aOldIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NodeRemoved(nsINavHistoryContainerResultNode *aParent, nsINavHistoryResultNode *aItem, uint32_t aOldIndex) = 0;

  /* void nodeMoved (in nsINavHistoryResultNode aNode, in nsINavHistoryContainerResultNode aOldParent, in unsigned long aOldIndex, in nsINavHistoryContainerResultNode aNewParent, in unsigned long aNewIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NodeMoved(nsINavHistoryResultNode *aNode, nsINavHistoryContainerResultNode *aOldParent, uint32_t aOldIndex, nsINavHistoryContainerResultNode *aNewParent, uint32_t aNewIndex) = 0;

  /* void nodeTitleChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewTitle); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NodeTitleChanged(nsINavHistoryResultNode *aNode, const nsACString& aNewTitle) = 0;

  /* void nodeURIChanged (in nsINavHistoryResultNode aNode, in AUTF8String aOldURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NodeURIChanged(nsINavHistoryResultNode *aNode, const nsACString& aOldURI) = 0;

  /* void nodeIconChanged (in nsINavHistoryResultNode aNode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NodeIconChanged(nsINavHistoryResultNode *aNode) = 0;

  /* void nodeHistoryDetailsChanged (in nsINavHistoryResultNode aNode, in PRTime aOldVisitDate, in unsigned long aOldAccessCount); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NodeHistoryDetailsChanged(nsINavHistoryResultNode *aNode, PRTime aOldVisitDate, uint32_t aOldAccessCount) = 0;

  /* void nodeTagsChanged (in nsINavHistoryResultNode aNode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NodeTagsChanged(nsINavHistoryResultNode *aNode) = 0;

  /* void nodeKeywordChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewKeyword); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NodeKeywordChanged(nsINavHistoryResultNode *aNode, const nsACString& aNewKeyword) = 0;

  /* void nodeDateAddedChanged (in nsINavHistoryResultNode aNode, in PRTime aNewValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NodeDateAddedChanged(nsINavHistoryResultNode *aNode, PRTime aNewValue) = 0;

  /* void nodeLastModifiedChanged (in nsINavHistoryResultNode aNode, in PRTime aNewValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NodeLastModifiedChanged(nsINavHistoryResultNode *aNode, PRTime aNewValue) = 0;

  /* void containerStateChanged (in nsINavHistoryContainerResultNode aContainerNode, in unsigned long aOldState, in unsigned long aNewState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ContainerStateChanged(nsINavHistoryContainerResultNode *aContainerNode, uint32_t aOldState, uint32_t aNewState) = 0;

  /* void invalidateContainer (in nsINavHistoryContainerResultNode aContainerNode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InvalidateContainer(nsINavHistoryContainerResultNode *aContainerNode) = 0;

  /* void sortingChanged (in unsigned short sortingMode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SortingChanged(uint16_t sortingMode) = 0;

  /* void batching (in boolean aToggleMode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Batching(bool aToggleMode) = 0;

  /* attribute nsINavHistoryResult result; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetResult(nsINavHistoryResult **aResult) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetResult(nsINavHistoryResult *aResult) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINavHistoryResultObserver, NS_INAVHISTORYRESULTOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINAVHISTORYRESULTOBSERVER \
  NS_IMETHOD GetObserveHistoryDetails(bool *aObserveHistoryDetails) override; \
  NS_IMETHOD NodeInserted(nsINavHistoryContainerResultNode *aParent, nsINavHistoryResultNode *aNode, uint32_t aNewIndex) override; \
  NS_IMETHOD NodeRemoved(nsINavHistoryContainerResultNode *aParent, nsINavHistoryResultNode *aItem, uint32_t aOldIndex) override; \
  NS_IMETHOD NodeMoved(nsINavHistoryResultNode *aNode, nsINavHistoryContainerResultNode *aOldParent, uint32_t aOldIndex, nsINavHistoryContainerResultNode *aNewParent, uint32_t aNewIndex) override; \
  NS_IMETHOD NodeTitleChanged(nsINavHistoryResultNode *aNode, const nsACString& aNewTitle) override; \
  NS_IMETHOD NodeURIChanged(nsINavHistoryResultNode *aNode, const nsACString& aOldURI) override; \
  NS_IMETHOD NodeIconChanged(nsINavHistoryResultNode *aNode) override; \
  NS_IMETHOD NodeHistoryDetailsChanged(nsINavHistoryResultNode *aNode, PRTime aOldVisitDate, uint32_t aOldAccessCount) override; \
  NS_IMETHOD NodeTagsChanged(nsINavHistoryResultNode *aNode) override; \
  NS_IMETHOD NodeKeywordChanged(nsINavHistoryResultNode *aNode, const nsACString& aNewKeyword) override; \
  NS_IMETHOD NodeDateAddedChanged(nsINavHistoryResultNode *aNode, PRTime aNewValue) override; \
  NS_IMETHOD NodeLastModifiedChanged(nsINavHistoryResultNode *aNode, PRTime aNewValue) override; \
  NS_IMETHOD ContainerStateChanged(nsINavHistoryContainerResultNode *aContainerNode, uint32_t aOldState, uint32_t aNewState) override; \
  NS_IMETHOD InvalidateContainer(nsINavHistoryContainerResultNode *aContainerNode) override; \
  NS_IMETHOD SortingChanged(uint16_t sortingMode) override; \
  NS_IMETHOD Batching(bool aToggleMode) override; \
  NS_IMETHOD GetResult(nsINavHistoryResult **aResult) override; \
  NS_IMETHOD SetResult(nsINavHistoryResult *aResult) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINAVHISTORYRESULTOBSERVER \
  nsresult GetObserveHistoryDetails(bool *aObserveHistoryDetails); \
  nsresult NodeInserted(nsINavHistoryContainerResultNode *aParent, nsINavHistoryResultNode *aNode, uint32_t aNewIndex); \
  nsresult NodeRemoved(nsINavHistoryContainerResultNode *aParent, nsINavHistoryResultNode *aItem, uint32_t aOldIndex); \
  nsresult NodeMoved(nsINavHistoryResultNode *aNode, nsINavHistoryContainerResultNode *aOldParent, uint32_t aOldIndex, nsINavHistoryContainerResultNode *aNewParent, uint32_t aNewIndex); \
  nsresult NodeTitleChanged(nsINavHistoryResultNode *aNode, const nsACString& aNewTitle); \
  nsresult NodeURIChanged(nsINavHistoryResultNode *aNode, const nsACString& aOldURI); \
  nsresult NodeIconChanged(nsINavHistoryResultNode *aNode); \
  nsresult NodeHistoryDetailsChanged(nsINavHistoryResultNode *aNode, PRTime aOldVisitDate, uint32_t aOldAccessCount); \
  nsresult NodeTagsChanged(nsINavHistoryResultNode *aNode); \
  nsresult NodeKeywordChanged(nsINavHistoryResultNode *aNode, const nsACString& aNewKeyword); \
  nsresult NodeDateAddedChanged(nsINavHistoryResultNode *aNode, PRTime aNewValue); \
  nsresult NodeLastModifiedChanged(nsINavHistoryResultNode *aNode, PRTime aNewValue); \
  nsresult ContainerStateChanged(nsINavHistoryContainerResultNode *aContainerNode, uint32_t aOldState, uint32_t aNewState); \
  nsresult InvalidateContainer(nsINavHistoryContainerResultNode *aContainerNode); \
  nsresult SortingChanged(uint16_t sortingMode); \
  nsresult Batching(bool aToggleMode); \
  nsresult GetResult(nsINavHistoryResult **aResult); \
  nsresult SetResult(nsINavHistoryResult *aResult); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINAVHISTORYRESULTOBSERVER(_to) \
  NS_IMETHOD GetObserveHistoryDetails(bool *aObserveHistoryDetails) override { return _to GetObserveHistoryDetails(aObserveHistoryDetails); } \
  NS_IMETHOD NodeInserted(nsINavHistoryContainerResultNode *aParent, nsINavHistoryResultNode *aNode, uint32_t aNewIndex) override { return _to NodeInserted(aParent, aNode, aNewIndex); } \
  NS_IMETHOD NodeRemoved(nsINavHistoryContainerResultNode *aParent, nsINavHistoryResultNode *aItem, uint32_t aOldIndex) override { return _to NodeRemoved(aParent, aItem, aOldIndex); } \
  NS_IMETHOD NodeMoved(nsINavHistoryResultNode *aNode, nsINavHistoryContainerResultNode *aOldParent, uint32_t aOldIndex, nsINavHistoryContainerResultNode *aNewParent, uint32_t aNewIndex) override { return _to NodeMoved(aNode, aOldParent, aOldIndex, aNewParent, aNewIndex); } \
  NS_IMETHOD NodeTitleChanged(nsINavHistoryResultNode *aNode, const nsACString& aNewTitle) override { return _to NodeTitleChanged(aNode, aNewTitle); } \
  NS_IMETHOD NodeURIChanged(nsINavHistoryResultNode *aNode, const nsACString& aOldURI) override { return _to NodeURIChanged(aNode, aOldURI); } \
  NS_IMETHOD NodeIconChanged(nsINavHistoryResultNode *aNode) override { return _to NodeIconChanged(aNode); } \
  NS_IMETHOD NodeHistoryDetailsChanged(nsINavHistoryResultNode *aNode, PRTime aOldVisitDate, uint32_t aOldAccessCount) override { return _to NodeHistoryDetailsChanged(aNode, aOldVisitDate, aOldAccessCount); } \
  NS_IMETHOD NodeTagsChanged(nsINavHistoryResultNode *aNode) override { return _to NodeTagsChanged(aNode); } \
  NS_IMETHOD NodeKeywordChanged(nsINavHistoryResultNode *aNode, const nsACString& aNewKeyword) override { return _to NodeKeywordChanged(aNode, aNewKeyword); } \
  NS_IMETHOD NodeDateAddedChanged(nsINavHistoryResultNode *aNode, PRTime aNewValue) override { return _to NodeDateAddedChanged(aNode, aNewValue); } \
  NS_IMETHOD NodeLastModifiedChanged(nsINavHistoryResultNode *aNode, PRTime aNewValue) override { return _to NodeLastModifiedChanged(aNode, aNewValue); } \
  NS_IMETHOD ContainerStateChanged(nsINavHistoryContainerResultNode *aContainerNode, uint32_t aOldState, uint32_t aNewState) override { return _to ContainerStateChanged(aContainerNode, aOldState, aNewState); } \
  NS_IMETHOD InvalidateContainer(nsINavHistoryContainerResultNode *aContainerNode) override { return _to InvalidateContainer(aContainerNode); } \
  NS_IMETHOD SortingChanged(uint16_t sortingMode) override { return _to SortingChanged(sortingMode); } \
  NS_IMETHOD Batching(bool aToggleMode) override { return _to Batching(aToggleMode); } \
  NS_IMETHOD GetResult(nsINavHistoryResult **aResult) override { return _to GetResult(aResult); } \
  NS_IMETHOD SetResult(nsINavHistoryResult *aResult) override { return _to SetResult(aResult); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINAVHISTORYRESULTOBSERVER(_to) \
  NS_IMETHOD GetObserveHistoryDetails(bool *aObserveHistoryDetails) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetObserveHistoryDetails(aObserveHistoryDetails); } \
  NS_IMETHOD NodeInserted(nsINavHistoryContainerResultNode *aParent, nsINavHistoryResultNode *aNode, uint32_t aNewIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NodeInserted(aParent, aNode, aNewIndex); } \
  NS_IMETHOD NodeRemoved(nsINavHistoryContainerResultNode *aParent, nsINavHistoryResultNode *aItem, uint32_t aOldIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NodeRemoved(aParent, aItem, aOldIndex); } \
  NS_IMETHOD NodeMoved(nsINavHistoryResultNode *aNode, nsINavHistoryContainerResultNode *aOldParent, uint32_t aOldIndex, nsINavHistoryContainerResultNode *aNewParent, uint32_t aNewIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NodeMoved(aNode, aOldParent, aOldIndex, aNewParent, aNewIndex); } \
  NS_IMETHOD NodeTitleChanged(nsINavHistoryResultNode *aNode, const nsACString& aNewTitle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NodeTitleChanged(aNode, aNewTitle); } \
  NS_IMETHOD NodeURIChanged(nsINavHistoryResultNode *aNode, const nsACString& aOldURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NodeURIChanged(aNode, aOldURI); } \
  NS_IMETHOD NodeIconChanged(nsINavHistoryResultNode *aNode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NodeIconChanged(aNode); } \
  NS_IMETHOD NodeHistoryDetailsChanged(nsINavHistoryResultNode *aNode, PRTime aOldVisitDate, uint32_t aOldAccessCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NodeHistoryDetailsChanged(aNode, aOldVisitDate, aOldAccessCount); } \
  NS_IMETHOD NodeTagsChanged(nsINavHistoryResultNode *aNode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NodeTagsChanged(aNode); } \
  NS_IMETHOD NodeKeywordChanged(nsINavHistoryResultNode *aNode, const nsACString& aNewKeyword) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NodeKeywordChanged(aNode, aNewKeyword); } \
  NS_IMETHOD NodeDateAddedChanged(nsINavHistoryResultNode *aNode, PRTime aNewValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NodeDateAddedChanged(aNode, aNewValue); } \
  NS_IMETHOD NodeLastModifiedChanged(nsINavHistoryResultNode *aNode, PRTime aNewValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NodeLastModifiedChanged(aNode, aNewValue); } \
  NS_IMETHOD ContainerStateChanged(nsINavHistoryContainerResultNode *aContainerNode, uint32_t aOldState, uint32_t aNewState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ContainerStateChanged(aContainerNode, aOldState, aNewState); } \
  NS_IMETHOD InvalidateContainer(nsINavHistoryContainerResultNode *aContainerNode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InvalidateContainer(aContainerNode); } \
  NS_IMETHOD SortingChanged(uint16_t sortingMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SortingChanged(sortingMode); } \
  NS_IMETHOD Batching(bool aToggleMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Batching(aToggleMode); } \
  NS_IMETHOD GetResult(nsINavHistoryResult **aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResult(aResult); } \
  NS_IMETHOD SetResult(nsINavHistoryResult *aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetResult(aResult); } 


/* starting interface:    nsINavHistoryResult */
#define NS_INAVHISTORYRESULT_IID_STR "c2229ce3-2159-4001-859c-7013c52f7619"

#define NS_INAVHISTORYRESULT_IID \
  {0xc2229ce3, 0x2159, 0x4001, \
    { 0x85, 0x9c, 0x70, 0x13, 0xc5, 0x2f, 0x76, 0x19 }}

class NS_NO_VTABLE nsINavHistoryResult : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INAVHISTORYRESULT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINavHistoryResult;

  /* attribute unsigned short sortingMode; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSortingMode(uint16_t *aSortingMode) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSortingMode(uint16_t aSortingMode) = 0;

  /* attribute boolean suppressNotifications; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSuppressNotifications(bool *aSuppressNotifications) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSuppressNotifications(bool aSuppressNotifications) = 0;

  /* void addObserver (in nsINavHistoryResultObserver aObserver, [optional] in boolean aOwnsWeak); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddObserver(nsINavHistoryResultObserver *aObserver, bool aOwnsWeak) = 0;

  /* void removeObserver (in nsINavHistoryResultObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveObserver(nsINavHistoryResultObserver *aObserver) = 0;

  /* readonly attribute nsINavHistoryContainerResultNode root; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRoot(nsINavHistoryContainerResultNode **aRoot) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINavHistoryResult, NS_INAVHISTORYRESULT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINAVHISTORYRESULT \
  NS_IMETHOD GetSortingMode(uint16_t *aSortingMode) override; \
  NS_IMETHOD SetSortingMode(uint16_t aSortingMode) override; \
  NS_IMETHOD GetSuppressNotifications(bool *aSuppressNotifications) override; \
  NS_IMETHOD SetSuppressNotifications(bool aSuppressNotifications) override; \
  NS_IMETHOD AddObserver(nsINavHistoryResultObserver *aObserver, bool aOwnsWeak) override; \
  NS_IMETHOD RemoveObserver(nsINavHistoryResultObserver *aObserver) override; \
  NS_IMETHOD GetRoot(nsINavHistoryContainerResultNode **aRoot) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINAVHISTORYRESULT \
  nsresult GetSortingMode(uint16_t *aSortingMode); \
  nsresult SetSortingMode(uint16_t aSortingMode); \
  nsresult GetSuppressNotifications(bool *aSuppressNotifications); \
  nsresult SetSuppressNotifications(bool aSuppressNotifications); \
  nsresult AddObserver(nsINavHistoryResultObserver *aObserver, bool aOwnsWeak); \
  nsresult RemoveObserver(nsINavHistoryResultObserver *aObserver); \
  nsresult GetRoot(nsINavHistoryContainerResultNode **aRoot); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINAVHISTORYRESULT(_to) \
  NS_IMETHOD GetSortingMode(uint16_t *aSortingMode) override { return _to GetSortingMode(aSortingMode); } \
  NS_IMETHOD SetSortingMode(uint16_t aSortingMode) override { return _to SetSortingMode(aSortingMode); } \
  NS_IMETHOD GetSuppressNotifications(bool *aSuppressNotifications) override { return _to GetSuppressNotifications(aSuppressNotifications); } \
  NS_IMETHOD SetSuppressNotifications(bool aSuppressNotifications) override { return _to SetSuppressNotifications(aSuppressNotifications); } \
  NS_IMETHOD AddObserver(nsINavHistoryResultObserver *aObserver, bool aOwnsWeak) override { return _to AddObserver(aObserver, aOwnsWeak); } \
  NS_IMETHOD RemoveObserver(nsINavHistoryResultObserver *aObserver) override { return _to RemoveObserver(aObserver); } \
  NS_IMETHOD GetRoot(nsINavHistoryContainerResultNode **aRoot) override { return _to GetRoot(aRoot); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINAVHISTORYRESULT(_to) \
  NS_IMETHOD GetSortingMode(uint16_t *aSortingMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSortingMode(aSortingMode); } \
  NS_IMETHOD SetSortingMode(uint16_t aSortingMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSortingMode(aSortingMode); } \
  NS_IMETHOD GetSuppressNotifications(bool *aSuppressNotifications) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSuppressNotifications(aSuppressNotifications); } \
  NS_IMETHOD SetSuppressNotifications(bool aSuppressNotifications) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSuppressNotifications(aSuppressNotifications); } \
  NS_IMETHOD AddObserver(nsINavHistoryResultObserver *aObserver, bool aOwnsWeak) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddObserver(aObserver, aOwnsWeak); } \
  NS_IMETHOD RemoveObserver(nsINavHistoryResultObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveObserver(aObserver); } \
  NS_IMETHOD GetRoot(nsINavHistoryContainerResultNode **aRoot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRoot(aRoot); } 


/* starting interface:    nsINavHistoryObserver */
#define NS_INAVHISTORYOBSERVER_IID_STR "0f0f45b0-13a1-44ae-a0ab-c6046ec6d4da"

#define NS_INAVHISTORYOBSERVER_IID \
  {0x0f0f45b0, 0x13a1, 0x44ae, \
    { 0xa0, 0xab, 0xc6, 0x04, 0x6e, 0xc6, 0xd4, 0xda }}

class NS_NO_VTABLE nsINavHistoryObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INAVHISTORYOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINavHistoryObserver;

  /* void onBeginUpdateBatch (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnBeginUpdateBatch(void) = 0;

  /* void onEndUpdateBatch (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnEndUpdateBatch(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINavHistoryObserver, NS_INAVHISTORYOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINAVHISTORYOBSERVER \
  NS_IMETHOD OnBeginUpdateBatch(void) override; \
  NS_IMETHOD OnEndUpdateBatch(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINAVHISTORYOBSERVER \
  nsresult OnBeginUpdateBatch(void); \
  nsresult OnEndUpdateBatch(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINAVHISTORYOBSERVER(_to) \
  NS_IMETHOD OnBeginUpdateBatch(void) override { return _to OnBeginUpdateBatch(); } \
  NS_IMETHOD OnEndUpdateBatch(void) override { return _to OnEndUpdateBatch(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINAVHISTORYOBSERVER(_to) \
  NS_IMETHOD OnBeginUpdateBatch(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnBeginUpdateBatch(); } \
  NS_IMETHOD OnEndUpdateBatch(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnEndUpdateBatch(); } 


/* starting interface:    nsINavHistoryQuery */
#define NS_INAVHISTORYQUERY_IID_STR "dc87ae79-22f1-4dcf-975b-852b01d210cb"

#define NS_INAVHISTORYQUERY_IID \
  {0xdc87ae79, 0x22f1, 0x4dcf, \
    { 0x97, 0x5b, 0x85, 0x2b, 0x01, 0xd2, 0x10, 0xcb }}

class NS_NO_VTABLE nsINavHistoryQuery : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INAVHISTORYQUERY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINavHistoryQuery;

  enum {
    TIME_RELATIVE_EPOCH = 0U,
    TIME_RELATIVE_TODAY = 1U,
    TIME_RELATIVE_NOW = 2U
  };

  /* attribute PRTime beginTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBeginTime(PRTime *aBeginTime) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetBeginTime(PRTime aBeginTime) = 0;

  /* attribute unsigned long beginTimeReference; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBeginTimeReference(uint32_t *aBeginTimeReference) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetBeginTimeReference(uint32_t aBeginTimeReference) = 0;

  /* readonly attribute boolean hasBeginTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHasBeginTime(bool *aHasBeginTime) = 0;

  /* readonly attribute PRTime absoluteBeginTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAbsoluteBeginTime(PRTime *aAbsoluteBeginTime) = 0;

  /* attribute PRTime endTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEndTime(PRTime *aEndTime) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetEndTime(PRTime aEndTime) = 0;

  /* attribute unsigned long endTimeReference; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEndTimeReference(uint32_t *aEndTimeReference) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetEndTimeReference(uint32_t aEndTimeReference) = 0;

  /* readonly attribute boolean hasEndTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHasEndTime(bool *aHasEndTime) = 0;

  /* readonly attribute PRTime absoluteEndTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAbsoluteEndTime(PRTime *aAbsoluteEndTime) = 0;

  /* attribute AString searchTerms; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchTerms(nsAString& aSearchTerms) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSearchTerms(const nsAString& aSearchTerms) = 0;

  /* readonly attribute boolean hasSearchTerms; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHasSearchTerms(bool *aHasSearchTerms) = 0;

  /* attribute long minVisits; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMinVisits(int32_t *aMinVisits) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMinVisits(int32_t aMinVisits) = 0;

  /* attribute long maxVisits; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMaxVisits(int32_t *aMaxVisits) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMaxVisits(int32_t aMaxVisits) = 0;

  /* void setTransitions (in Array<unsigned long> transitions); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTransitions(const nsTArray<uint32_t >& transitions) = 0;

  /* Array<unsigned long> getTransitions (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTransitions(nsTArray<uint32_t >& _retval) = 0;

  /* readonly attribute unsigned long transitionCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTransitionCount(uint32_t *aTransitionCount) = 0;

  /* attribute boolean onlyBookmarked; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOnlyBookmarked(bool *aOnlyBookmarked) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetOnlyBookmarked(bool aOnlyBookmarked) = 0;

  /* attribute boolean domainIsHost; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDomainIsHost(bool *aDomainIsHost) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDomainIsHost(bool aDomainIsHost) = 0;

  /* attribute AUTF8String domain; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDomain(nsACString& aDomain) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDomain(const nsACString& aDomain) = 0;

  /* readonly attribute boolean hasDomain; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHasDomain(bool *aHasDomain) = 0;

  /* attribute nsIURI uri; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUri(nsIURI **aUri) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetUri(nsIURI *aUri) = 0;

  /* readonly attribute boolean hasUri; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHasUri(bool *aHasUri) = 0;

  /* attribute boolean annotationIsNot; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAnnotationIsNot(bool *aAnnotationIsNot) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetAnnotationIsNot(bool aAnnotationIsNot) = 0;

  /* attribute AUTF8String annotation; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAnnotation(nsACString& aAnnotation) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetAnnotation(const nsACString& aAnnotation) = 0;

  /* readonly attribute boolean hasAnnotation; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHasAnnotation(bool *aHasAnnotation) = 0;

  /* attribute nsIVariant tags; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTags(nsIVariant **aTags) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTags(nsIVariant *aTags) = 0;

  /* attribute boolean tagsAreNot; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTagsAreNot(bool *aTagsAreNot) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTagsAreNot(bool aTagsAreNot) = 0;

  /* Array<ACString> getParents (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetParents(nsTArray<nsCString >& _retval) = 0;

  /* readonly attribute unsigned long parentCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetParentCount(uint32_t *aParentCount) = 0;

  /* void setParents (in Array<ACString> aGuids); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetParents(const nsTArray<nsCString >& aGuids) = 0;

  /* nsINavHistoryQuery clone (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Clone(nsINavHistoryQuery **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINavHistoryQuery, NS_INAVHISTORYQUERY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINAVHISTORYQUERY \
  NS_IMETHOD GetBeginTime(PRTime *aBeginTime) override; \
  NS_IMETHOD SetBeginTime(PRTime aBeginTime) override; \
  NS_IMETHOD GetBeginTimeReference(uint32_t *aBeginTimeReference) override; \
  NS_IMETHOD SetBeginTimeReference(uint32_t aBeginTimeReference) override; \
  NS_IMETHOD GetHasBeginTime(bool *aHasBeginTime) override; \
  NS_IMETHOD GetAbsoluteBeginTime(PRTime *aAbsoluteBeginTime) override; \
  NS_IMETHOD GetEndTime(PRTime *aEndTime) override; \
  NS_IMETHOD SetEndTime(PRTime aEndTime) override; \
  NS_IMETHOD GetEndTimeReference(uint32_t *aEndTimeReference) override; \
  NS_IMETHOD SetEndTimeReference(uint32_t aEndTimeReference) override; \
  NS_IMETHOD GetHasEndTime(bool *aHasEndTime) override; \
  NS_IMETHOD GetAbsoluteEndTime(PRTime *aAbsoluteEndTime) override; \
  NS_IMETHOD GetSearchTerms(nsAString& aSearchTerms) override; \
  NS_IMETHOD SetSearchTerms(const nsAString& aSearchTerms) override; \
  NS_IMETHOD GetHasSearchTerms(bool *aHasSearchTerms) override; \
  NS_IMETHOD GetMinVisits(int32_t *aMinVisits) override; \
  NS_IMETHOD SetMinVisits(int32_t aMinVisits) override; \
  NS_IMETHOD GetMaxVisits(int32_t *aMaxVisits) override; \
  NS_IMETHOD SetMaxVisits(int32_t aMaxVisits) override; \
  NS_IMETHOD SetTransitions(const nsTArray<uint32_t >& transitions) override; \
  NS_IMETHOD GetTransitions(nsTArray<uint32_t >& _retval) override; \
  NS_IMETHOD GetTransitionCount(uint32_t *aTransitionCount) override; \
  NS_IMETHOD GetOnlyBookmarked(bool *aOnlyBookmarked) override; \
  NS_IMETHOD SetOnlyBookmarked(bool aOnlyBookmarked) override; \
  NS_IMETHOD GetDomainIsHost(bool *aDomainIsHost) override; \
  NS_IMETHOD SetDomainIsHost(bool aDomainIsHost) override; \
  NS_IMETHOD GetDomain(nsACString& aDomain) override; \
  NS_IMETHOD SetDomain(const nsACString& aDomain) override; \
  NS_IMETHOD GetHasDomain(bool *aHasDomain) override; \
  NS_IMETHOD GetUri(nsIURI **aUri) override; \
  NS_IMETHOD SetUri(nsIURI *aUri) override; \
  NS_IMETHOD GetHasUri(bool *aHasUri) override; \
  NS_IMETHOD GetAnnotationIsNot(bool *aAnnotationIsNot) override; \
  NS_IMETHOD SetAnnotationIsNot(bool aAnnotationIsNot) override; \
  NS_IMETHOD GetAnnotation(nsACString& aAnnotation) override; \
  NS_IMETHOD SetAnnotation(const nsACString& aAnnotation) override; \
  NS_IMETHOD GetHasAnnotation(bool *aHasAnnotation) override; \
  NS_IMETHOD GetTags(nsIVariant **aTags) override; \
  NS_IMETHOD SetTags(nsIVariant *aTags) override; \
  NS_IMETHOD GetTagsAreNot(bool *aTagsAreNot) override; \
  NS_IMETHOD SetTagsAreNot(bool aTagsAreNot) override; \
  NS_IMETHOD GetParents(nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD GetParentCount(uint32_t *aParentCount) override; \
  NS_IMETHOD SetParents(const nsTArray<nsCString >& aGuids) override; \
  NS_IMETHOD Clone(nsINavHistoryQuery **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINAVHISTORYQUERY \
  nsresult GetBeginTime(PRTime *aBeginTime); \
  nsresult SetBeginTime(PRTime aBeginTime); \
  nsresult GetBeginTimeReference(uint32_t *aBeginTimeReference); \
  nsresult SetBeginTimeReference(uint32_t aBeginTimeReference); \
  nsresult GetHasBeginTime(bool *aHasBeginTime); \
  nsresult GetAbsoluteBeginTime(PRTime *aAbsoluteBeginTime); \
  nsresult GetEndTime(PRTime *aEndTime); \
  nsresult SetEndTime(PRTime aEndTime); \
  nsresult GetEndTimeReference(uint32_t *aEndTimeReference); \
  nsresult SetEndTimeReference(uint32_t aEndTimeReference); \
  nsresult GetHasEndTime(bool *aHasEndTime); \
  nsresult GetAbsoluteEndTime(PRTime *aAbsoluteEndTime); \
  nsresult GetSearchTerms(nsAString& aSearchTerms); \
  nsresult SetSearchTerms(const nsAString& aSearchTerms); \
  nsresult GetHasSearchTerms(bool *aHasSearchTerms); \
  nsresult GetMinVisits(int32_t *aMinVisits); \
  nsresult SetMinVisits(int32_t aMinVisits); \
  nsresult GetMaxVisits(int32_t *aMaxVisits); \
  nsresult SetMaxVisits(int32_t aMaxVisits); \
  nsresult SetTransitions(const nsTArray<uint32_t >& transitions); \
  nsresult GetTransitions(nsTArray<uint32_t >& _retval); \
  nsresult GetTransitionCount(uint32_t *aTransitionCount); \
  nsresult GetOnlyBookmarked(bool *aOnlyBookmarked); \
  nsresult SetOnlyBookmarked(bool aOnlyBookmarked); \
  nsresult GetDomainIsHost(bool *aDomainIsHost); \
  nsresult SetDomainIsHost(bool aDomainIsHost); \
  nsresult GetDomain(nsACString& aDomain); \
  nsresult SetDomain(const nsACString& aDomain); \
  nsresult GetHasDomain(bool *aHasDomain); \
  nsresult GetUri(nsIURI **aUri); \
  nsresult SetUri(nsIURI *aUri); \
  nsresult GetHasUri(bool *aHasUri); \
  nsresult GetAnnotationIsNot(bool *aAnnotationIsNot); \
  nsresult SetAnnotationIsNot(bool aAnnotationIsNot); \
  nsresult GetAnnotation(nsACString& aAnnotation); \
  nsresult SetAnnotation(const nsACString& aAnnotation); \
  nsresult GetHasAnnotation(bool *aHasAnnotation); \
  nsresult GetTags(nsIVariant **aTags); \
  nsresult SetTags(nsIVariant *aTags); \
  nsresult GetTagsAreNot(bool *aTagsAreNot); \
  nsresult SetTagsAreNot(bool aTagsAreNot); \
  nsresult GetParents(nsTArray<nsCString >& _retval); \
  nsresult GetParentCount(uint32_t *aParentCount); \
  nsresult SetParents(const nsTArray<nsCString >& aGuids); \
  nsresult Clone(nsINavHistoryQuery **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINAVHISTORYQUERY(_to) \
  NS_IMETHOD GetBeginTime(PRTime *aBeginTime) override { return _to GetBeginTime(aBeginTime); } \
  NS_IMETHOD SetBeginTime(PRTime aBeginTime) override { return _to SetBeginTime(aBeginTime); } \
  NS_IMETHOD GetBeginTimeReference(uint32_t *aBeginTimeReference) override { return _to GetBeginTimeReference(aBeginTimeReference); } \
  NS_IMETHOD SetBeginTimeReference(uint32_t aBeginTimeReference) override { return _to SetBeginTimeReference(aBeginTimeReference); } \
  NS_IMETHOD GetHasBeginTime(bool *aHasBeginTime) override { return _to GetHasBeginTime(aHasBeginTime); } \
  NS_IMETHOD GetAbsoluteBeginTime(PRTime *aAbsoluteBeginTime) override { return _to GetAbsoluteBeginTime(aAbsoluteBeginTime); } \
  NS_IMETHOD GetEndTime(PRTime *aEndTime) override { return _to GetEndTime(aEndTime); } \
  NS_IMETHOD SetEndTime(PRTime aEndTime) override { return _to SetEndTime(aEndTime); } \
  NS_IMETHOD GetEndTimeReference(uint32_t *aEndTimeReference) override { return _to GetEndTimeReference(aEndTimeReference); } \
  NS_IMETHOD SetEndTimeReference(uint32_t aEndTimeReference) override { return _to SetEndTimeReference(aEndTimeReference); } \
  NS_IMETHOD GetHasEndTime(bool *aHasEndTime) override { return _to GetHasEndTime(aHasEndTime); } \
  NS_IMETHOD GetAbsoluteEndTime(PRTime *aAbsoluteEndTime) override { return _to GetAbsoluteEndTime(aAbsoluteEndTime); } \
  NS_IMETHOD GetSearchTerms(nsAString& aSearchTerms) override { return _to GetSearchTerms(aSearchTerms); } \
  NS_IMETHOD SetSearchTerms(const nsAString& aSearchTerms) override { return _to SetSearchTerms(aSearchTerms); } \
  NS_IMETHOD GetHasSearchTerms(bool *aHasSearchTerms) override { return _to GetHasSearchTerms(aHasSearchTerms); } \
  NS_IMETHOD GetMinVisits(int32_t *aMinVisits) override { return _to GetMinVisits(aMinVisits); } \
  NS_IMETHOD SetMinVisits(int32_t aMinVisits) override { return _to SetMinVisits(aMinVisits); } \
  NS_IMETHOD GetMaxVisits(int32_t *aMaxVisits) override { return _to GetMaxVisits(aMaxVisits); } \
  NS_IMETHOD SetMaxVisits(int32_t aMaxVisits) override { return _to SetMaxVisits(aMaxVisits); } \
  NS_IMETHOD SetTransitions(const nsTArray<uint32_t >& transitions) override { return _to SetTransitions(transitions); } \
  NS_IMETHOD GetTransitions(nsTArray<uint32_t >& _retval) override { return _to GetTransitions(_retval); } \
  NS_IMETHOD GetTransitionCount(uint32_t *aTransitionCount) override { return _to GetTransitionCount(aTransitionCount); } \
  NS_IMETHOD GetOnlyBookmarked(bool *aOnlyBookmarked) override { return _to GetOnlyBookmarked(aOnlyBookmarked); } \
  NS_IMETHOD SetOnlyBookmarked(bool aOnlyBookmarked) override { return _to SetOnlyBookmarked(aOnlyBookmarked); } \
  NS_IMETHOD GetDomainIsHost(bool *aDomainIsHost) override { return _to GetDomainIsHost(aDomainIsHost); } \
  NS_IMETHOD SetDomainIsHost(bool aDomainIsHost) override { return _to SetDomainIsHost(aDomainIsHost); } \
  NS_IMETHOD GetDomain(nsACString& aDomain) override { return _to GetDomain(aDomain); } \
  NS_IMETHOD SetDomain(const nsACString& aDomain) override { return _to SetDomain(aDomain); } \
  NS_IMETHOD GetHasDomain(bool *aHasDomain) override { return _to GetHasDomain(aHasDomain); } \
  NS_IMETHOD GetUri(nsIURI **aUri) override { return _to GetUri(aUri); } \
  NS_IMETHOD SetUri(nsIURI *aUri) override { return _to SetUri(aUri); } \
  NS_IMETHOD GetHasUri(bool *aHasUri) override { return _to GetHasUri(aHasUri); } \
  NS_IMETHOD GetAnnotationIsNot(bool *aAnnotationIsNot) override { return _to GetAnnotationIsNot(aAnnotationIsNot); } \
  NS_IMETHOD SetAnnotationIsNot(bool aAnnotationIsNot) override { return _to SetAnnotationIsNot(aAnnotationIsNot); } \
  NS_IMETHOD GetAnnotation(nsACString& aAnnotation) override { return _to GetAnnotation(aAnnotation); } \
  NS_IMETHOD SetAnnotation(const nsACString& aAnnotation) override { return _to SetAnnotation(aAnnotation); } \
  NS_IMETHOD GetHasAnnotation(bool *aHasAnnotation) override { return _to GetHasAnnotation(aHasAnnotation); } \
  NS_IMETHOD GetTags(nsIVariant **aTags) override { return _to GetTags(aTags); } \
  NS_IMETHOD SetTags(nsIVariant *aTags) override { return _to SetTags(aTags); } \
  NS_IMETHOD GetTagsAreNot(bool *aTagsAreNot) override { return _to GetTagsAreNot(aTagsAreNot); } \
  NS_IMETHOD SetTagsAreNot(bool aTagsAreNot) override { return _to SetTagsAreNot(aTagsAreNot); } \
  NS_IMETHOD GetParents(nsTArray<nsCString >& _retval) override { return _to GetParents(_retval); } \
  NS_IMETHOD GetParentCount(uint32_t *aParentCount) override { return _to GetParentCount(aParentCount); } \
  NS_IMETHOD SetParents(const nsTArray<nsCString >& aGuids) override { return _to SetParents(aGuids); } \
  NS_IMETHOD Clone(nsINavHistoryQuery **_retval) override { return _to Clone(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINAVHISTORYQUERY(_to) \
  NS_IMETHOD GetBeginTime(PRTime *aBeginTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBeginTime(aBeginTime); } \
  NS_IMETHOD SetBeginTime(PRTime aBeginTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBeginTime(aBeginTime); } \
  NS_IMETHOD GetBeginTimeReference(uint32_t *aBeginTimeReference) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBeginTimeReference(aBeginTimeReference); } \
  NS_IMETHOD SetBeginTimeReference(uint32_t aBeginTimeReference) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBeginTimeReference(aBeginTimeReference); } \
  NS_IMETHOD GetHasBeginTime(bool *aHasBeginTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasBeginTime(aHasBeginTime); } \
  NS_IMETHOD GetAbsoluteBeginTime(PRTime *aAbsoluteBeginTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAbsoluteBeginTime(aAbsoluteBeginTime); } \
  NS_IMETHOD GetEndTime(PRTime *aEndTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEndTime(aEndTime); } \
  NS_IMETHOD SetEndTime(PRTime aEndTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEndTime(aEndTime); } \
  NS_IMETHOD GetEndTimeReference(uint32_t *aEndTimeReference) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEndTimeReference(aEndTimeReference); } \
  NS_IMETHOD SetEndTimeReference(uint32_t aEndTimeReference) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEndTimeReference(aEndTimeReference); } \
  NS_IMETHOD GetHasEndTime(bool *aHasEndTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasEndTime(aHasEndTime); } \
  NS_IMETHOD GetAbsoluteEndTime(PRTime *aAbsoluteEndTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAbsoluteEndTime(aAbsoluteEndTime); } \
  NS_IMETHOD GetSearchTerms(nsAString& aSearchTerms) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchTerms(aSearchTerms); } \
  NS_IMETHOD SetSearchTerms(const nsAString& aSearchTerms) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSearchTerms(aSearchTerms); } \
  NS_IMETHOD GetHasSearchTerms(bool *aHasSearchTerms) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasSearchTerms(aHasSearchTerms); } \
  NS_IMETHOD GetMinVisits(int32_t *aMinVisits) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMinVisits(aMinVisits); } \
  NS_IMETHOD SetMinVisits(int32_t aMinVisits) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMinVisits(aMinVisits); } \
  NS_IMETHOD GetMaxVisits(int32_t *aMaxVisits) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMaxVisits(aMaxVisits); } \
  NS_IMETHOD SetMaxVisits(int32_t aMaxVisits) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMaxVisits(aMaxVisits); } \
  NS_IMETHOD SetTransitions(const nsTArray<uint32_t >& transitions) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTransitions(transitions); } \
  NS_IMETHOD GetTransitions(nsTArray<uint32_t >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTransitions(_retval); } \
  NS_IMETHOD GetTransitionCount(uint32_t *aTransitionCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTransitionCount(aTransitionCount); } \
  NS_IMETHOD GetOnlyBookmarked(bool *aOnlyBookmarked) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOnlyBookmarked(aOnlyBookmarked); } \
  NS_IMETHOD SetOnlyBookmarked(bool aOnlyBookmarked) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOnlyBookmarked(aOnlyBookmarked); } \
  NS_IMETHOD GetDomainIsHost(bool *aDomainIsHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDomainIsHost(aDomainIsHost); } \
  NS_IMETHOD SetDomainIsHost(bool aDomainIsHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDomainIsHost(aDomainIsHost); } \
  NS_IMETHOD GetDomain(nsACString& aDomain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDomain(aDomain); } \
  NS_IMETHOD SetDomain(const nsACString& aDomain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDomain(aDomain); } \
  NS_IMETHOD GetHasDomain(bool *aHasDomain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasDomain(aHasDomain); } \
  NS_IMETHOD GetUri(nsIURI **aUri) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUri(aUri); } \
  NS_IMETHOD SetUri(nsIURI *aUri) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUri(aUri); } \
  NS_IMETHOD GetHasUri(bool *aHasUri) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasUri(aHasUri); } \
  NS_IMETHOD GetAnnotationIsNot(bool *aAnnotationIsNot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAnnotationIsNot(aAnnotationIsNot); } \
  NS_IMETHOD SetAnnotationIsNot(bool aAnnotationIsNot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAnnotationIsNot(aAnnotationIsNot); } \
  NS_IMETHOD GetAnnotation(nsACString& aAnnotation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAnnotation(aAnnotation); } \
  NS_IMETHOD SetAnnotation(const nsACString& aAnnotation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAnnotation(aAnnotation); } \
  NS_IMETHOD GetHasAnnotation(bool *aHasAnnotation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasAnnotation(aHasAnnotation); } \
  NS_IMETHOD GetTags(nsIVariant **aTags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTags(aTags); } \
  NS_IMETHOD SetTags(nsIVariant *aTags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTags(aTags); } \
  NS_IMETHOD GetTagsAreNot(bool *aTagsAreNot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTagsAreNot(aTagsAreNot); } \
  NS_IMETHOD SetTagsAreNot(bool aTagsAreNot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTagsAreNot(aTagsAreNot); } \
  NS_IMETHOD GetParents(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParents(_retval); } \
  NS_IMETHOD GetParentCount(uint32_t *aParentCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParentCount(aParentCount); } \
  NS_IMETHOD SetParents(const nsTArray<nsCString >& aGuids) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetParents(aGuids); } \
  NS_IMETHOD Clone(nsINavHistoryQuery **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clone(_retval); } 


/* starting interface:    nsINavHistoryQueryOptions */
#define NS_INAVHISTORYQUERYOPTIONS_IID_STR "8198dfa7-8061-4766-95cb-fa86b3c00a47"

#define NS_INAVHISTORYQUERYOPTIONS_IID \
  {0x8198dfa7, 0x8061, 0x4766, \
    { 0x95, 0xcb, 0xfa, 0x86, 0xb3, 0xc0, 0x0a, 0x47 }}

class NS_NO_VTABLE nsINavHistoryQueryOptions : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INAVHISTORYQUERYOPTIONS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINavHistoryQueryOptions;

  enum {
    SORT_BY_NONE = 0U,
    SORT_BY_TITLE_ASCENDING = 1U,
    SORT_BY_TITLE_DESCENDING = 2U,
    SORT_BY_DATE_ASCENDING = 3U,
    SORT_BY_DATE_DESCENDING = 4U,
    SORT_BY_URI_ASCENDING = 5U,
    SORT_BY_URI_DESCENDING = 6U,
    SORT_BY_VISITCOUNT_ASCENDING = 7U,
    SORT_BY_VISITCOUNT_DESCENDING = 8U,
    SORT_BY_DATEADDED_ASCENDING = 11U,
    SORT_BY_DATEADDED_DESCENDING = 12U,
    SORT_BY_LASTMODIFIED_ASCENDING = 13U,
    SORT_BY_LASTMODIFIED_DESCENDING = 14U,
    SORT_BY_TAGS_ASCENDING = 17U,
    SORT_BY_TAGS_DESCENDING = 18U,
    SORT_BY_FRECENCY_ASCENDING = 21U,
    SORT_BY_FRECENCY_DESCENDING = 22U,
    RESULTS_AS_URI = 0U,
    RESULTS_AS_VISIT = 1U,
    RESULTS_AS_DATE_QUERY = 3U,
    RESULTS_AS_SITE_QUERY = 4U,
    RESULTS_AS_DATE_SITE_QUERY = 5U,
    RESULTS_AS_TAGS_ROOT = 6U,
    RESULTS_AS_TAG_CONTENTS = 7U,
    RESULTS_AS_ROOTS_QUERY = 8U,
    RESULTS_AS_LEFT_PANE_QUERY = 9U
  };

  /* attribute unsigned short sortingMode; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSortingMode(uint16_t *aSortingMode) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSortingMode(uint16_t aSortingMode) = 0;

  /* attribute unsigned short resultType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetResultType(uint16_t *aResultType) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetResultType(uint16_t aResultType) = 0;

  /* attribute boolean excludeItems; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExcludeItems(bool *aExcludeItems) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetExcludeItems(bool aExcludeItems) = 0;

  /* attribute boolean excludeQueries; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExcludeQueries(bool *aExcludeQueries) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetExcludeQueries(bool aExcludeQueries) = 0;

  /* attribute boolean expandQueries; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExpandQueries(bool *aExpandQueries) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetExpandQueries(bool aExpandQueries) = 0;

  /* attribute boolean includeHidden; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIncludeHidden(bool *aIncludeHidden) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetIncludeHidden(bool aIncludeHidden) = 0;

  /* attribute unsigned long maxResults; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMaxResults(uint32_t *aMaxResults) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMaxResults(uint32_t aMaxResults) = 0;

  enum {
    QUERY_TYPE_HISTORY = 0U,
    QUERY_TYPE_BOOKMARKS = 1U,
    QUERY_TYPE_UNIFIED = 2U
  };

  /* attribute unsigned short queryType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetQueryType(uint16_t *aQueryType) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetQueryType(uint16_t aQueryType) = 0;

  /* attribute boolean asyncEnabled; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAsyncEnabled(bool *aAsyncEnabled) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetAsyncEnabled(bool aAsyncEnabled) = 0;

  /* nsINavHistoryQueryOptions clone (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Clone(nsINavHistoryQueryOptions **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINavHistoryQueryOptions, NS_INAVHISTORYQUERYOPTIONS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINAVHISTORYQUERYOPTIONS \
  NS_IMETHOD GetSortingMode(uint16_t *aSortingMode) override; \
  NS_IMETHOD SetSortingMode(uint16_t aSortingMode) override; \
  NS_IMETHOD GetResultType(uint16_t *aResultType) override; \
  NS_IMETHOD SetResultType(uint16_t aResultType) override; \
  NS_IMETHOD GetExcludeItems(bool *aExcludeItems) override; \
  NS_IMETHOD SetExcludeItems(bool aExcludeItems) override; \
  NS_IMETHOD GetExcludeQueries(bool *aExcludeQueries) override; \
  NS_IMETHOD SetExcludeQueries(bool aExcludeQueries) override; \
  NS_IMETHOD GetExpandQueries(bool *aExpandQueries) override; \
  NS_IMETHOD SetExpandQueries(bool aExpandQueries) override; \
  NS_IMETHOD GetIncludeHidden(bool *aIncludeHidden) override; \
  NS_IMETHOD SetIncludeHidden(bool aIncludeHidden) override; \
  NS_IMETHOD GetMaxResults(uint32_t *aMaxResults) override; \
  NS_IMETHOD SetMaxResults(uint32_t aMaxResults) override; \
  NS_IMETHOD GetQueryType(uint16_t *aQueryType) override; \
  NS_IMETHOD SetQueryType(uint16_t aQueryType) override; \
  NS_IMETHOD GetAsyncEnabled(bool *aAsyncEnabled) override; \
  NS_IMETHOD SetAsyncEnabled(bool aAsyncEnabled) override; \
  NS_IMETHOD Clone(nsINavHistoryQueryOptions **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINAVHISTORYQUERYOPTIONS \
  nsresult GetSortingMode(uint16_t *aSortingMode); \
  nsresult SetSortingMode(uint16_t aSortingMode); \
  nsresult GetResultType(uint16_t *aResultType); \
  nsresult SetResultType(uint16_t aResultType); \
  nsresult GetExcludeItems(bool *aExcludeItems); \
  nsresult SetExcludeItems(bool aExcludeItems); \
  nsresult GetExcludeQueries(bool *aExcludeQueries); \
  nsresult SetExcludeQueries(bool aExcludeQueries); \
  nsresult GetExpandQueries(bool *aExpandQueries); \
  nsresult SetExpandQueries(bool aExpandQueries); \
  nsresult GetIncludeHidden(bool *aIncludeHidden); \
  nsresult SetIncludeHidden(bool aIncludeHidden); \
  nsresult GetMaxResults(uint32_t *aMaxResults); \
  nsresult SetMaxResults(uint32_t aMaxResults); \
  nsresult GetQueryType(uint16_t *aQueryType); \
  nsresult SetQueryType(uint16_t aQueryType); \
  nsresult GetAsyncEnabled(bool *aAsyncEnabled); \
  nsresult SetAsyncEnabled(bool aAsyncEnabled); \
  nsresult Clone(nsINavHistoryQueryOptions **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINAVHISTORYQUERYOPTIONS(_to) \
  NS_IMETHOD GetSortingMode(uint16_t *aSortingMode) override { return _to GetSortingMode(aSortingMode); } \
  NS_IMETHOD SetSortingMode(uint16_t aSortingMode) override { return _to SetSortingMode(aSortingMode); } \
  NS_IMETHOD GetResultType(uint16_t *aResultType) override { return _to GetResultType(aResultType); } \
  NS_IMETHOD SetResultType(uint16_t aResultType) override { return _to SetResultType(aResultType); } \
  NS_IMETHOD GetExcludeItems(bool *aExcludeItems) override { return _to GetExcludeItems(aExcludeItems); } \
  NS_IMETHOD SetExcludeItems(bool aExcludeItems) override { return _to SetExcludeItems(aExcludeItems); } \
  NS_IMETHOD GetExcludeQueries(bool *aExcludeQueries) override { return _to GetExcludeQueries(aExcludeQueries); } \
  NS_IMETHOD SetExcludeQueries(bool aExcludeQueries) override { return _to SetExcludeQueries(aExcludeQueries); } \
  NS_IMETHOD GetExpandQueries(bool *aExpandQueries) override { return _to GetExpandQueries(aExpandQueries); } \
  NS_IMETHOD SetExpandQueries(bool aExpandQueries) override { return _to SetExpandQueries(aExpandQueries); } \
  NS_IMETHOD GetIncludeHidden(bool *aIncludeHidden) override { return _to GetIncludeHidden(aIncludeHidden); } \
  NS_IMETHOD SetIncludeHidden(bool aIncludeHidden) override { return _to SetIncludeHidden(aIncludeHidden); } \
  NS_IMETHOD GetMaxResults(uint32_t *aMaxResults) override { return _to GetMaxResults(aMaxResults); } \
  NS_IMETHOD SetMaxResults(uint32_t aMaxResults) override { return _to SetMaxResults(aMaxResults); } \
  NS_IMETHOD GetQueryType(uint16_t *aQueryType) override { return _to GetQueryType(aQueryType); } \
  NS_IMETHOD SetQueryType(uint16_t aQueryType) override { return _to SetQueryType(aQueryType); } \
  NS_IMETHOD GetAsyncEnabled(bool *aAsyncEnabled) override { return _to GetAsyncEnabled(aAsyncEnabled); } \
  NS_IMETHOD SetAsyncEnabled(bool aAsyncEnabled) override { return _to SetAsyncEnabled(aAsyncEnabled); } \
  NS_IMETHOD Clone(nsINavHistoryQueryOptions **_retval) override { return _to Clone(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINAVHISTORYQUERYOPTIONS(_to) \
  NS_IMETHOD GetSortingMode(uint16_t *aSortingMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSortingMode(aSortingMode); } \
  NS_IMETHOD SetSortingMode(uint16_t aSortingMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSortingMode(aSortingMode); } \
  NS_IMETHOD GetResultType(uint16_t *aResultType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResultType(aResultType); } \
  NS_IMETHOD SetResultType(uint16_t aResultType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetResultType(aResultType); } \
  NS_IMETHOD GetExcludeItems(bool *aExcludeItems) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExcludeItems(aExcludeItems); } \
  NS_IMETHOD SetExcludeItems(bool aExcludeItems) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetExcludeItems(aExcludeItems); } \
  NS_IMETHOD GetExcludeQueries(bool *aExcludeQueries) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExcludeQueries(aExcludeQueries); } \
  NS_IMETHOD SetExcludeQueries(bool aExcludeQueries) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetExcludeQueries(aExcludeQueries); } \
  NS_IMETHOD GetExpandQueries(bool *aExpandQueries) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExpandQueries(aExpandQueries); } \
  NS_IMETHOD SetExpandQueries(bool aExpandQueries) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetExpandQueries(aExpandQueries); } \
  NS_IMETHOD GetIncludeHidden(bool *aIncludeHidden) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIncludeHidden(aIncludeHidden); } \
  NS_IMETHOD SetIncludeHidden(bool aIncludeHidden) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIncludeHidden(aIncludeHidden); } \
  NS_IMETHOD GetMaxResults(uint32_t *aMaxResults) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMaxResults(aMaxResults); } \
  NS_IMETHOD SetMaxResults(uint32_t aMaxResults) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMaxResults(aMaxResults); } \
  NS_IMETHOD GetQueryType(uint16_t *aQueryType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetQueryType(aQueryType); } \
  NS_IMETHOD SetQueryType(uint16_t aQueryType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetQueryType(aQueryType); } \
  NS_IMETHOD GetAsyncEnabled(bool *aAsyncEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsyncEnabled(aAsyncEnabled); } \
  NS_IMETHOD SetAsyncEnabled(bool aAsyncEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsyncEnabled(aAsyncEnabled); } \
  NS_IMETHOD Clone(nsINavHistoryQueryOptions **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clone(_retval); } 


/* starting interface:    nsINavHistoryService */
#define NS_INAVHISTORYSERVICE_IID_STR "20c974ff-ee16-4828-9326-1b7c9e036622"

#define NS_INAVHISTORYSERVICE_IID \
  {0x20c974ff, 0xee16, 0x4828, \
    { 0x93, 0x26, 0x1b, 0x7c, 0x9e, 0x03, 0x66, 0x22 }}

class NS_NO_VTABLE nsINavHistoryService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INAVHISTORYSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINavHistoryService;

  enum {
    TRANSITION_LINK = 1U,
    TRANSITION_TYPED = 2U,
    TRANSITION_BOOKMARK = 3U,
    TRANSITION_EMBED = 4U,
    TRANSITION_REDIRECT_PERMANENT = 5U,
    TRANSITION_REDIRECT_TEMPORARY = 6U,
    TRANSITION_DOWNLOAD = 7U,
    TRANSITION_FRAMED_LINK = 8U,
    TRANSITION_RELOAD = 9U,
    DATABASE_STATUS_OK = 0U,
    DATABASE_STATUS_CREATE = 1U,
    DATABASE_STATUS_CORRUPT = 2U,
    DATABASE_STATUS_UPGRADED = 3U,
    DATABASE_STATUS_LOCKED = 4U
  };

  /* readonly attribute unsigned short databaseStatus; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDatabaseStatus(uint16_t *aDatabaseStatus) = 0;

  /* void markPageAsFollowedBookmark (in nsIURI aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MarkPageAsFollowedBookmark(nsIURI *aURI) = 0;

  /* void markPageAsTyped (in nsIURI aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MarkPageAsTyped(nsIURI *aURI) = 0;

  /* void markPageAsFollowedLink (in nsIURI aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MarkPageAsFollowedLink(nsIURI *aURI) = 0;

  /* boolean canAddURI (in nsIURI aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CanAddURI(nsIURI *aURI, bool *_retval) = 0;

  /* nsINavHistoryQuery getNewQuery (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNewQuery(nsINavHistoryQuery **_retval) = 0;

  /* nsINavHistoryQueryOptions getNewQueryOptions (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNewQueryOptions(nsINavHistoryQueryOptions **_retval) = 0;

  /* nsINavHistoryResult executeQuery (in nsINavHistoryQuery aQuery, in nsINavHistoryQueryOptions options); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ExecuteQuery(nsINavHistoryQuery *aQuery, nsINavHistoryQueryOptions *options, nsINavHistoryResult **_retval) = 0;

  /* void queryStringToQuery (in AUTF8String aQueryString, out nsINavHistoryQuery aQuery, out nsINavHistoryQueryOptions options); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD QueryStringToQuery(const nsACString& aQueryString, nsINavHistoryQuery **aQuery, nsINavHistoryQueryOptions **options) = 0;

  /* AUTF8String queryToQueryString (in nsINavHistoryQuery aQuery, in nsINavHistoryQueryOptions options); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD QueryToQueryString(nsINavHistoryQuery *aQuery, nsINavHistoryQueryOptions *options, nsACString& _retval) = 0;

  /* void addObserver (in nsINavHistoryObserver observer, [optional] in boolean ownsWeak); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddObserver(nsINavHistoryObserver *observer, bool ownsWeak) = 0;

  /* void removeObserver (in nsINavHistoryObserver observer); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveObserver(nsINavHistoryObserver *observer) = 0;

  /* Array<nsINavHistoryObserver> getObservers (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetObservers(nsTArray<RefPtr<nsINavHistoryObserver>>& _retval) = 0;

  /* readonly attribute boolean historyDisabled; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHistoryDisabled(bool *aHistoryDisabled) = 0;

  /* ACString makeGuid (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MakeGuid(nsACString& _retval) = 0;

  /* unsigned long long hashURL (in ACString aSpec, [optional] in ACString aMode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HashURL(const nsACString& aSpec, const nsACString& aMode, uint64_t *_retval) = 0;

  /* void recalculateOriginFrecencyStats ([optional] in nsIObserver aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RecalculateOriginFrecencyStats(nsIObserver *aCallback) = 0;

  /* readonly attribute mozIStorageConnection DBConnection; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDBConnection(mozIStorageConnection **aDBConnection) = 0;

  /* mozIStoragePendingStatement asyncExecuteLegacyQuery (in nsINavHistoryQuery aQuery, in nsINavHistoryQueryOptions aOptions, in mozIStorageStatementCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncExecuteLegacyQuery(nsINavHistoryQuery *aQuery, nsINavHistoryQueryOptions *aOptions, mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) = 0;

  /* readonly attribute nsIAsyncShutdownClient shutdownClient; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetShutdownClient(nsIAsyncShutdownClient **aShutdownClient) = 0;

  /* readonly attribute nsIAsyncShutdownClient connectionShutdownClient; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetConnectionShutdownClient(nsIAsyncShutdownClient **aConnectionShutdownClient) = 0;

  /* void decayFrecency (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DecayFrecency(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINavHistoryService, NS_INAVHISTORYSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINAVHISTORYSERVICE \
  NS_IMETHOD GetDatabaseStatus(uint16_t *aDatabaseStatus) override; \
  NS_IMETHOD MarkPageAsFollowedBookmark(nsIURI *aURI) override; \
  NS_IMETHOD MarkPageAsTyped(nsIURI *aURI) override; \
  NS_IMETHOD MarkPageAsFollowedLink(nsIURI *aURI) override; \
  NS_IMETHOD CanAddURI(nsIURI *aURI, bool *_retval) override; \
  NS_IMETHOD GetNewQuery(nsINavHistoryQuery **_retval) override; \
  NS_IMETHOD GetNewQueryOptions(nsINavHistoryQueryOptions **_retval) override; \
  NS_IMETHOD ExecuteQuery(nsINavHistoryQuery *aQuery, nsINavHistoryQueryOptions *options, nsINavHistoryResult **_retval) override; \
  NS_IMETHOD QueryStringToQuery(const nsACString& aQueryString, nsINavHistoryQuery **aQuery, nsINavHistoryQueryOptions **options) override; \
  NS_IMETHOD QueryToQueryString(nsINavHistoryQuery *aQuery, nsINavHistoryQueryOptions *options, nsACString& _retval) override; \
  NS_IMETHOD AddObserver(nsINavHistoryObserver *observer, bool ownsWeak) override; \
  NS_IMETHOD RemoveObserver(nsINavHistoryObserver *observer) override; \
  NS_IMETHOD GetObservers(nsTArray<RefPtr<nsINavHistoryObserver>>& _retval) override; \
  NS_IMETHOD GetHistoryDisabled(bool *aHistoryDisabled) override; \
  NS_IMETHOD MakeGuid(nsACString& _retval) override; \
  NS_IMETHOD HashURL(const nsACString& aSpec, const nsACString& aMode, uint64_t *_retval) override; \
  NS_IMETHOD RecalculateOriginFrecencyStats(nsIObserver *aCallback) override; \
  NS_IMETHOD GetDBConnection(mozIStorageConnection **aDBConnection) override; \
  NS_IMETHOD AsyncExecuteLegacyQuery(nsINavHistoryQuery *aQuery, nsINavHistoryQueryOptions *aOptions, mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) override; \
  NS_IMETHOD GetShutdownClient(nsIAsyncShutdownClient **aShutdownClient) override; \
  NS_IMETHOD GetConnectionShutdownClient(nsIAsyncShutdownClient **aConnectionShutdownClient) override; \
  NS_IMETHOD DecayFrecency(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINAVHISTORYSERVICE \
  nsresult GetDatabaseStatus(uint16_t *aDatabaseStatus); \
  nsresult MarkPageAsFollowedBookmark(nsIURI *aURI); \
  nsresult MarkPageAsTyped(nsIURI *aURI); \
  nsresult MarkPageAsFollowedLink(nsIURI *aURI); \
  nsresult CanAddURI(nsIURI *aURI, bool *_retval); \
  nsresult GetNewQuery(nsINavHistoryQuery **_retval); \
  nsresult GetNewQueryOptions(nsINavHistoryQueryOptions **_retval); \
  nsresult ExecuteQuery(nsINavHistoryQuery *aQuery, nsINavHistoryQueryOptions *options, nsINavHistoryResult **_retval); \
  nsresult QueryStringToQuery(const nsACString& aQueryString, nsINavHistoryQuery **aQuery, nsINavHistoryQueryOptions **options); \
  nsresult QueryToQueryString(nsINavHistoryQuery *aQuery, nsINavHistoryQueryOptions *options, nsACString& _retval); \
  nsresult AddObserver(nsINavHistoryObserver *observer, bool ownsWeak); \
  nsresult RemoveObserver(nsINavHistoryObserver *observer); \
  nsresult GetObservers(nsTArray<RefPtr<nsINavHistoryObserver>>& _retval); \
  nsresult GetHistoryDisabled(bool *aHistoryDisabled); \
  nsresult MakeGuid(nsACString& _retval); \
  nsresult HashURL(const nsACString& aSpec, const nsACString& aMode, uint64_t *_retval); \
  nsresult RecalculateOriginFrecencyStats(nsIObserver *aCallback); \
  nsresult GetDBConnection(mozIStorageConnection **aDBConnection); \
  nsresult AsyncExecuteLegacyQuery(nsINavHistoryQuery *aQuery, nsINavHistoryQueryOptions *aOptions, mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval); \
  nsresult GetShutdownClient(nsIAsyncShutdownClient **aShutdownClient); \
  nsresult GetConnectionShutdownClient(nsIAsyncShutdownClient **aConnectionShutdownClient); \
  nsresult DecayFrecency(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINAVHISTORYSERVICE(_to) \
  NS_IMETHOD GetDatabaseStatus(uint16_t *aDatabaseStatus) override { return _to GetDatabaseStatus(aDatabaseStatus); } \
  NS_IMETHOD MarkPageAsFollowedBookmark(nsIURI *aURI) override { return _to MarkPageAsFollowedBookmark(aURI); } \
  NS_IMETHOD MarkPageAsTyped(nsIURI *aURI) override { return _to MarkPageAsTyped(aURI); } \
  NS_IMETHOD MarkPageAsFollowedLink(nsIURI *aURI) override { return _to MarkPageAsFollowedLink(aURI); } \
  NS_IMETHOD CanAddURI(nsIURI *aURI, bool *_retval) override { return _to CanAddURI(aURI, _retval); } \
  NS_IMETHOD GetNewQuery(nsINavHistoryQuery **_retval) override { return _to GetNewQuery(_retval); } \
  NS_IMETHOD GetNewQueryOptions(nsINavHistoryQueryOptions **_retval) override { return _to GetNewQueryOptions(_retval); } \
  NS_IMETHOD ExecuteQuery(nsINavHistoryQuery *aQuery, nsINavHistoryQueryOptions *options, nsINavHistoryResult **_retval) override { return _to ExecuteQuery(aQuery, options, _retval); } \
  NS_IMETHOD QueryStringToQuery(const nsACString& aQueryString, nsINavHistoryQuery **aQuery, nsINavHistoryQueryOptions **options) override { return _to QueryStringToQuery(aQueryString, aQuery, options); } \
  NS_IMETHOD QueryToQueryString(nsINavHistoryQuery *aQuery, nsINavHistoryQueryOptions *options, nsACString& _retval) override { return _to QueryToQueryString(aQuery, options, _retval); } \
  NS_IMETHOD AddObserver(nsINavHistoryObserver *observer, bool ownsWeak) override { return _to AddObserver(observer, ownsWeak); } \
  NS_IMETHOD RemoveObserver(nsINavHistoryObserver *observer) override { return _to RemoveObserver(observer); } \
  NS_IMETHOD GetObservers(nsTArray<RefPtr<nsINavHistoryObserver>>& _retval) override { return _to GetObservers(_retval); } \
  NS_IMETHOD GetHistoryDisabled(bool *aHistoryDisabled) override { return _to GetHistoryDisabled(aHistoryDisabled); } \
  NS_IMETHOD MakeGuid(nsACString& _retval) override { return _to MakeGuid(_retval); } \
  NS_IMETHOD HashURL(const nsACString& aSpec, const nsACString& aMode, uint64_t *_retval) override { return _to HashURL(aSpec, aMode, _retval); } \
  NS_IMETHOD RecalculateOriginFrecencyStats(nsIObserver *aCallback) override { return _to RecalculateOriginFrecencyStats(aCallback); } \
  NS_IMETHOD GetDBConnection(mozIStorageConnection **aDBConnection) override { return _to GetDBConnection(aDBConnection); } \
  NS_IMETHOD AsyncExecuteLegacyQuery(nsINavHistoryQuery *aQuery, nsINavHistoryQueryOptions *aOptions, mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) override { return _to AsyncExecuteLegacyQuery(aQuery, aOptions, aCallback, _retval); } \
  NS_IMETHOD GetShutdownClient(nsIAsyncShutdownClient **aShutdownClient) override { return _to GetShutdownClient(aShutdownClient); } \
  NS_IMETHOD GetConnectionShutdownClient(nsIAsyncShutdownClient **aConnectionShutdownClient) override { return _to GetConnectionShutdownClient(aConnectionShutdownClient); } \
  NS_IMETHOD DecayFrecency(void) override { return _to DecayFrecency(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINAVHISTORYSERVICE(_to) \
  NS_IMETHOD GetDatabaseStatus(uint16_t *aDatabaseStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDatabaseStatus(aDatabaseStatus); } \
  NS_IMETHOD MarkPageAsFollowedBookmark(nsIURI *aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MarkPageAsFollowedBookmark(aURI); } \
  NS_IMETHOD MarkPageAsTyped(nsIURI *aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MarkPageAsTyped(aURI); } \
  NS_IMETHOD MarkPageAsFollowedLink(nsIURI *aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MarkPageAsFollowedLink(aURI); } \
  NS_IMETHOD CanAddURI(nsIURI *aURI, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanAddURI(aURI, _retval); } \
  NS_IMETHOD GetNewQuery(nsINavHistoryQuery **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNewQuery(_retval); } \
  NS_IMETHOD GetNewQueryOptions(nsINavHistoryQueryOptions **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNewQueryOptions(_retval); } \
  NS_IMETHOD ExecuteQuery(nsINavHistoryQuery *aQuery, nsINavHistoryQueryOptions *options, nsINavHistoryResult **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExecuteQuery(aQuery, options, _retval); } \
  NS_IMETHOD QueryStringToQuery(const nsACString& aQueryString, nsINavHistoryQuery **aQuery, nsINavHistoryQueryOptions **options) override { return !_to ? NS_ERROR_NULL_POINTER : _to->QueryStringToQuery(aQueryString, aQuery, options); } \
  NS_IMETHOD QueryToQueryString(nsINavHistoryQuery *aQuery, nsINavHistoryQueryOptions *options, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->QueryToQueryString(aQuery, options, _retval); } \
  NS_IMETHOD AddObserver(nsINavHistoryObserver *observer, bool ownsWeak) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddObserver(observer, ownsWeak); } \
  NS_IMETHOD RemoveObserver(nsINavHistoryObserver *observer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveObserver(observer); } \
  NS_IMETHOD GetObservers(nsTArray<RefPtr<nsINavHistoryObserver>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetObservers(_retval); } \
  NS_IMETHOD GetHistoryDisabled(bool *aHistoryDisabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHistoryDisabled(aHistoryDisabled); } \
  NS_IMETHOD MakeGuid(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MakeGuid(_retval); } \
  NS_IMETHOD HashURL(const nsACString& aSpec, const nsACString& aMode, uint64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HashURL(aSpec, aMode, _retval); } \
  NS_IMETHOD RecalculateOriginFrecencyStats(nsIObserver *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RecalculateOriginFrecencyStats(aCallback); } \
  NS_IMETHOD GetDBConnection(mozIStorageConnection **aDBConnection) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDBConnection(aDBConnection); } \
  NS_IMETHOD AsyncExecuteLegacyQuery(nsINavHistoryQuery *aQuery, nsINavHistoryQueryOptions *aOptions, mozIStorageStatementCallback *aCallback, mozIStoragePendingStatement **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncExecuteLegacyQuery(aQuery, aOptions, aCallback, _retval); } \
  NS_IMETHOD GetShutdownClient(nsIAsyncShutdownClient **aShutdownClient) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetShutdownClient(aShutdownClient); } \
  NS_IMETHOD GetConnectionShutdownClient(nsIAsyncShutdownClient **aConnectionShutdownClient) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConnectionShutdownClient(aConnectionShutdownClient); } \
  NS_IMETHOD DecayFrecency(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DecayFrecency(); } 


#endif /* __gen_nsINavHistoryService_h__ */
