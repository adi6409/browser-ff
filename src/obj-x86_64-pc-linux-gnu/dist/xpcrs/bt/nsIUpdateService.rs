//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/mozapps/update/nsIUpdateService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUpdatePatch",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString URL; */
                    Method {
                        name: "GetURL",
                        params: &[Param { name: "aURL", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString finalURL; */
                    Method {
                        name: "GetFinalURL",
                        params: &[Param { name: "aFinalURL", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetFinalURL",
                        params: &[Param { name: "aFinalURL", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long size; */
                    Method {
                        name: "GetSize",
                        params: &[Param { name: "aSize", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString state; */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "aState", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetState",
                        params: &[Param { name: "aState", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute long errorCode; */
                    Method {
                        name: "GetErrorCode",
                        params: &[Param { name: "aErrorCode", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetErrorCode",
                        params: &[Param { name: "aErrorCode", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean selected; */
                    Method {
                        name: "GetSelected",
                        params: &[Param { name: "aSelected", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSelected",
                        params: &[Param { name: "aSelected", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Element serialize (in Document updates); */
                    Method {
                        name: "Serialize",
                        params: &[Param { name: "updates", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUpdate",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString displayVersion; */
                    Method {
                        name: "GetDisplayVersion",
                        params: &[Param { name: "aDisplayVersion", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString appVersion; */
                    Method {
                        name: "GetAppVersion",
                        params: &[Param { name: "aAppVersion", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString previousAppVersion; */
                    Method {
                        name: "GetPreviousAppVersion",
                        params: &[Param { name: "aPreviousAppVersion", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString buildID; */
                    Method {
                        name: "GetBuildID",
                        params: &[Param { name: "aBuildID", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString detailsURL; */
                    Method {
                        name: "GetDetailsURL",
                        params: &[Param { name: "aDetailsURL", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString serviceURL; */
                    Method {
                        name: "GetServiceURL",
                        params: &[Param { name: "aServiceURL", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString channel; */
                    Method {
                        name: "GetChannel",
                        params: &[Param { name: "aChannel", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean unsupported; */
                    Method {
                        name: "GetUnsupported",
                        params: &[Param { name: "aUnsupported", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute long long promptWaitTime; */
                    Method {
                        name: "GetPromptWaitTime",
                        params: &[Param { name: "aPromptWaitTime", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPromptWaitTime",
                        params: &[Param { name: "aPromptWaitTime", ty: "i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean isCompleteUpdate; */
                    Method {
                        name: "GetIsCompleteUpdate",
                        params: &[Param { name: "aIsCompleteUpdate", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetIsCompleteUpdate",
                        params: &[Param { name: "aIsCompleteUpdate", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute long long installDate; */
                    Method {
                        name: "GetInstallDate",
                        params: &[Param { name: "aInstallDate", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetInstallDate",
                        params: &[Param { name: "aInstallDate", ty: "i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString statusText; */
                    Method {
                        name: "GetStatusText",
                        params: &[Param { name: "aStatusText", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetStatusText",
                        params: &[Param { name: "aStatusText", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIUpdatePatch selectedPatch; */
                    Method {
                        name: "GetSelectedPatch",
                        params: &[Param { name: "aSelectedPatch", ty: "*mut *const nsIUpdatePatch" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString state; */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "aState", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetState",
                        params: &[Param { name: "aState", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute long errorCode; */
                    Method {
                        name: "GetErrorCode",
                        params: &[Param { name: "aErrorCode", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetErrorCode",
                        params: &[Param { name: "aErrorCode", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean elevationFailure; */
                    Method {
                        name: "GetElevationFailure",
                        params: &[Param { name: "aElevationFailure", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetElevationFailure",
                        params: &[Param { name: "aElevationFailure", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long patchCount; */
                    Method {
                        name: "GetPatchCount",
                        params: &[Param { name: "aPatchCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIUpdatePatch getPatchAt (in unsigned long index); */
                    Method {
                        name: "GetPatchAt",
                        params: &[Param { name: "index", ty: "u32" }, Param { name: "_retval", ty: "*mut *const nsIUpdatePatch" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Element serialize (in Document updates); */
                    Method {
                        name: "Serialize",
                        params: &[Param { name: "updates", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUpdateCheckListener",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIUpdateChecker",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void checkForUpdates (in nsIUpdateCheckListener listener, in boolean force); */
                    Method {
                        name: "CheckForUpdates",
                        params: &[Param { name: "listener", ty: "*const nsIUpdateCheckListener" }, Param { name: "force", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void stopCurrentCheck (); */
                    Method {
                        name: "StopCurrentCheck",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIApplicationUpdateService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* bool checkForBackgroundUpdates (); */
                    Method {
                        name: "CheckForBackgroundUpdates",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIUpdateChecker backgroundChecker; */
                    Method {
                        name: "GetBackgroundChecker",
                        params: &[Param { name: "aBackgroundChecker", ty: "*mut *const nsIUpdateChecker" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIUpdate selectUpdate (in Array<nsIUpdate> updates); */
                    Method {
                        name: "SelectUpdate",
                        params: &[Param { name: "updates", ty: "*const thin_vec::ThinVec<RefPtr<nsIUpdate>>" }, Param { name: "_retval", ty: "*mut *const nsIUpdate" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addDownloadListener (in nsIRequestObserver listener); */
                    Method {
                        name: "AddDownloadListener",
                        params: &[Param { name: "listener", ty: "*const nsIRequestObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeDownloadListener (in nsIRequestObserver listener); */
                    Method {
                        name: "RemoveDownloadListener",
                        params: &[Param { name: "listener", ty: "*const nsIRequestObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool downloadUpdate (in nsIUpdate update, in boolean background); */
                    Method {
                        name: "DownloadUpdate",
                        params: &[Param { name: "update", ty: "*const nsIUpdate" }, Param { name: "background", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void stopDownload (); */
                    Method {
                        name: "StopDownload",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isDownloading; */
                    Method {
                        name: "GetIsDownloading",
                        params: &[Param { name: "aIsDownloading", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean canCheckForUpdates; */
                    Method {
                        name: "GetCanCheckForUpdates",
                        params: &[Param { name: "aCanCheckForUpdates", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean elevationRequired; */
                    Method {
                        name: "GetElevationRequired",
                        params: &[Param { name: "aElevationRequired", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean canApplyUpdates; */
                    Method {
                        name: "GetCanApplyUpdates",
                        params: &[Param { name: "aCanApplyUpdates", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isOtherInstanceHandlingUpdates; */
                    Method {
                        name: "GetIsOtherInstanceHandlingUpdates",
                        params: &[Param { name: "aIsOtherInstanceHandlingUpdates", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean canStageUpdates; */
                    Method {
                        name: "GetCanStageUpdates",
                        params: &[Param { name: "aCanStageUpdates", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean manualUpdateOnly; */
                    Method {
                        name: "GetManualUpdateOnly",
                        params: &[Param { name: "aManualUpdateOnly", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUpdateProcessor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void processUpdate (); */
                    Method {
                        name: "ProcessUpdate",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void fixUpdateDirectoryPerms (in boolean useServiceOnFailure); */
                    Method {
                        name: "FixUpdateDirectoryPerms",
                        params: &[Param { name: "useServiceOnFailure", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUpdateSyncManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* bool isOtherInstanceRunning (); */
                    Method {
                        name: "IsOtherInstanceRunning",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void resetLock (); */
                    Method {
                        name: "ResetLock",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUpdateManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIUpdate getUpdateAt (in long index); */
                    Method {
                        name: "GetUpdateAt",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut *const nsIUpdate" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long getUpdateCount (); */
                    Method {
                        name: "GetUpdateCount",
                        params: &[Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIUpdate readyUpdate; */
                    Method {
                        name: "GetReadyUpdate",
                        params: &[Param { name: "aReadyUpdate", ty: "*mut *const nsIUpdate" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetReadyUpdate",
                        params: &[Param { name: "aReadyUpdate", ty: "*const nsIUpdate" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIUpdate downloadingUpdate; */
                    Method {
                        name: "GetDownloadingUpdate",
                        params: &[Param { name: "aDownloadingUpdate", ty: "*mut *const nsIUpdate" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDownloadingUpdate",
                        params: &[Param { name: "aDownloadingUpdate", ty: "*const nsIUpdate" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addUpdateToHistory (in nsIUpdate update); */
                    Method {
                        name: "AddUpdateToHistory",
                        params: &[Param { name: "update", ty: "*const nsIUpdate" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void saveUpdates (); */
                    Method {
                        name: "SaveUpdates",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void refreshUpdateStatus (); */
                    Method {
                        name: "RefreshUpdateStatus",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void elevationOptedIn (); */
                    Method {
                        name: "ElevationOptedIn",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void cleanupDownloadingUpdate (); */
                    Method {
                        name: "CleanupDownloadingUpdate",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void cleanupReadyUpdate (); */
                    Method {
                        name: "CleanupReadyUpdate",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

