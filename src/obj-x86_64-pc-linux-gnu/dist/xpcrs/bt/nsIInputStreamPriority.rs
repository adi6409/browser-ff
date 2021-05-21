//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIInputStreamPriority.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInputStreamPriority",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute unsigned long priority; */
                    Method {
                        name: "GetPriority",
                        params: &[Param { name: "aPriority", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPriority",
                        params: &[Param { name: "aPriority", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

