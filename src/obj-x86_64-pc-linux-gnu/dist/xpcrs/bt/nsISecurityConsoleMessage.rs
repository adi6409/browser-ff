//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsISecurityConsoleMessage.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISecurityConsoleMessage",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute AString tag; */
                    Method {
                        name: "GetTag",
                        params: &[Param { name: "aTag", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetTag",
                        params: &[Param { name: "aTag", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString category; */
                    Method {
                        name: "GetCategory",
                        params: &[Param { name: "aCategory", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCategory",
                        params: &[Param { name: "aCategory", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

