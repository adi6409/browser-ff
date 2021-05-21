/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/startup/public/nsIAppStartup.idl
 */

#ifndef __gen_nsIAppStartup_h__
#define __gen_nsIAppStartup_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIToolkitProfile; /* forward declaration */


/* starting interface:    nsIAppStartup */
#define NS_IAPPSTARTUP_IID_STR "6621f6d5-6c04-4a0e-9e74-447db221484e"

#define NS_IAPPSTARTUP_IID \
  {0x6621f6d5, 0x6c04, 0x4a0e, \
    { 0x9e, 0x74, 0x44, 0x7d, 0xb2, 0x21, 0x48, 0x4e }}

class NS_NO_VTABLE nsIAppStartup : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAPPSTARTUP_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAppStartup;

  /* void createHiddenWindow (); */
  NS_IMETHOD CreateHiddenWindow(void) = 0;

  /* void destroyHiddenWindow (); */
  NS_IMETHOD DestroyHiddenWindow(void) = 0;

  /* void run (); */
  NS_IMETHOD Run(void) = 0;

  /* void enterLastWindowClosingSurvivalArea (); */
  NS_IMETHOD EnterLastWindowClosingSurvivalArea(void) = 0;

  /* void exitLastWindowClosingSurvivalArea (); */
  NS_IMETHOD ExitLastWindowClosingSurvivalArea(void) = 0;

  /* readonly attribute boolean automaticSafeModeNecessary; */
  NS_IMETHOD GetAutomaticSafeModeNecessary(bool *aAutomaticSafeModeNecessary) = 0;

  /* void restartInSafeMode (in uint32_t aQuitMode); */
  NS_IMETHOD RestartInSafeMode(uint32_t aQuitMode) = 0;

  /* void createInstanceWithProfile (in nsIToolkitProfile aProfile); */
  NS_IMETHOD CreateInstanceWithProfile(nsIToolkitProfile *aProfile) = 0;

  /* bool trackStartupCrashBegin (); */
  NS_IMETHOD TrackStartupCrashBegin(bool *_retval) = 0;

  /* void trackStartupCrashEnd (); */
  NS_IMETHOD TrackStartupCrashEnd(void) = 0;

  enum {
    eConsiderQuit = 1U,
    eAttemptQuit = 2U,
    eForceQuit = 3U,
    eRestart = 16U
  };

  /* bool quit (in uint32_t aMode, [optional] in int32_t aExitCode); */
  NS_IMETHOD Quit(uint32_t aMode, int32_t aExitCode, bool *_retval) = 0;

  /* [infallible] readonly attribute boolean shuttingDown; */
  NS_IMETHOD GetShuttingDown(bool *aShuttingDown) = 0;
  inline bool  GetShuttingDown()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetShuttingDown(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* readonly attribute boolean startingUp; */
  NS_IMETHOD GetStartingUp(bool *aStartingUp) = 0;

  /* [noscript] void doneStartingUp (); */
  NS_IMETHOD DoneStartingUp(void) = 0;

  /* readonly attribute boolean restarting; */
  NS_IMETHOD GetRestarting(bool *aRestarting) = 0;

  /* readonly attribute boolean wasRestarted; */
  NS_IMETHOD GetWasRestarted(bool *aWasRestarted) = 0;

  /* readonly attribute int64_t secondsSinceLastOSRestart; */
  NS_IMETHOD GetSecondsSinceLastOSRestart(int64_t *aSecondsSinceLastOSRestart) = 0;

  /* [implicit_jscontext] jsval getStartupInfo (); */
  NS_IMETHOD GetStartupInfo(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* attribute boolean interrupted; */
  NS_IMETHOD GetInterrupted(bool *aInterrupted) = 0;
  NS_IMETHOD SetInterrupted(bool aInterrupted) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAppStartup, NS_IAPPSTARTUP_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAPPSTARTUP \
  NS_IMETHOD CreateHiddenWindow(void) override; \
  NS_IMETHOD DestroyHiddenWindow(void) override; \
  NS_IMETHOD Run(void) override; \
  NS_IMETHOD EnterLastWindowClosingSurvivalArea(void) override; \
  NS_IMETHOD ExitLastWindowClosingSurvivalArea(void) override; \
  NS_IMETHOD GetAutomaticSafeModeNecessary(bool *aAutomaticSafeModeNecessary) override; \
  NS_IMETHOD RestartInSafeMode(uint32_t aQuitMode) override; \
  NS_IMETHOD CreateInstanceWithProfile(nsIToolkitProfile *aProfile) override; \
  NS_IMETHOD TrackStartupCrashBegin(bool *_retval) override; \
  NS_IMETHOD TrackStartupCrashEnd(void) override; \
  NS_IMETHOD Quit(uint32_t aMode, int32_t aExitCode, bool *_retval) override; \
  using nsIAppStartup::GetShuttingDown; \
  NS_IMETHOD GetShuttingDown(bool *aShuttingDown) override; \
  NS_IMETHOD GetStartingUp(bool *aStartingUp) override; \
  NS_IMETHOD DoneStartingUp(void) override; \
  NS_IMETHOD GetRestarting(bool *aRestarting) override; \
  NS_IMETHOD GetWasRestarted(bool *aWasRestarted) override; \
  NS_IMETHOD GetSecondsSinceLastOSRestart(int64_t *aSecondsSinceLastOSRestart) override; \
  NS_IMETHOD GetStartupInfo(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetInterrupted(bool *aInterrupted) override; \
  NS_IMETHOD SetInterrupted(bool aInterrupted) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAPPSTARTUP \
  nsresult CreateHiddenWindow(void); \
  nsresult DestroyHiddenWindow(void); \
  nsresult Run(void); \
  nsresult EnterLastWindowClosingSurvivalArea(void); \
  nsresult ExitLastWindowClosingSurvivalArea(void); \
  nsresult GetAutomaticSafeModeNecessary(bool *aAutomaticSafeModeNecessary); \
  nsresult RestartInSafeMode(uint32_t aQuitMode); \
  nsresult CreateInstanceWithProfile(nsIToolkitProfile *aProfile); \
  nsresult TrackStartupCrashBegin(bool *_retval); \
  nsresult TrackStartupCrashEnd(void); \
  nsresult Quit(uint32_t aMode, int32_t aExitCode, bool *_retval); \
  using nsIAppStartup::GetShuttingDown; \
  nsresult GetShuttingDown(bool *aShuttingDown); \
  nsresult GetStartingUp(bool *aStartingUp); \
  nsresult DoneStartingUp(void); \
  nsresult GetRestarting(bool *aRestarting); \
  nsresult GetWasRestarted(bool *aWasRestarted); \
  nsresult GetSecondsSinceLastOSRestart(int64_t *aSecondsSinceLastOSRestart); \
  nsresult GetStartupInfo(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetInterrupted(bool *aInterrupted); \
  nsresult SetInterrupted(bool aInterrupted); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAPPSTARTUP(_to) \
  NS_IMETHOD CreateHiddenWindow(void) override { return _to CreateHiddenWindow(); } \
  NS_IMETHOD DestroyHiddenWindow(void) override { return _to DestroyHiddenWindow(); } \
  NS_IMETHOD Run(void) override { return _to Run(); } \
  NS_IMETHOD EnterLastWindowClosingSurvivalArea(void) override { return _to EnterLastWindowClosingSurvivalArea(); } \
  NS_IMETHOD ExitLastWindowClosingSurvivalArea(void) override { return _to ExitLastWindowClosingSurvivalArea(); } \
  NS_IMETHOD GetAutomaticSafeModeNecessary(bool *aAutomaticSafeModeNecessary) override { return _to GetAutomaticSafeModeNecessary(aAutomaticSafeModeNecessary); } \
  NS_IMETHOD RestartInSafeMode(uint32_t aQuitMode) override { return _to RestartInSafeMode(aQuitMode); } \
  NS_IMETHOD CreateInstanceWithProfile(nsIToolkitProfile *aProfile) override { return _to CreateInstanceWithProfile(aProfile); } \
  NS_IMETHOD TrackStartupCrashBegin(bool *_retval) override { return _to TrackStartupCrashBegin(_retval); } \
  NS_IMETHOD TrackStartupCrashEnd(void) override { return _to TrackStartupCrashEnd(); } \
  NS_IMETHOD Quit(uint32_t aMode, int32_t aExitCode, bool *_retval) override { return _to Quit(aMode, aExitCode, _retval); } \
  using nsIAppStartup::GetShuttingDown; \
  NS_IMETHOD GetShuttingDown(bool *aShuttingDown) override { return _to GetShuttingDown(aShuttingDown); } \
  NS_IMETHOD GetStartingUp(bool *aStartingUp) override { return _to GetStartingUp(aStartingUp); } \
  NS_IMETHOD DoneStartingUp(void) override { return _to DoneStartingUp(); } \
  NS_IMETHOD GetRestarting(bool *aRestarting) override { return _to GetRestarting(aRestarting); } \
  NS_IMETHOD GetWasRestarted(bool *aWasRestarted) override { return _to GetWasRestarted(aWasRestarted); } \
  NS_IMETHOD GetSecondsSinceLastOSRestart(int64_t *aSecondsSinceLastOSRestart) override { return _to GetSecondsSinceLastOSRestart(aSecondsSinceLastOSRestart); } \
  NS_IMETHOD GetStartupInfo(JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetStartupInfo(cx, _retval); } \
  NS_IMETHOD GetInterrupted(bool *aInterrupted) override { return _to GetInterrupted(aInterrupted); } \
  NS_IMETHOD SetInterrupted(bool aInterrupted) override { return _to SetInterrupted(aInterrupted); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAPPSTARTUP(_to) \
  NS_IMETHOD CreateHiddenWindow(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateHiddenWindow(); } \
  NS_IMETHOD DestroyHiddenWindow(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DestroyHiddenWindow(); } \
  NS_IMETHOD Run(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Run(); } \
  NS_IMETHOD EnterLastWindowClosingSurvivalArea(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnterLastWindowClosingSurvivalArea(); } \
  NS_IMETHOD ExitLastWindowClosingSurvivalArea(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExitLastWindowClosingSurvivalArea(); } \
  NS_IMETHOD GetAutomaticSafeModeNecessary(bool *aAutomaticSafeModeNecessary) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAutomaticSafeModeNecessary(aAutomaticSafeModeNecessary); } \
  NS_IMETHOD RestartInSafeMode(uint32_t aQuitMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RestartInSafeMode(aQuitMode); } \
  NS_IMETHOD CreateInstanceWithProfile(nsIToolkitProfile *aProfile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateInstanceWithProfile(aProfile); } \
  NS_IMETHOD TrackStartupCrashBegin(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TrackStartupCrashBegin(_retval); } \
  NS_IMETHOD TrackStartupCrashEnd(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TrackStartupCrashEnd(); } \
  NS_IMETHOD Quit(uint32_t aMode, int32_t aExitCode, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Quit(aMode, aExitCode, _retval); } \
  NS_IMETHOD GetShuttingDown(bool *aShuttingDown) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetShuttingDown(aShuttingDown); } \
  NS_IMETHOD GetStartingUp(bool *aStartingUp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStartingUp(aStartingUp); } \
  NS_IMETHOD DoneStartingUp(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoneStartingUp(); } \
  NS_IMETHOD GetRestarting(bool *aRestarting) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRestarting(aRestarting); } \
  NS_IMETHOD GetWasRestarted(bool *aWasRestarted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWasRestarted(aWasRestarted); } \
  NS_IMETHOD GetSecondsSinceLastOSRestart(int64_t *aSecondsSinceLastOSRestart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecondsSinceLastOSRestart(aSecondsSinceLastOSRestart); } \
  NS_IMETHOD GetStartupInfo(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStartupInfo(cx, _retval); } \
  NS_IMETHOD GetInterrupted(bool *aInterrupted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInterrupted(aInterrupted); } \
  NS_IMETHOD SetInterrupted(bool aInterrupted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInterrupted(aInterrupted); } 


#endif /* __gen_nsIAppStartup_h__ */
