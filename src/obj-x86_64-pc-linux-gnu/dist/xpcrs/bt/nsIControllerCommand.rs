//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/commandhandler/nsIControllerCommand.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIControllerCommand",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean isCommandEnabled (in string aCommandName, in nsISupports aCommandContext); */
                    Method {
                        name: "IsCommandEnabled",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aCommandContext", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getCommandStateParams (in string aCommandName, in nsICommandParams aParams, in nsISupports aCommandContext); */
                    Method {
                        name: "GetCommandStateParams",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aParams", ty: "*const nsICommandParams" }, Param { name: "aCommandContext", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void doCommand (in string aCommandName, in nsISupports aCommandContext); */
                    Method {
                        name: "DoCommand",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aCommandContext", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void doCommandParams (in string aCommandName, in nsICommandParams aParams, in nsISupports aCommandContext); */
                    Method {
                        name: "DoCommandParams",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aParams", ty: "*const nsICommandParams" }, Param { name: "aCommandContext", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

