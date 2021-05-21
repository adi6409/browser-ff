/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/mozISyncedBookmarksMirror.idl
 */

#ifndef __gen_mozISyncedBookmarksMirror_h__
#define __gen_mozISyncedBookmarksMirror_h__


#ifndef __gen_mozIServicesLogSink_h__
#include "mozIServicesLogSink.h"
#endif

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
class mozIPlacesPendingOperation; /* forward declaration */

class mozIStorageConnection; /* forward declaration */

class nsIPropertyBag; /* forward declaration */


/* starting interface:    mozISyncedBookmarksMirrorProgressListener */
#define MOZISYNCEDBOOKMARKSMIRRORPROGRESSLISTENER_IID_STR "6239ffe3-6ffd-49ac-8b1d-958407395bf9"

#define MOZISYNCEDBOOKMARKSMIRRORPROGRESSLISTENER_IID \
  {0x6239ffe3, 0x6ffd, 0x49ac, \
    { 0x8b, 0x1d, 0x95, 0x84, 0x07, 0x39, 0x5b, 0xf9 }}

class NS_NO_VTABLE mozISyncedBookmarksMirrorProgressListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISYNCEDBOOKMARKSMIRRORPROGRESSLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozISyncedBookmarksMirrorProgressListener;

  /* void onFetchLocalTree (in long long took, in long long itemCount, in long long deletedCount, in nsIPropertyBag problems); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnFetchLocalTree(int64_t took, int64_t itemCount, int64_t deletedCount, nsIPropertyBag *problems) = 0;

  /* void onFetchRemoteTree (in long long took, in long long itemCount, in long long deletedCount, in nsIPropertyBag problems); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnFetchRemoteTree(int64_t took, int64_t itemCount, int64_t deletedCount, nsIPropertyBag *problems) = 0;

  /* void onMerge (in long long took, in nsIPropertyBag counts); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnMerge(int64_t took, nsIPropertyBag *counts) = 0;

  /* void onApply (in long long took); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnApply(int64_t took) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozISyncedBookmarksMirrorProgressListener, MOZISYNCEDBOOKMARKSMIRRORPROGRESSLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISYNCEDBOOKMARKSMIRRORPROGRESSLISTENER \
  NS_IMETHOD OnFetchLocalTree(int64_t took, int64_t itemCount, int64_t deletedCount, nsIPropertyBag *problems) override; \
  NS_IMETHOD OnFetchRemoteTree(int64_t took, int64_t itemCount, int64_t deletedCount, nsIPropertyBag *problems) override; \
  NS_IMETHOD OnMerge(int64_t took, nsIPropertyBag *counts) override; \
  NS_IMETHOD OnApply(int64_t took) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISYNCEDBOOKMARKSMIRRORPROGRESSLISTENER \
  nsresult OnFetchLocalTree(int64_t took, int64_t itemCount, int64_t deletedCount, nsIPropertyBag *problems); \
  nsresult OnFetchRemoteTree(int64_t took, int64_t itemCount, int64_t deletedCount, nsIPropertyBag *problems); \
  nsresult OnMerge(int64_t took, nsIPropertyBag *counts); \
  nsresult OnApply(int64_t took); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISYNCEDBOOKMARKSMIRRORPROGRESSLISTENER(_to) \
  NS_IMETHOD OnFetchLocalTree(int64_t took, int64_t itemCount, int64_t deletedCount, nsIPropertyBag *problems) override { return _to OnFetchLocalTree(took, itemCount, deletedCount, problems); } \
  NS_IMETHOD OnFetchRemoteTree(int64_t took, int64_t itemCount, int64_t deletedCount, nsIPropertyBag *problems) override { return _to OnFetchRemoteTree(took, itemCount, deletedCount, problems); } \
  NS_IMETHOD OnMerge(int64_t took, nsIPropertyBag *counts) override { return _to OnMerge(took, counts); } \
  NS_IMETHOD OnApply(int64_t took) override { return _to OnApply(took); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISYNCEDBOOKMARKSMIRRORPROGRESSLISTENER(_to) \
  NS_IMETHOD OnFetchLocalTree(int64_t took, int64_t itemCount, int64_t deletedCount, nsIPropertyBag *problems) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnFetchLocalTree(took, itemCount, deletedCount, problems); } \
  NS_IMETHOD OnFetchRemoteTree(int64_t took, int64_t itemCount, int64_t deletedCount, nsIPropertyBag *problems) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnFetchRemoteTree(took, itemCount, deletedCount, problems); } \
  NS_IMETHOD OnMerge(int64_t took, nsIPropertyBag *counts) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnMerge(took, counts); } \
  NS_IMETHOD OnApply(int64_t took) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnApply(took); } 


/* starting interface:    mozISyncedBookmarksMirrorCallback */
#define MOZISYNCEDBOOKMARKSMIRRORCALLBACK_IID_STR "d23fdfea-92c8-409d-a516-08ae395d578f"

