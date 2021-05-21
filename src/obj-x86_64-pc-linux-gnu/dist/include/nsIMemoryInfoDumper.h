/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIMemoryInfoDumper.idl
 */

#ifndef __gen_nsIMemoryInfoDumper_h__
#define __gen_nsIMemoryInfoDumper_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */

class nsICycleCollectorLogSink; /* forward declaration */


/* starting interface:    nsIFinishDumpingCallback */
#define NS_IFINISHDUMPINGCALLBACK_IID_STR "2dea18fc-fbfa-4bf7-ad45-0efaf5495f5e"

#define NS_IFINISHDUMPINGCALLBACK_IID \
  {0x2dea18fc, 0xfbfa, 0x4bf7, \
    { 0xad, 0x45, 0x0e, 0xfa, 0xf5, 0x49, 0x5f, 0x5e }}

class NS_NO_VTABLE nsIFinishDumpingCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFINISHDUMPINGCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFinishDumpingCallback;

  /* void callback (in nsISupports data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Callback(nsISupports *data) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFinishDumpingCallback, NS_IFINISHDUMPINGCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFINISHDUMPINGCALLBACK \
  NS_IMETHOD Callback(nsISupports *data) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFINISHDUMPINGCALLBACK \
  nsresult Callback(nsISupports *data); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFINISHDUMPINGCALLBACK(_to) \
  NS_IMETHOD Callback(nsISupports *data) override { return _to Callback(data); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFINISHDUMPINGCALLBACK(_to) \
  NS_IMETHOD Callback(nsISupports *data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Callback(data); } 


/* starting interface:    nsIDumpGCAndCCLogsCallback */
#define NS_IDUMPGCANDCCLOGSCALLBACK_IID_STR "dc1b2b24-65bd-441b-b6bd-cb5825a7ed14"

#define NS_IDUMPGCANDCCLOGSCALLBACK_IID \
  {0xdc1b2b24, 0x65bd, 0x441b, \
    { 0xb6, 0xbd, 0xcb, 0x58, 0x25, 0xa7, 0xed, 0x14 }}

class NS_NO_VTABLE nsIDumpGCAndCCLogsCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDUMPGCANDCCLOGSCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDumpGCAndCCLogsCallback;

  /* void onDump (in nsIFile aGCLog, in nsIFile aCCLog, in bool aIsParent); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnDump(nsIFile *aGCLog, nsIFile *aCCLog, bool aIsParent) = 0;

  /* void onFinish (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnFinish(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDumpGCAndCCLogsCallback, NS_IDUMPGCANDCCLOGSCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDUMPGCANDCCLOGSCALLBACK \
  NS_IMETHOD OnDump(nsIFile *aGCLog, nsIFile *aCCLog, bool aIsParent) override; \
  NS_IMETHOD OnFinish(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDUMPGCANDCCLOGSCALLBACK \
  nsresult OnDump(nsIFile *aGCLog, nsIFile *aCCLog, bool aIsParent); \
  nsresult OnFinish(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDUMPGCANDCCLOGSCALLBACK(_to) \
  NS_IMETHOD OnDump(nsIFile *aGCLog, nsIFile *aCCLog, bool aIsParent) override { return _to OnDump(aGCLog, aCCLog, aIsParent); } \
  NS_IMETHOD OnFinish(void) override { return _to OnFinish(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDUMPGCANDCCLOGSCALLBACK(_to) \
  NS_IMETHOD OnDump(nsIFile *aGCLog, nsIFile *aCCLog, bool aIsParent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnDump(aGCLog, aCCLog, aIsParent); } \
  NS_IMETHOD OnFinish(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnFinish(); } 


/* starting interface:    nsIMemoryInfoDumper */
#define NS_IMEMORYINFODUMPER_IID_STR "48541b74-47ee-4a62-9557-7f4b809bda5c"

#define NS_IMEMORYINFODUMPER_IID \
  {0x48541b74, 0x47ee, 0x4a62, \
    { 0x95, 0x57, 0x7f, 0x4b, 0x80, 0x9b, 0xda, 0x5c }}

