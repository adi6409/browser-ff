/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/telemetry/core/nsITelemetry.idl
 */

#ifndef __gen_nsITelemetry_h__
#define __gen_nsITelemetry_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIFile_h__
#include "nsIFile.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIFetchTelemetryDataCallback */
#define NS_IFETCHTELEMETRYDATACALLBACK_IID_STR "3d3b9075-5549-4244-9c08-b64fefa1dd60"

#define NS_IFETCHTELEMETRYDATACALLBACK_IID \
  {0x3d3b9075, 0x5549, 0x4244, \
    { 0x9c, 0x08, 0xb6, 0x4f, 0xef, 0xa1, 0xdd, 0x60 }}

class NS_NO_VTABLE nsIFetchTelemetryDataCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFETCHTELEMETRYDATACALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFetchTelemetryDataCallback;

  /* void complete (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Complete(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFetchTelemetryDataCallback, NS_IFETCHTELEMETRYDATACALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFETCHTELEMETRYDATACALLBACK \
  NS_IMETHOD Complete(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFETCHTELEMETRYDATACALLBACK \
  nsresult Complete(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFETCHTELEMETRYDATACALLBACK(_to) \
  NS_IMETHOD Complete(void) override { return _to Complete(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFETCHTELEMETRYDATACALLBACK(_to) \
  NS_IMETHOD Complete(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Complete(); } 


/* starting interface:    nsITelemetry */
#define NS_ITELEMETRY_IID_STR "273d2dd0-6c63-475a-b864-cb65160a1909"

#define NS_ITELEMETRY_IID \
  {0x273d2dd0, 0x6c63, 0x475a, \
    { 0xb8, 0x64, 0xcb, 0x65, 0x16, 0x0a, 0x19, 0x09 }}

