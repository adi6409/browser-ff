//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIHTMLInlineTableEditor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHTMLInlineTableEditor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [can_run_script] attribute boolean inlineTableEditingEnabled; */
                    Method {
                        name: "GetInlineTableEditingEnabled",
                        params: &[Param { name: "aInlineTableEditingEnabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetInlineTableEditingEnabled",
                        params: &[Param { name: "aInlineTableEditingEnabled", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void refreshInlineTableEditingUI (); */
                    Method {
                        name: "RefreshInlineTableEditingUI",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

