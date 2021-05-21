//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIServiceWorkerManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIServiceWorkerUnregisterCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void unregisterSucceeded (in bool aState); */
                    Method {
                        name: "UnregisterSucceeded",
                        params: &[Param { name: "aState", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unregisterFailed (); */
                    Method {
                        name: "UnregisterFailed",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIServiceWorkerInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString id; */
                    Method {
                        name: "GetId",
                        params: &[Param { name: "aId", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString scriptSpec; */
                    Method {
                        name: "GetScriptSpec",
                        params: &[Param { name: "aScriptSpec", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString cacheName; */
                    Method {
                        name: "GetCacheName",
                        params: &[Param { name: "aCacheName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned short state; */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "aState", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIWorkerDebugger debugger; */
                    Method {
                        name: "GetDebugger",
                        params: &[Param { name: "aDebugger", ty: "*mut*const nsIWorkerDebugger" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute bool handlesFetchEvents; */
                    Method {
                        name: "GetHandlesFetchEvents",
                        params: &[Param { name: "aHandlesFetchEvents", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime installedTime; */
                    Method {
                        name: "GetInstalledTime",
                        params: &[Param { name: "aInstalledTime", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime activatedTime; */
                    Method {
                        name: "GetActivatedTime",
                        params: &[Param { name: "aActivatedTime", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime redundantTime; */
                    Method {
                        name: "GetRedundantTime",
                        params: &[Param { name: "aRedundantTime", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void attachDebugger (); */
                    Method {
                        name: "AttachDebugger",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void detachDebugger (); */
                    Method {
                        name: "DetachDebugger",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIServiceWorkerRegistrationInfoListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onChange (); */
                    Method {
                        name: "OnChange",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIServiceWorkerRegistrationInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "GetPrincipal",
                        params: &[Param { name: "aPrincipal", ty: "*mut*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString scope; */
                    Method {
                        name: "GetScope",
                        params: &[Param { name: "aScope", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString scriptSpec; */
                    Method {
                        name: "GetScriptSpec",
                        params: &[Param { name: "aScriptSpec", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned short updateViaCache; */
                    Method {
                        name: "GetUpdateViaCache",
                        params: &[Param { name: "aUpdateViaCache", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime lastUpdateTime; */
                    Method {
                        name: "GetLastUpdateTime",
                        params: &[Param { name: "aLastUpdateTime", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIServiceWorkerInfo evaluatingWorker; */
                    Method {
                        name: "GetEvaluatingWorker",
                        params: &[Param { name: "aEvaluatingWorker", ty: "*mut *const nsIServiceWorkerInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIServiceWorkerInfo installingWorker; */
                    Method {
                        name: "GetInstallingWorker",
                        params: &[Param { name: "aInstallingWorker", ty: "*mut *const nsIServiceWorkerInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIServiceWorkerInfo waitingWorker; */
                    Method {
                        name: "GetWaitingWorker",
                        params: &[Param { name: "aWaitingWorker", ty: "*mut *const nsIServiceWorkerInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIServiceWorkerInfo activeWorker; */
                    Method {
                        name: "GetActiveWorker",
                        params: &[Param { name: "aActiveWorker", ty: "*mut *const nsIServiceWorkerInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIServiceWorkerInfo getWorkerByID (in unsigned long long aID); */
                    Method {
                        name: "GetWorkerByID",
                        params: &[Param { name: "aID", ty: "u64" }, Param { name: "_retval", ty: "*mut *const nsIServiceWorkerInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addListener (in nsIServiceWorkerRegistrationInfoListener listener); */
                    Method {
                        name: "AddListener",
                        params: &[Param { name: "listener", ty: "*const nsIServiceWorkerRegistrationInfoListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeListener (in nsIServiceWorkerRegistrationInfoListener listener); */
                    Method {
                        name: "RemoveListener",
                        params: &[Param { name: "listener", ty: "*const nsIServiceWorkerRegistrationInfoListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void forceShutdown (); */
                    Method {
                        name: "ForceShutdown",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIServiceWorkerManagerListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onRegister (in nsIServiceWorkerRegistrationInfo aInfo); */
                    Method {
                        name: "OnRegister",
                        params: &[Param { name: "aInfo", ty: "*const nsIServiceWorkerRegistrationInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onUnregister (in nsIServiceWorkerRegistrationInfo aInfo); */
                    Method {
                        name: "OnUnregister",
                        params: &[Param { name: "aInfo", ty: "*const nsIServiceWorkerRegistrationInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIServiceWorkerManager",
            base: Some("nsISupports"),
            methods: Err("jscontext is unsupported"),
        },

        ]; D}