class NS_NO_VTABLE nsITelemetry : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITELEMETRY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITelemetry;

  enum {
    HISTOGRAM_EXPONENTIAL = 0U,
    HISTOGRAM_LINEAR = 1U,
    HISTOGRAM_BOOLEAN = 2U,
    HISTOGRAM_FLAG = 3U,
    HISTOGRAM_COUNT = 4U,
    HISTOGRAM_CATEGORICAL = 5U,
    SCALAR_TYPE_COUNT = 0U,
    SCALAR_TYPE_STRING = 1U,
    SCALAR_TYPE_BOOLEAN = 2U,
    DATASET_ALL_CHANNELS = 0U,
    DATASET_PRERELEASE_CHANNELS = 1U
  };

  /* [implicit_jscontext] jsval getCategoricalLabels (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCategoricalLabels(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval getSnapshotForHistograms ([optional] in ACString aStoreName, [optional] in boolean aClearStore, [optional] in boolean aFilterTest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSnapshotForHistograms(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval getSnapshotForKeyedHistograms ([optional] in ACString aStoreName, [optional] in boolean aClearStore, [optional] in boolean aFilterTest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSnapshotForKeyedHistograms(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval getSnapshotForScalars ([optional] in ACString aStoreName, [optional] in boolean aClearStore, [optional] in boolean aFilterTest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSnapshotForScalars(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval getSnapshotForKeyedScalars ([optional] in ACString aStoreName, [optional] in boolean aClearStore, [optional] in boolean aFilterTest); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSnapshotForKeyedScalars(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* readonly attribute uint32_t lastShutdownDuration; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastShutdownDuration(uint32_t *aLastShutdownDuration) = 0;

  /* readonly attribute uint32_t failedProfileLockCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFailedProfileLockCount(uint32_t *aFailedProfileLockCount) = 0;

  /* [implicit_jscontext] readonly attribute jsval slowSQL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSlowSQL(JSContext* cx, JS::MutableHandleValue aSlowSQL) = 0;

  /* [implicit_jscontext] readonly attribute jsval debugSlowSQL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDebugSlowSQL(JSContext* cx, JS::MutableHandleValue aDebugSlowSQL) = 0;

  enum {
    INCLUDE_OLD_LOADEVENTS = 1U,
    KEEP_LOADEVENTS_NEW = 2U,
    INCLUDE_PRIVATE_FIELDS_IN_LOADEVENTS = 4U,
    EXCLUDE_STACKINFO_FROM_LOADEVENTS = 8U
  };

  /* [implicit_jscontext] Promise getUntrustedModuleLoadEvents ([optional] in unsigned long aFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUntrustedModuleLoadEvents(uint32_t aFlags, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext] Promise getLoadedModules (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLoadedModules(JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext] readonly attribute jsval lateWrites; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLateWrites(JSContext* cx, JS::MutableHandleValue aLateWrites) = 0;

  /* [implicit_jscontext] jsval getHistogramById (in ACString id); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHistogramById(const nsACString& id, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval getKeyedHistogramById (in ACString id); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetKeyedHistogramById(const nsACString& id, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* attribute boolean canRecordBase; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCanRecordBase(bool *aCanRecordBase) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCanRecordBase(bool aCanRecordBase) = 0;

  /* attribute boolean canRecordExtended; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCanRecordExtended(bool *aCanRecordExtended) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCanRecordExtended(bool aCanRecordExtended) = 0;

  /* readonly attribute boolean canRecordReleaseData; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCanRecordReleaseData(bool *aCanRecordReleaseData) = 0;

  /* readonly attribute boolean canRecordPrereleaseData; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCanRecordPrereleaseData(bool *aCanRecordPrereleaseData) = 0;

  /* readonly attribute boolean isOfficialTelemetry; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsOfficialTelemetry(bool *aIsOfficialTelemetry) = 0;

  /* void setHistogramRecordingEnabled (in ACString id, in boolean enabled); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetHistogramRecordingEnabled(const nsACString& id, bool enabled) = 0;

  /* void asyncFetchTelemetryData (in nsIFetchTelemetryDataCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncFetchTelemetryData(nsIFetchTelemetryDataCallback *aCallback) = 0;

  /* [implicit_jscontext] readonly attribute jsval fileIOReports; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFileIOReports(JSContext* cx, JS::MutableHandleValue aFileIOReports) = 0;

  /* double msSinceProcessStart (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MsSinceProcessStart(double *_retval) = 0;

  /* double msSinceProcessStartIncludingSuspend (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MsSinceProcessStartIncludingSuspend(double *_retval) = 0;

  /* double msSinceProcessStartExcludingSuspend (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MsSinceProcessStartExcludingSuspend(double *_retval) = 0;

  /* double msSystemNow (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MsSystemNow(double *_retval) = 0;

  /* [implicit_jscontext] void scalarAdd (in ACString aName, in jsval aValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ScalarAdd(const nsACString& aName, JS::HandleValue aValue, JSContext* cx) = 0;

  /* [implicit_jscontext] void scalarSet (in ACString aName, in jsval aValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ScalarSet(const nsACString& aName, JS::HandleValue aValue, JSContext* cx) = 0;

  /* [implicit_jscontext] void scalarSetMaximum (in ACString aName, in jsval aValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ScalarSetMaximum(const nsACString& aName, JS::HandleValue aValue, JSContext* cx) = 0;

  /* [implicit_jscontext] void keyedScalarAdd (in ACString aName, in AString aKey, in jsval aValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD KeyedScalarAdd(const nsACString& aName, const nsAString& aKey, JS::HandleValue aValue, JSContext* cx) = 0;

  /* [implicit_jscontext] void keyedScalarSet (in ACString aName, in AString aKey, in jsval aValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD KeyedScalarSet(const nsACString& aName, const nsAString& aKey, JS::HandleValue aValue, JSContext* cx) = 0;

  /* [implicit_jscontext] void keyedScalarSetMaximum (in ACString aName, in AString aKey, in jsval aValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD KeyedScalarSetMaximum(const nsACString& aName, const nsAString& aKey, JS::HandleValue aValue, JSContext* cx) = 0;

  /* void clearScalars (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearScalars(void) = 0;

  /* void flushBatchedChildTelemetry (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FlushBatchedChildTelemetry(void) = 0;

  /* [implicit_jscontext,optional_argc] void recordEvent (in ACString aCategory, in ACString aMethod, in ACString aObject, [optional] in jsval aValue, [optional] in jsval extra); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RecordEvent(const nsACString& aCategory, const nsACString& aMethod, const nsACString& aObject, JS::HandleValue aValue, JS::HandleValue extra, JSContext* cx, uint8_t _argc) = 0;

  /* void setEventRecordingEnabled (in ACString aCategory, in boolean aEnabled); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetEventRecordingEnabled(const nsACString& aCategory, bool aEnabled) = 0;

  /* [implicit_jscontext,optional_argc] jsval snapshotEvents (in uint32_t aDataset, [optional] in boolean aClear, [optional] in uint32_t aEventLimit); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SnapshotEvents(uint32_t aDataset, bool aClear, uint32_t aEventLimit, JSContext* cx, uint8_t _argc, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] void registerEvents (in ACString aCategory, in jsval aEventData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterEvents(const nsACString& aCategory, JS::HandleValue aEventData, JSContext* cx) = 0;

  /* [implicit_jscontext] void registerBuiltinEvents (in ACString aCategory, in jsval aEventData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterBuiltinEvents(const nsACString& aCategory, JS::HandleValue aEventData, JSContext* cx) = 0;

  /* [implicit_jscontext] void registerScalars (in ACString aCategoryName, in jsval aScalarData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterScalars(const nsACString& aCategoryName, JS::HandleValue aScalarData, JSContext* cx) = 0;

  /* [implicit_jscontext] void registerBuiltinScalars (in ACString aCategoryName, in jsval aScalarData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterBuiltinScalars(const nsACString& aCategoryName, JS::HandleValue aScalarData, JSContext* cx) = 0;

  /* void clearEvents (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearEvents(void) = 0;

  /* [implicit_jscontext] jsval getAllStores (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAllStores(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* void earlyInit (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EarlyInit(void) = 0;

  /* void delayedInit (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DelayedInit(void) = 0;

  /* void shutdown (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Shutdown(void) = 0;

  /* [implicit_jscontext] Promise gatherMemory (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GatherMemory(JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext] jsval getOriginSnapshot ([optional] in boolean aClear); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOriginSnapshot(bool aClear, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] Promise getEncodedOriginSnapshot ([optional] in boolean aClear); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEncodedOriginSnapshot(bool aClear, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* void clearOrigins (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearOrigins(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITelemetry, NS_ITELEMETRY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITELEMETRY \
  NS_IMETHOD GetCategoricalLabels(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetSnapshotForHistograms(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetSnapshotForKeyedHistograms(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetSnapshotForScalars(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetSnapshotForKeyedScalars(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetLastShutdownDuration(uint32_t *aLastShutdownDuration) override; \
  NS_IMETHOD GetFailedProfileLockCount(uint32_t *aFailedProfileLockCount) override; \
  NS_IMETHOD GetSlowSQL(JSContext* cx, JS::MutableHandleValue aSlowSQL) override; \
  NS_IMETHOD GetDebugSlowSQL(JSContext* cx, JS::MutableHandleValue aDebugSlowSQL) override; \
  NS_IMETHOD GetUntrustedModuleLoadEvents(uint32_t aFlags, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetLoadedModules(JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetLateWrites(JSContext* cx, JS::MutableHandleValue aLateWrites) override; \
  NS_IMETHOD GetHistogramById(const nsACString& id, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetKeyedHistogramById(const nsACString& id, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetCanRecordBase(bool *aCanRecordBase) override; \
  NS_IMETHOD SetCanRecordBase(bool aCanRecordBase) override; \
  NS_IMETHOD GetCanRecordExtended(bool *aCanRecordExtended) override; \
  NS_IMETHOD SetCanRecordExtended(bool aCanRecordExtended) override; \
  NS_IMETHOD GetCanRecordReleaseData(bool *aCanRecordReleaseData) override; \
  NS_IMETHOD GetCanRecordPrereleaseData(bool *aCanRecordPrereleaseData) override; \
  NS_IMETHOD GetIsOfficialTelemetry(bool *aIsOfficialTelemetry) override; \
  NS_IMETHOD SetHistogramRecordingEnabled(const nsACString& id, bool enabled) override; \
  NS_IMETHOD AsyncFetchTelemetryData(nsIFetchTelemetryDataCallback *aCallback) override; \
  NS_IMETHOD GetFileIOReports(JSContext* cx, JS::MutableHandleValue aFileIOReports) override; \
  NS_IMETHOD MsSinceProcessStart(double *_retval) override; \
  NS_IMETHOD MsSinceProcessStartIncludingSuspend(double *_retval) override; \
  NS_IMETHOD MsSinceProcessStartExcludingSuspend(double *_retval) override; \
  NS_IMETHOD MsSystemNow(double *_retval) override; \
  NS_IMETHOD ScalarAdd(const nsACString& aName, JS::HandleValue aValue, JSContext* cx) override; \
  NS_IMETHOD ScalarSet(const nsACString& aName, JS::HandleValue aValue, JSContext* cx) override; \
  NS_IMETHOD ScalarSetMaximum(const nsACString& aName, JS::HandleValue aValue, JSContext* cx) override; \
  NS_IMETHOD KeyedScalarAdd(const nsACString& aName, const nsAString& aKey, JS::HandleValue aValue, JSContext* cx) override; \
  NS_IMETHOD KeyedScalarSet(const nsACString& aName, const nsAString& aKey, JS::HandleValue aValue, JSContext* cx) override; \
  NS_IMETHOD KeyedScalarSetMaximum(const nsACString& aName, const nsAString& aKey, JS::HandleValue aValue, JSContext* cx) override; \
  NS_IMETHOD ClearScalars(void) override; \
  NS_IMETHOD FlushBatchedChildTelemetry(void) override; \
  NS_IMETHOD RecordEvent(const nsACString& aCategory, const nsACString& aMethod, const nsACString& aObject, JS::HandleValue aValue, JS::HandleValue extra, JSContext* cx, uint8_t _argc) override; \
  NS_IMETHOD SetEventRecordingEnabled(const nsACString& aCategory, bool aEnabled) override; \
  NS_IMETHOD SnapshotEvents(uint32_t aDataset, bool aClear, uint32_t aEventLimit, JSContext* cx, uint8_t _argc, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD RegisterEvents(const nsACString& aCategory, JS::HandleValue aEventData, JSContext* cx) override; \
  NS_IMETHOD RegisterBuiltinEvents(const nsACString& aCategory, JS::HandleValue aEventData, JSContext* cx) override; \
  NS_IMETHOD RegisterScalars(const nsACString& aCategoryName, JS::HandleValue aScalarData, JSContext* cx) override; \
  NS_IMETHOD RegisterBuiltinScalars(const nsACString& aCategoryName, JS::HandleValue aScalarData, JSContext* cx) override; \
  NS_IMETHOD ClearEvents(void) override; \
  NS_IMETHOD GetAllStores(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD EarlyInit(void) override; \
  NS_IMETHOD DelayedInit(void) override; \
  NS_IMETHOD Shutdown(void) override; \
  NS_IMETHOD GatherMemory(JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetOriginSnapshot(bool aClear, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetEncodedOriginSnapshot(bool aClear, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD ClearOrigins(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITELEMETRY \
  nsresult GetCategoricalLabels(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetSnapshotForHistograms(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetSnapshotForKeyedHistograms(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetSnapshotForScalars(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetSnapshotForKeyedScalars(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetLastShutdownDuration(uint32_t *aLastShutdownDuration); \
  nsresult GetFailedProfileLockCount(uint32_t *aFailedProfileLockCount); \
  nsresult GetSlowSQL(JSContext* cx, JS::MutableHandleValue aSlowSQL); \
  nsresult GetDebugSlowSQL(JSContext* cx, JS::MutableHandleValue aDebugSlowSQL); \
  nsresult GetUntrustedModuleLoadEvents(uint32_t aFlags, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult GetLoadedModules(JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult GetLateWrites(JSContext* cx, JS::MutableHandleValue aLateWrites); \
  nsresult GetHistogramById(const nsACString& id, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetKeyedHistogramById(const nsACString& id, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetCanRecordBase(bool *aCanRecordBase); \
  nsresult SetCanRecordBase(bool aCanRecordBase); \
  nsresult GetCanRecordExtended(bool *aCanRecordExtended); \
  nsresult SetCanRecordExtended(bool aCanRecordExtended); \
  nsresult GetCanRecordReleaseData(bool *aCanRecordReleaseData); \
  nsresult GetCanRecordPrereleaseData(bool *aCanRecordPrereleaseData); \
  nsresult GetIsOfficialTelemetry(bool *aIsOfficialTelemetry); \
  nsresult SetHistogramRecordingEnabled(const nsACString& id, bool enabled); \
  nsresult AsyncFetchTelemetryData(nsIFetchTelemetryDataCallback *aCallback); \
  nsresult GetFileIOReports(JSContext* cx, JS::MutableHandleValue aFileIOReports); \
  nsresult MsSinceProcessStart(double *_retval); \
  nsresult MsSinceProcessStartIncludingSuspend(double *_retval); \
  nsresult MsSinceProcessStartExcludingSuspend(double *_retval); \
  nsresult MsSystemNow(double *_retval); \
  nsresult ScalarAdd(const nsACString& aName, JS::HandleValue aValue, JSContext* cx); \
  nsresult ScalarSet(const nsACString& aName, JS::HandleValue aValue, JSContext* cx); \
  nsresult ScalarSetMaximum(const nsACString& aName, JS::HandleValue aValue, JSContext* cx); \
  nsresult KeyedScalarAdd(const nsACString& aName, const nsAString& aKey, JS::HandleValue aValue, JSContext* cx); \
  nsresult KeyedScalarSet(const nsACString& aName, const nsAString& aKey, JS::HandleValue aValue, JSContext* cx); \
  nsresult KeyedScalarSetMaximum(const nsACString& aName, const nsAString& aKey, JS::HandleValue aValue, JSContext* cx); \
  nsresult ClearScalars(void); \
  nsresult FlushBatchedChildTelemetry(void); \
  nsresult RecordEvent(const nsACString& aCategory, const nsACString& aMethod, const nsACString& aObject, JS::HandleValue aValue, JS::HandleValue extra, JSContext* cx, uint8_t _argc); \
  nsresult SetEventRecordingEnabled(const nsACString& aCategory, bool aEnabled); \
  nsresult SnapshotEvents(uint32_t aDataset, bool aClear, uint32_t aEventLimit, JSContext* cx, uint8_t _argc, JS::MutableHandleValue _retval); \
  nsresult RegisterEvents(const nsACString& aCategory, JS::HandleValue aEventData, JSContext* cx); \
  nsresult RegisterBuiltinEvents(const nsACString& aCategory, JS::HandleValue aEventData, JSContext* cx); \
  nsresult RegisterScalars(const nsACString& aCategoryName, JS::HandleValue aScalarData, JSContext* cx); \
  nsresult RegisterBuiltinScalars(const nsACString& aCategoryName, JS::HandleValue aScalarData, JSContext* cx); \
  nsresult ClearEvents(void); \
  nsresult GetAllStores(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult EarlyInit(void); \
  nsresult DelayedInit(void); \
  nsresult Shutdown(void); \
  nsresult GatherMemory(JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult GetOriginSnapshot(bool aClear, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetEncodedOriginSnapshot(bool aClear, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult ClearOrigins(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITELEMETRY(_to) \
  NS_IMETHOD GetCategoricalLabels(JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetCategoricalLabels(cx, _retval); } \
  NS_IMETHOD GetSnapshotForHistograms(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetSnapshotForHistograms(aStoreName, aClearStore, aFilterTest, cx, _retval); } \
  NS_IMETHOD GetSnapshotForKeyedHistograms(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetSnapshotForKeyedHistograms(aStoreName, aClearStore, aFilterTest, cx, _retval); } \
  NS_IMETHOD GetSnapshotForScalars(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetSnapshotForScalars(aStoreName, aClearStore, aFilterTest, cx, _retval); } \
  NS_IMETHOD GetSnapshotForKeyedScalars(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetSnapshotForKeyedScalars(aStoreName, aClearStore, aFilterTest, cx, _retval); } \
  NS_IMETHOD GetLastShutdownDuration(uint32_t *aLastShutdownDuration) override { return _to GetLastShutdownDuration(aLastShutdownDuration); } \
  NS_IMETHOD GetFailedProfileLockCount(uint32_t *aFailedProfileLockCount) override { return _to GetFailedProfileLockCount(aFailedProfileLockCount); } \
  NS_IMETHOD GetSlowSQL(JSContext* cx, JS::MutableHandleValue aSlowSQL) override { return _to GetSlowSQL(cx, aSlowSQL); } \
  NS_IMETHOD GetDebugSlowSQL(JSContext* cx, JS::MutableHandleValue aDebugSlowSQL) override { return _to GetDebugSlowSQL(cx, aDebugSlowSQL); } \
  NS_IMETHOD GetUntrustedModuleLoadEvents(uint32_t aFlags, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to GetUntrustedModuleLoadEvents(aFlags, cx, _retval); } \
  NS_IMETHOD GetLoadedModules(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to GetLoadedModules(cx, _retval); } \
  NS_IMETHOD GetLateWrites(JSContext* cx, JS::MutableHandleValue aLateWrites) override { return _to GetLateWrites(cx, aLateWrites); } \
  NS_IMETHOD GetHistogramById(const nsACString& id, JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetHistogramById(id, cx, _retval); } \
  NS_IMETHOD GetKeyedHistogramById(const nsACString& id, JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetKeyedHistogramById(id, cx, _retval); } \
  NS_IMETHOD GetCanRecordBase(bool *aCanRecordBase) override { return _to GetCanRecordBase(aCanRecordBase); } \
  NS_IMETHOD SetCanRecordBase(bool aCanRecordBase) override { return _to SetCanRecordBase(aCanRecordBase); } \
  NS_IMETHOD GetCanRecordExtended(bool *aCanRecordExtended) override { return _to GetCanRecordExtended(aCanRecordExtended); } \
  NS_IMETHOD SetCanRecordExtended(bool aCanRecordExtended) override { return _to SetCanRecordExtended(aCanRecordExtended); } \
  NS_IMETHOD GetCanRecordReleaseData(bool *aCanRecordReleaseData) override { return _to GetCanRecordReleaseData(aCanRecordReleaseData); } \
  NS_IMETHOD GetCanRecordPrereleaseData(bool *aCanRecordPrereleaseData) override { return _to GetCanRecordPrereleaseData(aCanRecordPrereleaseData); } \
  NS_IMETHOD GetIsOfficialTelemetry(bool *aIsOfficialTelemetry) override { return _to GetIsOfficialTelemetry(aIsOfficialTelemetry); } \
  NS_IMETHOD SetHistogramRecordingEnabled(const nsACString& id, bool enabled) override { return _to SetHistogramRecordingEnabled(id, enabled); } \
  NS_IMETHOD AsyncFetchTelemetryData(nsIFetchTelemetryDataCallback *aCallback) override { return _to AsyncFetchTelemetryData(aCallback); } \
  NS_IMETHOD GetFileIOReports(JSContext* cx, JS::MutableHandleValue aFileIOReports) override { return _to GetFileIOReports(cx, aFileIOReports); } \
  NS_IMETHOD MsSinceProcessStart(double *_retval) override { return _to MsSinceProcessStart(_retval); } \
  NS_IMETHOD MsSinceProcessStartIncludingSuspend(double *_retval) override { return _to MsSinceProcessStartIncludingSuspend(_retval); } \
  NS_IMETHOD MsSinceProcessStartExcludingSuspend(double *_retval) override { return _to MsSinceProcessStartExcludingSuspend(_retval); } \
  NS_IMETHOD MsSystemNow(double *_retval) override { return _to MsSystemNow(_retval); } \
  NS_IMETHOD ScalarAdd(const nsACString& aName, JS::HandleValue aValue, JSContext* cx) override { return _to ScalarAdd(aName, aValue, cx); } \
  NS_IMETHOD ScalarSet(const nsACString& aName, JS::HandleValue aValue, JSContext* cx) override { return _to ScalarSet(aName, aValue, cx); } \
  NS_IMETHOD ScalarSetMaximum(const nsACString& aName, JS::HandleValue aValue, JSContext* cx) override { return _to ScalarSetMaximum(aName, aValue, cx); } \
  NS_IMETHOD KeyedScalarAdd(const nsACString& aName, const nsAString& aKey, JS::HandleValue aValue, JSContext* cx) override { return _to KeyedScalarAdd(aName, aKey, aValue, cx); } \
  NS_IMETHOD KeyedScalarSet(const nsACString& aName, const nsAString& aKey, JS::HandleValue aValue, JSContext* cx) override { return _to KeyedScalarSet(aName, aKey, aValue, cx); } \
  NS_IMETHOD KeyedScalarSetMaximum(const nsACString& aName, const nsAString& aKey, JS::HandleValue aValue, JSContext* cx) override { return _to KeyedScalarSetMaximum(aName, aKey, aValue, cx); } \
  NS_IMETHOD ClearScalars(void) override { return _to ClearScalars(); } \
  NS_IMETHOD FlushBatchedChildTelemetry(void) override { return _to FlushBatchedChildTelemetry(); } \
  NS_IMETHOD RecordEvent(const nsACString& aCategory, const nsACString& aMethod, const nsACString& aObject, JS::HandleValue aValue, JS::HandleValue extra, JSContext* cx, uint8_t _argc) override { return _to RecordEvent(aCategory, aMethod, aObject, aValue, extra, cx, _argc); } \
  NS_IMETHOD SetEventRecordingEnabled(const nsACString& aCategory, bool aEnabled) override { return _to SetEventRecordingEnabled(aCategory, aEnabled); } \
  NS_IMETHOD SnapshotEvents(uint32_t aDataset, bool aClear, uint32_t aEventLimit, JSContext* cx, uint8_t _argc, JS::MutableHandleValue _retval) override { return _to SnapshotEvents(aDataset, aClear, aEventLimit, cx, _argc, _retval); } \
  NS_IMETHOD RegisterEvents(const nsACString& aCategory, JS::HandleValue aEventData, JSContext* cx) override { return _to RegisterEvents(aCategory, aEventData, cx); } \
  NS_IMETHOD RegisterBuiltinEvents(const nsACString& aCategory, JS::HandleValue aEventData, JSContext* cx) override { return _to RegisterBuiltinEvents(aCategory, aEventData, cx); } \
  NS_IMETHOD RegisterScalars(const nsACString& aCategoryName, JS::HandleValue aScalarData, JSContext* cx) override { return _to RegisterScalars(aCategoryName, aScalarData, cx); } \
  NS_IMETHOD RegisterBuiltinScalars(const nsACString& aCategoryName, JS::HandleValue aScalarData, JSContext* cx) override { return _to RegisterBuiltinScalars(aCategoryName, aScalarData, cx); } \
  NS_IMETHOD ClearEvents(void) override { return _to ClearEvents(); } \
  NS_IMETHOD GetAllStores(JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetAllStores(cx, _retval); } \
  NS_IMETHOD EarlyInit(void) override { return _to EarlyInit(); } \
  NS_IMETHOD DelayedInit(void) override { return _to DelayedInit(); } \
  NS_IMETHOD Shutdown(void) override { return _to Shutdown(); } \
  NS_IMETHOD GatherMemory(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to GatherMemory(cx, _retval); } \
  NS_IMETHOD GetOriginSnapshot(bool aClear, JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetOriginSnapshot(aClear, cx, _retval); } \
  NS_IMETHOD GetEncodedOriginSnapshot(bool aClear, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to GetEncodedOriginSnapshot(aClear, cx, _retval); } \
  NS_IMETHOD ClearOrigins(void) override { return _to ClearOrigins(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITELEMETRY(_to) \
  NS_IMETHOD GetCategoricalLabels(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCategoricalLabels(cx, _retval); } \
  NS_IMETHOD GetSnapshotForHistograms(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSnapshotForHistograms(aStoreName, aClearStore, aFilterTest, cx, _retval); } \
  NS_IMETHOD GetSnapshotForKeyedHistograms(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSnapshotForKeyedHistograms(aStoreName, aClearStore, aFilterTest, cx, _retval); } \
  NS_IMETHOD GetSnapshotForScalars(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSnapshotForScalars(aStoreName, aClearStore, aFilterTest, cx, _retval); } \
  NS_IMETHOD GetSnapshotForKeyedScalars(const nsACString& aStoreName, bool aClearStore, bool aFilterTest, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSnapshotForKeyedScalars(aStoreName, aClearStore, aFilterTest, cx, _retval); } \
  NS_IMETHOD GetLastShutdownDuration(uint32_t *aLastShutdownDuration) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastShutdownDuration(aLastShutdownDuration); } \
  NS_IMETHOD GetFailedProfileLockCount(uint32_t *aFailedProfileLockCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFailedProfileLockCount(aFailedProfileLockCount); } \
  NS_IMETHOD GetSlowSQL(JSContext* cx, JS::MutableHandleValue aSlowSQL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSlowSQL(cx, aSlowSQL); } \
  NS_IMETHOD GetDebugSlowSQL(JSContext* cx, JS::MutableHandleValue aDebugSlowSQL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDebugSlowSQL(cx, aDebugSlowSQL); } \
  NS_IMETHOD GetUntrustedModuleLoadEvents(uint32_t aFlags, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUntrustedModuleLoadEvents(aFlags, cx, _retval); } \
  NS_IMETHOD GetLoadedModules(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadedModules(cx, _retval); } \
  NS_IMETHOD GetLateWrites(JSContext* cx, JS::MutableHandleValue aLateWrites) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLateWrites(cx, aLateWrites); } \
  NS_IMETHOD GetHistogramById(const nsACString& id, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHistogramById(id, cx, _retval); } \
  NS_IMETHOD GetKeyedHistogramById(const nsACString& id, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeyedHistogramById(id, cx, _retval); } \
  NS_IMETHOD GetCanRecordBase(bool *aCanRecordBase) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanRecordBase(aCanRecordBase); } \
  NS_IMETHOD SetCanRecordBase(bool aCanRecordBase) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCanRecordBase(aCanRecordBase); } \
  NS_IMETHOD GetCanRecordExtended(bool *aCanRecordExtended) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanRecordExtended(aCanRecordExtended); } \
  NS_IMETHOD SetCanRecordExtended(bool aCanRecordExtended) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCanRecordExtended(aCanRecordExtended); } \
  NS_IMETHOD GetCanRecordReleaseData(bool *aCanRecordReleaseData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanRecordReleaseData(aCanRecordReleaseData); } \
  NS_IMETHOD GetCanRecordPrereleaseData(bool *aCanRecordPrereleaseData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanRecordPrereleaseData(aCanRecordPrereleaseData); } \
  NS_IMETHOD GetIsOfficialTelemetry(bool *aIsOfficialTelemetry) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsOfficialTelemetry(aIsOfficialTelemetry); } \
  NS_IMETHOD SetHistogramRecordingEnabled(const nsACString& id, bool enabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHistogramRecordingEnabled(id, enabled); } \
  NS_IMETHOD AsyncFetchTelemetryData(nsIFetchTelemetryDataCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncFetchTelemetryData(aCallback); } \
  NS_IMETHOD GetFileIOReports(JSContext* cx, JS::MutableHandleValue aFileIOReports) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileIOReports(cx, aFileIOReports); } \
  NS_IMETHOD MsSinceProcessStart(double *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MsSinceProcessStart(_retval); } \
  NS_IMETHOD MsSinceProcessStartIncludingSuspend(double *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MsSinceProcessStartIncludingSuspend(_retval); } \
  NS_IMETHOD MsSinceProcessStartExcludingSuspend(double *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MsSinceProcessStartExcludingSuspend(_retval); } \
  NS_IMETHOD MsSystemNow(double *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MsSystemNow(_retval); } \
  NS_IMETHOD ScalarAdd(const nsACString& aName, JS::HandleValue aValue, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScalarAdd(aName, aValue, cx); } \
  NS_IMETHOD ScalarSet(const nsACString& aName, JS::HandleValue aValue, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScalarSet(aName, aValue, cx); } \
  NS_IMETHOD ScalarSetMaximum(const nsACString& aName, JS::HandleValue aValue, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScalarSetMaximum(aName, aValue, cx); } \
  NS_IMETHOD KeyedScalarAdd(const nsACString& aName, const nsAString& aKey, JS::HandleValue aValue, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->KeyedScalarAdd(aName, aKey, aValue, cx); } \
  NS_IMETHOD KeyedScalarSet(const nsACString& aName, const nsAString& aKey, JS::HandleValue aValue, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->KeyedScalarSet(aName, aKey, aValue, cx); } \
  NS_IMETHOD KeyedScalarSetMaximum(const nsACString& aName, const nsAString& aKey, JS::HandleValue aValue, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->KeyedScalarSetMaximum(aName, aKey, aValue, cx); } \
  NS_IMETHOD ClearScalars(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearScalars(); } \
  NS_IMETHOD FlushBatchedChildTelemetry(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FlushBatchedChildTelemetry(); } \
  NS_IMETHOD RecordEvent(const nsACString& aCategory, const nsACString& aMethod, const nsACString& aObject, JS::HandleValue aValue, JS::HandleValue extra, JSContext* cx, uint8_t _argc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RecordEvent(aCategory, aMethod, aObject, aValue, extra, cx, _argc); } \
  NS_IMETHOD SetEventRecordingEnabled(const nsACString& aCategory, bool aEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEventRecordingEnabled(aCategory, aEnabled); } \
  NS_IMETHOD SnapshotEvents(uint32_t aDataset, bool aClear, uint32_t aEventLimit, JSContext* cx, uint8_t _argc, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SnapshotEvents(aDataset, aClear, aEventLimit, cx, _argc, _retval); } \
  NS_IMETHOD RegisterEvents(const nsACString& aCategory, JS::HandleValue aEventData, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterEvents(aCategory, aEventData, cx); } \
  NS_IMETHOD RegisterBuiltinEvents(const nsACString& aCategory, JS::HandleValue aEventData, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterBuiltinEvents(aCategory, aEventData, cx); } \
  NS_IMETHOD RegisterScalars(const nsACString& aCategoryName, JS::HandleValue aScalarData, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterScalars(aCategoryName, aScalarData, cx); } \
  NS_IMETHOD RegisterBuiltinScalars(const nsACString& aCategoryName, JS::HandleValue aScalarData, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterBuiltinScalars(aCategoryName, aScalarData, cx); } \
  NS_IMETHOD ClearEvents(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearEvents(); } \
  NS_IMETHOD GetAllStores(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllStores(cx, _retval); } \
  NS_IMETHOD EarlyInit(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EarlyInit(); } \
  NS_IMETHOD DelayedInit(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DelayedInit(); } \
  NS_IMETHOD Shutdown(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Shutdown(); } \
  NS_IMETHOD GatherMemory(JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GatherMemory(cx, _retval); } \
  NS_IMETHOD GetOriginSnapshot(bool aClear, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginSnapshot(aClear, cx, _retval); } \
  NS_IMETHOD GetEncodedOriginSnapshot(bool aClear, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEncodedOriginSnapshot(aClear, cx, _retval); } \
  NS_IMETHOD ClearOrigins(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearOrigins(); } 


#endif /* __gen_nsITelemetry_h__ */
