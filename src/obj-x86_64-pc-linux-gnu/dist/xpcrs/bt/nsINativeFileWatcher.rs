//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/filewatcher/nsINativeFileWatcher.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINativeFileWatcherErrorCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void complete (in nsresult xpcomError, in long osError); */
                    Method {
                        name: "Complete",
                        params: &[Param { name: "xpcomError", ty: "::nserror::nsresult" }, Param { name: "osError", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINativeFileWatcherCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void changed (in AString resourcePath, in int32_t flags); */
                    Method {
                        name: "Changed",
                        params: &[Param { name: "resourcePath", ty: "*const ::nsstring::nsAString" }, Param { name: "flags", ty: "int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINativeFileWatcherSuccessCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void complete (in AString resourcePath); */
                    Method {
                        name: "Complete",
                        params: &[Param { name: "resourcePath", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINativeFileWatcherService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void addPath (in AString pathToWatch, in nsINativeFileWatcherCallback onChange, [optional] in nsINativeFileWatcherErrorCallback onError, [optional] in nsINativeFileWatcherSuccessCallback onSuccess); */
                    Method {
                        name: "AddPath",
                        params: &[Param { name: "pathToWatch", ty: "*const ::nsstring::nsAString" }, Param { name: "onChange", ty: "*const nsINativeFileWatcherCallback" }, Param { name: "onError", ty: "*const nsINativeFileWatcherErrorCallback" }, Param { name: "onSuccess", ty: "*const nsINativeFileWatcherSuccessCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removePath (in AString pathToUnwatch, in nsINativeFileWatcherCallback onChange, [optional] in nsINativeFileWatcherErrorCallback onError, [optional] in nsINativeFileWatcherSuccessCallback onSuccess); */
                    Method {
                        name: "RemovePath",
                        params: &[Param { name: "pathToUnwatch", ty: "*const ::nsstring::nsAString" }, Param { name: "onChange", ty: "*const nsINativeFileWatcherCallback" }, Param { name: "onError", ty: "*const nsINativeFileWatcherErrorCallback" }, Param { name: "onSuccess", ty: "*const nsINativeFileWatcherSuccessCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

