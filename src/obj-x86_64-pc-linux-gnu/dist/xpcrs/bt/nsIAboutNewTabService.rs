//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/browser/components/newtab/nsIAboutNewTabService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAboutNewTabService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString defaultURL; */
                    Method {
                        name: "GetDefaultURL",
                        params: &[Param { name: "aDefaultURL", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIChannel aboutHomeChannel (in nsIURI aURI, in nsILoadInfo aLoadInfo); */
                    Method {
                        name: "AboutHomeChannel",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aLoadInfo", ty: "*const nsILoadInfo" }, Param { name: "_retval", ty: "*mut*const nsIChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString welcomeURL; */
                    Method {
                        name: "GetWelcomeURL",
                        params: &[Param { name: "aWelcomeURL", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

