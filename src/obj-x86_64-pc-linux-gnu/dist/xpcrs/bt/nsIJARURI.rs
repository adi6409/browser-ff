//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libjar/nsIJARURI.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIJARURI",
            base: Some("nsIURL"),
            methods: Ok(&[
                    /* readonly attribute nsIURI JARFile; */
                    Method {
                        name: "GetJARFile",
                        params: &[Param { name: "aJARFile", ty: "*mut *const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String JAREntry; */
                    Method {
                        name: "GetJAREntry",
                        params: &[Param { name: "aJAREntry", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIJARURIMutator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setSpecBaseCharset (in AUTF8String aSpec, in nsIURI aBase, in string aCharset); */
                    Method {
                        name: "SetSpecBaseCharset",
                        params: &[Param { name: "aSpec", ty: "*const ::nsstring::nsACString" }, Param { name: "aBase", ty: "*const nsIURI" }, Param { name: "aCharset", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

