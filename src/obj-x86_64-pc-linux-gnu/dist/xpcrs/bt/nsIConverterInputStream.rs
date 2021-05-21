//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIConverterInputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIConverterInputStream",
            base: Some("nsIUnicharInputStream"),
            methods: Ok(&[
                    /* void init (in nsIInputStream aStream, in string aCharset, in long aBufferSize, in char16_t aReplacementChar); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aCharset", ty: "*const libc::c_char" }, Param { name: "aBufferSize", ty: "i32" }, Param { name: "aReplacementChar", ty: "char16_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

