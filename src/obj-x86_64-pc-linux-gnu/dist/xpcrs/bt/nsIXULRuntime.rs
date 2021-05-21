//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIXULRuntime.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXULRuntime",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute boolean inSafeMode; */
                    Method {
                        name: "GetInSafeMode",
                        params: &[Param { name: "aInSafeMode", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean fissionAutostart; */
                    Method {
                        name: "GetFissionAutostart",
                        params: &[Param { name: "aFissionAutostart", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIXULRuntime_ExperimentStatus fissionExperimentStatus; */
                    Method {
                        name: "GetFissionExperimentStatus",
                        params: &[Param { name: "aFissionExperimentStatus", ty: "*mut u8" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIXULRuntime_FissionDecisionStatus fissionDecisionStatus; */
                    Method {
                        name: "GetFissionDecisionStatus",
                        params: &[Param { name: "aFissionDecisionStatus", ty: "*mut u8" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString fissionDecisionStatusString; */
                    Method {
                        name: "GetFissionDecisionStatusString",
                        params: &[Param { name: "aFissionDecisionStatusString", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean sessionHistoryInParent; */
                    Method {
                        name: "GetSessionHistoryInParent",
                        params: &[Param { name: "aSessionHistoryInParent", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean logConsoleErrors; */
                    Method {
                        name: "GetLogConsoleErrors",
                        params: &[Param { name: "aLogConsoleErrors", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetLogConsoleErrors",
                        params: &[Param { name: "aLogConsoleErrors", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String OS; */
                    Method {
                        name: "GetOS",
                        params: &[Param { name: "aOS", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String XPCOMABI; */
                    Method {
                        name: "GetXPCOMABI",
                        params: &[Param { name: "aXPCOMABI", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String widgetToolkit; */
                    Method {
                        name: "GetWidgetToolkit",
                        params: &[Param { name: "aWidgetToolkit", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long processType; */
                    Method {
                        name: "GetProcessType",
                        params: &[Param { name: "aProcessType", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long processID; */
                    Method {
                        name: "GetProcessID",
                        params: &[Param { name: "aProcessID", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint64_t uniqueProcessID; */
                    Method {
                        name: "GetUniqueProcessID",
                        params: &[Param { name: "aUniqueProcessID", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String remoteType; */
                    Method {
                        name: "GetRemoteType",
                        params: &[Param { name: "aRemoteType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean browserTabsRemoteAutostart; */
                    Method {
                        name: "GetBrowserTabsRemoteAutostart",
                        params: &[Param { name: "aBrowserTabsRemoteAutostart", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t maxWebProcessCount; */
                    Method {
                        name: "GetMaxWebProcessCount",
                        params: &[Param { name: "aMaxWebProcessCount", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean accessibilityEnabled; */
                    Method {
                        name: "GetAccessibilityEnabled",
                        params: &[Param { name: "aAccessibilityEnabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean accessibleHandlerUsed; */
                    Method {
                        name: "GetAccessibleHandlerUsed",
                        params: &[Param { name: "aAccessibleHandlerUsed", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString accessibilityInstantiator; */
                    Method {
                        name: "GetAccessibilityInstantiator",
                        params: &[Param { name: "aAccessibilityInstantiator", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean shouldBlockIncompatJaws; */
                    Method {
                        name: "GetShouldBlockIncompatJaws",
                        params: &[Param { name: "aShouldBlockIncompatJaws", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean is64Bit; */
                    Method {
                        name: "GetIs64Bit",
                        params: &[Param { name: "aIs64Bit", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void invalidateCachesOnRestart (); */
                    Method {
                        name: "InvalidateCachesOnRestart",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void ensureContentProcess (); */
                    Method {
                        name: "EnsureContentProcess",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime replacedLockTime; */
                    Method {
                        name: "GetReplacedLockTime",
                        params: &[Param { name: "aReplacedLockTime", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isReleaseOrBeta; */
                    Method {
                        name: "GetIsReleaseOrBeta",
                        params: &[Param { name: "aIsReleaseOrBeta", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isOfficialBranding; */
                    Method {
                        name: "GetIsOfficialBranding",
                        params: &[Param { name: "aIsOfficialBranding", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String defaultUpdateChannel; */
                    Method {
                        name: "GetDefaultUpdateChannel",
                        params: &[Param { name: "aDefaultUpdateChannel", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String distributionID; */
                    Method {
                        name: "GetDistributionID",
                        params: &[Param { name: "aDistributionID", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean windowsDLLBlocklistStatus; */
                    Method {
                        name: "GetWindowsDLLBlocklistStatus",
                        params: &[Param { name: "aWindowsDLLBlocklistStatus", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean restartedByOS; */
                    Method {
                        name: "GetRestartedByOS",
                        params: &[Param { name: "aRestartedByOS", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString processStartupShortcut; */
                    Method {
                        name: "GetProcessStartupShortcut",
                        params: &[Param { name: "aProcessStartupShortcut", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t launcherProcessState; */
                    Method {
                        name: "GetLauncherProcessState",
                        params: &[Param { name: "aLauncherProcessState", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString lastAppVersion; */
                    Method {
                        name: "GetLastAppVersion",
                        params: &[Param { name: "aLastAppVersion", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString lastAppBuildID; */
                    Method {
                        name: "GetLastAppBuildID",
                        params: &[Param { name: "aLastAppBuildID", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

