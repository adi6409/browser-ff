//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIEditorMailSupport.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEditorMailSupport",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [can_run_script] Node insertAsCitedQuotation (in AString aQuotedText, in AString aCitation, in boolean aInsertHTML); */
                    Method {
                        name: "InsertAsCitedQuotation",
                        params: &[Param { name: "aQuotedText", ty: "*const ::nsstring::nsAString" }, Param { name: "aCitation", ty: "*const ::nsstring::nsAString" }, Param { name: "aInsertHTML", ty: "bool" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void rewrap (in boolean aRespectNewlines); */
                    Method {
                        name: "Rewrap",
                        params: &[Param { name: "aRespectNewlines", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

