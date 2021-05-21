//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleHyperText.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleHyperText",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute long linkCount; */
                    Method {
                        name: "GetLinkCount",
                        params: &[Param { name: "aLinkCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessibleHyperLink getLinkAt (in long index); */
                    Method {
                        name: "GetLinkAt",
                        params: &[Param { name: "index", ty: "i32" }, Param { name: "_retval", ty: "*mut *const nsIAccessibleHyperLink" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long getLinkIndex (in nsIAccessibleHyperLink link); */
                    Method {
                        name: "GetLinkIndex",
                        params: &[Param { name: "link", ty: "*const nsIAccessibleHyperLink" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long getLinkIndexAtOffset (in long offset); */
                    Method {
                        name: "GetLinkIndexAtOffset",
                        params: &[Param { name: "offset", ty: "i32" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

