//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsILocalCertService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILocalCertService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void getOrCreateCert (in ACString nickname, in nsILocalCertGetCallback cb); */
                    Method {
                        name: "GetOrCreateCert",
                        params: &[Param { name: "nickname", ty: "*const ::nsstring::nsACString" }, Param { name: "cb", ty: "*const nsILocalCertGetCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void removeCert (in ACString nickname, in nsILocalCertCallback cb); */
                    Method {
                        name: "RemoveCert",
                        params: &[Param { name: "nickname", ty: "*const ::nsstring::nsACString" }, Param { name: "cb", ty: "*const nsILocalCertCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute boolean loginPromptRequired; */
                    Method {
                        name: "GetLoginPromptRequired",
                        params: &[Param { name: "aLoginPromptRequired", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsILocalCertGetCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleCert (in nsIX509Cert cert, in nsresult result); */
                    Method {
                        name: "HandleCert",
                        params: &[Param { name: "cert", ty: "*const nsIX509Cert" }, Param { name: "result", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsILocalCertCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleResult (in nsresult result); */
                    Method {
                        name: "HandleResult",
                        params: &[Param { name: "result", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