class NS_NO_VTABLE nsIMemoryInfoDumper : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMEMORYINFODUMPER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMemoryInfoDumper;

  /* void dumpMemoryReportsToNamedFile (in AString aFilename, in nsIFinishDumpingCallback aFinishDumping, in nsISupports aFinishDumpingData, in boolean aAnonymize, in boolean aMinimizeMemoryUsage); */
  NS_IMETHOD DumpMemoryReportsToNamedFile(const nsAString& aFilename, nsIFinishDumpingCallback *aFinishDumping, nsISupports *aFinishDumpingData, bool aAnonymize, bool aMinimizeMemoryUsage) = 0;

  /* void dumpMemoryInfoToTempDir (in AString aIdentifier, in boolean aAnonymize, in boolean aMinimizeMemoryUsage); */
  NS_IMETHOD DumpMemoryInfoToTempDir(const nsAString& aIdentifier, bool aAnonymize, bool aMinimizeMemoryUsage) = 0;

  /* void dumpGCAndCCLogsToFile (in AString aIdentifier, in bool aDumpAllTraces, in bool aDumpChildProcesses, in nsIDumpGCAndCCLogsCallback aCallback); */
  NS_IMETHOD DumpGCAndCCLogsToFile(const nsAString& aIdentifier, bool aDumpAllTraces, bool aDumpChildProcesses, nsIDumpGCAndCCLogsCallback *aCallback) = 0;

  /* void dumpGCAndCCLogsToSink (in bool aDumpAllTraces, in nsICycleCollectorLogSink aSink); */
  NS_IMETHOD DumpGCAndCCLogsToSink(bool aDumpAllTraces, nsICycleCollectorLogSink *aSink) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMemoryInfoDumper, NS_IMEMORYINFODUMPER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMEMORYINFODUMPER \
  NS_IMETHOD DumpMemoryReportsToNamedFile(const nsAString& aFilename, nsIFinishDumpingCallback *aFinishDumping, nsISupports *aFinishDumpingData, bool aAnonymize, bool aMinimizeMemoryUsage) override; \
  NS_IMETHOD DumpMemoryInfoToTempDir(const nsAString& aIdentifier, bool aAnonymize, bool aMinimizeMemoryUsage) override; \
  NS_IMETHOD DumpGCAndCCLogsToFile(const nsAString& aIdentifier, bool aDumpAllTraces, bool aDumpChildProcesses, nsIDumpGCAndCCLogsCallback *aCallback) override; \
  NS_IMETHOD DumpGCAndCCLogsToSink(bool aDumpAllTraces, nsICycleCollectorLogSink *aSink) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMEMORYINFODUMPER \
  nsresult DumpMemoryReportsToNamedFile(const nsAString& aFilename, nsIFinishDumpingCallback *aFinishDumping, nsISupports *aFinishDumpingData, bool aAnonymize, bool aMinimizeMemoryUsage); \
  nsresult DumpMemoryInfoToTempDir(const nsAString& aIdentifier, bool aAnonymize, bool aMinimizeMemoryUsage); \
  nsresult DumpGCAndCCLogsToFile(const nsAString& aIdentifier, bool aDumpAllTraces, bool aDumpChildProcesses, nsIDumpGCAndCCLogsCallback *aCallback); \
  nsresult DumpGCAndCCLogsToSink(bool aDumpAllTraces, nsICycleCollectorLogSink *aSink); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMEMORYINFODUMPER(_to) \
  NS_IMETHOD DumpMemoryReportsToNamedFile(const nsAString& aFilename, nsIFinishDumpingCallback *aFinishDumping, nsISupports *aFinishDumpingData, bool aAnonymize, bool aMinimizeMemoryUsage) override { return _to DumpMemoryReportsToNamedFile(aFilename, aFinishDumping, aFinishDumpingData, aAnonymize, aMinimizeMemoryUsage); } \
  NS_IMETHOD DumpMemoryInfoToTempDir(const nsAString& aIdentifier, bool aAnonymize, bool aMinimizeMemoryUsage) override { return _to DumpMemoryInfoToTempDir(aIdentifier, aAnonymize, aMinimizeMemoryUsage); } \
  NS_IMETHOD DumpGCAndCCLogsToFile(const nsAString& aIdentifier, bool aDumpAllTraces, bool aDumpChildProcesses, nsIDumpGCAndCCLogsCallback *aCallback) override { return _to DumpGCAndCCLogsToFile(aIdentifier, aDumpAllTraces, aDumpChildProcesses, aCallback); } \
  NS_IMETHOD DumpGCAndCCLogsToSink(bool aDumpAllTraces, nsICycleCollectorLogSink *aSink) override { return _to DumpGCAndCCLogsToSink(aDumpAllTraces, aSink); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMEMORYINFODUMPER(_to) \
  NS_IMETHOD DumpMemoryReportsToNamedFile(const nsAString& aFilename, nsIFinishDumpingCallback *aFinishDumping, nsISupports *aFinishDumpingData, bool aAnonymize, bool aMinimizeMemoryUsage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DumpMemoryReportsToNamedFile(aFilename, aFinishDumping, aFinishDumpingData, aAnonymize, aMinimizeMemoryUsage); } \
  NS_IMETHOD DumpMemoryInfoToTempDir(const nsAString& aIdentifier, bool aAnonymize, bool aMinimizeMemoryUsage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DumpMemoryInfoToTempDir(aIdentifier, aAnonymize, aMinimizeMemoryUsage); } \
  NS_IMETHOD DumpGCAndCCLogsToFile(const nsAString& aIdentifier, bool aDumpAllTraces, bool aDumpChildProcesses, nsIDumpGCAndCCLogsCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DumpGCAndCCLogsToFile(aIdentifier, aDumpAllTraces, aDumpChildProcesses, aCallback); } \
  NS_IMETHOD DumpGCAndCCLogsToSink(bool aDumpAllTraces, nsICycleCollectorLogSink *aSink) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DumpGCAndCCLogsToSink(aDumpAllTraces, aSink); } 


#endif /* __gen_nsIMemoryInfoDumper_h__ */
