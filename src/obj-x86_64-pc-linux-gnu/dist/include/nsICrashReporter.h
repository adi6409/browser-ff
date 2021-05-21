/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsICrashReporter.idl
 */

#ifndef __gen_nsICrashReporter_h__
#define __gen_nsICrashReporter_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */

class nsIURL; /* forward declaration */


/* starting interface:    nsICrashReporter */
#define NS_ICRASHREPORTER_IID_STR "4b74c39a-cf69-4a8a-8e63-169d81ad1ecf"

#define NS_ICRASHREPORTER_IID \
  {0x4b74c39a, 0xcf69, 0x4a8a, \
    { 0x8e, 0x63, 0x16, 0x9d, 0x81, 0xad, 0x1e, 0xcf }}

class NS_NO_VTABLE nsICrashReporter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICRASHREPORTER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICrashReporter;

  /* readonly attribute boolean enabled; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEnabled(bool *aEnabled) = 0;

  /* [noscript] void setEnabled (in bool enabled); */
  NS_IMETHOD SetEnabled(bool enabled) = 0;

  /* attribute nsIURL serverURL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetServerURL(nsIURL **aServerURL) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetServerURL(nsIURL *aServerURL) = 0;

  /* attribute nsIFile minidumpPath; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMinidumpPath(nsIFile **aMinidumpPath) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMinidumpPath(nsIFile *aMinidumpPath) = 0;

  /* nsIFile getMinidumpForID (in AString id); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMinidumpForID(const nsAString& id, nsIFile **_retval) = 0;

  /* nsIFile getExtraFileForID (in AString id); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExtraFileForID(const nsAString& id, nsIFile **_retval) = 0;

  /* void annotateCrashReport (in AUTF8String key, in AUTF8String data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AnnotateCrashReport(const nsACString& key, const nsACString& data) = 0;

  /* void removeCrashReportAnnotation (in AUTF8String key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveCrashReportAnnotation(const nsACString& key) = 0;

  /* boolean isAnnotationWhitelistedForPing (in ACString value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsAnnotationWhitelistedForPing(const nsACString& value, bool *_retval) = 0;

  /* void appendAppNotesToCrashReport (in ACString data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AppendAppNotesToCrashReport(const nsACString& data) = 0;

  /* void registerAppMemory (in unsigned long long ptr, in unsigned long long size); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterAppMemory(uint64_t ptr, uint64_t size) = 0;

  /* [noscript] void writeMinidumpForException (in voidPtr aExceptionInfo); */
  NS_IMETHOD WriteMinidumpForException(void * aExceptionInfo) = 0;

  /* [noscript] void appendObjCExceptionInfoToAppNotes (in voidPtr aException); */
  NS_IMETHOD AppendObjCExceptionInfoToAppNotes(void * aException) = 0;

  /* attribute boolean submitReports; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSubmitReports(bool *aSubmitReports) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSubmitReports(bool aSubmitReports) = 0;

  /* void UpdateCrashEventsDir (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateCrashEventsDir(void) = 0;

  /* void saveMemoryReport (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SaveMemoryReport(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICrashReporter, NS_ICRASHREPORTER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICRASHREPORTER \
  NS_IMETHOD GetEnabled(bool *aEnabled) override; \
  NS_IMETHOD SetEnabled(bool enabled) override; \
  NS_IMETHOD GetServerURL(nsIURL **aServerURL) override; \
  NS_IMETHOD SetServerURL(nsIURL *aServerURL) override; \
  NS_IMETHOD GetMinidumpPath(nsIFile **aMinidumpPath) override; \
  NS_IMETHOD SetMinidumpPath(nsIFile *aMinidumpPath) override; \
  NS_IMETHOD GetMinidumpForID(const nsAString& id, nsIFile **_retval) override; \
  NS_IMETHOD GetExtraFileForID(const nsAString& id, nsIFile **_retval) override; \
  NS_IMETHOD AnnotateCrashReport(const nsACString& key, const nsACString& data) override; \
  NS_IMETHOD RemoveCrashReportAnnotation(const nsACString& key) override; \
  NS_IMETHOD IsAnnotationWhitelistedForPing(const nsACString& value, bool *_retval) override; \
  NS_IMETHOD AppendAppNotesToCrashReport(const nsACString& data) override; \
  NS_IMETHOD RegisterAppMemory(uint64_t ptr, uint64_t size) override; \
  NS_IMETHOD WriteMinidumpForException(void * aExceptionInfo) override; \
  NS_IMETHOD AppendObjCExceptionInfoToAppNotes(void * aException) override; \
  NS_IMETHOD GetSubmitReports(bool *aSubmitReports) override; \
  NS_IMETHOD SetSubmitReports(bool aSubmitReports) override; \
  NS_IMETHOD UpdateCrashEventsDir(void) override; \
  NS_IMETHOD SaveMemoryReport(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICRASHREPORTER \
  nsresult GetEnabled(bool *aEnabled); \
  nsresult SetEnabled(bool enabled); \
  nsresult GetServerURL(nsIURL **aServerURL); \
  nsresult SetServerURL(nsIURL *aServerURL); \
  nsresult GetMinidumpPath(nsIFile **aMinidumpPath); \
  nsresult SetMinidumpPath(nsIFile *aMinidumpPath); \
  nsresult GetMinidumpForID(const nsAString& id, nsIFile **_retval); \
  nsresult GetExtraFileForID(const nsAString& id, nsIFile **_retval); \
  nsresult AnnotateCrashReport(const nsACString& key, const nsACString& data); \
  nsresult RemoveCrashReportAnnotation(const nsACString& key); \
  nsresult IsAnnotationWhitelistedForPing(const nsACString& value, bool *_retval); \
  nsresult AppendAppNotesToCrashReport(const nsACString& data); \
  nsresult RegisterAppMemory(uint64_t ptr, uint64_t size); \
  nsresult WriteMinidumpForException(void * aExceptionInfo); \
  nsresult AppendObjCExceptionInfoToAppNotes(void * aException); \
  nsresult GetSubmitReports(bool *aSubmitReports); \
  nsresult SetSubmitReports(bool aSubmitReports); \
  nsresult UpdateCrashEventsDir(void); \
  nsresult SaveMemoryReport(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICRASHREPORTER(_to) \
  NS_IMETHOD GetEnabled(bool *aEnabled) override { return _to GetEnabled(aEnabled); } \
  NS_IMETHOD SetEnabled(bool enabled) override { return _to SetEnabled(enabled); } \
  NS_IMETHOD GetServerURL(nsIURL **aServerURL) override { return _to GetServerURL(aServerURL); } \
  NS_IMETHOD SetServerURL(nsIURL *aServerURL) override { return _to SetServerURL(aServerURL); } \
  NS_IMETHOD GetMinidumpPath(nsIFile **aMinidumpPath) override { return _to GetMinidumpPath(aMinidumpPath); } \
  NS_IMETHOD SetMinidumpPath(nsIFile *aMinidumpPath) override { return _to SetMinidumpPath(aMinidumpPath); } \
  NS_IMETHOD GetMinidumpForID(const nsAString& id, nsIFile **_retval) override { return _to GetMinidumpForID(id, _retval); } \
  NS_IMETHOD GetExtraFileForID(const nsAString& id, nsIFile **_retval) override { return _to GetExtraFileForID(id, _retval); } \
  NS_IMETHOD AnnotateCrashReport(const nsACString& key, const nsACString& data) override { return _to AnnotateCrashReport(key, data); } \
  NS_IMETHOD RemoveCrashReportAnnotation(const nsACString& key) override { return _to RemoveCrashReportAnnotation(key); } \
  NS_IMETHOD IsAnnotationWhitelistedForPing(const nsACString& value, bool *_retval) override { return _to IsAnnotationWhitelistedForPing(value, _retval); } \
  NS_IMETHOD AppendAppNotesToCrashReport(const nsACString& data) override { return _to AppendAppNotesToCrashReport(data); } \
  NS_IMETHOD RegisterAppMemory(uint64_t ptr, uint64_t size) override { return _to RegisterAppMemory(ptr, size); } \
  NS_IMETHOD WriteMinidumpForException(void * aExceptionInfo) override { return _to WriteMinidumpForException(aExceptionInfo); } \
  NS_IMETHOD AppendObjCExceptionInfoToAppNotes(void * aException) override { return _to AppendObjCExceptionInfoToAppNotes(aException); } \
  NS_IMETHOD GetSubmitReports(bool *aSubmitReports) override { return _to GetSubmitReports(aSubmitReports); } \
  NS_IMETHOD SetSubmitReports(bool aSubmitReports) override { return _to SetSubmitReports(aSubmitReports); } \
  NS_IMETHOD UpdateCrashEventsDir(void) override { return _to UpdateCrashEventsDir(); } \
  NS_IMETHOD SaveMemoryReport(void) override { return _to SaveMemoryReport(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICRASHREPORTER(_to) \
  NS_IMETHOD GetEnabled(bool *aEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEnabled(aEnabled); } \
  NS_IMETHOD SetEnabled(bool enabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEnabled(enabled); } \
  NS_IMETHOD GetServerURL(nsIURL **aServerURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetServerURL(aServerURL); } \
  NS_IMETHOD SetServerURL(nsIURL *aServerURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetServerURL(aServerURL); } \
  NS_IMETHOD GetMinidumpPath(nsIFile **aMinidumpPath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMinidumpPath(aMinidumpPath); } \
  NS_IMETHOD SetMinidumpPath(nsIFile *aMinidumpPath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMinidumpPath(aMinidumpPath); } \
  NS_IMETHOD GetMinidumpForID(const nsAString& id, nsIFile **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMinidumpForID(id, _retval); } \
  NS_IMETHOD GetExtraFileForID(const nsAString& id, nsIFile **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExtraFileForID(id, _retval); } \
  NS_IMETHOD AnnotateCrashReport(const nsACString& key, const nsACString& data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AnnotateCrashReport(key, data); } \
  NS_IMETHOD RemoveCrashReportAnnotation(const nsACString& key) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveCrashReportAnnotation(key); } \
  NS_IMETHOD IsAnnotationWhitelistedForPing(const nsACString& value, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsAnnotationWhitelistedForPing(value, _retval); } \
  NS_IMETHOD AppendAppNotesToCrashReport(const nsACString& data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendAppNotesToCrashReport(data); } \
  NS_IMETHOD RegisterAppMemory(uint64_t ptr, uint64_t size) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterAppMemory(ptr, size); } \
  NS_IMETHOD WriteMinidumpForException(void * aExceptionInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteMinidumpForException(aExceptionInfo); } \
  NS_IMETHOD AppendObjCExceptionInfoToAppNotes(void * aException) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendObjCExceptionInfoToAppNotes(aException); } \
  NS_IMETHOD GetSubmitReports(bool *aSubmitReports) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSubmitReports(aSubmitReports); } \
  NS_IMETHOD SetSubmitReports(bool aSubmitReports) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSubmitReports(aSubmitReports); } \
  NS_IMETHOD UpdateCrashEventsDir(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateCrashEventsDir(); } \
  NS_IMETHOD SaveMemoryReport(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SaveMemoryReport(); } 


#endif /* __gen_nsICrashReporter_h__ */
