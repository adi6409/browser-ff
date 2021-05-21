//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIIDNService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIDNService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* ACString convertUTF8toACE (in AUTF8String input); */
                    Method {
                        name: "ConvertUTF8toACE",
                        params: &[Param { name: "input", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String convertACEtoUTF8 (in ACString input); */
                    Method {
                        name: "ConvertACEtoUTF8",
                        params: &[Param { name: "input", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isACE (in ACString input); */
                    Method {
                        name: "IsACE",
                        params: &[Param { name: "input", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String normalize (in AUTF8String input); */
                    Method {
                        name: "Normalize",
                        params: &[Param { name: "input", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String convertToDisplayIDN (in AUTF8String input, out boolean isASCII); */
                    Method {
                        name: "ConvertToDisplayIDN",
                        params: &[Param { name: "input", ty: "*const ::nsstring::nsACString" }, Param { name: "isASCII", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

