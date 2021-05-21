//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIConsoleService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIConsoleService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void logMessage (in nsIConsoleMessage message); */
                    Method {
                        name: "LogMessage",
                        params: &[Param { name: "message", ty: "*const nsIConsoleMessage" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void logStringMessage (in wstring message); */
                    Method {
                        name: "LogStringMessage",
                        params: &[Param { name: "message", ty: "*const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<nsIConsoleMessage> getMessageArray (); */
                    Method {
                        name: "GetMessageArray",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<RefPtr<nsIConsoleMessage>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerListener (in nsIConsoleListener listener); */
                    Method {
                        name: "RegisterListener",
                        params: &[Param { name: "listener", ty: "*const nsIConsoleListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unregisterListener (in nsIConsoleListener listener); */
                    Method {
                        name: "UnregisterListener",
                        params: &[Param { name: "listener", ty: "*const nsIConsoleListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void reset (); */
                    Method {
                        name: "Reset",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void resetWindow (in uint64_t windowInnerId); */
                    Method {
                        name: "ResetWindow",
                        params: &[Param { name: "windowInnerId", ty: "uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

