//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleApplication.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleApplication",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString appName; */
                    Method {
                        name: "GetAppName",
                        params: &[Param { name: "aAppName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString appVersion; */
                    Method {
                        name: "GetAppVersion",
                        params: &[Param { name: "aAppVersion", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString platformName; */
                    Method {
                        name: "GetPlatformName",
                        params: &[Param { name: "aPlatformName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString platformVersion; */
                    Method {
                        name: "GetPlatformVersion",
                        params: &[Param { name: "aPlatformVersion", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

