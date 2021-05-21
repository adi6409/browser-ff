/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/services/interfaces/mozIBridgedSyncEngine.idl
 */

#ifndef __gen_mozIBridgedSyncEngine_h__
#define __gen_mozIBridgedSyncEngine_h__


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
class nsIVariant; /* forward declaration */


/* starting interface:    mozIBridgedSyncEngineCallback */
#define MOZIBRIDGEDSYNCENGINECALLBACK_IID_STR "9b7dd2a3-df99-4469-9ea9-61b222098695"

#define MOZIBRIDGEDSYNCENGINECALLBACK_IID \
  {0x9b7dd2a3, 0xdf99, 0x4469, \
    { 0x9e, 0xa9, 0x61, 0xb2, 0x22, 0x09, 0x86, 0x95 }}

class NS_NO_VTABLE mozIBridgedSyncEngineCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIBRIDGEDSYNCENGINECALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIBridgedSyncEngineCallback;

  /* void handleSuccess (in nsIVariant result); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleSuccess(nsIVariant *result) = 0;

  /* void handleError (in nsresult code, in AUTF8String message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleError(nsresult code, const nsACString& message) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIBridgedSyncEngineCallback, MOZIBRIDGEDSYNCENGINECALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIBRIDGEDSYNCENGINECALLBACK \
  NS_IMETHOD HandleSuccess(nsIVariant *result) override; \
  NS_IMETHOD HandleError(nsresult code, const nsACString& message) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIBRIDGEDSYNCENGINECALLBACK \
  nsresult HandleSuccess(nsIVariant *result); \
  nsresult HandleError(nsresult code, const nsACString& message); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIBRIDGEDSYNCENGINECALLBACK(_to) \
  NS_IMETHOD HandleSuccess(nsIVariant *result) override { return _to HandleSuccess(result); } \
  NS_IMETHOD HandleError(nsresult code, const nsACString& message) override { return _to HandleError(code, message); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIBRIDGEDSYNCENGINECALLBACK(_to) \
  NS_IMETHOD HandleSuccess(nsIVariant *result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleSuccess(result); } \
  NS_IMETHOD HandleError(nsresult code, const nsACString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleError(code, message); } 


/* starting interface:    mozIBridgedSyncEngineApplyCallback */
#define MOZIBRIDGEDSYNCENGINEAPPLYCALLBACK_IID_STR "2776cdd5-799a-4009-b2f3-356d940a5244"

#define MOZIBRIDGEDSYNCENGINEAPPLYCALLBACK_IID \
  {0x2776cdd5, 0x799a, 0x4009, \
    { 0xb2, 0xf3, 0x35, 0x6d, 0x94, 0x0a, 0x52, 0x44 }}

class NS_NO_VTABLE mozIBridgedSyncEngineApplyCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIBRIDGEDSYNCENGINEAPPLYCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIBridgedSyncEngineApplyCallback;

  /* void handleSuccess (in Array<AUTF8String> outgoingEnvelopesAsJSON); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleSuccess(const nsTArray<nsCString >& outgoingEnvelopesAsJSON) = 0;

  /* void handleError (in nsresult code, in AUTF8String message); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleError(nsresult code, const nsACString& message) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIBridgedSyncEngineApplyCallback, MOZIBRIDGEDSYNCENGINEAPPLYCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIBRIDGEDSYNCENGINEAPPLYCALLBACK \
  NS_IMETHOD HandleSuccess(const nsTArray<nsCString >& outgoingEnvelopesAsJSON) override; \
  NS_IMETHOD HandleError(nsresult code, const nsACString& message) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIBRIDGEDSYNCENGINEAPPLYCALLBACK \
  nsresult HandleSuccess(const nsTArray<nsCString >& outgoingEnvelopesAsJSON); \
  nsresult HandleError(nsresult code, const nsACString& message); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIBRIDGEDSYNCENGINEAPPLYCALLBACK(_to) \
  NS_IMETHOD HandleSuccess(const nsTArray<nsCString >& outgoingEnvelopesAsJSON) override { return _to HandleSuccess(outgoingEnvelopesAsJSON); } \
  NS_IMETHOD HandleError(nsresult code, const nsACString& message) override { return _to HandleError(code, message); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIBRIDGEDSYNCENGINEAPPLYCALLBACK(_to) \
  NS_IMETHOD HandleSuccess(const nsTArray<nsCString >& outgoingEnvelopesAsJSON) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleSuccess(outgoingEnvelopesAsJSON); } \
  NS_IMETHOD HandleError(nsresult code, const nsACString& message) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleError(code, message); } 


/* starting interface:    mozIBridgedSyncEngine */
#define MOZIBRIDGEDSYNCENGINE_IID_STR "3b2b80be-c30e-4498-8065-01809cfe8d47"

