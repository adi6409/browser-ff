//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULCommandDispatcher.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULCommandDispatcher",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [setter_can_run_script] attribute Element focusedElement; */
                    Method {
                        name: "GetFocusedElement",
                        params: &[Param { name: "aFocusedElement", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetFocusedElement",
                        params: &[Param { name: "aFocusedElement", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [setter_can_run_script] attribute mozIDOMWindowProxy focusedWindow; */
                    Method {
                        name: "GetFocusedWindow",
                        params: &[Param { name: "aFocusedWindow", ty: "*mut*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetFocusedWindow",
                        params: &[Param { name: "aFocusedWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addCommandUpdater (in Element updater, in AString events, in AString targets); */
                    Method {
                        name: "AddCommandUpdater",
                        params: &[Param { name: "updater", ty: "*const libc::c_void" }, Param { name: "events", ty: "*const ::nsstring::nsAString" }, Param { name: "targets", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeCommandUpdater (in Element updater); */
                    Method {
                        name: "RemoveCommandUpdater",
                        params: &[Param { name: "updater", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void updateCommands (in AString eventName); */
                    Method {
                        name: "UpdateCommands",
                        params: &[Param { name: "eventName", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIController getControllerForCommand (in string command); */
                    Method {
                        name: "GetControllerForCommand",
                        params: &[Param { name: "command", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut*const nsIController" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIControllers getControllers (); */
                    Method {
                        name: "GetControllers",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIControllers" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void advanceFocus (); */
                    Method {
                        name: "AdvanceFocus",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void rewindFocus (); */
                    Method {
                        name: "RewindFocus",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void advanceFocusIntoSubtree (in Element elt); */
                    Method {
                        name: "AdvanceFocusIntoSubtree",
                        params: &[Param { name: "elt", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void lock (); */
                    Method {
                        name: "Lock",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void unlock (); */
                    Method {
                        name: "Unlock",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

