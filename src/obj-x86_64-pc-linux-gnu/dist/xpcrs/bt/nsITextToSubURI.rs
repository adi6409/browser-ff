//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/uconv/nsITextToSubURI.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITextToSubURI",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* ACString ConvertAndEscape (in ACString charset, in AString text); */
                    Method {
                        name: "ConvertAndEscape",
                        params: &[Param { name: "charset", ty: "*const ::nsstring::nsACString" }, Param { name: "text", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString UnEscapeAndConvert (in ACString charset, in ACString text); */
                    Method {
                        name: "UnEscapeAndConvert",
                        params: &[Param { name: "charset", ty: "*const ::nsstring::nsACString" }, Param { name: "text", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString unEscapeURIForUI (in AUTF8String aURIFragment); */
                    Method {
                        name: "UnEscapeURIForUI",
                        params: &[Param { name: "aURIFragment", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString unEscapeNonAsciiURI (in ACString aCharset, in AUTF8String aURIFragment); */
                    Method {
                        name: "UnEscapeNonAsciiURI",
                        params: &[Param { name: "aCharset", ty: "*const ::nsstring::nsACString" }, Param { name: "aURIFragment", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

