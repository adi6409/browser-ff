//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIClientAuthDialogs.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClientAuthDialogs",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] boolean chooseCertificate (in AUTF8String hostname, in long port, in AUTF8String organization, in AUTF8String issuerOrg, in nsIArray certList, out unsigned long selectedIndex, out boolean rememberClientAuthCertificate); */
                    Method {
                        name: "ChooseCertificate",
                        params: &[Param { name: "hostname", ty: "*const ::nsstring::nsACString" }, Param { name: "port", ty: "i32" }, Param { name: "organization", ty: "*const ::nsstring::nsACString" }, Param { name: "issuerOrg", ty: "*const ::nsstring::nsACString" }, Param { name: "certList", ty: "*const nsIArray" }, Param { name: "selectedIndex", ty: "*mut u32" }, Param { name: "rememberClientAuthCertificate", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

