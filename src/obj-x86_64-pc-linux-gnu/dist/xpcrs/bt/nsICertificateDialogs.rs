//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICertificateDialogs.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICertificateDialogs",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] boolean confirmDownloadCACert (in nsIInterfaceRequestor ctx, in nsIX509Cert cert, out unsigned long trust); */
                    Method {
                        name: "ConfirmDownloadCACert",
                        params: &[Param { name: "ctx", ty: "*const nsIInterfaceRequestor" }, Param { name: "cert", ty: "*const nsIX509Cert" }, Param { name: "trust", ty: "*mut u32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] boolean setPKCS12FilePassword (in nsIInterfaceRequestor ctx, out AString password); */
                    Method {
                        name: "SetPKCS12FilePassword",
                        params: &[Param { name: "ctx", ty: "*const nsIInterfaceRequestor" }, Param { name: "password", ty: "*mut ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] boolean getPKCS12FilePassword (in nsIInterfaceRequestor ctx, out AString password); */
                    Method {
                        name: "GetPKCS12FilePassword",
                        params: &[Param { name: "ctx", ty: "*const nsIInterfaceRequestor" }, Param { name: "password", ty: "*mut ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

