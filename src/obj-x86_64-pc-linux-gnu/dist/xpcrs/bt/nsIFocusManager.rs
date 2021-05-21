//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIFocusManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFocusManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute mozIDOMWindowProxy activeWindow; */
                    Method {
                        name: "GetActiveWindow",
                        params: &[Param { name: "aActiveWindow", ty: "*mut*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute BrowsingContext activeBrowsingContext; */
                    Method {
                        name: "GetActiveBrowsingContext",
                        params: &[Param { name: "aActiveBrowsingContext", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute mozIDOMWindowProxy focusedWindow; */
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

                    /* readonly attribute BrowsingContext focusedContentBrowsingContext; */
                    Method {
                        name: "GetFocusedContentBrowsingContext",
                        params: &[Param { name: "aFocusedContentBrowsingContext", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Element focusedElement; */
                    Method {
                        name: "GetFocusedElement",
                        params: &[Param { name: "aFocusedElement", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* uint32_t getLastFocusMethod (in mozIDOMWindowProxy window); */
                    Method {
                        name: "GetLastFocusMethod",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void setFocus (in Element aElement, in unsigned long aFlags); */
                    Method {
                        name: "SetFocus",
                        params: &[Param { name: "aElement", ty: "*const libc::c_void" }, Param { name: "aFlags", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Element moveFocus (in mozIDOMWindowProxy aWindow, in Element aStartElement, in unsigned long aType, in unsigned long aFlags); */
                    Method {
                        name: "MoveFocus",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "aStartElement", ty: "*const libc::c_void" }, Param { name: "aType", ty: "u32" }, Param { name: "aFlags", ty: "u32" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearFocus (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "ClearFocus",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Element getFocusedElementForWindow (in mozIDOMWindowProxy aWindow, in boolean aDeep, out mozIDOMWindowProxy aFocusedWindow); */
                    Method {
                        name: "GetFocusedElementForWindow",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }, Param { name: "aDeep", ty: "bool" }, Param { name: "aFocusedWindow", ty: "*mut*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void moveCaretToFocus (in mozIDOMWindowProxy aWindow); */
                    Method {
                        name: "MoveCaretToFocus",
                        params: &[Param { name: "aWindow", ty: "*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean elementIsFocusable (in Element aElement, in unsigned long aFlags); */
                    Method {
                        name: "ElementIsFocusable",
                        params: &[Param { name: "aElement", ty: "*const libc::c_void" }, Param { name: "aFlags", ty: "u32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

