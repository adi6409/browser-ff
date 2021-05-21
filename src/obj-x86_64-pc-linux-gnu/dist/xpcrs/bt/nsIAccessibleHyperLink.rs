//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleHyperLink.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleHyperLink",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute long startIndex; */
                    Method {
                        name: "GetStartIndex",
                        params: &[Param { name: "aStartIndex", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long endIndex; */
                    Method {
                        name: "GetEndIndex",
                        params: &[Param { name: "aEndIndex", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean valid; */
                    Method {
                        name: "GetValid",
                        params: &[Param { name: "aValid", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long anchorCount; */
                    Method {
                        name: "GetAnchorCount",
                        params: &[Param { name: "aAnchorCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIURI getURI (in long index); */
                    Method {
                        name: "GetURI",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessible getAnchor (in long index); */
                    Method {
                        name: "GetAnchor",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut*const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

