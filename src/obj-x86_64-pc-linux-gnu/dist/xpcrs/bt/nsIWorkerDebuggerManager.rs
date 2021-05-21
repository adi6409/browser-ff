//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/workers/nsIWorkerDebuggerManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWorkerDebuggerManagerListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onRegister (in nsIWorkerDebugger debugger); */
                    Method {
                        name: "OnRegister",
                        params: &[Param { name: "debugger", ty: "*const nsIWorkerDebugger" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onUnregister (in nsIWorkerDebugger debugger); */
                    Method {
                        name: "OnUnregister",
                        params: &[Param { name: "debugger", ty: "*const nsIWorkerDebugger" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIWorkerDebuggerManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsISimpleEnumerator getWorkerDebuggerEnumerator (); */
                    Method {
                        name: "GetWorkerDebuggerEnumerator",
                        params: &[Param { name: "_retval", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addListener (in nsIWorkerDebuggerManagerListener listener); */
                    Method {
                        name: "AddListener",
                        params: &[Param { name: "listener", ty: "*const nsIWorkerDebuggerManagerListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeListener (in nsIWorkerDebuggerManagerListener listener); */
                    Method {
                        name: "RemoveListener",
                        params: &[Param { name: "listener", ty: "*const nsIWorkerDebuggerManagerListener" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

