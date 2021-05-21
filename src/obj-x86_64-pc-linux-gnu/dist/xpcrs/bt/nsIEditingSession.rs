//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/composer/nsIEditingSession.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEditingSession",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long editorStatus; */
                    Method {
                        name: "GetEditorStatus",
                        params: &[Param { name: "aEditorStatus", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void makeWindowEditable (in mozIDOMWindowProxy window, in string aEditorType, in boolean doAfterUriLoad, in boolean aMakeWholeDocumentEditable, in boolean aInteractive); */
                    Method {
                        name: "MakeWindowEditable",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }, Param { name: "aEditorType", ty: "*const libc::c_char" }, Param { name: "doAfterUriLoad", ty: "bool" }, Param { name: "aMakeWholeDocumentEditable", ty: "bool" }, Param { name: "aInteractive", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean windowIsEditable (in mozIDOMWindowProxy window); */
                    Method {
                        name: "WindowIsEditable",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIEditor getEditorForWindow (in mozIDOMWindowProxy window); */
                    Method {
                        name: "GetEditorForWindow",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }, Param { name: "_retval", ty: "*mut*const nsIEditor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void tearDownEditorOnWindow (in mozIDOMWindowProxy window); */
                    Method {
                        name: "TearDownEditorOnWindow",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

