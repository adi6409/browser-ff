/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/bitsdownload/nsIBits.idl
 */

#ifndef __gen_nsIBits_h__
#define __gen_nsIBits_h__


#ifndef __gen_nsIRequest_h__
#include "nsIRequest.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIRequest; /* forward declaration */

class nsIRequestObserver; /* forward declaration */

class nsIBitsRequest; /* forward declaration */

class nsIBitsNewRequestCallback; /* forward declaration */

class nsIBitsCallback; /* forward declaration */

typedef int32_t  nsProxyUsage;

typedef int32_t  nsBitsErrorType;

typedef int32_t  nsBitsErrorAction;

typedef int32_t  nsBitsErrorStage;


/* starting interface:    nsIBits */
#define NS_IBITS_IID_STR "495d6f3d-9748-4d30-8ce5-0290c0001edf"

#define NS_IBITS_IID \
  {0x495d6f3d, 0x9748, 0x4d30, \
    { 0x8c, 0xe5, 0x02, 0x90, 0xc0, 0x00, 0x1e, 0xdf }}

class NS_NO_VTABLE nsIBits : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBITS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBits;

  enum {
    ERROR_TYPE_SUCCESS = 0,
    ERROR_TYPE_UNKNOWN = 1,
    ERROR_TYPE_METHOD_THREW = 2,
    ERROR_TYPE_METHOD_TIMEOUT = 3,
    ERROR_TYPE_NULL_ARGUMENT = 4,
    ERROR_TYPE_INVALID_ARGUMENT = 5,
    ERROR_TYPE_NOT_INITIALIZED = 6,
    ERROR_TYPE_NO_UTF8_CONVERSION = 7,
    ERROR_TYPE_INVALID_GUID = 8,
    ERROR_TYPE_PIPE_NOT_CONNECTED = 9,
    ERROR_TYPE_PIPE_TIMEOUT = 10,
    ERROR_TYPE_PIPE_BAD_WRITE_COUNT = 11,
    ERROR_TYPE_PIPE_API_ERROR = 12,
    ERROR_TYPE_FAILED_TO_CREATE_BITS_JOB = 13,
    ERROR_TYPE_FAILED_TO_ADD_FILE_TO_JOB = 14,
    ERROR_TYPE_FAILED_TO_APPLY_BITS_JOB_SETTINGS = 15,
    ERROR_TYPE_FAILED_TO_RESUME_BITS_JOB = 16,
    ERROR_TYPE_OTHER_BITS_ERROR = 17,
    ERROR_TYPE_OTHER_BITS_CLIENT_ERROR = 18,
    ERROR_TYPE_BITS_JOB_NOT_FOUND = 19,
    ERROR_TYPE_FAILED_TO_GET_BITS_JOB = 20,
    ERROR_TYPE_FAILED_TO_SUSPEND_BITS_JOB = 21,
    ERROR_TYPE_FAILED_TO_COMPLETE_BITS_JOB = 22,
    ERROR_TYPE_PARTIALLY_COMPLETED_BITS_JOB = 23,
    ERROR_TYPE_FAILED_TO_CANCEL_BITS_JOB = 24,
    ERROR_TYPE_MISSING_RESULT_DATA = 25,
    ERROR_TYPE_MISSING_CALLBACK = 26,
    ERROR_TYPE_CALLBACK_ON_WRONG_THREAD = 27,
    ERROR_TYPE_MISSING_BITS_SERVICE = 28,
    ERROR_TYPE_BITS_SERVICE_ON_WRONG_THREAD = 29,
    ERROR_TYPE_MISSING_BITS_REQUEST = 30,
    ERROR_TYPE_BITS_REQUEST_ON_WRONG_THREAD = 31,
    ERROR_TYPE_MISSING_OBSERVER = 32,
    ERROR_TYPE_OBSERVER_ON_WRONG_THREAD = 33,
    ERROR_TYPE_MISSING_CONTEXT = 34,
    ERROR_TYPE_CONTEXT_ON_WRONG_THREAD = 35,
    ERROR_TYPE_FAILED_TO_START_THREAD = 36,
    ERROR_TYPE_FAILED_TO_CONSTRUCT_TASK_RUNNABLE = 37,
    ERROR_TYPE_FAILED_TO_DISPATCH_RUNNABLE = 38,
    ERROR_TYPE_TRANSFER_ALREADY_COMPLETE = 39,
    ERROR_TYPE_OPERATION_ALREADY_IN_PROGRESS = 40,
    ERROR_TYPE_MISSING_BITS_CLIENT = 41,
    ERROR_TYPE_FAILED_TO_GET_JOB_STATUS = 42,
    ERROR_TYPE_BITS_STATE_ERROR = 43,
    ERROR_TYPE_BITS_STATE_TRANSIENT_ERROR = 44,
    ERROR_TYPE_BITS_STATE_CANCELLED = 45,
    ERROR_TYPE_BITS_STATE_UNEXPECTED = 46,
    ERROR_TYPE_VERIFICATION_FAILURE = 47,
    ERROR_TYPE_ACCESS_DENIED_EXPECTED = 48,
    ERROR_TYPE_FAILED_TO_CONNECT_TO_BCM = 49,
    ERROR_TYPE_USE_AFTER_REQUEST_SHUTDOWN = 50,
    ERROR_ACTION_UNKNOWN = 1,
    ERROR_ACTION_NONE = 2,
    ERROR_ACTION_START_DOWNLOAD = 3,
    ERROR_ACTION_MONITOR_DOWNLOAD = 4,
    ERROR_ACTION_CHANGE_MONITOR_INTERVAL = 5,
    ERROR_ACTION_CANCEL = 6,
    ERROR_ACTION_SET_PRIORITY = 7,
    ERROR_ACTION_COMPLETE = 8,
    ERROR_ACTION_SUSPEND = 9,
    ERROR_ACTION_RESUME = 10,
    ERROR_ACTION_SET_NO_PROGRESS_TIMEOUT = 11,
    ERROR_STAGE_UNKNOWN = 1,
    ERROR_STAGE_PRETASK = 2,
    ERROR_STAGE_COMMAND_THREAD = 3,
    ERROR_STAGE_AGENT_COMMUNICATION = 4,
    ERROR_STAGE_BITS_CLIENT = 5,
    ERROR_STAGE_MAIN_THREAD = 6,
    ERROR_STAGE_MONITOR = 7,
    ERROR_STAGE_VERIFICATION = 8,
    ERROR_CODE_TYPE_NONE = 1,
    ERROR_CODE_TYPE_NSRESULT = 2,
    ERROR_CODE_TYPE_HRESULT = 3,
    ERROR_CODE_TYPE_STRING = 4,
    ERROR_CODE_TYPE_EXCEPTION = 5
  };

  /* readonly attribute boolean initialized; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInitialized(bool *aInitialized) = 0;

  /* void init (in AUTF8String jobName, in AUTF8String savePathPrefix, in unsigned long monitorTimeoutMs); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(const nsACString& jobName, const nsACString& savePathPrefix, uint32_t monitorTimeoutMs) = 0;

  /* void startDownload (in AUTF8String downloadURL, in AUTF8String saveRelativePath, in nsProxyUsage proxy, in unsigned long noProgressTimeoutSecs, in unsigned long monitorIntervalMs, in nsIRequestObserver observer, in nsISupports context, in nsIBitsNewRequestCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StartDownload(const nsACString& downloadURL, const nsACString& saveRelativePath, nsProxyUsage proxy, uint32_t noProgressTimeoutSecs, uint32_t monitorIntervalMs, nsIRequestObserver *observer, nsISupports *context, nsIBitsNewRequestCallback *callback) = 0;

  enum {
    PROXY_NONE = 1,
    PROXY_PRECONFIG = 2,
    PROXY_AUTODETECT = 3
  };

  /* void monitorDownload (in AUTF8String id, in unsigned long monitorIntervalMs, in nsIRequestObserver observer, in nsISupports context, in nsIBitsNewRequestCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MonitorDownload(const nsACString& id, uint32_t monitorIntervalMs, nsIRequestObserver *observer, nsISupports *context, nsIBitsNewRequestCallback *callback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBits, NS_IBITS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBITS \
  NS_IMETHOD GetInitialized(bool *aInitialized) override; \
  NS_IMETHOD Init(const nsACString& jobName, const nsACString& savePathPrefix, uint32_t monitorTimeoutMs) override; \
  NS_IMETHOD StartDownload(const nsACString& downloadURL, const nsACString& saveRelativePath, nsProxyUsage proxy, uint32_t noProgressTimeoutSecs, uint32_t monitorIntervalMs, nsIRequestObserver *observer, nsISupports *context, nsIBitsNewRequestCallback *callback) override; \
  NS_IMETHOD MonitorDownload(const nsACString& id, uint32_t monitorIntervalMs, nsIRequestObserver *observer, nsISupports *context, nsIBitsNewRequestCallback *callback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBITS \
  nsresult GetInitialized(bool *aInitialized); \
  nsresult Init(const nsACString& jobName, const nsACString& savePathPrefix, uint32_t monitorTimeoutMs); \
  nsresult StartDownload(const nsACString& downloadURL, const nsACString& saveRelativePath, nsProxyUsage proxy, uint32_t noProgressTimeoutSecs, uint32_t monitorIntervalMs, nsIRequestObserver *observer, nsISupports *context, nsIBitsNewRequestCallback *callback); \
  nsresult MonitorDownload(const nsACString& id, uint32_t monitorIntervalMs, nsIRequestObserver *observer, nsISupports *context, nsIBitsNewRequestCallback *callback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBITS(_to) \
  NS_IMETHOD GetInitialized(bool *aInitialized) override { return _to GetInitialized(aInitialized); } \
  NS_IMETHOD Init(const nsACString& jobName, const nsACString& savePathPrefix, uint32_t monitorTimeoutMs) override { return _to Init(jobName, savePathPrefix, monitorTimeoutMs); } \
  NS_IMETHOD StartDownload(const nsACString& downloadURL, const nsACString& saveRelativePath, nsProxyUsage proxy, uint32_t noProgressTimeoutSecs, uint32_t monitorIntervalMs, nsIRequestObserver *observer, nsISupports *context, nsIBitsNewRequestCallback *callback) override { return _to StartDownload(downloadURL, saveRelativePath, proxy, noProgressTimeoutSecs, monitorIntervalMs, observer, context, callback); } \
  NS_IMETHOD MonitorDownload(const nsACString& id, uint32_t monitorIntervalMs, nsIRequestObserver *observer, nsISupports *context, nsIBitsNewRequestCallback *callback) override { return _to MonitorDownload(id, monitorIntervalMs, observer, context, callback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBITS(_to) \
  NS_IMETHOD GetInitialized(bool *aInitialized) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInitialized(aInitialized); } \
  NS_IMETHOD Init(const nsACString& jobName, const nsACString& savePathPrefix, uint32_t monitorTimeoutMs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(jobName, savePathPrefix, monitorTimeoutMs); } \
  NS_IMETHOD StartDownload(const nsACString& downloadURL, const nsACString& saveRelativePath, nsProxyUsage proxy, uint32_t noProgressTimeoutSecs, uint32_t monitorIntervalMs, nsIRequestObserver *observer, nsISupports *context, nsIBitsNewRequestCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartDownload(downloadURL, saveRelativePath, proxy, noProgressTimeoutSecs, monitorIntervalMs, observer, context, callback); } \
  NS_IMETHOD MonitorDownload(const nsACString& id, uint32_t monitorIntervalMs, nsIRequestObserver *observer, nsISupports *context, nsIBitsNewRequestCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MonitorDownload(id, monitorIntervalMs, observer, context, callback); } 


/* starting interface:    nsIBitsNewRequestCallback */
#define NS_IBITSNEWREQUESTCALLBACK_IID_STR "aa12e433-5b9f-452d-b5c9-840a9541328b"

