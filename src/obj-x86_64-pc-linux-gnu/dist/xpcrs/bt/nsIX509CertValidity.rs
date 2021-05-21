//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIX509CertValidity.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIX509CertValidity",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute PRTime notBefore; */
                    Method {
                        name: "GetNotBefore",
                        params: &[Param { name: "aNotBefore", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AString notBeforeLocalTime; */
                    Method {
                        name: "GetNotBeforeLocalTime",
                        params: &[Param { name: "aNotBeforeLocalTime", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString notBeforeLocalDay; */
                    Method {
                        name: "GetNotBeforeLocalDay",
                        params: &[Param { name: "aNotBeforeLocalDay", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AString notBeforeGMT; */
                    Method {
                        name: "GetNotBeforeGMT",
                        params: &[Param { name: "aNotBeforeGMT", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime notAfter; */
                    Method {
                        name: "GetNotAfter",
                        params: &[Param { name: "aNotAfter", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AString notAfterLocalTime; */
                    Method {
                        name: "GetNotAfterLocalTime",
                        params: &[Param { name: "aNotAfterLocalTime", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString notAfterLocalDay; */
                    Method {
                        name: "GetNotAfterLocalDay",
                        params: &[Param { name: "aNotAfterLocalDay", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AString notAfterGMT; */
                    Method {
                        name: "GetNotAfterGMT",
                        params: &[Param { name: "aNotAfterGMT", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

