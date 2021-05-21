//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/commandhandler/nsICommandManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICommandManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void addCommandObserver (in nsIObserver aCommandObserver, in string aCommandToObserve); */
                    Method {
                        name: "AddCommandObserver",
                        params: &[Param { name: "aCommandObserver", ty: "*const nsIObserver" }, Param { name: "aCommandToObserve", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeCommandObserver (in nsIObserver aCommandObserver, in string aCommandObserved); */
                    Method {
                        name: "RemoveCommandObserver",
                        params: &[Param { name: "aCommandObserver", ty: "*const nsIObserver" }, Param { name: "aCommandObserved", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isCommandSupported (in string aCommandName, in mozIDOMWindowProxy aTargetWindow); */
                    Method {
                        name: "IsCommandSupported",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aTargetWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isCommandEnabled (in string aCommandName, in mozIDOMWindowProxy aTargetWindow); */
                    Method {
                        name: "IsCommandEnabled",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aTargetWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getCommandState (in string aCommandName, in mozIDOMWindowProxy aTargetWindow, in nsICommandParams aCommandParams); */
                    Method {
                        name: "GetCommandState",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aTargetWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "aCommandParams", ty: "*const nsICommandParams" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void doCommand (in string aCommandName, in nsICommandParams aCommandParams, in mozIDOMWindowProxy aTargetWindow); */
                    Method {
                        name: "DoCommand",
                        params: &[Param { name: "aCommandName", ty: "*const libc::c_char" }, Param { name: "aCommandParams", ty: "*const nsICommandParams" }, Param { name: "aTargetWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