#define NS_IBITSNEWREQUESTCALLBACK_IID \
  {0xaa12e433, 0x5b9f, 0x452d, \
    { 0xb5, 0xc9, 0x84, 0x0a, 0x95, 0x41, 0x32, 0x8b }}

class NS_NO_VTABLE nsIBitsNewRequestCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBITSNEWREQUESTCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBitsNewRequestCallback;

  /* void success (in nsIBitsRequest request); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Success(nsIBitsRequest *request) = 0;

  /* void failure (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Failure(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage) = 0;

  /* void failureNsresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in nsresult errorCode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FailureNsresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, nsresult errorCode) = 0;

  /* void failureHresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in long errorCode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FailureHresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, int32_t errorCode) = 0;

  /* void failureString (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in AUTF8String errorMessage); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FailureString(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, const nsACString& errorMessage) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBitsNewRequestCallback, NS_IBITSNEWREQUESTCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBITSNEWREQUESTCALLBACK \
  NS_IMETHOD Success(nsIBitsRequest *request) override; \
  NS_IMETHOD Failure(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage) override; \
  NS_IMETHOD FailureNsresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, nsresult errorCode) override; \
  NS_IMETHOD FailureHresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, int32_t errorCode) override; \
  NS_IMETHOD FailureString(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, const nsACString& errorMessage) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBITSNEWREQUESTCALLBACK \
  nsresult Success(nsIBitsRequest *request); \
  nsresult Failure(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage); \
  nsresult FailureNsresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, nsresult errorCode); \
  nsresult FailureHresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, int32_t errorCode); \
  nsresult FailureString(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, const nsACString& errorMessage); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBITSNEWREQUESTCALLBACK(_to) \
  NS_IMETHOD Success(nsIBitsRequest *request) override { return _to Success(request); } \
  NS_IMETHOD Failure(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage) override { return _to Failure(errorType, errorAction, errorStage); } \
  NS_IMETHOD FailureNsresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, nsresult errorCode) override { return _to FailureNsresult(errorType, errorAction, errorStage, errorCode); } \
  NS_IMETHOD FailureHresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, int32_t errorCode) override { return _to FailureHresult(errorType, errorAction, errorStage, errorCode); } \
  NS_IMETHOD FailureString(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, const nsACString& errorMessage) override { return _to FailureString(errorType, errorAction, errorStage, errorMessage); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBITSNEWREQUESTCALLBACK(_to) \
  NS_IMETHOD Success(nsIBitsRequest *request) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Success(request); } \
  NS_IMETHOD Failure(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Failure(errorType, errorAction, errorStage); } \
  NS_IMETHOD FailureNsresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, nsresult errorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FailureNsresult(errorType, errorAction, errorStage, errorCode); } \
  NS_IMETHOD FailureHresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, int32_t errorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FailureHresult(errorType, errorAction, errorStage, errorCode); } \
  NS_IMETHOD FailureString(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, const nsACString& errorMessage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FailureString(errorType, errorAction, errorStage, errorMessage); } 


/* starting interface:    nsIBitsRequest */
#define NS_IBITSREQUEST_IID_STR "ab9da0e9-06bf-4e73-bb1b-c0f2ea9ecc3e"

