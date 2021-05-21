//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIConverterOutputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIConverterOutputStream",
            base: Some("nsIUnicharOutputStream"),
            methods: Ok(&[
                    /* void init (in nsIOutputStream aOutStream, in string aCharset); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aOutStream", ty: "*const nsIOutputStream" }, Param { name: "aCharset", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

