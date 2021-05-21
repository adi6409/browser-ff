/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIXULRuntime.idl
 */

#ifndef __gen_nsIXULRuntime_h__
#define __gen_nsIXULRuntime_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

namespace mozilla {
// Simple C++ getter for nsIXULRuntime::browserTabsRemoteAutostart
// This getter is a temporary function that checks for special
// conditions in which e10s support is not great yet, and should
// therefore be disabled. Bug 1065561 tracks its removal.
bool BrowserTabsRemoteAutostart();
uint32_t GetMaxWebProcessCount();
// Returns the value of the fission.autostart pref. Since fission can be
// disabled on a per-window basis, this should only be used when you need the
// global value of the pref. For other use cases, you should use
// nsILoadContext::UseRemoteSubframes instead. This will also check for special
// conditions, like safe mode, which may require fission to be disabled, or
// environment variable MOZ_FORCE_ENABLE_FISSION, used by mach run to
// enable fission regardless of pref settings.
bool FissionAutostart();
// Returns true if fission.sessionHistoryInParent or FissionAutostart() is true.
bool SessionHistoryInParent();
}

/* starting interface:    nsIXULRuntime */
#define NS_IXULRUNTIME_IID_STR "03602fac-fa3f-4a50-9baa-b88456fb4a0f"

#define NS_IXULRUNTIME_IID \
  {0x03602fac, 0xfa3f, 0x4a50, \
    { 0x9b, 0xaa, 0xb8, 0x84, 0x56, 0xfb, 0x4a, 0x0f }}