#define NS_IBITSREQUEST_IID \
  {0xab9da0e9, 0x06bf, 0x4e73, \
    { 0xbb, 0x1b, 0xc0, 0xf2, 0xea, 0x9e, 0xcc, 0x3e }}

class NS_NO_VTABLE nsIBitsRequest : public nsIRequest {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBITSREQUEST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBitsRequest;

  /* readonly attribute AUTF8String bitsId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBitsId(nsACString& aBitsId) = 0;

  /* readonly attribute nsBitsErrorType transferError; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTransferError(nsBitsErrorType *aTransferError) = 0;

  /* void changeMonitorInterval (in unsigned long monitorIntervalMs, in nsIBitsCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ChangeMonitorInterval(uint32_t monitorIntervalMs, nsIBitsCallback *callback) = 0;

  /* void cancelAsync (in nsresult status, in nsIBitsCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CancelAsync(nsresult status, nsIBitsCallback *callback) = 0;

  /* void setPriorityHigh (in nsIBitsCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPriorityHigh(nsIBitsCallback *callback) = 0;

  /* void setPriorityLow (in nsIBitsCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPriorityLow(nsIBitsCallback *callback) = 0;

  /* void setNoProgressTimeout (in unsigned long timeoutSecs, in nsIBitsCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetNoProgressTimeout(uint32_t timeoutSecs, nsIBitsCallback *callback) = 0;

  /* void complete (in nsIBitsCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Complete(nsIBitsCallback *callback) = 0;

  /* void suspendAsync (in nsIBitsCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SuspendAsync(nsIBitsCallback *callback) = 0;

  /* void resumeAsync (in nsIBitsCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ResumeAsync(nsIBitsCallback *callback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBitsRequest, NS_IBITSREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBITSREQUEST \
  NS_IMETHOD GetBitsId(nsACString& aBitsId) override; \
  NS_IMETHOD GetTransferError(nsBitsErrorType *aTransferError) override; \
  NS_IMETHOD ChangeMonitorInterval(uint32_t monitorIntervalMs, nsIBitsCallback *callback) override; \
  NS_IMETHOD CancelAsync(nsresult status, nsIBitsCallback *callback) override; \
  NS_IMETHOD SetPriorityHigh(nsIBitsCallback *callback) override; \
  NS_IMETHOD SetPriorityLow(nsIBitsCallback *callback) override; \
  NS_IMETHOD SetNoProgressTimeout(uint32_t timeoutSecs, nsIBitsCallback *callback) override; \
  NS_IMETHOD Complete(nsIBitsCallback *callback) override; \
  NS_IMETHOD SuspendAsync(nsIBitsCallback *callback) override; \
  NS_IMETHOD ResumeAsync(nsIBitsCallback *callback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBITSREQUEST \
  nsresult GetBitsId(nsACString& aBitsId); \
  nsresult GetTransferError(nsBitsErrorType *aTransferError); \
  nsresult ChangeMonitorInterval(uint32_t monitorIntervalMs, nsIBitsCallback *callback); \
  nsresult CancelAsync(nsresult status, nsIBitsCallback *callback); \
  nsresult SetPriorityHigh(nsIBitsCallback *callback); \
  nsresult SetPriorityLow(nsIBitsCallback *callback); \
  nsresult SetNoProgressTimeout(uint32_t timeoutSecs, nsIBitsCallback *callback); \
  nsresult Complete(nsIBitsCallback *callback); \
  nsresult SuspendAsync(nsIBitsCallback *callback); \
  nsresult ResumeAsync(nsIBitsCallback *callback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBITSREQUEST(_to) \
  NS_IMETHOD GetBitsId(nsACString& aBitsId) override { return _to GetBitsId(aBitsId); } \
  NS_IMETHOD GetTransferError(nsBitsErrorType *aTransferError) override { return _to GetTransferError(aTransferError); } \
  NS_IMETHOD ChangeMonitorInterval(uint32_t monitorIntervalMs, nsIBitsCallback *callback) override { return _to ChangeMonitorInterval(monitorIntervalMs, callback); } \
  NS_IMETHOD CancelAsync(nsresult status, nsIBitsCallback *callback) override { return _to CancelAsync(status, callback); } \
  NS_IMETHOD SetPriorityHigh(nsIBitsCallback *callback) override { return _to SetPriorityHigh(callback); } \
  NS_IMETHOD SetPriorityLow(nsIBitsCallback *callback) override { return _to SetPriorityLow(callback); } \
  NS_IMETHOD SetNoProgressTimeout(uint32_t timeoutSecs, nsIBitsCallback *callback) override { return _to SetNoProgressTimeout(timeoutSecs, callback); } \
  NS_IMETHOD Complete(nsIBitsCallback *callback) override { return _to Complete(callback); } \
  NS_IMETHOD SuspendAsync(nsIBitsCallback *callback) override { return _to SuspendAsync(callback); } \
  NS_IMETHOD ResumeAsync(nsIBitsCallback *callback) override { return _to ResumeAsync(callback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBITSREQUEST(_to) \
  NS_IMETHOD GetBitsId(nsACString& aBitsId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBitsId(aBitsId); } \
  NS_IMETHOD GetTransferError(nsBitsErrorType *aTransferError) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTransferError(aTransferError); } \
  NS_IMETHOD ChangeMonitorInterval(uint32_t monitorIntervalMs, nsIBitsCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ChangeMonitorInterval(monitorIntervalMs, callback); } \
  NS_IMETHOD CancelAsync(nsresult status, nsIBitsCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CancelAsync(status, callback); } \
  NS_IMETHOD SetPriorityHigh(nsIBitsCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPriorityHigh(callback); } \
  NS_IMETHOD SetPriorityLow(nsIBitsCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPriorityLow(callback); } \
  NS_IMETHOD SetNoProgressTimeout(uint32_t timeoutSecs, nsIBitsCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNoProgressTimeout(timeoutSecs, callback); } \
  NS_IMETHOD Complete(nsIBitsCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Complete(callback); } \
  NS_IMETHOD SuspendAsync(nsIBitsCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SuspendAsync(callback); } \
  NS_IMETHOD ResumeAsync(nsIBitsCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResumeAsync(callback); } 


/* starting interface:    nsIBitsCallback */
#define NS_IBITSCALLBACK_IID_STR "ea657e66-6bad-4e41-84d9-c6d107e9799d"

#define NS_IBITSCALLBACK_IID \
  {0xea657e66, 0x6bad, 0x4e41, \
    { 0x84, 0xd9, 0xc6, 0xd1, 0x07, 0xe9, 0x79, 0x9d }}

class NS_NO_VTABLE nsIBitsCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBITSCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBitsCallback;

