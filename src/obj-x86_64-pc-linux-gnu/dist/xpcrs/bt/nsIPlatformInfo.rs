//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIPlatformInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPlatformInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString platformVersion; */
                    Method {
                        name: "GetPlatformVersion",
                        params: &[Param { name: "aPlatformVersion", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString platformBuildID; */
                    Method {
                        name: "GetPlatformBuildID",
                        params: &[Param { name: "aPlatformBuildID", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

