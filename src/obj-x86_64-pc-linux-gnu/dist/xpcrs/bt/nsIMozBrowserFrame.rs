//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/html/nsIMozBrowserFrame.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMozBrowserFrame",
            base: Some("nsIDOMMozBrowserFrame"),
            methods: Ok(&[
                    /* [infallible] readonly attribute boolean reallyIsBrowser; */
                    Method {
                        name: "GetReallyIsBrowser",
                        params: &[Param { name: "aReallyIsBrowser", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void initializeBrowserAPI (); */
                    Method {
                        name: "InitializeBrowserAPI",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void destroyBrowserFrameScripts (); */
                    Method {
                        name: "DestroyBrowserFrameScripts",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

