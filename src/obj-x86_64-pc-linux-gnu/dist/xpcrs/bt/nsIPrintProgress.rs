//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/printing/nsIPrintProgress.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrintProgress",
            base: Some("nsIWebProgressListener"),
            methods: Ok(&[
                    /* void openProgressDialog (in mozIDOMWindowProxy parent, in string dialogURL, in nsISupports parameters, in nsIObserver openDialogObserver, out boolean notifyOnOpen); */
                    Method {
                        name: "OpenProgressDialog",
                        params: &[Param { name: "parent", ty: "*const mozIDOMWindowProxy" }, Param { name: "dialogURL", ty: "*const libc::c_char" }, Param { name: "parameters", ty: "*const nsISupports" }, Param { name: "openDialogObserver", ty: "*const nsIObserver" }, Param { name: "notifyOnOpen", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void closeProgressDialog (in boolean forceClose); */
                    Method {
                        name: "CloseProgressDialog",
                        params: &[Param { name: "forceClose", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerListener (in nsIWebProgressListener listener); */
                    Method {
                        name: "RegisterListener",
                        params: &[Param { name: "listener", ty: "*const nsIWebProgressListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unregisterListener (in nsIWebProgressListener listener); */
                    Method {
                        name: "UnregisterListener",
                        params: &[Param { name: "listener", ty: "*const nsIWebProgressListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void doneIniting (); */
                    Method {
                        name: "DoneIniting",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIPrompt getPrompter (); */
                    Method {
                        name: "GetPrompter",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIPrompt" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean processCanceledByUser; */
                    Method {
                        name: "GetProcessCanceledByUser",
                        params: &[Param { name: "aProcessCanceledByUser", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetProcessCanceledByUser",
                        params: &[Param { name: "aProcessCanceledByUser", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

