//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/xre/nsIXREDirProvider.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXREDirProvider",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setUserDataDirectory (in nsIFile aFile, in boolean aLocal); */
                    Method {
                        name: "SetUserDataDirectory",
                        params: &[Param { name: "aFile", ty: "*const nsIFile" }, Param { name: "aLocal", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getInstallHash (); */
                    Method {
                        name: "GetInstallHash",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

