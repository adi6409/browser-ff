//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsISeekableStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISeekableStream",
            base: Some("nsITellableStream"),
            methods: Ok(&[
                    /* void seek (in long whence, in long long offset); */
                    Method {
                        name: "Seek",
                        params: &[Param { name: "whence", ty: "i32" }, Param { name: "offset", ty: "i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setEOF (); */
                    Method {
                        name: "SetEOF",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