  /* void success (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Success(void) = 0;

  /* void failure (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Failure(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage) = 0;

  /* void failureNsresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in nsresult errorCode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FailureNsresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, nsresult errorCode) = 0;

  /* void failureHresult (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in long errorCode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FailureHresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, int32_t errorCode) = 0;

  /* void failureString (in nsBitsErrorType errorType, in nsBitsErrorAction errorAction, in nsBitsErrorStage errorStage, in AUTF8String errorMessage); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FailureString(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, const nsACString& errorMessage) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBitsCallback, NS_IBITSCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBITSCALLBACK \
  NS_IMETHOD Success(void) override; \
  NS_IMETHOD Failure(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage) override; \
  NS_IMETHOD FailureNsresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, nsresult errorCode) override; \
  NS_IMETHOD FailureHresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, int32_t errorCode) override; \
  NS_IMETHOD FailureString(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, const nsACString& errorMessage) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBITSCALLBACK \
  nsresult Success(void); \
  nsresult Failure(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage); \
  nsresult FailureNsresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, nsresult errorCode); \
  nsresult FailureHresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, int32_t errorCode); \
  nsresult FailureString(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, const nsACString& errorMessage); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBITSCALLBACK(_to) \
  NS_IMETHOD Success(void) override { return _to Success(); } \
  NS_IMETHOD Failure(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage) override { return _to Failure(errorType, errorAction, errorStage); } \
  NS_IMETHOD FailureNsresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, nsresult errorCode) override { return _to FailureNsresult(errorType, errorAction, errorStage, errorCode); } \
  NS_IMETHOD FailureHresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, int32_t errorCode) override { return _to FailureHresult(errorType, errorAction, errorStage, errorCode); } \
  NS_IMETHOD FailureString(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, const nsACString& errorMessage) override { return _to FailureString(errorType, errorAction, errorStage, errorMessage); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBITSCALLBACK(_to) \
  NS_IMETHOD Success(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Success(); } \
  NS_IMETHOD Failure(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Failure(errorType, errorAction, errorStage); } \
  NS_IMETHOD FailureNsresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, nsresult errorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FailureNsresult(errorType, errorAction, errorStage, errorCode); } \
  NS_IMETHOD FailureHresult(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, int32_t errorCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FailureHresult(errorType, errorAction, errorStage, errorCode); } \
  NS_IMETHOD FailureString(nsBitsErrorType errorType, nsBitsErrorAction errorAction, nsBitsErrorStage errorStage, const nsACString& errorMessage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FailureString(errorType, errorAction, errorStage, errorMessage); } 

#define NS_BITS_CID \
  { 0xa334de05, 0xb9de, 0x46a1, \
    { 0x98, 0xa9, 0x3f, 0x5c, 0xed, 0x82, 0x1e, 0x68 } }
#define NS_BITS_CONTRACTID "@mozilla.org/bits;1"

#endif /* __gen_nsIBits_h__ */
