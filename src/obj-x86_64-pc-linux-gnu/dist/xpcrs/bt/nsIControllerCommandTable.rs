//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/commandhandler/nsIControllerCommandTable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIControllerCommandTable",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void makeImmutable (); */
                    Method {
                        name: "MakeImmutable",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerCommand (in string aCommandName, in nsIControllerCommand aCommand); */
                    Method {
                        name: "RegisterCommand",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aCommand", ty: "*const nsIControllerCommand" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unregisterCommand (in string aCommandName, in nsIControllerCommand aCommand); */
                    Method {
                        name: "UnregisterCommand",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aCommand", ty: "*const nsIControllerCommand" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIControllerCommand findCommandHandler (in string aCommandName); */
                    Method {
                        name: "FindCommandHandler",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const nsIControllerCommand" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isCommandEnabled (in string aCommandName, in nsISupports aCommandRefCon); */
                    Method {
                        name: "IsCommandEnabled",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aCommandRefCon", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void updateCommandState (in string aCommandName, in nsISupports aCommandRefCon); */
                    Method {
                        name: "UpdateCommandState",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aCommandRefCon", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean supportsCommand (in string aCommandName, in nsISupports aCommandRefCon); */
                    Method {
                        name: "SupportsCommand",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aCommandRefCon", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void doCommand (in string aCommandName, in nsISupports aCommandRefCon); */
                    Method {
                        name: "DoCommand",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aCommandRefCon", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void doCommandParams (in string aCommandName, in nsICommandParams aParam, in nsISupports aCommandRefCon); */
                    Method {
                        name: "DoCommandParams",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aParam", ty: "*const nsICommandParams" }, Param { name: "aCommandRefCon", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getCommandState (in string aCommandName, in nsICommandParams aParam, in nsISupports aCommandRefCon); */
                    Method {
                        name: "GetCommandState",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aParam", ty: "*const nsICommandParams" }, Param { name: "aCommandRefCon", ty: "*const nsISupports" }],
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

