//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsINSSVersion.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINSSVersion",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] readonly attribute AString NSPR_MinVersion; */
                    Method {
                        name: "GetNSPR_MinVersion",
                        params: &[Param { name: "aNSPR_MinVersion", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AString NSS_MinVersion; */
                    Method {
                        name: "GetNSS_MinVersion",
                        params: &[Param { name: "aNSS_MinVersion", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AString NSSUTIL_MinVersion; */
                    Method {
                        name: "GetNSSUTIL_MinVersion",
                        params: &[Param { name: "aNSSUTIL_MinVersion", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AString NSSSSL_MinVersion; */
                    Method {
                        name: "GetNSSSSL_MinVersion",
                        params: &[Param { name: "aNSSSSL_MinVersion", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AString NSSSMIME_MinVersion; */
                    Method {
                        name: "GetNSSSMIME_MinVersion",
                        params: &[Param { name: "aNSSSMIME_MinVersion", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AString NSPR_Version; */
                    Method {
                        name: "GetNSPR_Version",
                        params: &[Param { name: "aNSPR_Version", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AString NSS_Version; */
                    Method {
                        name: "GetNSS_Version",
                        params: &[Param { name: "aNSS_Version", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AString NSSUTIL_Version; */
                    Method {
                        name: "GetNSSUTIL_Version",
                        params: &[Param { name: "aNSSUTIL_Version", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AString NSSSSL_Version; */
                    Method {
                        name: "GetNSSSSL_Version",
                        params: &[Param { name: "aNSSSSL_Version", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AString NSSSMIME_Version; */
                    Method {
                        name: "GetNSSSMIME_Version",
                        params: &[Param { name: "aNSSSMIME_Version", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