#define MOZISYNCEDBOOKMARKSMIRRORCALLBACK_IID \
  {0xd23fdfea, 0x92c8, 0x409d, \
    { 0xa5, 0x16, 0x08, 0xae, 0x39, 0x5d, 0x57, 0x8f }}

class NS_NO_VTABLE mozISyncedBookmarksMirrorCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISYNCEDBOOKMARKSMIRRORCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozISyncedBookmarksMirrorCallback;

  /* void handleSuccess (in bool result); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleSuccess(bool result) = 0;

  /* void handleError (in nsresult code, in AString message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleError(nsresult code, const nsAString& message) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozISyncedBookmarksMirrorCallback, MOZISYNCEDBOOKMARKSMIRRORCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISYNCEDBOOKMARKSMIRRORCALLBACK \
  NS_IMETHOD HandleSuccess(bool result) override; \
  NS_IMETHOD HandleError(nsresult code, const nsAString& message) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISYNCEDBOOKMARKSMIRRORCALLBACK \
  nsresult HandleSuccess(bool result); \
  nsresult HandleError(nsresult code, const nsAString& message); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISYNCEDBOOKMARKSMIRRORCALLBACK(_to) \
  NS_IMETHOD HandleSuccess(bool result) override { return _to HandleSuccess(result); } \
  NS_IMETHOD HandleError(nsresult code, const nsAString& message) override { return _to HandleError(code, message); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISYNCEDBOOKMARKSMIRRORCALLBACK(_to) \
  NS_IMETHOD HandleSuccess(bool result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleSuccess(result); } \
  NS_IMETHOD HandleError(nsresult code, const nsAString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleError(code, message); } 


/* starting interface:    mozISyncedBookmarksMirrorLogger */
#define MOZISYNCEDBOOKMARKSMIRRORLOGGER_IID_STR "37485984-a6ab-46e3-9b0c-e8b613413ef3"

#define MOZISYNCEDBOOKMARKSMIRRORLOGGER_IID \
  {0x37485984, 0xa6ab, 0x46e3, \
    { 0x9b, 0x0c, 0xe8, 0xb6, 0x13, 0x41, 0x3e, 0xf3 }}

