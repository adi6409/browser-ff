//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIHTMLObjectResizer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHTMLObjectResizer",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [can_run_script] attribute boolean objectResizingEnabled; */
                    Method {
                        name: "GetObjectResizingEnabled",
                        params: &[Param { name: "aObjectResizingEnabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetObjectResizingEnabled",
                        params: &[Param { name: "aObjectResizingEnabled", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void hideResizers (); */
                    Method {
                        name: "HideResizers",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void refreshResizers (); */
                    Method {
                        name: "RefreshResizers",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

