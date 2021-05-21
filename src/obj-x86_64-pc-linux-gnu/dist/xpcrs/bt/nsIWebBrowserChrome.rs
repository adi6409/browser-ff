//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIWebBrowserChrome.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebBrowserChrome",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setLinkStatus (in AString status); */
                    Method {
                        name: "SetLinkStatus",
                        params: &[Param { name: "status", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long chromeFlags; */
                    Method {
                        name: "GetChromeFlags",
                        params: &[Param { name: "aChromeFlags", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetChromeFlags",
                        params: &[Param { name: "aChromeFlags", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void showAsModal (); */
                    Method {
                        name: "ShowAsModal",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isWindowModal (); */
                    Method {
                        name: "IsWindowModal",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