class NS_NO_VTABLE mozISyncedBookmarksMirrorLogger : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISYNCEDBOOKMARKSMIRRORLOGGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozISyncedBookmarksMirrorLogger;

  enum {
    LEVEL_OFF = 0,
    LEVEL_ERROR = 1,
    LEVEL_WARN = 2,
    LEVEL_DEBUG = 3,
    LEVEL_TRACE = 4
  };

  /* attribute short maxLevel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMaxLevel(int16_t *aMaxLevel) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMaxLevel(int16_t aMaxLevel) = 0;

  /* void error (in AString message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Error(const nsAString& message) = 0;

  /* void warn (in AString message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Warn(const nsAString& message) = 0;

  /* void debug (in AString message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Debug(const nsAString& message) = 0;

  /* void trace (in AString message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Trace(const nsAString& message) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozISyncedBookmarksMirrorLogger, MOZISYNCEDBOOKMARKSMIRRORLOGGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISYNCEDBOOKMARKSMIRRORLOGGER \
  NS_IMETHOD GetMaxLevel(int16_t *aMaxLevel) override; \
  NS_IMETHOD SetMaxLevel(int16_t aMaxLevel) override; \
  NS_IMETHOD Error(const nsAString& message) override; \
  NS_IMETHOD Warn(const nsAString& message) override; \
  NS_IMETHOD Debug(const nsAString& message) override; \
  NS_IMETHOD Trace(const nsAString& message) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISYNCEDBOOKMARKSMIRRORLOGGER \
  nsresult GetMaxLevel(int16_t *aMaxLevel); \
  nsresult SetMaxLevel(int16_t aMaxLevel); \
  nsresult Error(const nsAString& message); \
  nsresult Warn(const nsAString& message); \
  nsresult Debug(const nsAString& message); \
  nsresult Trace(const nsAString& message); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISYNCEDBOOKMARKSMIRRORLOGGER(_to) \
  NS_IMETHOD GetMaxLevel(int16_t *aMaxLevel) override { return _to GetMaxLevel(aMaxLevel); } \
  NS_IMETHOD SetMaxLevel(int16_t aMaxLevel) override { return _to SetMaxLevel(aMaxLevel); } \
  NS_IMETHOD Error(const nsAString& message) override { return _to Error(message); } \
  NS_IMETHOD Warn(const nsAString& message) override { return _to Warn(message); } \
  NS_IMETHOD Debug(const nsAString& message) override { return _to Debug(message); } \
  NS_IMETHOD Trace(const nsAString& message) override { return _to Trace(message); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISYNCEDBOOKMARKSMIRRORLOGGER(_to) \
  NS_IMETHOD GetMaxLevel(int16_t *aMaxLevel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMaxLevel(aMaxLevel); } \
  NS_IMETHOD SetMaxLevel(int16_t aMaxLevel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMaxLevel(aMaxLevel); } \
  NS_IMETHOD Error(const nsAString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Error(message); } \
  NS_IMETHOD Warn(const nsAString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Warn(message); } \
  NS_IMETHOD Debug(const nsAString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Debug(message); } \
  NS_IMETHOD Trace(const nsAString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Trace(message); } 


/* starting interface:    mozISyncedBookmarksMerger */
#define MOZISYNCEDBOOKMARKSMERGER_IID_STR "f0a6217d-8344-4e68-9995-bbf5554be86e"

#define MOZISYNCEDBOOKMARKSMERGER_IID \
  {0xf0a6217d, 0x8344, 0x4e68, \
    { 0x99, 0x95, 0xbb, 0xf5, 0x55, 0x4b, 0xe8, 0x6e }}

