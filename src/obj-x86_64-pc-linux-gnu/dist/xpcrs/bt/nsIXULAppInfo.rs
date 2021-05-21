//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIXULAppInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXULAppInfo",
            base: Some("nsIPlatformInfo"),
            methods: Ok(&[
                    /* readonly attribute ACString vendor; */
                    Method {
                        name: "GetVendor",
                        params: &[Param { name: "aVendor", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString ID; */
                    Method {
                        name: "GetID",
                        params: &[Param { name: "aID", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString version; */
                    Method {
                        name: "GetVersion",
                        params: &[Param { name: "aVersion", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString appBuildID; */
                    Method {
                        name: "GetAppBuildID",
                        params: &[Param { name: "aAppBuildID", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString UAName; */
                    Method {
                        name: "GetUAName",
                        params: &[Param { name: "aUAName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString sourceURL; */
                    Method {
                        name: "GetSourceURL",
                        params: &[Param { name: "aSourceURL", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString updateURL; */
                    Method {
                        name: "GetUpdateURL",
                        params: &[Param { name: "aUpdateURL", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

