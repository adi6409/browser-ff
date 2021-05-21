//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowcreator/nsIWindowCreator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWindowCreator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIWebBrowserChrome createChromeWindow (in nsIWebBrowserChrome parent, in uint32_t chromeFlags, in nsIOpenWindowInfo aOpenWindowInfo, out boolean cancel); */
                    Method {
                        name: "CreateChromeWindow",
                        params: &[Param { name: "parent", ty: "*const nsIWebBrowserChrome" }, Param { name: "chromeFlags", ty: "uint32_t" }, Param { name: "aOpenWindowInfo", ty: "*const nsIOpenWindowInfo" }, Param { name: "cancel", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut*const nsIWebBrowserChrome" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