class NS_NO_VTABLE mozISyncedBookmarksMerger : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISYNCEDBOOKMARKSMERGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozISyncedBookmarksMerger;

  enum {
    KIND_BOOKMARK = 1,
    KIND_QUERY = 2,
    KIND_FOLDER = 3,
    KIND_LIVEMARK = 4,
    KIND_SEPARATOR = 5,
    VALIDITY_VALID = 1,
    VALIDITY_REUPLOAD = 2,
    VALIDITY_REPLACE = 3
  };

  /* attribute mozIStorageConnection db; */
  NS_IMETHOD GetDb(mozIStorageConnection **aDb) = 0;
  NS_IMETHOD SetDb(mozIStorageConnection *aDb) = 0;

  /* attribute mozIServicesLogSink logger; */
  NS_IMETHOD GetLogger(mozIServicesLogSink **aLogger) = 0;
  NS_IMETHOD SetLogger(mozIServicesLogSink *aLogger) = 0;

  /* mozIPlacesPendingOperation merge (in long long localTimeSeconds, in long long remoteTimeSeconds, in Array<AString> weakUploads, in mozISyncedBookmarksMirrorCallback callback); */
  NS_IMETHOD Merge(int64_t localTimeSeconds, int64_t remoteTimeSeconds, const nsTArray<nsString >& weakUploads, mozISyncedBookmarksMirrorCallback *callback, mozIPlacesPendingOperation **_retval) = 0;

  /* void reset (); */
  NS_IMETHOD Reset(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozISyncedBookmarksMerger, MOZISYNCEDBOOKMARKSMERGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISYNCEDBOOKMARKSMERGER \
  NS_IMETHOD GetDb(mozIStorageConnection **aDb) override; \
  NS_IMETHOD SetDb(mozIStorageConnection *aDb) override; \
  NS_IMETHOD GetLogger(mozIServicesLogSink **aLogger) override; \
  NS_IMETHOD SetLogger(mozIServicesLogSink *aLogger) override; \
  NS_IMETHOD Merge(int64_t localTimeSeconds, int64_t remoteTimeSeconds, const nsTArray<nsString >& weakUploads, mozISyncedBookmarksMirrorCallback *callback, mozIPlacesPendingOperation **_retval) override; \
  NS_IMETHOD Reset(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISYNCEDBOOKMARKSMERGER \
  nsresult GetDb(mozIStorageConnection **aDb); \
  nsresult SetDb(mozIStorageConnection *aDb); \
  nsresult GetLogger(mozIServicesLogSink **aLogger); \
  nsresult SetLogger(mozIServicesLogSink *aLogger); \
  nsresult Merge(int64_t localTimeSeconds, int64_t remoteTimeSeconds, const nsTArray<nsString >& weakUploads, mozISyncedBookmarksMirrorCallback *callback, mozIPlacesPendingOperation **_retval); \
  nsresult Reset(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISYNCEDBOOKMARKSMERGER(_to) \
  NS_IMETHOD GetDb(mozIStorageConnection **aDb) override { return _to GetDb(aDb); } \
  NS_IMETHOD SetDb(mozIStorageConnection *aDb) override { return _to SetDb(aDb); } \
  NS_IMETHOD GetLogger(mozIServicesLogSink **aLogger) override { return _to GetLogger(aLogger); } \
  NS_IMETHOD SetLogger(mozIServicesLogSink *aLogger) override { return _to SetLogger(aLogger); } \
  NS_IMETHOD Merge(int64_t localTimeSeconds, int64_t remoteTimeSeconds, const nsTArray<nsString >& weakUploads, mozISyncedBookmarksMirrorCallback *callback, mozIPlacesPendingOperation **_retval) override { return _to Merge(localTimeSeconds, remoteTimeSeconds, weakUploads, callback, _retval); } \
  NS_IMETHOD Reset(void) override { return _to Reset(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISYNCEDBOOKMARKSMERGER(_to) \
  NS_IMETHOD GetDb(mozIStorageConnection **aDb) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDb(aDb); } \
  NS_IMETHOD SetDb(mozIStorageConnection *aDb) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDb(aDb); } \
  NS_IMETHOD GetLogger(mozIServicesLogSink **aLogger) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLogger(aLogger); } \
  NS_IMETHOD SetLogger(mozIServicesLogSink *aLogger) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLogger(aLogger); } \
  NS_IMETHOD Merge(int64_t localTimeSeconds, int64_t remoteTimeSeconds, const nsTArray<nsString >& weakUploads, mozISyncedBookmarksMirrorCallback *callback, mozIPlacesPendingOperation **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Merge(localTimeSeconds, remoteTimeSeconds, weakUploads, callback, _retval); } \
  NS_IMETHOD Reset(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reset(); } 


#endif /* __gen_mozISyncedBookmarksMirror_h__ */