#define MOZIBRIDGEDSYNCENGINE_IID \
  {0x3b2b80be, 0xc30e, 0x4498, \
    { 0x80, 0x65, 0x01, 0x80, 0x9c, 0xfe, 0x8d, 0x47 }}

class NS_NO_VTABLE mozIBridgedSyncEngine : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIBRIDGEDSYNCENGINE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIBridgedSyncEngine;

  /* readonly attribute long storageVersion; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStorageVersion(int32_t *aStorageVersion) = 0;

  /* readonly attribute boolean allowSkippedRecord; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAllowSkippedRecord(bool *aAllowSkippedRecord) = 0;

  /* attribute mozIServicesLogSink logger; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLogger(mozIServicesLogSink **aLogger) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLogger(mozIServicesLogSink *aLogger) = 0;

  /* void getLastSync (in mozIBridgedSyncEngineCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastSync(mozIBridgedSyncEngineCallback *callback) = 0;

  /* void setLastSync (in long long lastSyncMillis, in mozIBridgedSyncEngineCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLastSync(int64_t lastSyncMillis, mozIBridgedSyncEngineCallback *callback) = 0;

  /* void getSyncId (in mozIBridgedSyncEngineCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSyncId(mozIBridgedSyncEngineCallback *callback) = 0;

  /* void resetSyncId (in mozIBridgedSyncEngineCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ResetSyncId(mozIBridgedSyncEngineCallback *callback) = 0;

  /* void ensureCurrentSyncId (in AUTF8String newSyncId, in mozIBridgedSyncEngineCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EnsureCurrentSyncId(const nsACString& newSyncId, mozIBridgedSyncEngineCallback *callback) = 0;

  /* void syncStarted (in mozIBridgedSyncEngineCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SyncStarted(mozIBridgedSyncEngineCallback *callback) = 0;

  /* void storeIncoming (in Array<AUTF8String> incomingEnvelopesAsJSON, in mozIBridgedSyncEngineCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StoreIncoming(const nsTArray<nsCString >& incomingEnvelopesAsJSON, mozIBridgedSyncEngineCallback *callback) = 0;

  /* void apply (in mozIBridgedSyncEngineApplyCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Apply(mozIBridgedSyncEngineApplyCallback *callback) = 0;

  /* void setUploaded (in long long newTimestampMillis, in Array<AUTF8String> uploadedIds, in mozIBridgedSyncEngineCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetUploaded(int64_t newTimestampMillis, const nsTArray<nsCString >& uploadedIds, mozIBridgedSyncEngineCallback *callback) = 0;

  /* void syncFinished (in mozIBridgedSyncEngineCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SyncFinished(mozIBridgedSyncEngineCallback *callback) = 0;

  /* void reset (in mozIBridgedSyncEngineCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Reset(mozIBridgedSyncEngineCallback *callback) = 0;

  /* void wipe (in mozIBridgedSyncEngineCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Wipe(mozIBridgedSyncEngineCallback *callback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIBridgedSyncEngine, MOZIBRIDGEDSYNCENGINE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIBRIDGEDSYNCENGINE \
  NS_IMETHOD GetStorageVersion(int32_t *aStorageVersion) override; \
  NS_IMETHOD GetAllowSkippedRecord(bool *aAllowSkippedRecord) override; \
  NS_IMETHOD GetLogger(mozIServicesLogSink **aLogger) override; \
  NS_IMETHOD SetLogger(mozIServicesLogSink *aLogger) override; \
  NS_IMETHOD GetLastSync(mozIBridgedSyncEngineCallback *callback) override; \
  NS_IMETHOD SetLastSync(int64_t lastSyncMillis, mozIBridgedSyncEngineCallback *callback) override; \
  NS_IMETHOD GetSyncId(mozIBridgedSyncEngineCallback *callback) override; \
  NS_IMETHOD ResetSyncId(mozIBridgedSyncEngineCallback *callback) override; \
  NS_IMETHOD EnsureCurrentSyncId(const nsACString& newSyncId, mozIBridgedSyncEngineCallback *callback) override; \
  NS_IMETHOD SyncStarted(mozIBridgedSyncEngineCallback *callback) override; \
  NS_IMETHOD StoreIncoming(const nsTArray<nsCString >& incomingEnvelopesAsJSON, mozIBridgedSyncEngineCallback *callback) override; \
  NS_IMETHOD Apply(mozIBridgedSyncEngineApplyCallback *callback) override; \
  NS_IMETHOD SetUploaded(int64_t newTimestampMillis, const nsTArray<nsCString >& uploadedIds, mozIBridgedSyncEngineCallback *callback) override; \
  NS_IMETHOD SyncFinished(mozIBridgedSyncEngineCallback *callback) override; \
  NS_IMETHOD Reset(mozIBridgedSyncEngineCallback *callback) override; \
  NS_IMETHOD Wipe(mozIBridgedSyncEngineCallback *callback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIBRIDGEDSYNCENGINE \
  nsresult GetStorageVersion(int32_t *aStorageVersion); \
  nsresult GetAllowSkippedRecord(bool *aAllowSkippedRecord); \
  nsresult GetLogger(mozIServicesLogSink **aLogger); \
  nsresult SetLogger(mozIServicesLogSink *aLogger); \
  nsresult GetLastSync(mozIBridgedSyncEngineCallback *callback); \
  nsresult SetLastSync(int64_t lastSyncMillis, mozIBridgedSyncEngineCallback *callback); \
  nsresult GetSyncId(mozIBridgedSyncEngineCallback *callback); \
  nsresult ResetSyncId(mozIBridgedSyncEngineCallback *callback); \
  nsresult EnsureCurrentSyncId(const nsACString& newSyncId, mozIBridgedSyncEngineCallback *callback); \
  nsresult SyncStarted(mozIBridgedSyncEngineCallback *callback); \
  nsresult StoreIncoming(const nsTArray<nsCString >& incomingEnvelopesAsJSON, mozIBridgedSyncEngineCallback *callback); \
  nsresult Apply(mozIBridgedSyncEngineApplyCallback *callback); \
  nsresult SetUploaded(int64_t newTimestampMillis, const nsTArray<nsCString >& uploadedIds, mozIBridgedSyncEngineCallback *callback); \
  nsresult SyncFinished(mozIBridgedSyncEngineCallback *callback); \
  nsresult Reset(mozIBridgedSyncEngineCallback *callback); \
  nsresult Wipe(mozIBridgedSyncEngineCallback *callback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIBRIDGEDSYNCENGINE(_to) \
  NS_IMETHOD GetStorageVersion(int32_t *aStorageVersion) override { return _to GetStorageVersion(aStorageVersion); } \
  NS_IMETHOD GetAllowSkippedRecord(bool *aAllowSkippedRecord) override { return _to GetAllowSkippedRecord(aAllowSkippedRecord); } \
  NS_IMETHOD GetLogger(mozIServicesLogSink **aLogger) override { return _to GetLogger(aLogger); } \
  NS_IMETHOD SetLogger(mozIServicesLogSink *aLogger) override { return _to SetLogger(aLogger); } \
  NS_IMETHOD GetLastSync(mozIBridgedSyncEngineCallback *callback) override { return _to GetLastSync(callback); } \
  NS_IMETHOD SetLastSync(int64_t lastSyncMillis, mozIBridgedSyncEngineCallback *callback) override { return _to SetLastSync(lastSyncMillis, callback); } \
  NS_IMETHOD GetSyncId(mozIBridgedSyncEngineCallback *callback) override { return _to GetSyncId(callback); } \
  NS_IMETHOD ResetSyncId(mozIBridgedSyncEngineCallback *callback) override { return _to ResetSyncId(callback); } \
  NS_IMETHOD EnsureCurrentSyncId(const nsACString& newSyncId, mozIBridgedSyncEngineCallback *callback) override { return _to EnsureCurrentSyncId(newSyncId, callback); } \
  NS_IMETHOD SyncStarted(mozIBridgedSyncEngineCallback *callback) override { return _to SyncStarted(callback); } \
  NS_IMETHOD StoreIncoming(const nsTArray<nsCString >& incomingEnvelopesAsJSON, mozIBridgedSyncEngineCallback *callback) override { return _to StoreIncoming(incomingEnvelopesAsJSON, callback); } \
  NS_IMETHOD Apply(mozIBridgedSyncEngineApplyCallback *callback) override { return _to Apply(callback); } \
  NS_IMETHOD SetUploaded(int64_t newTimestampMillis, const nsTArray<nsCString >& uploadedIds, mozIBridgedSyncEngineCallback *callback) override { return _to SetUploaded(newTimestampMillis, uploadedIds, callback); } \
  NS_IMETHOD SyncFinished(mozIBridgedSyncEngineCallback *callback) override { return _to SyncFinished(callback); } \
  NS_IMETHOD Reset(mozIBridgedSyncEngineCallback *callback) override { return _to Reset(callback); } \
  NS_IMETHOD Wipe(mozIBridgedSyncEngineCallback *callback) override { return _to Wipe(callback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIBRIDGEDSYNCENGINE(_to) \
  NS_IMETHOD GetStorageVersion(int32_t *aStorageVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStorageVersion(aStorageVersion); } \
  NS_IMETHOD GetAllowSkippedRecord(bool *aAllowSkippedRecord) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowSkippedRecord(aAllowSkippedRecord); } \
  NS_IMETHOD GetLogger(mozIServicesLogSink **aLogger) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLogger(aLogger); } \
  NS_IMETHOD SetLogger(mozIServicesLogSink *aLogger) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLogger(aLogger); } \
  NS_IMETHOD GetLastSync(mozIBridgedSyncEngineCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastSync(callback); } \
  NS_IMETHOD SetLastSync(int64_t lastSyncMillis, mozIBridgedSyncEngineCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLastSync(lastSyncMillis, callback); } \
  NS_IMETHOD GetSyncId(mozIBridgedSyncEngineCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSyncId(callback); } \
  NS_IMETHOD ResetSyncId(mozIBridgedSyncEngineCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetSyncId(callback); } \
  NS_IMETHOD EnsureCurrentSyncId(const nsACString& newSyncId, mozIBridgedSyncEngineCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnsureCurrentSyncId(newSyncId, callback); } \
  NS_IMETHOD SyncStarted(mozIBridgedSyncEngineCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SyncStarted(callback); } \
  NS_IMETHOD StoreIncoming(const nsTArray<nsCString >& incomingEnvelopesAsJSON, mozIBridgedSyncEngineCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StoreIncoming(incomingEnvelopesAsJSON, callback); } \
  NS_IMETHOD Apply(mozIBridgedSyncEngineApplyCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Apply(callback); } \
  NS_IMETHOD SetUploaded(int64_t newTimestampMillis, const nsTArray<nsCString >& uploadedIds, mozIBridgedSyncEngineCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUploaded(newTimestampMillis, uploadedIds, callback); } \
  NS_IMETHOD SyncFinished(mozIBridgedSyncEngineCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SyncFinished(callback); } \
  NS_IMETHOD Reset(mozIBridgedSyncEngineCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reset(callback); } \
  NS_IMETHOD Wipe(mozIBridgedSyncEngineCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Wipe(callback); } 


#endif /* __gen_mozIBridgedSyncEngine_h__ */
