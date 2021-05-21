//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsISlowScriptDebug.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISlowScriptDebugCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleSlowScriptDebug (in nsIDOMWindow aWindow); */
                    Method {
                        name: "HandleSlowScriptDebug",
                        params: &[Param { name: "aWindow", ty: "*const nsIDOMWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISlowScriptDebuggerStartupCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void finishDebuggerStartup (); */
                    Method {
                        name: "FinishDebuggerStartup",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISlowScriptDebugRemoteCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleSlowScriptDebug (in EventTarget aBrowser, in nsISlowScriptDebuggerStartupCallback aCallback); */
                    Method {
                        name: "HandleSlowScriptDebug",
                        params: &[Param { name: "aBrowser", ty: "*const libc::c_void" }, Param { name: "aCallback", ty: "*const nsISlowScriptDebuggerStartupCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISlowScriptDebug",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute nsISlowScriptDebugCallback activationHandler; */
                    Method {
                        name: "GetActivationHandler",
                        params: &[Param { name: "aActivationHandler", ty: "*mut *const nsISlowScriptDebugCallback" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetActivationHandler",
                        params: &[Param { name: "aActivationHandler", ty: "*const nsISlowScriptDebugCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsISlowScriptDebugRemoteCallback remoteActivationHandler; */
                    Method {
                        name: "GetRemoteActivationHandler",
                        params: &[Param { name: "aRemoteActivationHandler", ty: "*mut *const nsISlowScriptDebugRemoteCallback" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetRemoteActivationHandler",
                        params: &[Param { name: "aRemoteActivationHandler", ty: "*const nsISlowScriptDebugRemoteCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

