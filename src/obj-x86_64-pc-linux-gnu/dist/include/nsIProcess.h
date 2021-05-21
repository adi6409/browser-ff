/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIProcess.idl
 */

#ifndef __gen_nsIProcess_h__
#define __gen_nsIProcess_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */

class nsIObserver; /* forward declaration */


/* starting interface:    nsIProcess */
#define NS_IPROCESS_IID_STR "609610de-9954-4a63-8a7c-346350a86403"

#define NS_IPROCESS_IID \
  {0x609610de, 0x9954, 0x4a63, \
    { 0x8a, 0x7c, 0x34, 0x63, 0x50, 0xa8, 0x64, 0x03 }}

class NS_NO_VTABLE nsIProcess : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROCESS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProcess;

  /* void init (in nsIFile executable); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIFile *executable) = 0;

  /* void kill (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Kill(void) = 0;

  /* void run (in boolean blocking, [array, size_is (count)] in string args, in unsigned long count); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Run(bool blocking, const char * *args, uint32_t count) = 0;

  /* void runAsync ([array, size_is (count)] in string args, in unsigned long count, [optional] in nsIObserver observer, [optional] in boolean holdWeak); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RunAsync(const char * *args, uint32_t count, nsIObserver *observer, bool holdWeak) = 0;

  /* void runw (in boolean blocking, [array, size_is (count)] in wstring args, in unsigned long count); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Runw(bool blocking, const char16_t * *args, uint32_t count) = 0;

  /* void runwAsync ([array, size_is (count)] in wstring args, in unsigned long count, [optional] in nsIObserver observer, [optional] in boolean holdWeak); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RunwAsync(const char16_t * *args, uint32_t count, nsIObserver *observer, bool holdWeak) = 0;

  /* attribute boolean startHidden; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStartHidden(bool *aStartHidden) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetStartHidden(bool aStartHidden) = 0;

  /* attribute boolean noShell; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNoShell(bool *aNoShell) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetNoShell(bool aNoShell) = 0;

  /* readonly attribute unsigned long pid; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPid(uint32_t *aPid) = 0;

  /* readonly attribute long exitValue; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExitValue(int32_t *aExitValue) = 0;

  /* readonly attribute boolean isRunning; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsRunning(bool *aIsRunning) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProcess, NS_IPROCESS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROCESS \
  NS_IMETHOD Init(nsIFile *executable) override; \
  NS_IMETHOD Kill(void) override; \
  NS_IMETHOD Run(bool blocking, const char * *args, uint32_t count) override; \
  NS_IMETHOD RunAsync(const char * *args, uint32_t count, nsIObserver *observer, bool holdWeak) override; \
  NS_IMETHOD Runw(bool blocking, const char16_t * *args, uint32_t count) override; \
  NS_IMETHOD RunwAsync(const char16_t * *args, uint32_t count, nsIObserver *observer, bool holdWeak) override; \
  NS_IMETHOD GetStartHidden(bool *aStartHidden) override; \
  NS_IMETHOD SetStartHidden(bool aStartHidden) override; \
  NS_IMETHOD GetNoShell(bool *aNoShell) override; \
  NS_IMETHOD SetNoShell(bool aNoShell) override; \
  NS_IMETHOD GetPid(uint32_t *aPid) override; \
  NS_IMETHOD GetExitValue(int32_t *aExitValue) override; \
  NS_IMETHOD GetIsRunning(bool *aIsRunning) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROCESS \
  nsresult Init(nsIFile *executable); \
  nsresult Kill(void); \
  nsresult Run(bool blocking, const char * *args, uint32_t count); \
  nsresult RunAsync(const char * *args, uint32_t count, nsIObserver *observer, bool holdWeak); \
  nsresult Runw(bool blocking, const char16_t * *args, uint32_t count); \
  nsresult RunwAsync(const char16_t * *args, uint32_t count, nsIObserver *observer, bool holdWeak); \
  nsresult GetStartHidden(bool *aStartHidden); \
  nsresult SetStartHidden(bool aStartHidden); \
  nsresult GetNoShell(bool *aNoShell); \
  nsresult SetNoShell(bool aNoShell); \
  nsresult GetPid(uint32_t *aPid); \
  nsresult GetExitValue(int32_t *aExitValue); \
  nsresult GetIsRunning(bool *aIsRunning); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROCESS(_to) \
  NS_IMETHOD Init(nsIFile *executable) override { return _to Init(executable); } \
  NS_IMETHOD Kill(void) override { return _to Kill(); } \
  NS_IMETHOD Run(bool blocking, const char * *args, uint32_t count) override { return _to Run(blocking, args, count); } \
  NS_IMETHOD RunAsync(const char * *args, uint32_t count, nsIObserver *observer, bool holdWeak) override { return _to RunAsync(args, count, observer, holdWeak); } \
  NS_IMETHOD Runw(bool blocking, const char16_t * *args, uint32_t count) override { return _to Runw(blocking, args, count); } \
  NS_IMETHOD RunwAsync(const char16_t * *args, uint32_t count, nsIObserver *observer, bool holdWeak) override { return _to RunwAsync(args, count, observer, holdWeak); } \
  NS_IMETHOD GetStartHidden(bool *aStartHidden) override { return _to GetStartHidden(aStartHidden); } \
  NS_IMETHOD SetStartHidden(bool aStartHidden) override { return _to SetStartHidden(aStartHidden); } \
  NS_IMETHOD GetNoShell(bool *aNoShell) override { return _to GetNoShell(aNoShell); } \
  NS_IMETHOD SetNoShell(bool aNoShell) override { return _to SetNoShell(aNoShell); } \
  NS_IMETHOD GetPid(uint32_t *aPid) override { return _to GetPid(aPid); } \
  NS_IMETHOD GetExitValue(int32_t *aExitValue) override { return _to GetExitValue(aExitValue); } \
  NS_IMETHOD GetIsRunning(bool *aIsRunning) override { return _to GetIsRunning(aIsRunning); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROCESS(_to) \
  NS_IMETHOD Init(nsIFile *executable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(executable); } \
  NS_IMETHOD Kill(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Kill(); } \
  NS_IMETHOD Run(bool blocking, const char * *args, uint32_t count) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Run(blocking, args, count); } \
  NS_IMETHOD RunAsync(const char * *args, uint32_t count, nsIObserver *observer, bool holdWeak) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RunAsync(args, count, observer, holdWeak); } \
  NS_IMETHOD Runw(bool blocking, const char16_t * *args, uint32_t count) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Runw(blocking, args, count); } \
  NS_IMETHOD RunwAsync(const char16_t * *args, uint32_t count, nsIObserver *observer, bool holdWeak) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RunwAsync(args, count, observer, holdWeak); } \
  NS_IMETHOD GetStartHidden(bool *aStartHidden) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStartHidden(aStartHidden); } \
  NS_IMETHOD SetStartHidden(bool aStartHidden) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetStartHidden(aStartHidden); } \
  NS_IMETHOD GetNoShell(bool *aNoShell) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNoShell(aNoShell); } \
  NS_IMETHOD SetNoShell(bool aNoShell) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNoShell(aNoShell); } \
  NS_IMETHOD GetPid(uint32_t *aPid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPid(aPid); } \
  NS_IMETHOD GetExitValue(int32_t *aExitValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExitValue(aExitValue); } \
  NS_IMETHOD GetIsRunning(bool *aIsRunning) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsRunning(aIsRunning); } 


#define NS_PROCESS_CONTRACTID "@mozilla.org/process/util;1"

#endif /* __gen_nsIProcess_h__ */
