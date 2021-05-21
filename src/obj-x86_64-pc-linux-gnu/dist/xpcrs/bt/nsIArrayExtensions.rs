//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIArrayExtensions.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIArrayExtensions",
            base: Some("nsIArray"),
            methods: Ok(&[
                    /* uint32_t Count (); */
                    Method {
                        name: "Count",
                        params: &[Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISupports GetElementAt (in uint32_t index); */
                    Method {
                        name: "GetElementAt",
                        params: &[Param { name: "index", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

