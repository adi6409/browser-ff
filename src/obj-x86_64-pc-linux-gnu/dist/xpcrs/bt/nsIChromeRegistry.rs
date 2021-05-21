//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/chrome/nsIChromeRegistry.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIChromeRegistry",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIURI convertChromeURL (in nsIURI aChromeURL); */
                    Method {
                        name: "ConvertChromeURL",
                        params: &[Param { name: "aChromeURL", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void checkForNewChrome (); */
                    Method {
                        name: "CheckForNewChrome",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIXULChromeRegistry",
            base: Some("nsIChromeRegistry"),
            methods: Ok(&[
                    /* boolean isLocaleRTL (in ACString package); */
                    Method {
                        name: "IsLocaleRTL",
                        params: &[Param { name: "package", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean allowScriptsForPackage (in nsIURI url); */
                    Method {
                        name: "AllowScriptsForPackage",
                        params: &[Param { name: "url", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean allowContentToAccess (in nsIURI url); */
                    Method {
                        name: "AllowContentToAccess",
                        params: &[Param { name: "url", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean canLoadURLRemotely (in nsIURI url); */
                    Method {
                        name: "CanLoadURLRemotely",
                        params: &[Param { name: "url", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean mustLoadURLRemotely (in nsIURI url); */
                    Method {
                        name: "MustLoadURLRemotely",
                        params: &[Param { name: "url", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

