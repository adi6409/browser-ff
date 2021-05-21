//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/xul/nsIController.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIController",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean isCommandEnabled (in string command); */
                    Method {
                        name: "IsCommandEnabled",
                        params: &[Param { name: "command", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean supportsCommand (in string command); */
                    Method {
                        name: "SupportsCommand",
                        params: &[Param { name: "command", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void doCommand (in string command); */
                    Method {
                        name: "DoCommand",
                        params: &[Param { name: "command", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onEvent (in string eventName); */
                    Method {
                        name: "OnEvent",
                        params: &[Param { name: "eventName", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICommandController",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void getCommandStateWithParams (in string command, in nsICommandParams aCommandParams); */
                    Method {
                        name: "GetCommandStateWithParams",
                        params: &[Param { name: "command", ty: "*const libc::c_char" }, Param { name: "aCommandParams", ty: "*const nsICommandParams" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void doCommandWithParams (in string command, in nsICommandParams aCommandParams); */
                    Method {
                        name: "DoCommandWithParams",
                        params: &[Param { name: "command", ty: "*const libc::c_char" }, Param { name: "aCommandParams", ty: "*const nsICommandParams" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<ACString> getSupportedCommands (); */
                    Method {
                        name: "GetSupportedCommands",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

