//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIPaperMargin.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPaperMargin",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [infallible] readonly attribute double top; */
                    Method {
                        name: "GetTop",
                        params: &[Param { name: "aTop", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute double right; */
                    Method {
                        name: "GetRight",
                        params: &[Param { name: "aRight", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute double bottom; */
                    Method {
                        name: "GetBottom",
                        params: &[Param { name: "aBottom", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute double left; */
                    Method {
                        name: "GetLeft",
                        params: &[Param { name: "aLeft", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

