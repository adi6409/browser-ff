//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/printing/nsIPrintProgressParams.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrintProgressParams",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute AString docTitle; */
                    Method {
                        name: "GetDocTitle",
                        params: &[Param { name: "aDocTitle", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDocTitle",
                        params: &[Param { name: "aDocTitle", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString docURL; */
                    Method {
                        name: "GetDocURL",
                        params: &[Param { name: "aDocURL", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDocURL",
                        params: &[Param { name: "aDocURL", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

