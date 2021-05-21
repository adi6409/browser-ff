//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIMultiplexInputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMultiplexInputStream",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long count; */
                    Method {
                        name: "GetCount",
                        params: &[Param { name: "aCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void appendStream (in nsIInputStream stream); */
                    Method {
                        name: "AppendStream",
                        params: &[Param { name: "stream", ty: "*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIInputStream getStream (in unsigned long index); */
                    Method {
                        name: "GetStream",
                        params: &[Param { name: "index", ty: "u32" }, Param { name: "_retval", ty: "*mut *const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