class NS_NO_VTABLE nsIXULRuntime : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXULRUNTIME_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIXULRuntime;

  /* readonly attribute boolean inSafeMode; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInSafeMode(bool *aInSafeMode) = 0;

  enum ExperimentStatus : uint8_t {
    eExperimentStatusUnenrolled = 0,
    eExperimentStatusControl = 1,
    eExperimentStatusTreatment = 2,
    eExperimentStatusDisqualified = 3,
    eExperimentStatusCount = 4,
  };

  enum FissionDecisionStatus : uint8_t {
    eFissionStatusUnknown = 0,
    eFissionExperimentControl = 1,
    eFissionExperimentTreatment = 2,
    eFissionDisabledByE10sEnv = 3,
    eFissionEnabledByEnv = 4,
    eFissionDisabledBySafeMode = 5,
    eFissionEnabledByDefault = 6,
    eFissionDisabledByDefault = 7,
    eFissionEnabledByUserPref = 8,
    eFissionDisabledByUserPref = 9,
    eFissionDisabledByE10sOther = 10,
  };

  /* readonly attribute boolean fissionAutostart; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFissionAutostart(bool *aFissionAutostart) = 0;

  /* readonly attribute nsIXULRuntime_ExperimentStatus fissionExperimentStatus; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFissionExperimentStatus(nsIXULRuntime::ExperimentStatus *aFissionExperimentStatus) = 0;

  /* readonly attribute nsIXULRuntime_FissionDecisionStatus fissionDecisionStatus; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFissionDecisionStatus(nsIXULRuntime::FissionDecisionStatus *aFissionDecisionStatus) = 0;

  /* readonly attribute ACString fissionDecisionStatusString; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFissionDecisionStatusString(nsACString& aFissionDecisionStatusString) = 0;

  /* readonly attribute boolean sessionHistoryInParent; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSessionHistoryInParent(bool *aSessionHistoryInParent) = 0;

  /* attribute boolean logConsoleErrors; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLogConsoleErrors(bool *aLogConsoleErrors) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLogConsoleErrors(bool aLogConsoleErrors) = 0;

  /* readonly attribute AUTF8String OS; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOS(nsACString& aOS) = 0;

  /* readonly attribute AUTF8String XPCOMABI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetXPCOMABI(nsACString& aXPCOMABI) = 0;

  /* readonly attribute AUTF8String widgetToolkit; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWidgetToolkit(nsACString& aWidgetToolkit) = 0;

  enum {
    PROCESS_TYPE_DEFAULT = 0U,
    PROCESS_TYPE_PLUGIN = 1U,
    PROCESS_TYPE_CONTENT = 2U,
    PROCESS_TYPE_IPDLUNITTEST = 3U,
    PROCESS_TYPE_GMPLUGIN = 4U,
    PROCESS_TYPE_GPU = 5U,
    PROCESS_TYPE_VR = 6U,
    PROCESS_TYPE_RDD = 7U,
    PROCESS_TYPE_SOCKET = 8U,
    PROCESS_TYPE_SANDBOX_BROKER = 9U,
    PROCESS_TYPE_FORKSERVER = 10U
  };

  /* readonly attribute unsigned long processType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProcessType(uint32_t *aProcessType) = 0;

  /* readonly attribute unsigned long processID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProcessID(uint32_t *aProcessID) = 0;

  /* readonly attribute uint64_t uniqueProcessID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUniqueProcessID(uint64_t *aUniqueProcessID) = 0;

  /* readonly attribute AUTF8String remoteType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRemoteType(nsACString& aRemoteType) = 0;

  /* readonly attribute boolean browserTabsRemoteAutostart; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBrowserTabsRemoteAutostart(bool *aBrowserTabsRemoteAutostart) = 0;

  /* readonly attribute uint32_t maxWebProcessCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMaxWebProcessCount(uint32_t *aMaxWebProcessCount) = 0;

  enum {
    E10S_MULTI_EXPERIMENT = 1U
  };

  /* readonly attribute boolean accessibilityEnabled; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAccessibilityEnabled(bool *aAccessibilityEnabled) = 0;

  /* readonly attribute boolean accessibleHandlerUsed; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAccessibleHandlerUsed(bool *aAccessibleHandlerUsed) = 0;

  /* readonly attribute AString accessibilityInstantiator; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAccessibilityInstantiator(nsAString& aAccessibilityInstantiator) = 0;

  /* readonly attribute boolean shouldBlockIncompatJaws; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetShouldBlockIncompatJaws(bool *aShouldBlockIncompatJaws) = 0;

  /* readonly attribute boolean is64Bit; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIs64Bit(bool *aIs64Bit) = 0;

  /* void invalidateCachesOnRestart (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InvalidateCachesOnRestart(void) = 0;

  /* void ensureContentProcess (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EnsureContentProcess(void) = 0;

  /* readonly attribute PRTime replacedLockTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetReplacedLockTime(PRTime *aReplacedLockTime) = 0;

  /* readonly attribute boolean isReleaseOrBeta; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsReleaseOrBeta(bool *aIsReleaseOrBeta) = 0;

  /* readonly attribute boolean isOfficialBranding; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsOfficialBranding(bool *aIsOfficialBranding) = 0;

  /* readonly attribute AUTF8String defaultUpdateChannel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultUpdateChannel(nsACString& aDefaultUpdateChannel) = 0;

  /* readonly attribute AUTF8String distributionID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDistributionID(nsACString& aDistributionID) = 0;

  /* readonly attribute boolean windowsDLLBlocklistStatus; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWindowsDLLBlocklistStatus(bool *aWindowsDLLBlocklistStatus) = 0;

  /* readonly attribute boolean restartedByOS; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRestartedByOS(bool *aRestartedByOS) = 0;

  /* readonly attribute AString processStartupShortcut; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProcessStartupShortcut(nsAString& aProcessStartupShortcut) = 0;

  /* readonly attribute uint32_t launcherProcessState; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLauncherProcessState(uint32_t *aLauncherProcessState) = 0;

  /* readonly attribute ACString lastAppVersion; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastAppVersion(nsACString& aLastAppVersion) = 0;

  /* readonly attribute ACString lastAppBuildID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastAppBuildID(nsACString& aLastAppBuildID) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXULRuntime, NS_IXULRUNTIME_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXULRUNTIME \
  NS_IMETHOD GetInSafeMode(bool *aInSafeMode) override; \
  NS_IMETHOD GetFissionAutostart(bool *aFissionAutostart) override; \
  NS_IMETHOD GetFissionExperimentStatus(nsIXULRuntime::ExperimentStatus *aFissionExperimentStatus) override; \
  NS_IMETHOD GetFissionDecisionStatus(nsIXULRuntime::FissionDecisionStatus *aFissionDecisionStatus) override; \
  NS_IMETHOD GetFissionDecisionStatusString(nsACString& aFissionDecisionStatusString) override; \
  NS_IMETHOD GetSessionHistoryInParent(bool *aSessionHistoryInParent) override; \
  NS_IMETHOD GetLogConsoleErrors(bool *aLogConsoleErrors) override; \
  NS_IMETHOD SetLogConsoleErrors(bool aLogConsoleErrors) override; \
  NS_IMETHOD GetOS(nsACString& aOS) override; \
  NS_IMETHOD GetXPCOMABI(nsACString& aXPCOMABI) override; \
  NS_IMETHOD GetWidgetToolkit(nsACString& aWidgetToolkit) override; \
  NS_IMETHOD GetProcessType(uint32_t *aProcessType) override; \
  NS_IMETHOD GetProcessID(uint32_t *aProcessID) override; \
  NS_IMETHOD GetUniqueProcessID(uint64_t *aUniqueProcessID) override; \
  NS_IMETHOD GetRemoteType(nsACString& aRemoteType) override; \
  NS_IMETHOD GetBrowserTabsRemoteAutostart(bool *aBrowserTabsRemoteAutostart) override; \
  NS_IMETHOD GetMaxWebProcessCount(uint32_t *aMaxWebProcessCount) override; \
  NS_IMETHOD GetAccessibilityEnabled(bool *aAccessibilityEnabled) override; \
  NS_IMETHOD GetAccessibleHandlerUsed(bool *aAccessibleHandlerUsed) override; \
  NS_IMETHOD GetAccessibilityInstantiator(nsAString& aAccessibilityInstantiator) override; \
  NS_IMETHOD GetShouldBlockIncompatJaws(bool *aShouldBlockIncompatJaws) override; \
  NS_IMETHOD GetIs64Bit(bool *aIs64Bit) override; \
  NS_IMETHOD InvalidateCachesOnRestart(void) override; \
  NS_IMETHOD EnsureContentProcess(void) override; \
  NS_IMETHOD GetReplacedLockTime(PRTime *aReplacedLockTime) override; \
  NS_IMETHOD GetIsReleaseOrBeta(bool *aIsReleaseOrBeta) override; \
  NS_IMETHOD GetIsOfficialBranding(bool *aIsOfficialBranding) override; \
  NS_IMETHOD GetDefaultUpdateChannel(nsACString& aDefaultUpdateChannel) override; \
  NS_IMETHOD GetDistributionID(nsACString& aDistributionID) override; \
  NS_IMETHOD GetWindowsDLLBlocklistStatus(bool *aWindowsDLLBlocklistStatus) override; \
  NS_IMETHOD GetRestartedByOS(bool *aRestartedByOS) override; \
  NS_IMETHOD GetProcessStartupShortcut(nsAString& aProcessStartupShortcut) override; \
  NS_IMETHOD GetLauncherProcessState(uint32_t *aLauncherProcessState) override; \
  NS_IMETHOD GetLastAppVersion(nsACString& aLastAppVersion) override; \
  NS_IMETHOD GetLastAppBuildID(nsACString& aLastAppBuildID) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXULRUNTIME \
  nsresult GetInSafeMode(bool *aInSafeMode); \
  nsresult GetFissionAutostart(bool *aFissionAutostart); \
  nsresult GetFissionExperimentStatus(nsIXULRuntime::ExperimentStatus *aFissionExperimentStatus); \
  nsresult GetFissionDecisionStatus(nsIXULRuntime::FissionDecisionStatus *aFissionDecisionStatus); \
  nsresult GetFissionDecisionStatusString(nsACString& aFissionDecisionStatusString); \
  nsresult GetSessionHistoryInParent(bool *aSessionHistoryInParent); \
  nsresult GetLogConsoleErrors(bool *aLogConsoleErrors); \
  nsresult SetLogConsoleErrors(bool aLogConsoleErrors); \
  nsresult GetOS(nsACString& aOS); \
  nsresult GetXPCOMABI(nsACString& aXPCOMABI); \
  nsresult GetWidgetToolkit(nsACString& aWidgetToolkit); \
  nsresult GetProcessType(uint32_t *aProcessType); \
  nsresult GetProcessID(uint32_t *aProcessID); \
  nsresult GetUniqueProcessID(uint64_t *aUniqueProcessID); \
  nsresult GetRemoteType(nsACString& aRemoteType); \
  nsresult GetBrowserTabsRemoteAutostart(bool *aBrowserTabsRemoteAutostart); \
  nsresult GetMaxWebProcessCount(uint32_t *aMaxWebProcessCount); \
  nsresult GetAccessibilityEnabled(bool *aAccessibilityEnabled); \
  nsresult GetAccessibleHandlerUsed(bool *aAccessibleHandlerUsed); \
  nsresult GetAccessibilityInstantiator(nsAString& aAccessibilityInstantiator); \
  nsresult GetShouldBlockIncompatJaws(bool *aShouldBlockIncompatJaws); \
  nsresult GetIs64Bit(bool *aIs64Bit); \
  nsresult InvalidateCachesOnRestart(void); \
  nsresult EnsureContentProcess(void); \
  nsresult GetReplacedLockTime(PRTime *aReplacedLockTime); \
  nsresult GetIsReleaseOrBeta(bool *aIsReleaseOrBeta); \
  nsresult GetIsOfficialBranding(bool *aIsOfficialBranding); \
  nsresult GetDefaultUpdateChannel(nsACString& aDefaultUpdateChannel); \
  nsresult GetDistributionID(nsACString& aDistributionID); \
  nsresult GetWindowsDLLBlocklistStatus(bool *aWindowsDLLBlocklistStatus); \
  nsresult GetRestartedByOS(bool *aRestartedByOS); \
  nsresult GetProcessStartupShortcut(nsAString& aProcessStartupShortcut); \
  nsresult GetLauncherProcessState(uint32_t *aLauncherProcessState); \
  nsresult GetLastAppVersion(nsACString& aLastAppVersion); \
  nsresult GetLastAppBuildID(nsACString& aLastAppBuildID); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXULRUNTIME(_to) \
  NS_IMETHOD GetInSafeMode(bool *aInSafeMode) override { return _to GetInSafeMode(aInSafeMode); } \
  NS_IMETHOD GetFissionAutostart(bool *aFissionAutostart) override { return _to GetFissionAutostart(aFissionAutostart); } \
  NS_IMETHOD GetFissionExperimentStatus(nsIXULRuntime::ExperimentStatus *aFissionExperimentStatus) override { return _to GetFissionExperimentStatus(aFissionExperimentStatus); } \
  NS_IMETHOD GetFissionDecisionStatus(nsIXULRuntime::FissionDecisionStatus *aFissionDecisionStatus) override { return _to GetFissionDecisionStatus(aFissionDecisionStatus); } \
  NS_IMETHOD GetFissionDecisionStatusString(nsACString& aFissionDecisionStatusString) override { return _to GetFissionDecisionStatusString(aFissionDecisionStatusString); } \
  NS_IMETHOD GetSessionHistoryInParent(bool *aSessionHistoryInParent) override { return _to GetSessionHistoryInParent(aSessionHistoryInParent); } \
  NS_IMETHOD GetLogConsoleErrors(bool *aLogConsoleErrors) override { return _to GetLogConsoleErrors(aLogConsoleErrors); } \
  NS_IMETHOD SetLogConsoleErrors(bool aLogConsoleErrors) override { return _to SetLogConsoleErrors(aLogConsoleErrors); } \
  NS_IMETHOD GetOS(nsACString& aOS) override { return _to GetOS(aOS); } \
  NS_IMETHOD GetXPCOMABI(nsACString& aXPCOMABI) override { return _to GetXPCOMABI(aXPCOMABI); } \
  NS_IMETHOD GetWidgetToolkit(nsACString& aWidgetToolkit) override { return _to GetWidgetToolkit(aWidgetToolkit); } \
  NS_IMETHOD GetProcessType(uint32_t *aProcessType) override { return _to GetProcessType(aProcessType); } \
  NS_IMETHOD GetProcessID(uint32_t *aProcessID) override { return _to GetProcessID(aProcessID); } \
  NS_IMETHOD GetUniqueProcessID(uint64_t *aUniqueProcessID) override { return _to GetUniqueProcessID(aUniqueProcessID); } \
  NS_IMETHOD GetRemoteType(nsACString& aRemoteType) override { return _to GetRemoteType(aRemoteType); } \
  NS_IMETHOD GetBrowserTabsRemoteAutostart(bool *aBrowserTabsRemoteAutostart) override { return _to GetBrowserTabsRemoteAutostart(aBrowserTabsRemoteAutostart); } \
  NS_IMETHOD GetMaxWebProcessCount(uint32_t *aMaxWebProcessCount) override { return _to GetMaxWebProcessCount(aMaxWebProcessCount); } \
  NS_IMETHOD GetAccessibilityEnabled(bool *aAccessibilityEnabled) override { return _to GetAccessibilityEnabled(aAccessibilityEnabled); } \
  NS_IMETHOD GetAccessibleHandlerUsed(bool *aAccessibleHandlerUsed) override { return _to GetAccessibleHandlerUsed(aAccessibleHandlerUsed); } \
  NS_IMETHOD GetAccessibilityInstantiator(nsAString& aAccessibilityInstantiator) override { return _to GetAccessibilityInstantiator(aAccessibilityInstantiator); } \
  NS_IMETHOD GetShouldBlockIncompatJaws(bool *aShouldBlockIncompatJaws) override { return _to GetShouldBlockIncompatJaws(aShouldBlockIncompatJaws); } \
  NS_IMETHOD GetIs64Bit(bool *aIs64Bit) override { return _to GetIs64Bit(aIs64Bit); } \
  NS_IMETHOD InvalidateCachesOnRestart(void) override { return _to InvalidateCachesOnRestart(); } \
  NS_IMETHOD EnsureContentProcess(void) override { return _to EnsureContentProcess(); } \
  NS_IMETHOD GetReplacedLockTime(PRTime *aReplacedLockTime) override { return _to GetReplacedLockTime(aReplacedLockTime); } \
  NS_IMETHOD GetIsReleaseOrBeta(bool *aIsReleaseOrBeta) override { return _to GetIsReleaseOrBeta(aIsReleaseOrBeta); } \
  NS_IMETHOD GetIsOfficialBranding(bool *aIsOfficialBranding) override { return _to GetIsOfficialBranding(aIsOfficialBranding); } \
  NS_IMETHOD GetDefaultUpdateChannel(nsACString& aDefaultUpdateChannel) override { return _to GetDefaultUpdateChannel(aDefaultUpdateChannel); } \
  NS_IMETHOD GetDistributionID(nsACString& aDistributionID) override { return _to GetDistributionID(aDistributionID); } \
  NS_IMETHOD GetWindowsDLLBlocklistStatus(bool *aWindowsDLLBlocklistStatus) override { return _to GetWindowsDLLBlocklistStatus(aWindowsDLLBlocklistStatus); } \
  NS_IMETHOD GetRestartedByOS(bool *aRestartedByOS) override { return _to GetRestartedByOS(aRestartedByOS); } \
  NS_IMETHOD GetProcessStartupShortcut(nsAString& aProcessStartupShortcut) override { return _to GetProcessStartupShortcut(aProcessStartupShortcut); } \
  NS_IMETHOD GetLauncherProcessState(uint32_t *aLauncherProcessState) override { return _to GetLauncherProcessState(aLauncherProcessState); } \
  NS_IMETHOD GetLastAppVersion(nsACString& aLastAppVersion) override { return _to GetLastAppVersion(aLastAppVersion); } \
  NS_IMETHOD GetLastAppBuildID(nsACString& aLastAppBuildID) override { return _to GetLastAppBuildID(aLastAppBuildID); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXULRUNTIME(_to) \
  NS_IMETHOD GetInSafeMode(bool *aInSafeMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInSafeMode(aInSafeMode); } \
  NS_IMETHOD GetFissionAutostart(bool *aFissionAutostart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFissionAutostart(aFissionAutostart); } \
  NS_IMETHOD GetFissionExperimentStatus(nsIXULRuntime::ExperimentStatus *aFissionExperimentStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFissionExperimentStatus(aFissionExperimentStatus); } \
  NS_IMETHOD GetFissionDecisionStatus(nsIXULRuntime::FissionDecisionStatus *aFissionDecisionStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFissionDecisionStatus(aFissionDecisionStatus); } \
  NS_IMETHOD GetFissionDecisionStatusString(nsACString& aFissionDecisionStatusString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFissionDecisionStatusString(aFissionDecisionStatusString); } \
  NS_IMETHOD GetSessionHistoryInParent(bool *aSessionHistoryInParent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSessionHistoryInParent(aSessionHistoryInParent); } \
  NS_IMETHOD GetLogConsoleErrors(bool *aLogConsoleErrors) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLogConsoleErrors(aLogConsoleErrors); } \
  NS_IMETHOD SetLogConsoleErrors(bool aLogConsoleErrors) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLogConsoleErrors(aLogConsoleErrors); } \
  NS_IMETHOD GetOS(nsACString& aOS) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOS(aOS); } \
  NS_IMETHOD GetXPCOMABI(nsACString& aXPCOMABI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetXPCOMABI(aXPCOMABI); } \
  NS_IMETHOD GetWidgetToolkit(nsACString& aWidgetToolkit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWidgetToolkit(aWidgetToolkit); } \
  NS_IMETHOD GetProcessType(uint32_t *aProcessType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProcessType(aProcessType); } \
  NS_IMETHOD GetProcessID(uint32_t *aProcessID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProcessID(aProcessID); } \
  NS_IMETHOD GetUniqueProcessID(uint64_t *aUniqueProcessID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUniqueProcessID(aUniqueProcessID); } \
  NS_IMETHOD GetRemoteType(nsACString& aRemoteType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRemoteType(aRemoteType); } \
  NS_IMETHOD GetBrowserTabsRemoteAutostart(bool *aBrowserTabsRemoteAutostart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBrowserTabsRemoteAutostart(aBrowserTabsRemoteAutostart); } \
  NS_IMETHOD GetMaxWebProcessCount(uint32_t *aMaxWebProcessCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMaxWebProcessCount(aMaxWebProcessCount); } \
  NS_IMETHOD GetAccessibilityEnabled(bool *aAccessibilityEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAccessibilityEnabled(aAccessibilityEnabled); } \
  NS_IMETHOD GetAccessibleHandlerUsed(bool *aAccessibleHandlerUsed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAccessibleHandlerUsed(aAccessibleHandlerUsed); } \
  NS_IMETHOD GetAccessibilityInstantiator(nsAString& aAccessibilityInstantiator) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAccessibilityInstantiator(aAccessibilityInstantiator); } \
  NS_IMETHOD GetShouldBlockIncompatJaws(bool *aShouldBlockIncompatJaws) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetShouldBlockIncompatJaws(aShouldBlockIncompatJaws); } \
  NS_IMETHOD GetIs64Bit(bool *aIs64Bit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIs64Bit(aIs64Bit); } \
  NS_IMETHOD InvalidateCachesOnRestart(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InvalidateCachesOnRestart(); } \
  NS_IMETHOD EnsureContentProcess(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnsureContentProcess(); } \
  NS_IMETHOD GetReplacedLockTime(PRTime *aReplacedLockTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReplacedLockTime(aReplacedLockTime); } \
  NS_IMETHOD GetIsReleaseOrBeta(bool *aIsReleaseOrBeta) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsReleaseOrBeta(aIsReleaseOrBeta); } \
  NS_IMETHOD GetIsOfficialBranding(bool *aIsOfficialBranding) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsOfficialBranding(aIsOfficialBranding); } \
  NS_IMETHOD GetDefaultUpdateChannel(nsACString& aDefaultUpdateChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultUpdateChannel(aDefaultUpdateChannel); } \
  NS_IMETHOD GetDistributionID(nsACString& aDistributionID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDistributionID(aDistributionID); } \
  NS_IMETHOD GetWindowsDLLBlocklistStatus(bool *aWindowsDLLBlocklistStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWindowsDLLBlocklistStatus(aWindowsDLLBlocklistStatus); } \
  NS_IMETHOD GetRestartedByOS(bool *aRestartedByOS) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRestartedByOS(aRestartedByOS); } \
  NS_IMETHOD GetProcessStartupShortcut(nsAString& aProcessStartupShortcut) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProcessStartupShortcut(aProcessStartupShortcut); } \
  NS_IMETHOD GetLauncherProcessState(uint32_t *aLauncherProcessState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLauncherProcessState(aLauncherProcessState); } \
  NS_IMETHOD GetLastAppVersion(nsACString& aLastAppVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastAppVersion(aLastAppVersion); } \
  NS_IMETHOD GetLastAppBuildID(nsACString& aLastAppBuildID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastAppBuildID(aLastAppBuildID); } 


#endif /* __gen_nsIXULRuntime_h__ */
