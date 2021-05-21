//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsITooltipTextProvider.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITooltipTextProvider",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean getNodeText (in Node aNode, out wstring aText, out wstring aDirection); */
                    Method {
                        name: "GetNodeText",
                        params: &[Param { name: "aNode", ty: "*const libc::c_void" }, Param { name: "aText", ty: "*mut *const i16" }, Param { name: "aDirection", ty: "*mut *const i16" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

